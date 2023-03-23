use anyhow::anyhow;
use clap::Parser;
use dns_lookup::lookup_host;
use env_logger::Env;
use log::{error, info};
use std::{
    io,
    net::{IpAddr, SocketAddr, ToSocketAddrs},
    str::FromStr,
};

// https://chrisbergeron.com/2018/05/04/diy_dynamic_dns_digital_ocean/

/// Updates A record of domain at DigitialOcean with current host public IPV4.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Domain Name
    #[arg(short, long)]
    domain_name: String,

    /// If not set prints all ids for domain instead of updating.
    #[arg(long)]
    domain_record_id: Option<u64>,

    /// Digitalocean api token
    #[arg(short, long)]
    api_token: String,

    /// Url to get current public IP
    #[arg(short, long, default_value_t=String::from("https://icanhazip.com"))]
    url_check_ip: String
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let _ = main_try().map_err(|e| error!("{}", e.to_string()));
}

fn main_try() -> anyhow::Result<()> {
    let args = Args::parse();

    // ipv4 only agent
    let agent = ureq::AgentBuilder::new()
        .resolver(ipv4_resolve)
        .build();

    if let Some(domain_record_id) = args.domain_record_id {
        // get ips for domain
        let domain_dns_ip = lookup_host(&args.domain_name)?;
        // only get ipv4
        let domain_dns_ip: Vec<&std::net::IpAddr> =
            domain_dns_ip.iter().filter(|p| p.is_ipv4()).collect();
        // convert to IpAddr since Vec should only have one ip
        let domain_dns_ip = domain_dns_ip
            .first()
            .ok_or(anyhow!("Could not find a IPV4 address for domain."))?;

        // current ip of host
        let host_ip = agent.get(&args.url_check_ip).call()?.into_string()?;
        let host_ip = IpAddr::from_str(host_ip.trim())?;

        info!(
            "Current domain ip: {} | Current host ip: {}",
            domain_dns_ip, host_ip
        );
        if **domain_dns_ip == host_ip {
            info!("IPs equal. Not updating.");
            return Ok(());
        }

        // Ips diffrent
        // https://docs.digitalocean.com/reference/api/api-reference/#operation/domains_update_record
        let _ = agent
            .put(
                format!(
                    "https://api.digitalocean.com/v2/domains/{}/records/{}",
                    args.domain_name, domain_record_id
                )
                .as_str(),
            )
            .set(
                "Authorization",
                format!("Bearer {}", args.api_token).as_str(),
            )
            .send_json(ureq::json!({
                "type": "A",
                "data": host_ip.to_string()
            }))?;

        info!(
            "Successfully changed IP to {} for {}",
            host_ip, args.domain_name
        );
    } else {
        // print domain record ids
        // https://docs.digitalocean.com/reference/api/api-reference/#operation/domains_list_records
        let resp: serde_json::Value = agent
            .get(
                format!(
                    "https://api.digitalocean.com/v2/domains/{}/records",
                    args.domain_name
                )
                .as_str(),
            )
            .set(
                "Authorization",
                format!("Bearer {}", args.api_token).as_str(),
            )
            .send_json(ureq::json!({}))?
            .into_json()?;

        info!("{}", serde_json::to_string_pretty(&resp)?);
    }
    Ok(())
}

/// Only resolve ipv4
fn ipv4_resolve(netloc: &str) -> io::Result<Vec<SocketAddr>> {
    match netloc.to_socket_addrs() {
        Ok(x) => Ok(x.filter(|p| p.is_ipv4()).collect()),
        Err(e) => Err(e),
    }
}
