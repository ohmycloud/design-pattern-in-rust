use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, line_ending, space0, space1},
    combinator::{map, map_res},
    multi::{many0, many1},
    sequence::{pair, preceded, terminated},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug)]
struct StationData {
    info: HashMap<String, String>,
    data: Vec<(i32, Vec<f64>)>,
}

fn parse_key_value(input: &str) -> IResult<&str, (String, String)> {
    let (input, key) = terminated(take_until("="), tag("="))(input)?;
    let (input, value) = preceded(space0, take_until("\n"))(input)?;
    Ok((input, (key.trim().to_string(), value.trim().to_string())))
}

fn parse_temperature(input: &str) -> IResult<&str, f64> {
    map_res(
        preceded(
            space0,
            pair(
                nom::branch::alt((tag("-"), tag(""))),
                pair(digit1, preceded(tag("."), digit1)),
            ),
        ),
        |(sign, (int_part, frac_part))| {
            format!("{}{}.{}", sign, int_part, frac_part).parse::<f64>()
        },
    )(input)
}

fn parse_observation(input: &str) -> IResult<&str, (i32, Vec<f64>)> {
    let (input, year) = map_res(terminated(digit1, space1), str::parse)(input)?;
    let (input, temps) = many1(parse_temperature)(input)?;
    let temps: Vec<f64> = temps.into_iter().filter(|&t| t > -99.0).collect();
    Ok((input, (year, temps)))
}

fn parse_station_data(input: &str) -> IResult<&str, StationData> {
    let (input, info) = many0(terminated(parse_key_value, line_ending))(input)?;
    let (input, _) = pair(tag("Obs:"), line_ending)(input)?;
    let (input, data) = many1(terminated(parse_observation, line_ending))(input)?;

    Ok((
        input,
        StationData {
            info: info.into_iter().collect(),
            data,
        },
    ))
}

fn main() {
    let input = r#"Name= Jan Mayen
Country= NORWAY
Lat= 70.9
Long= 8.7
Height= 10
Start year= 1921
End year= 2009
Obs:
1921 -4.4 -7.1 -6.8 -4.3 -0.8 2.2 4.7 5.8 2.7 -2.0 -2.1 -4.0
1922 -0.9 -1.7 -6.2 -3.7 -1.6 2.9 4.8 6.3 2.7 -0.2 -3.8 -2.6
2008 -2.8 -2.7 -4.6 -1.8 1.1 3.3 6.1 6.9 5.8 1.2 -3.5 -0.8
2009 -2.3 -5.3 -3.2 -1.6 2.0 2.9 6.7 7.2 3.8 0.6 -0.3 -1.3
"#;

    match parse_station_data(input) {
        Ok((_, station_data)) => println!("{:#?}", station_data),
        Err(e) => eprintln!("Error: {}", e),
    }
}