use std::collections::HashMap;

#[derive(Debug)]
struct StationData {
    info: HashMap<String, String>,
    data: Vec<(i32, Vec<f64>)>,
}

#[derive(Debug, PartialEq)]
enum ParserState {
    KeyValue,
    Observations,
    Done,
}

fn parse_station_data(input: &str) -> Result<StationData, String> {
    let mut state = ParserState::KeyValue;
    let mut station_data = StationData {
        info: HashMap::new(),
        data: Vec::new(),
    };

    for line in input.lines() {
        match state {
            ParserState::KeyValue => {
                if line.starts_with("Obs:") {
                    state = ParserState::Observations;
                } else if let Some((key, value)) = line.split_once('=') {
                    station_data.info.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
            ParserState::Observations => {
                if line.trim().is_empty() {
                    state = ParserState::Done;
                } else {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Ok(year) = parts[0].parse::<i32>() {
                        let temps: Vec<f64> = parts[1..]
                            .iter()
                            .filter_map(|&s| s.parse::<f64>().ok())
                            .filter(|&t| t > -99.0)
                            .collect();
                        if !temps.is_empty() {
                            station_data.data.push((year, temps));
                        }
                    }
                }
            }
            ParserState::Done => break,
        }
    }

    Ok(station_data)
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
2009 -2.3 -5.3 -3.2 -1.6 2.0 2.9 6.7 7.2 3.8 0.6 -0.3 -1.3"#;

    match parse_station_data(input) {
        Ok(station_data) => println!("{:#?}", station_data),
        Err(e) => eprintln!("Error: {}", e),
    }
}