use anyhow::{anyhow, Result};
use arrow::util::pretty::print_batches;
use futures::TryStreamExt;
use spiceai::ClientBuilder;
use sql_query_builder as sql;

pub struct PbpQuery {
    passing: sql::Select,
    receiving: sql::Select,
    rushing: sql::Select,
}

// This would NOT be acceptable to avoid SQL injection (use parameterized queries)
// but we're assuming the Spice API prevents any meaningful SQL injection
// so this just handles input parameters a bit more sanely
fn safe_spice(input: &str) -> String {
    // Step 1: Remove control characters
    let sanitized: String = input
        .chars()
        .filter(|c| !c.is_control())
        .map(|c| match c {
            '\'' => "''".to_string(),   // Escape single quotes
            '\\' => "\\\\".to_string(), // Escape backslashes
            _ => c.to_string(),
        })
        .collect();

    sanitized
}

impl PbpQuery {
    pub fn passing(year: u16) -> sql::Select {
        let query = sql::Select::new()
            .select(
                "
                game_id,
                MIN(game_date) as game_date,
                posteam as team,
                passer_player_id as player_id,
                passer_player_name as player_name,
                SUM(passing_yards) as passing_yards,
                SUM(pass_touchdown) as pass_touchdowns,
                SUM(interception) as interceptions,
                SUM(CASE WHEN passing_yards > 50 THEN pass_touchdown ELSE 0 END) as passing_50yd_td
            ",
            )
            .from(&format!("pbp{}", year))
            .where_clause("passer_player_name IS NOT NULL")
            .group_by("game_id, posteam, passer_player_id, passer_player_name");

        query
    }

