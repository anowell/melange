use anyhow::{anyhow, Result};
use arrow::util::pretty::print_batches;
use clap::Parser;
use fff::pbp::PbpQuery;
use futures::TryStreamExt;
use itertools::Itertools;
use spiceai::ClientBuilder;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Loads data for a given year
    #[arg(short = 'y', long = "year", default_value_t = 2024)]
    year: u16,

    /// Filter by team
    #[arg(long = "player")]
    player: Option<String>,

    /// Filtering week number or range (e.g. 3 or 3-5)
    #[arg(short = 'w', long = "week", alias = "weeks")]
    weeks: Option<WeekArg>,

    /// Filter by team
    #[arg(short = 't', long = "team")]
    team: Option<String>,
    // TODO: position filtering requires cross-referencing roster info
    // https://github.com/nflverse/nflverse-data/releases/tag/weekly_rosters
    // Filter by position
    // #[arg(short = 'p', long = "pos", value_enum)]
    // position: Option<Position>,
}

#[derive(Copy, Clone, Debug)]
enum WeekArg {
    Week(u16),
    WeekRange(u16, u16),
}

impl std::str::FromStr for WeekArg {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<u16> = s.split('-').map(str::parse).try_collect()?;
        match &*parts {
            [single] => Ok(WeekArg::Week(*single)),
            [start, end] if start <= end => Ok(WeekArg::WeekRange(*start, *end)),
            _ => Err(s.parse::<u16>().unwrap_err()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut client = ClientBuilder::new()
        .flight_url("http://localhost:50051")
        .build()
        .await
        .unwrap();

    let mut query_builder = PbpQuery::year(args.year);

    if let Some(player_name) = args.player {
        query_builder = query_builder.filter_player(&player_name);
    }

    if let Some(team) = args.team {
        query_builder = query_builder.filter_team(&team);
    }

    match args.weeks {
        Some(WeekArg::Week(week)) => {
            query_builder = query_builder.filter_week(week);
        }
        Some(WeekArg::WeekRange(start, end)) => {
            query_builder = query_builder.filter_week_range(start, end);
        }
        None => {}
    }

    let query = query_builder.sql();
    if args.verbose > 0 {
        println!("query: {query}");
    }

    let flight = client
        .query(&query)
        .await
        .map_err(|err| anyhow!("Query error: {}", err))?;

    let batches: Vec<_> = flight.try_collect().await?;
    print_batches(&batches)?;
    Ok(())
}
