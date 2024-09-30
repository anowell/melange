use sql_query_builder as sql;

use crate::{rosters::RosterQuery, safe_spice};

pub struct PbpQuery {
    passing: sql::Select,
    receiving: sql::Select,
    rushing: sql::Select,
    roster: RosterQuery,
    limit: u16,
    join_roster: bool,
}

impl PbpQuery {
    /// Query to collect all the the passing stats
    pub fn passing(year: u16) -> sql::Select {
        

        sql::Select::new()
            .select(
                "
                game_id,
                MIN(game_date) as game_date,
                MIN(week) as week,
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
            .group_by("game_id, posteam, passer_player_id, passer_player_name")
    }

    /// Query to collect all the the receiving stats
    pub fn receiving(year: u16) -> sql::Select {
        
        sql::Select::new()
            .select("
                game_id,
                MIN(game_date) as game_date,
                MIN(week) as week,
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
            .group_by("game_id, posteam, receiver_player_id, receiver_player_name")
    }

    /// Query to collect all the the rushing stats
    pub fn rushing(year: u16) -> sql::Select {
        
        sql::Select::new()
            .select("
                game_id,
                MIN(game_date) as game_date,
                MIN(week) as week,
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
            .group_by("game_id, posteam, rusher_player_id, rusher_player_name")
    }

    pub fn year(year: u16) -> Self {
        let passing = Self::passing(year);
        let receiving = Self::receiving(year);
        let rushing = Self::rushing(year);
        let roster = RosterQuery::new(year);
        Self {
            passing,
            receiving,
            rushing,
            roster,
            limit: 500,
            join_roster: false,
        }
    }

    pub fn filter_player(mut self, player_name: &str) -> Self {
        self.roster = self.roster.name_search(player_name);
        self.join_roster = true;
        self

        // self.filter_passer(player_name)
        //     .filter_rusher(player_name)
        //     .filter_receiver(player_name)
    }

    // pub fn filter_passer(mut self, player_name: &str) -> Self {
    //     self.passing = self.passing.where_and(&format!(
    //         "passer_player_name = '{}'",
    //         safe_spice(player_name)
    //     ));
    //     self
    // }

    // pub fn filter_rusher(mut self, player_name: &str) -> Self {
    //     self.rushing = self.rushing.where_and(&format!(
    //         "receiver_player_name = '{}'",
    //         safe_spice(player_name)
    //     ));
    //     self
    // }

    // pub fn filter_receiver(mut self, player_name: &str) -> Self {
    //     self.receiving = self.receiving.where_and(&format!(
    //         "rusher_player_name = '{}'",
    //         safe_spice(player_name)
    //     ));
    //     self
    // }

    pub fn filter_player_id(self, player_id: &str) -> Self {
        self.filter_passer_id(player_id)
            .filter_rusher_id(player_id)
            .filter_receiver_id(player_id)
    }

    pub fn filter_passer_id(mut self, player_id: &str) -> Self {
        self.passing = self
            .passing
            .where_and(&format!("passer_player_id = '{}'", safe_spice(player_id)));
        self
    }

    pub fn filter_rusher_id(mut self, player_id: &str) -> Self {
        self.rushing = self
            .rushing
            .where_and(&format!("receiver_player_id = '{}'", safe_spice(player_id)));
        self
    }

    pub fn filter_receiver_id(mut self, player_id: &str) -> Self {
        self.receiving = self
            .receiving
            .where_and(&format!("rusher_player_id = '{}'", safe_spice(player_id)));
        self
    }

    pub fn filter_position(mut self, position: &str) -> Self {
        self.roster = self.roster.position(position);
        self.join_roster = true;
        self
    }

    fn where_and_each(mut self, clause: &str) -> Self {
        self.passing = self.passing.where_and(clause);
        self.receiving = self.receiving.where_and(clause);
        self.rushing = self.rushing.where_and(clause);
        self
    }

    pub fn filter_week(mut self, week: u16) -> Self {
        self.roster = self.roster.single_week(Some(week));
        let clause = format!("week = {week}");
        self.where_and_each(&clause)
    }

    pub fn filter_week_range(mut self, start: u16, end: u16) -> Self {
        self.roster = self.roster.week_range(start, end);
        let clause = format!("week BETWEEN {start} AND {end}");
        self.where_and_each(&clause)
    }

    pub fn filter_team(mut self, team: &str) -> Self {
        self.roster = self.roster.team(team);
        let clause = format!("posteam = '{}'", safe_spice(team));
        self.where_and_each(&clause)
    }

    pub fn join_roster(mut self) -> Self {
        self.join_roster = true;
        self
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = limit;
        self
    }

    /// Joins queries for passing, rushing, and receiving stats on a per-user, per-game basis
    pub fn sql(self) -> String {
        let coalesce = |field| format!("COALESCE(p.{0}, rx.{0}, r.{0}) AS {0}", field);

        let pbp_join = sql::Select::new()
            .select(&coalesce("game_id"))
            .select(&coalesce("game_date"))
            .select(&coalesce("week"))
            .select(&coalesce("team"))
            .select(&coalesce("player_id"))
            .select(&coalesce("player_name"))
            .select(
                "
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
            // Raw because this query builder doesn't have a helper for FULL/OUTER JOIN
            .raw_after(
                sql::SelectClause::From,
                "FULL JOIN receiving rx USING (player_id, game_id)",
            )
            .raw_after(
                sql::SelectClause::From,
                "FULL JOIN rushing r USING (player_id, game_id)",
            );

        let mut join = sql::Select::new()
            .with("passing", self.passing)
            .with("receiving", self.receiving)
            .with("rushing", self.rushing)
            .with("pbpjoin", pbp_join)
            .select("*")
            .from("pbpjoin")
            .order_by("game_date, passing_yards DESC NULLS LAST, receiving_yards DESC NULLS LAST, rushing_yards DESC NULLS LAST");

        if self.join_roster {
            join = join
                .with("rosters", self.roster.query())
                // Calculating age was harder than expected
                // Using cast to Int64 (seconds): https://github.com/apache/arrow-rs/blob/0a4d8a14b58e45ef92e31541f0b51a5b25de5f10/arrow-cast/src/cast/mod.rs#L275
                // After v43, consider `extract(days from duration) / 365.25`: https://github.com/apache/datafusion/pull/12514
                .select("ROUND(arrow_cast(pbpjoin.game_date::DATE - rosters.birth_date::DATE, 'Int64') / (60*60*24*365.25), 1) AS age")
                .inner_join(
                    "rosters ON pbpjoin.player_id = rosters.id AND pbpjoin.week = rosters.week",
                );
        }

        join.limit(&self.limit.to_string()).as_string()
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
