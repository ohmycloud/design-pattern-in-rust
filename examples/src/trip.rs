use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
enum State {
    ExpectingCountry,
    ExpectingDestination,
}

#[derive(Debug)]
struct Destination {
    name: String,
    lat: f64,
    long: f64,
    sales: i32,
}

#[derive(Debug)]
struct Country {
    name: String,
    destinations: Vec<Destination>,
}

fn parse_trip_data<P: AsRef<Path>>(filename: P) -> io::Result<Vec<Country>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut countries = Vec::new();
    let mut current_country: Option<Country> = None;
    let mut state = State::ExpectingCountry;

    for line in reader.lines() {
        let line = line?;
        match state {
            State::ExpectingCountry => {
                if !line.trim().is_empty() {
                    if let Some(country) = current_country {
                        countries.push(country);
                    }
                    current_country = Some(Country {
                        name: line.trim().to_string(),
                        destinations: Vec::new(),
                    });
                    state = State::ExpectingDestination;
                }
            }
            State::ExpectingDestination => {
                if line.trim().is_empty() {
                    state = State::ExpectingCountry;
                } else {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() == 3 {
                        let name = parts[0].trim().to_string();
                        let coords: Vec<&str> = parts[1].split(',').collect();
                        let lat = coords[0].trim().parse::<f64>().unwrap_or(0.0);
                        let long = coords[1].trim().parse::<f64>().unwrap_or(0.0);
                        let sales = parts[2].trim().parse::<i32>().unwrap_or(0);

                        if let Some(ref mut country) = current_country {
                            country.destinations.push(Destination {
                                name,
                                lat,
                                long,
                                sales,
                            });
                        }
                    }
                }
            }
        }
    }

    if let Some(country) = current_country {
        countries.push(country);
    }

    Ok(countries)
}

fn main() -> io::Result<()> {
    let parsed_data = parse_trip_data("data/trips.txt")?;

    // Get all country names
    println!("Country names:");
    for country in &parsed_data {
        println!("{}", country.name);
    }
    println!("{}", "-".repeat(45));

    // Get all destinations
    println!("All destinations:");
    for country in &parsed_data {
        for destination in &country.destinations {
            println!("{}", destination.name);
        }
    }
    println!("{}", "-".repeat(45));

    // Get ticket sales
    println!("Ticket sales:");
    for country in &parsed_data {
        let total_sales: i32 = country.destinations.iter().map(|d| d.sales).sum();
        println!("{}\t{}", country.name, total_sales);
    }
    println!("{}", "-".repeat(45));

    // Get latitudes
    println!("Latitudes:");
    for country in &parsed_data {
        for destination in &country.destinations {
            println!("{}", destination.lat);
        }
    }

    Ok(())
}