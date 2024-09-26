## Melange

Melange is an experiment in using [spice.ai](https://spice.ai) to explore NFL data in the aim of developing Fantasy Football strategy.

> _The spice must flow._
>                 -Dune

`Fantasy + Spice = Melange`


## Status

Experimental (mainly to try out Spice.ai OSS). For now, Melange reimplements part of the pola.rs exploration from [fantasy-football-forecasting](https://github.com/anowell/fantasy-football-forecasting). Melange:

- uses a spicepod to manage datasets with years of play-by-play data
- exposes a CLI to explore the data
- surfaces play-by-play statistics that are used in fantasy scoring calculations


## Usage

Install spice (CLI or Docker container), then start spice.

```bash
# Local CLI
cd spiceai && spice run

# Or Docker (in theory?)
docker run --rm -it -p 8090:8090 -v $(pwd)/spiceai:/app spiceai/spiceai
```


Run melange:

```
$ cargo run -- --help
Usage: melange [OPTIONS]

Options:
  -v, --verbose...
  -y, --year <YEAR>      Loads data for a given year [default: 2024]
      --player <PLAYER>  Filter by team
  -w, --week <WEEKS>     Filtering week number or range (e.g. 3 or 3-5)
  -t, --team <TEAM>      Filter by team
  -h, --help             Print help
  -V, --version          Print version


# Show fantasy stats for Geno Smith in all 2024 games
$ cargo run -- --year 2024 --player 'G.Smith'
+-----------------+------------+------+------------+-------------+---------------+-----------------+---------------+-----------------+------------+-----------------+----------------------+--------------------+-------------------+---------------+-----------------+------------------+-----------------+
| game_id         | game_date  | team | player_id  | player_name | passing_yards | pass_touchdowns | interceptions | passing_50yd_td | receptions | receiving_yards | receiving_touchdowns | receiving_2pt_conv | receiving_50yd_td | rushing_yards | rush_touchdowns | rushing_2pt_conv | rushing_50yd_td |
+-----------------+------------+------+------------+-------------+---------------+-----------------+---------------+-----------------+------------+-----------------+----------------------+--------------------+-------------------+---------------+-----------------+------------------+-----------------+
| 2024_01_DEN_SEA | 2024-09-08 | SEA  | 00-0030565 | G.Smith     | 171.0         | 1.0             | 1.0           | 0.0             |            |                 |                      |                    |                   |               |                 |                  |                 |
| 2024_02_SEA_NE  | 2024-09-15 | SEA  | 00-0030565 | G.Smith     | 327.0         | 1.0             | 0.0           | 1.0             |            |                 |                      |                    |                   |               |                 |                  |                 |
| 2024_03_MIA_SEA | 2024-09-22 | SEA  | 00-0030565 | G.Smith     | 289.0         | 1.0             | 2.0           | 1.0             |            |                 |                      |                    |                   |               |                 |                  |                 |
+-----------------+------------+------+------------+-------------+---------------+-----------------+---------------+-----------------+------------+-----------------+----------------------+--------------------+-------------------+---------------+-----------------+------------------+-----------------+


# Show fantasy stats for Seahawks in 2024 week 3
$ cargo run -- --year 2024 --week 3 --team SEA
+-----------------+------------+------+------------+----------------+---------------+-----------------+---------------+-----------------+------------+-----------------+----------------------+--------------------+-------------------+---------------+-----------------+------------------+-----------------+
| game_id         | game_date  | team | player_id  | player_name    | passing_yards | pass_touchdowns | interceptions | passing_50yd_td | receptions | receiving_yards | receiving_touchdowns | receiving_2pt_conv | receiving_50yd_td | rushing_yards | rush_touchdowns | rushing_2pt_conv | rushing_50yd_td |
+-----------------+------------+------+------------+----------------+---------------+-----------------+---------------+-----------------+------------+-----------------+----------------------+--------------------+-------------------+---------------+-----------------+------------------+-----------------+
| 2024_03_MIA_SEA | 2024-09-22 | SEA  | 00-0030565 | G.Smith        | 289.0         | 1.0             | 2.0           | 1.0             |            |                 |                      |                    |                   | -2.0          | 0.0             | 0                | 0.0             |
| 2024_03_MIA_SEA | 2024-09-22 | SEA  | 00-0035640 | D.Metcalf      |               |                 |               |                 | 4.0        | 104.0           | 1.0                  | 0                  | 1.0               |               |                 |                  |                 |
| 2024_03_MIA_SEA | 2024-09-22 | SEA  | 00-0035644 | N.Fant         |               |                 |               |                 | 6.0        | 60.0            | 0.0                  | 0                  | 0.0               |               |                 |                  |                 |
| 2024_03_MIA_SEA | 2024-09-22 | SEA  | 00-0032211 | T.Lockett      |               |                 |               |                 | 5.0        | 46.0            | 0.0                  | 0                  | 0.0               |               |                 |                  |                 |
| 2024_03_MIA_SEA | 2024-09-22 | SEA  | 00-0038543 | J.Smith-Njigba |               |                 |               |                 | 3.0        | 39.0            | 0.0                  | 0                  | 0.0               |               |                 |                  |                 |
| 2024_03_MIA_SEA | 2024-09-22 | SEA  | 00-0039165 | Z.Charbonnet   |               |                 |               |                 | 3.0        | 16.0            | 0.0                  | 0                  | 0.0               |               |                 |                  |                 |
| 2024_03_MIA_SEA | 2024-09-22 | SEA  | 00-0039793 | A.Barner       |               |                 |               |                 | 3.0        | 13.0            | 0.0                  | 0                  | 0.0               |               |                 |                  |                 |
| 2024_03_MIA_SEA | 2024-09-22 | SEA  | 00-0033439 | P.Brown        |               |                 |               |                 | 1.0        | 9.0             | 0.0                  | 0                  | 0.0               |               |                 |                  |                 |
| 2024_03_MIA_SEA | 2024-09-22 | SEA  | 00-0036268 | L.Shenault     |               |                 |               |                 | 1.0        | 2.0             | 0.0                  | 0                  | 0.0               |               |                 |                  |                 |
| 2024_03_MIA_SEA | 2024-09-22 | SEA  | 00-0039165 | Z.Charbonnet   |               |                 |               |                 |            |                 |                      |                    |                   | 91.0          | 2.0             | 0                | 0.0             |
| 2024_03_MIA_SEA | 2024-09-22 | SEA  | 00-0038636 | K.McIntosh     |               |                 |               |                 |            |                 |                      |                    |                   | 11.0          | 0.0             | 0                | 0.0             |
+-----------------+------------+------+------------+----------------+---------------+-----------------+---------------+-----------------+------------+-----------------+----------------------+--------------------+-------------------+---------------+-----------------+------------------+-----------------+
```
