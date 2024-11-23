use chrono::FixedOffset;
use dateparser::parse_with_timezone;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
use nom::character::complete::{digit1, newline, not_line_ending, space1};
use nom::multi::{many0, separated_list1};
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
pub struct ParsedData {
    pub timestamp: i64,
    pub client_ip: String,
    pub client_port: u32,
    pub server_ip: String,
    pub server_port: u32,
    pub data_identifier: String,
    pub payload: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IpPortPair {
    pub ip: String,
    pub port: u32,
}

#[derive(Debug, PartialEq)]
pub struct NetworkInfo {
    pub client_ip: String,
    pub client_port: u32,
    pub server_ip: String,
    pub server_port: u32,
    pub protocol: Option<String>,
}

pub fn str_as_unix_time(server_time: &str) -> i64 {
    parse_with_timezone(server_time, &FixedOffset::west_opt(0).unwrap())
        .map(|x| x.timestamp_millis() + (-8 * 3600 * 1000i64))
        .unwrap_or(0i64)
}

pub fn bytes_to_uint8(array: &[u8]) -> Option<u8> {
    if let Ok(slice) = array.try_into() {
        Some(u8::from_le_bytes(slice))
    } else {
        None
    }
}

// 解析服务器时间
fn parse_server_time(input: &str) -> IResult<&str, i64> {
    let mut parser = tuple((
        separated_list1(tag("-"), digit1),
        space1,
        separated_list1(tag(":"), digit1),
        tag("."),
        digit1,
    ));

    let (input, (date, _, time, _, micro_seconds)) = parser(input)?;
    let datetime = format!("{} {}.{}", date.join("-"), time.join(":"), micro_seconds);
    let unix_time = str_as_unix_time(&datetime);
    Ok((input, unix_time))
}

// 解析 IP 地址或域名
fn parse_ip_or_domain(input: &str) -> IResult<&str, &str> {
    let (input, ip_or_domain) = take_until1(":")(input)?;
    Ok((input, ip_or_domain))
}

// 解析端口号
fn parse_port(input: &str) -> IResult<&str, String> {
    let (input, port) = digit1(input)?;

    Ok((input, port.into()))
}

// 解析IP地址:端口号对儿
fn parse_ip_port_pair(input: &str) -> IResult<&str, IpPortPair> {
    let mut parser = separated_pair(parse_ip_or_domain, tag(":"), parse_port);
    let (input, (ip, port)) = parser(input)?;

    Ok((
        input,
        IpPortPair {
            ip: ip.to_string(),
            port: port.parse::<u32>().unwrap(),
        },
    ))
}

fn parse_iot_log(input: &str) -> IResult<&str, NetworkInfo> {
    let mut parser = tuple((
        tag("["),
        separated_pair(parse_ip_port_pair, tag("#"), parse_ip_port_pair),
        tag("]"),
    ));
    let (input, (_, (client, server), _)) = parser(input)?;

    let protocol = match client.port {
        0 => "mqtt",
        _ => "iec104",
    };

    Ok((
        input,
        NetworkInfo {
            client_ip: client.ip,
            client_port: client.port,
            server_ip: server.ip,
            server_port: server.port,
            protocol: Some(protocol.into()),
        },
    ))
}

fn parse_network_info(input: &str) -> IResult<&str, NetworkInfo> {
    let (input, network_info) = parse_iot_log(input)?;
    Ok((input, network_info))
}

fn parse_payload(input: &str) -> IResult<&str, (&str, &str)> {
    let mut parser = tuple((alt((tag("D:"), tag("R:"))), not_line_ending));
    let (input, (data_identifier, json)) = parser(input)?;
    Ok((input, (data_identifier, json)))
}

pub fn parse_log(input: &str) -> IResult<&str, ParsedData> {
    let mut parser = tuple((
        parse_server_time,
        space1,
        parse_network_info,
        space1,
        parse_payload,
        many0(newline),
    ));
    let (input, (ts, _, network_info, _, (data_identifier, payload), _)) = parser(input)?;
    let res = ParsedData {
        timestamp: ts,
        client_ip: network_info.client_ip,
        client_port: network_info.client_port,
        server_ip: network_info.server_ip,
        server_port: network_info.server_port,
        data_identifier: data_identifier.to_string(),
        payload: payload.to_string(),
    };
    Ok((input, res))
}

fn main() {
    let log = "2024-05-05 23:59:58.846 [223.104.43.11:11686#10.0.1.88:5003] R:6822eee05c460d03030001001940000080c843001a40003373c843001b400033b3c84300";
    match parse_log(log) {
        Ok((_, parsed_data)) => {
            println!("Parsed Data: {:?}", parsed_data);
        }
        Err(e) => println!("Failed to parse log: {:?}", e),
    }
}
