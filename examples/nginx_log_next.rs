use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use std::net::{IpAddr, Ipv4Addr};
use url::Url;
use winnow::{ascii::digit1, combinator::separated, PResult, Parser};

#[derive(Debug)]
enum HttpProtocol {
    // Http1_0,
    Http1_1,
    // Http2,
}

#[derive(Debug)]
enum HttpMethod {
    Get,
    // Post,
    // Head,
    // Options,
    // Put,
    // Delete,
    // Trace,
    // Connect,
}

#[allow(unused)]
#[derive(Debug)]
struct NginxLog {
    ip: IpAddr,
    datetime: DateTime<Utc>,
    method: HttpMethod,
    url: Url,
    protocol: HttpProtocol,
    status: u16,
    bytes: u32,
    referrer: String,
    user_agent: String,
}

fn main() -> Result<()> {
    let log = parse_nginx_log(
        r#"93.180.71.3 - - [17/May/2015:08:05:32 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)""#,
    ).map_err(|e| anyhow!("Failed to parse log: {:?}", e))?;
    println!("{:?}", log);
    Ok(())
}

fn parse_nginx_log(s: &str) -> PResult<NginxLog> {
    let input = &mut (&*s);
    Ok(NginxLog {
        ip: parse_ip(input)?,
        datetime: Utc::now(),
        method: HttpMethod::Get,
        url: Url::parse("http://example.com").unwrap(),
        protocol: HttpProtocol::Http1_1,
        status: 200,
        bytes: 0,
        referrer: String::new(),
        user_agent: String::new(),
    })
}

fn parse_ip(s: &mut &str) -> PResult<IpAddr> {
    let octet = digit1.parse_to::<u8>();
    let ip: Vec<u8> = separated(4, octet, '.').parse_next(s)?;
    Ok(IpAddr::V4(Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3])))
}

// fn parse_datetime(s: &str) -> Result<DateTime<Utc>, Error> {
//     todo!()
// }

// fn parse_http_method(s: &str) -> Result<HttpMethod, Error> {
//     todo!()
// }

// fn parse_http_protocol(s: &str) -> Result<HttpProtocol, Error> {
//     todo!()
// }

// fn parse_url(s: &str) -> Result<Url, Error> {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ip() {
        let mut input = "192.168.1.1";
        let result = parse_ip(&mut input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));

        let mut invalid_input = "192.168.1";
        assert!(parse_ip(&mut invalid_input).is_err());

        let mut invalid_range = "256.168.1.1";
        assert!(parse_ip(&mut invalid_range).is_err());
    }
}
