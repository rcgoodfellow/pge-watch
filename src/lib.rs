#![allow(unused)]

use anyhow::Result;
use serde::{Deserialize, Deserializer, Serialize};
use std::fs;

const SERVER: &str = "ewapi.cloudapi.pge.com";
const PATH: &str = "single-address-outages";

pub fn from_file(file: &str) -> Result<Vec<PremStatus>> {
    let text = fs::read_to_string(file)?;
    let mut info: Vec<PremStatus> = serde_json::from_str(&text)?;
    massage(&mut info);
    Ok(info)
}

pub async fn request(street_address: &str) -> Result<Vec<PremStatus>> {
    let client = reqwest::Client::new();
    let url = format!("https://{}/{}", SERVER, PATH);
    let req = client
        .get(url)
        .header("Accept", "*/*")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .header("Origin", "https://pgealerts.alerts.pge.com")
        .header("Pragma", "no-cache")
        .header("Referer", "https://pgealerts.alerts.pge.com/")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-site")
        .header("User-Agent", "Server Application")
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", "macOS")
        .query(&[
            ("address", street_address),
            ("event_package_alut_versions", "NoXSA"),
        ]);

    let mut resp = req.send().await?.json::<Vec<PremStatus>>().await?;

    massage(&mut resp);

    Ok(resp)
}

fn massage(status: &mut Vec<PremStatus>) {
    for s in status {
        for d in &mut s.sp_meter_transformer_details {
            if let Some(o) = &mut d.current_outage {
                massage_date(&mut o.outage_start);
                massage_date(&mut o.last_update);
                massage_date(&mut o.current_etor);
            }
            if let Some(o) = &mut d.most_recent_outage {
                massage_date(&mut o.outage_start);
                massage_date(&mut o.last_update);
            }
            if let Some(r) = &mut d.power_restored_at {
                if !r.is_empty() {
                    massage_local_date(r);
                }
            }
        }
    }
}

fn massage_date(date: &mut String) {
    if let Ok(t) = chrono::DateTime::parse_from_rfc3339(date) {
        let lt = t.with_timezone(&chrono_tz::US::Pacific);
        *date = lt.naive_local().format("%b %-d %Y %l:%M %p").to_string();
    }
}

fn massage_local_date(date: &mut String) {
    if let Ok(t) =
        chrono::NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S")
    {
        *date = t.format("%b %-d %Y %l:%M %p").to_string();
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PremStatus {
    pub sp_meter_transformer_details: Vec<MeterTransformerDetails>,
    pub std_dt_updated: String,
    pub prem_id: String,
    pub prem_city: String,
    pub prem_county: String,
    pub prem_state: String,
    pub prem_zip: String,
    pub prem_address1: String,
    pub prem_long: f32,
    pub prem_lat: f32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MeterTransformerDetails {
    pub power_restored_window: Option<Window>,
    pub current_outage: Option<CurrentOutageInfo>,
    pub epss_status: EpssStatus,
    pub meter_id: String,
    pub sp: String,
    pub power_restored_at: Option<String>,
    pub power_restored_recently: bool,
    pub most_recent_outage: Option<RecentOutageInfo>,
    pub forecasted_outages: Vec<ForecastedOutageInfo>,
    #[serde(alias = "smartMeterStatus")]
    pub smart_meter_status: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CurrentOutageInfo {
    pub outage_start: String,
    pub current_etor: String,
    pub est_customers: String,
    pub std_dt_updated: String,
    pub fts_flag: String,
    pub outage_cause: String,
    pub last_update: String,
    pub version: String,
    pub crew_current_status: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RecentOutageInfo {
    pub outage_id: String,
    pub outage_start: String,
    pub est_customers: String,
    pub std_dt_updated: String,
    pub fts_flag: String,
    pub outage_cause: String,
    pub last_update: String,
    pub version: String,
    pub crew_current_status: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EpssStatus {
    pub ready: String,
    pub capable: String,
    pub planned: String,
    pub enabled: String,
}

//TODO this is a guest on the data structure
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ForecastedOutageInfo {
    pub outage_id: String,
    pub outage_start: String,
    pub est_customers: String,
    pub std_dt_updated: String,
    pub fts_flag: String,
    pub outage_cause: String,
    pub last_update: String,
    pub version: String,
    pub crew_current_status: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Window {
    pub start: String,
    pub end: String,
}
