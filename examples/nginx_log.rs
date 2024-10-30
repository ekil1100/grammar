use anyhow::{anyhow, Error, Result};
use regex::Regex;

#[allow(unused)]
#[derive(Debug)]
struct NginxLog {
    ip: String,
    datetime: String,
    method: String,
    url: String,
    protocol: String,
    status: u16,
    bytes: u32,
    referrer: String,
    user_agent: String,
}

fn main() -> Result<()> {
    let log = parse_nginx_log(
        r#"93.180.71.3 - - [17/May/2015:08:05:32 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)""#,
    )?;
    println!("{:?}", log);
    Ok(())
}

fn parse_nginx_log(text: &str) -> Result<NginxLog, Error> {
    let re = Regex::new(
        r#"^(?<ip>\S*)\s+\S+\s+\S+\s+\[(?<date>[^\]]+)\]\s+"(?<method>\S+)\s+(?<url>\S+)\s+(?<proto>[^"]+)"\s+(?<status>\d+)\s+(?<bytes>\d+)\s+"(?<referrer>[^"]+)"\s+"(?<ua>[^"]+)"$"#,
    )?;
    let m = re.captures(text).ok_or(anyhow!("no match"))?;
    Ok(NginxLog {
        ip: m.name("ip").unwrap().as_str().to_string(),
        datetime: m.name("date").unwrap().as_str().to_string(),
        method: m.name("method").unwrap().as_str().to_string(),
        url: m.name("url").unwrap().as_str().to_string(),
        protocol: m.name("proto").unwrap().as_str().to_string(),
        status: m.name("status").unwrap().as_str().parse()?,
        bytes: m.name("bytes").unwrap().as_str().parse()?,
        referrer: m.name("referrer").unwrap().as_str().to_string(),
        user_agent: m.name("ua").unwrap().as_str().to_string(),
    })
}
