use crate::{serde_utils, Ctx, Result};
use anyhow::{anyhow, Context};
use arrow::{array::RecordBatch, json::ArrayWriter};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use fff::{pbp::PbpQuery, rosters::RosterQuery};
use futures::TryStreamExt;
use itertools::Itertools;
use serde::Deserialize;
use serde_json::{Map, Value};
use tokio::sync::Mutex;

type JsonRecords = Vec<Map<String, Value>>;

pub(crate) fn router() -> Router<Ctx> {
    Router::new()
        .route("/v1/stats", get(get_stats))
        .route("/v1/players", get(search_players))
        .route("/v1/chat", post(post_chat))
}

#[derive(Copy, Clone, Debug)]
enum Weeks {
    Week(u16),
    WeekRange(u16, u16),
}

impl std::str::FromStr for Weeks {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<u16> = s.split('-').map(str::parse).try_collect()?;
        match &*parts {
            [single] => Ok(Weeks::Week(*single)),
            [start, end] if start <= end => Ok(Weeks::WeekRange(*start, *end)),
            _ => Err(s.parse::<u16>().unwrap_err()),
        }
    }
}

#[derive(Debug, Deserialize)]
struct SearchPlayersParams {
    search: String,
    year: Option<u16>,
    week: Option<u16>,
}

impl SearchPlayersParams {
    fn make_query(&self) -> String {
        let year = self.year.unwrap_or(2024);
        let query = RosterQuery::new(year)
            .name_search(&self.search)
            .single_week(self.week.clone());
        query.sql()
    }
}

#[axum::debug_handler]
async fn search_players(
    State(ctx): State<Ctx>,
    params: Query<SearchPlayersParams>,
) -> Result<Json<Vec<Map<String, Value>>>> {
    let query = params.make_query();
    query_spice(&ctx.spice, &query).await.map(Json)
}

#[derive(Debug, Deserialize)]
struct GetStatsParams {
    year: Option<u16>,
    player: Option<String>,
    position: Option<String>,

    #[serde(with = "serde_utils::string_opt", default)]
    weeks: Option<Weeks>,
    team: Option<String>,
}

impl GetStatsParams {
    fn make_query(&self) -> String {
        tracing::trace!("GetStatsParams {:?}", self);
        let mut query_builder = PbpQuery::year(self.year.unwrap_or(2024)).join_roster();

        if let Some(player_name) = &self.player {
            query_builder = query_builder.filter_player(&player_name);
        }

        if let Some(team) = &self.team {
            query_builder = query_builder.filter_team(&team);
        }

        if let Some(position) = &self.position {
            query_builder = query_builder.filter_position(&position);
        }

        match self.weeks {
            Some(Weeks::Week(week)) => {
                query_builder = query_builder.filter_week(week);
            }
            Some(Weeks::WeekRange(start, end)) => {
                query_builder = query_builder.filter_week_range(start, end);
            }
            None => {}
        }

        let query = query_builder.sql();
        query
    }
}

async fn get_stats(
    State(ctx): State<Ctx>,
    params: Query<GetStatsParams>,
) -> Result<Json<Vec<Map<String, Value>>>> {
    let query = params.make_query();
    query_spice(&ctx.spice, &query).await.map(Json)
}

async fn query_spice(spice: &Mutex<spiceai::Client>, query: &str) -> Result<JsonRecords> {
    tracing::info!("query: {query}");

    let mut spice = spice.lock().await;
    let flight = spice
        .query(&query)
        .await
        .map_err(|err| anyhow!("Query error: {}", err))?;

    tracing::info!("collecting query results");
    let batches: Vec<_> = flight
        .try_collect()
        .await
        .context("collecting query results")?;

    tracing::info!("converting {} record batcheds to json", batches.len());
    let json_resp = record_batches_to_json(batches, 500);
    Ok(json_resp)
}

async fn post_chat() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

fn record_batches_to_json(record_batches: Vec<RecordBatch>, max_records: usize) -> JsonRecords {
    if record_batches.is_empty() {
        return Vec::new();
    }

    let buf = Vec::new();
    let mut writer = ArrayWriter::new(buf);
    let mut total_rows_written = 0;

    for batch in record_batches.iter() {
        let num_rows_in_batch = batch.num_rows();
        // if total_rows_written + num_rows_in_batch > max_records {
        //     // If the current batch exceeds the limit, only write the remaining rows
        //     let remaining_rows = max_records - total_rows_written;
        //     let sliced_batch = batch.slice(0, remaining_rows);
        //     writer.write_batches(&[&sliced_batch]).unwrap();
        //     total_rows_written += remaining_rows;
        // } else {
        writer.write_batches(&[batch]).unwrap();
        total_rows_written += num_rows_in_batch;
        // }

        if total_rows_written >= max_records {
            break;
        }
    }

    writer.finish().unwrap();
    let json_data = writer.into_inner();

    // Convert the resulting buffer into JSON
    let json_rows = serde_json::from_reader(json_data.as_slice()).expect("arrow json writer");
    json_rows
}
