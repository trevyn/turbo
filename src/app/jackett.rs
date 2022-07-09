// https://github.com/Jackett/Jackett/releases/latest/download/Jackett.Binaries.LinuxAMDx64.tar.gz

use serde::Deserialize;
use turbocharger::prelude::*;
use turbosql::Turbosql;

#[derive(Deserialize, Debug)]
pub struct ConfigResponse {
 pub api_key: String,
 pub app_version: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct JackettResults {
 pub results: Vec<JackettResult>,
}

#[backend]
#[serde(rename_all = "PascalCase")]
#[derive(Turbosql)]
pub struct JackettResult {
 pub rowid: Option<i64>,
 pub tracker: Option<String>,
 pub tracker_id: Option<String>,
 pub category_desc: Option<String>,
 pub title: Option<String>,
 pub guid: Option<String>,
 pub link: Option<String>,
 pub details: Option<String>,
 pub publish_date: Option<String>,
 pub category: Option<Vec<i64>>,
 pub size: Option<i64>,
 pub seeders: Option<i64>,
 pub peers: Option<i64>,
 pub gain: Option<f64>,
}
