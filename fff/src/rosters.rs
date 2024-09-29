use sql_query_builder as sql;

use crate::safe_spice;

pub struct RosterQuery {
    query: sql::Select,
    year: u16,
}

const ROSTER_FIELDS: &str =
    "gsis_id AS id, full_name, team, position, birth_date, height, weight, week, status";

impl RosterQuery {
    pub fn new(year: u16) -> RosterQuery {
        let query = sql::Select::new()
            .select(ROSTER_FIELDS)
            .from(&format!("roster{year} r"));

        RosterQuery { query, year }
    }

    pub fn team(mut self, team: &str) -> Self {
        self.query = self
            .query
            .where_and(&format!("team = '{}'", safe_spice(team)));
        self
    }

    pub fn position(mut self, position: &str) -> Self {
        self.query = self.query.where_and(&format!(
            "position = '{}'",
            safe_spice(&position.to_uppercase())
        ));
        self
    }

    pub fn name_search(mut self, search: &str) -> Self {
        self.query = self
            .query
            .where_and(&format!("full_name ILIKE '%{}%'", safe_spice(search)));
        self
    }

    pub fn single_week(mut self, week: Option<u16>) -> Self {
        match week {
            Some(week) => self.query = self.query.where_and(&format!("week = {week}")),
            None => {
                let year = self.year;
                self.query = self.query.where_and(&format!(
                    "week = ( SELECT MAX(week) FROM roster{year} WHERE gsis_id = r.gsis_id )",
                ));
            }
        }
        self
    }

    pub fn week_range(mut self, start: u16, end: u16) -> Self {
        let clause = format!("week BETWEEN {start} AND {end}");
        self.query = self.query.where_and(&clause);
        self
    }

    pub fn query(self) -> sql::Select {
        self.query
    }

    pub fn sql(self) -> String {
        self.query.to_string()
    }
}
