# DigitalOcean DDNS

Simple program to update the DNS A record for a domain at DigitalOcean with the current host public IPV4.

Rust program based on the bash scripts from [chrisbergeron.com/2018/05/04/diy_dynamic_dns_digital_ocean/](https://chrisbergeron.com/2018/05/04/diy_dynamic_dns_digital_ocean/).

## Building

Install `Rust` and just run:

```
cargo build --release
```

## Usage
Run `dyndns --help` or `cargo run -- --help`:
```
Updates A record of domain at DigitialOcean with current host public IPV4

Usage: dyndns [OPTIONS] --domain-name <DOMAIN_NAME> --api-token <API_TOKEN>

Options:
  -d, --domain-name <DOMAIN_NAME>
          Domain Name
      --domain-record-id <DOMAIN_RECORD_ID>
          If not set prints all ids for domain instead of updating
  -a, --api-token <API_TOKEN>
          Digitalocean api token
  -u, --url-check-ip <URL_CHECK_IP>
          Url to get current public ip [default: https://icanhazip.com]
  -h, --help
          Print help
  -V, --version
```

Example:
```
dyndns -d example.com -a dop_v1_... --domain-record-id 12345678
```

The `domain-record-id` is the id of the A record at DigitalOcean. Help how to create an api token for DigitalOcean at [docs.digitalocean.com](https://docs.digitalocean.com/reference/api/create-personal-access-token/).


To determine the correct id  for the A record run the prevoius command without `--domain-record-id`.
An A record needs to be created once in the DigitalOcean Dashboard.
```
dyndns -d example.com -a dop_v1_...
```
Returns:
```json
{
    "domain_records": [
    ...
    {
        "data": "12.123.123.123",
        "flags": null,
        "id": 12345678,
        "name": "@",
        "port": null,
        "priority": null,
        "tag": null,
        "ttl": 600,
        "type": "A",
        "weight": null
    }
    ],
    ...
}
```
The `domain-record-id` is found at `"id"`.