    pub fn receiving(year: u16) -> sql::Select {
        let query = sql::Select::new()
            .select("
                game_id,
                MIN(game_date) as game_date,
                posteam as team,
                receiver_player_id as player_id,
                receiver_player_name as player_name,
                SUM(complete_pass) as receptions,
                SUM(receiving_yards) as receiving_yards,
                SUM(pass_touchdown) as receiving_touchdowns,
                SUM(CASE WHEN two_point_conv_result = 'success' THEN 1 ELSE 0 END) as receiving_2pt_conv,
                SUM(CASE WHEN receiving_yards > 50 THEN pass_touchdown ELSE 0 END) as receiving_50yd_td
            ")
            .from(&format!("pbp{}", year))
            .where_clause("receiver_player_name IS NOT NULL")
            .group_by("game_id, posteam, receiver_player_id, receiver_player_name");
        query
    }

    pub fn rushing(year: u16) -> sql::Select {
        let query = sql::Select::new()
            .select("
                game_id,
                MIN(game_date) as game_date,
                posteam as team,
                rusher_player_id as player_id,
                rusher_player_name as player_name,
                SUM(rushing_yards) as rushing_yards,
                SUM(rush_touchdown) as rush_touchdowns,
                SUM(CASE WHEN two_point_conv_result = 'success' THEN 1 ELSE 0 END) as rushing_2pt_conv,
                SUM(CASE WHEN rushing_yards > 50 THEN rush_touchdown ELSE 0 END) as rushing_50yd_td
            ")
            .from(&format!("pbp{}", year))
            .where_clause("rusher_player_name IS NOT NULL")
            .group_by("game_id, posteam, rusher_player_id, rusher_player_name");
        query
    }

    pub fn year(year: u16) -> Self {
        let passing = Self::passing(year);
        let receiving = Self::receiving(year);
        let rushing = Self::rushing(year);
        Self {
            passing,
            receiving,
            rushing,
        }
    }

    pub fn filter_player(self, player_name: &str) -> Self {
        self.filter_passer(player_name)
            .filter_rusher(player_name)
            .filter_receiver(player_name)
    }

    pub fn filter_passer(mut self, player_name: &str) -> Self {
        self.passing = self.passing.where_and(&format!(
            "passer_player_name = '{}'",
            safe_spice(player_name)
        ));
        self
    }

    pub fn filter_rusher(mut self, player_name: &str) -> Self {
        self.rushing = self.rushing.where_and(&format!(
            "receiver_player_name = '{}'",
            safe_spice(player_name)
        ));
        self
    }

    pub fn filter_receiver(mut self, player_name: &str) -> Self {
        self.receiving = self.receiving.where_and(&format!(
            "rusher_player_name = '{}'",
            safe_spice(player_name)
        ));
        self
    }

    fn where_and_each(mut self, clause: &str) -> Self {
        self.passing = self.passing.where_and(&clause);
        self.receiving = self.receiving.where_and(&clause);
        self.rushing = self.rushing.where_and(&clause);
        self
    }

    pub fn filter_week(self, week: u16) -> Self {
        let clause = format!("week = {week}");
        self.where_and_each(&clause)
    }

    pub fn filter_week_range(self, start: u16, end: u16) -> Self {
        let clause = format!("week BETWEEN {start} AND {end}");
        self.where_and_each(&clause)
    }

    pub fn filter_team(self, team: &str) -> Self {
        let clause = format!("posteam = '{}'", safe_spice(team));
        self.where_and_each(&clause)
    }

    pub fn sql(self) -> String {
        let join = sql::Select::new()
            .with("passing", self.passing)
            .with("receiving", self.receiving)
            .with("rushing", self.rushing)
            // .select("*")
            .select(
                "
                COALESCE(p.game_id, rx.game_id, r.game_id) AS game_id,
                COALESCE(p.game_date, rx.game_date, r.game_date) AS game_date,
                COALESCE(p.team, rx.team, r.team) AS team,
                COALESCE(p.player_id, rx.player_id, r.player_id) AS player_id,
                COALESCE(p.player_name, rx.player_name, r.player_name) AS player_name,
                p.passing_yards,
                p.pass_touchdowns,
                p.interceptions,
                p.passing_50yd_td,
                rx.receptions,
                rx.receiving_yards,
                rx.receiving_touchdowns,
                rx.receiving_2pt_conv,
                rx.receiving_50yd_td,
                r.rushing_yards,
                r.rush_touchdowns,
                r.rushing_2pt_conv,
                r.rushing_50yd_td
            ",
            )
            .from("passing p")
            .raw_after(
                sql::SelectClause::From,
                "FULL JOIN receiving rx USING (player_id, game_id)",
            )
            .raw_after(
                sql::SelectClause::From,
                "FULL JOIN rushing r USING (player_id, game_id)",
            )
            .order_by("game_date, passing_yards DESC NULLS LAST, receiving_yards DESC NULLS LAST, rushing_yards DESC NULLS LAST")
            .limit("100");
        join.as_string()
    }
}

// static FUMBLING_QUERY: &str = r#"
//     SELECT
//         game_id,
//         FIRST(game_date),
//         posteam as team,
//         fumbled_1_player_id as player_id,
//         fumbled_1_player_name as player_name,
//         SUM(fumble_lost) as fumbles_lost
//     FROM pbp2024
//     WHERE fumbled_1_player_name IS NOT NULL
//     GROUP BY game_id, posteam, fumbled_1_player_id, fumbled_1_player_name
// "#;

// static KICKING_QUERY: &str = r#"
//     SELECT
//         game_id,
//         FIRST(game_date),
//         posteam as team,
//         kicker_player_id as player_id,
//         kicker_player_name as player_name,
//         SUM(CASE WHEN extra_point_result = 'good' THEN 1 ELSE 0 END) as pat_made,
//         SUM(CASE WHEN field_goal_result = 'made' THEN 1 ELSE 0 END) as fg_made,
//         SUM(CASE WHEN kick_distance >= 40 AND field_goal_result = 'made' THEN 1 ELSE 0 END) as fg_40plus_made,
//         SUM(CASE WHEN kick_distance >= 50 AND field_goal_result = 'made' THEN 1 ELSE 0 END) as fg_50plus_made
//     FROM pbp2024
//     WHERE play_type != 'kickoff' AND kicker_player_name IS NOT NULL
//     GROUP BY game_id, posteam, kicker_player_id, kicker_player_name
// "#;

// static RETURNING_QUERY: &str = r#"
//     SELECT
//         game_id,
//         FIRST(game_date),
//         team,
//         player_id,
//         player_name,
//         SUM(return_touchdown) as td_returns
//     FROM (
//         SELECT
//             game_id,
//             FIRST(game_date),
//             posteam as team,
//             COALESCE(
//                 lateral_kickoff_returner_player_id,
//                 lateral_punt_returner_player_id,
//                 kickoff_returner_player_id,
//                 punt_returner_player_id
//             ) as player_id,
//             COALESCE(
//                 lateral_kickoff_returner_player_name,
//                 lateral_punt_returner_player_name,
//                 kickoff_returner_player_name,
//                 punt_returner_player_name
//             ) as player_name,
//             return_touchdown
//         FROM pbp2024
//         WHERE return_touchdown = 1.0
//     ) as coalesced_players
//     GROUP BY game_id, team, player_id, player_name
// "#;
