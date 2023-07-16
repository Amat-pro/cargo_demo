# A lib for influxDB Client used Rust

## Api-Docs
* [InfluxDB OSS API Service (2.4.0)](https://docs.influxdata.com/influxdb/v2.4/api/)

## Authentications
* BasicAuthentication
  * /api/v2
* TokenAuthentication
  * /api/v2/authorizations
  * /api/v2/authorizations/*

## Clients
* [client: use hyper which is a lower-level, high-level lib (HTTP)](https://github.com/hyperium/hyper)
* [client_v2: use reqwest which is a high-level lib (HTTP/HTTPS)](https://github.com/seanmonstar/reqwest)