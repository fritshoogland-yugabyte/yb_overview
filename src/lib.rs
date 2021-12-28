extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceId {
    pub instance_seqno: i64,
    pub permanent_uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_us: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Registration {
    pub private_rpc_addresses: Vec<PrivateRpcAddresses>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_addresses: Option<Vec<HttpAddresses>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_info: Option<CloudInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrivateRpcAddresses {
    pub host: String,
    pub port: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpAddresses {
    pub host: String,
    pub port: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloudInfo {
    pub placement_cloud: String,
    pub placement_region: String,
    pub placement_zone: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Masters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    pub instance_id: InstanceId,
    pub registration: Registration,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllMasters {
    pub masters: Vec<Masters>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub code: String,
    pub message: String,
    pub posix_code: i32,
    pub source_file: String,
    pub source_line: i32,
    pub errors: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllTabletServers {
    #[serde(rename = "")]
    pub tabletservers: HashMap<String, TabletServerStatus>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PathMetrics {
    pub path: String,
    pub space_used: i64,
    pub total_space_size: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TabletServerStatus {
    pub time_since_hb: String,
    pub time_since_hb_sec: f32,
    pub status: String,
    pub uptime_seconds: i64,
    pub ram_used: String,
    pub ram_used_bytes: i64,
    pub num_sst_files: i32,
    pub total_sst_file_size: String,
    pub total_sst_file_size_bytes: i32,
    pub uncompressed_sst_file_size: String,
    pub uncompressed_sst_file_size_bytes: i32,
    pub path_metrics: Vec<PathMetrics>,
    pub read_ops_per_sec: f32,
    pub write_ops_per_sec: f32,
    pub user_tablets_total: i32,
    pub user_tablets_leaders: i32,
    pub system_tablets_total: i32,
    pub system_tablets_leaders: i32,
    pub active_tablets: i32,
    pub cloud: String,
    pub region: String,
    pub zone: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MasterHealthCheck {
    pub dead_nodes: Vec<String>,
    pub most_recent_uptime: i64,
    pub under_replicated_tablets: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MasterDumpEntities {
    pub keyspaces: Vec<KeySpaces>,
    pub tables: Vec<Tables>,
    pub tablets: Vec<Tablets>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeySpaces {
    pub keyspace_id: String,
    pub keyspace_name: String,
    pub keyspace_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tables {
    pub table_id: String,
    pub keyspace_id: String,
    pub table_name: String,
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tablets {
    pub table_id: String,
    pub tablet_id: String,
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<Vec<Replicas>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Replicas {
    #[serde(rename = "type")]
    pub replica_type: String,
    pub server_uuid: String,
    pub addr: String,
}

#[derive(Debug)]
pub struct KeySpacesNoId {
    pub keyspace_name: String,
    pub keyspace_type: String,
}

#[derive(Debug)]
pub struct TablesNoId {
    pub keyspace_id: String,
    pub table_name: String,
    pub state: String,
}

#[derive(Debug)]
pub struct TabletsNoId {
    pub table_id: String,
    pub state: String,
    pub replicas: Option<Vec<Replicas>>,
    pub leader: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metrics {
    #[serde(rename = "type")]
    pub metrics_type: String,
    pub id: String,
    pub attributes: Attributes,
    pub metrics: Vec<NamedMetrics>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    pub namespace_name: Option<String>,
    pub table_name: Option<String>,
    pub table_id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum NamedMetrics {
    MetricValue {
        name: String,
        value: i64,
    },
    MetricLatency {
        name: String,
        total_count: i64,
        min: i64,
        mean: f64,
        percentile_75: i64,
        percentile_95: i64,
        percentile_99: i64,
        percentile_99_9: i64,
        percentile_99_99: i64,
        max: i64,
        total_sum: i64,
    }
}