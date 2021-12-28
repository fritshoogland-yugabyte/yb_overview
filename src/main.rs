use structopt::StructOpt;
use std::process;
use port_scanner::scan_port_addr;
use std::collections::HashMap;

//use ybtool::{AllMasters, KeySpacesNoId, MasterHealthCheck, Metrics, TablesNoId, TabletsNoId, NamedMetrics};
use ybtool::{AllMasters, KeySpacesNoId, MasterHealthCheck, TablesNoId, TabletsNoId};
use ybtool::AllTabletServers;
use ybtool::MasterDumpEntities;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(short, long, default_value = "localhost:7000")]
    //#[structopt(short, long, default_value = "192.168.66.80:7000,192.168.66.81:7000,192.168.66.82:7000")]
    masters: String,
}

fn main() {
    let options = Opts::from_args();
    let masters_vec: Vec<&str> = options.masters.split(",").collect();

    let mut master_to_use: Option<String> = None;
    for hostnames in &masters_vec {
        if scan_port_addr(hostnames) {
            master_to_use = Some(hostnames.to_string());
            break;
        } else {
            println!("Warning: master not responding on: {}", hostnames.to_string());
        };
    }
    if master_to_use.is_none() {
        println!("No responding masters have been found in: {}", options.masters);
        process::exit(1);
    }

    let master_data = reqwest::blocking::get(format!("http://{}/api/v1/masters", master_to_use.as_ref().unwrap()))
        .unwrap_or_else(|e| {
            eprintln!("Error reading from URL: {}", e);
            std::process::exit(1);
        })
        .text().unwrap();
    let master_parse: AllMasters = serde_json::from_str(&master_data)
        .unwrap_or_else(|e| {
            eprintln!("Error parsing response: {}", e);
            process::exit(1);
        });

    println!("\n{:32} {:20} {:13} {:8}", "Master UUID", "RPC Host:Port", "State", "Role");
    for master_status in master_parse.masters {
        print!("{:32} ", master_status.instance_id.permanent_uuid);
        print!("{:20} ", format!("{}:{}", master_status.registration.private_rpc_addresses[0].host, master_status.registration.private_rpc_addresses[0].port));
        let role = &master_status.role.unwrap_or("UNKNOWN".to_string());
        let state = match master_status.error {
            Some(master_status_error) => master_status_error.code,
            None => "ALIVE".to_string(),
        };
        print!("{:13} ", state);
        println!("{:8}", role);
    }

    let tserver_data = reqwest::blocking::get(format!("http://{}/api/v1/tablet-servers", master_to_use.as_ref().unwrap()))
        .unwrap_or_else(|e| {
            eprintln!("Error reading from URL: {}", e);
            process::exit(1);
        })
        .text().unwrap();
    let tserver_parse: AllTabletServers = serde_json::from_str(&tserver_data)
        .unwrap_or_else(|e| {
            eprintln!("Error parsing response: {}", e);
            process::exit(1);
        });

    println!("\n{:20} {:15} {:7} {:>8} {:>8} {:>10} {:>10} {:>13} {:>10} {:>10}", "HTTP Host:Port", "Heartbeat delay", "Status", "Rd op/s", "Wr op/s", "Uptime", "SST tot sz", "SST uncomp sz", "SST #files", "Memory");
    for (tserver_name, tserver_status) in &tserver_parse.tabletservers {
        print!("{:20} ", tserver_name);
        print!("{:>15} ", tserver_status.time_since_hb);
        print!("{:7} ", tserver_status.status);
        print!("{:>8.0} ", tserver_status.read_ops_per_sec);
        print!("{:>8.0} ", tserver_status.write_ops_per_sec);
        print!("{:>10} ", tserver_status.uptime_seconds);
        print!("{:>10} ", tserver_status.total_sst_file_size);
        print!("{:>13} ", tserver_status.uncompressed_sst_file_size);
        print!("{:>10} ", tserver_status.num_sst_files);
        println!("{:>10}", tserver_status.ram_used);
    }

    println!("\n{:<20} {:<20} {:>10} {:>10} {:>7}", "Hostname", "Path", "Used MB", "Total MB", "Used %");
    for (tserver_name, tserver_status) in &tserver_parse.tabletservers {
        for paths in &tserver_status.path_metrics {
            let p_su = paths.space_used as f64;
            let p_ts = paths.total_space_size as f64;
            println!("{:<20} {:<20} {:>10} {:>10}  {: >3.2}%", tserver_name, paths.path, paths.space_used / 1024 / 1024, paths.total_space_size / 1024 / 1024, p_su / p_ts * 100 as f64);
        }
    }
    println!("");

    let master_healthcheck_data = reqwest::blocking::get(format!("http://{}/api/v1/health-check", master_to_use.as_ref().unwrap()))
        .unwrap_or_else(|e| {
            eprintln!("Error reading from URL: {}", e);
            process::exit(1);
        })
        .text().unwrap();
    let master_healthcheck_parse: MasterHealthCheck = serde_json::from_str(&master_healthcheck_data)
        .unwrap_or_else(|e| {
            eprintln!("Error parsing response: {}", e);
            process::exit(1);
        });

    for dead_node in &master_healthcheck_parse.dead_nodes {
        println!("Missing (dead) tablet server: {}", dead_node);
    }
    // for under_replicated_tablet in &master_healthcheck_parse.under_replicated_tablets {
    //     println!("Under replicated tablet: {}", under_replicated_tablet);
    // }

    let master_dumpentities_data = reqwest::blocking::get(format!("http://{}/dump-entities", master_to_use.as_ref().unwrap()))
        .unwrap_or_else(|e| {
            eprintln!("Error reading from URL: {}", e);
            process::exit(1);
        })
        .text().unwrap();
    let master_dumpentities_parse: MasterDumpEntities = serde_json::from_str(&master_dumpentities_data)
        .unwrap_or_else(|e| {
            eprintln!("Error parsing response: {}", e);
            process::exit(1);
        });
    //println!("{:?}",master_dumpentities_parse.keyspaces);

    let mut hash_by_keyspace_id = HashMap::new();
    for keyspace in master_dumpentities_parse.keyspaces {
        let temp = KeySpacesNoId {
            keyspace_name: keyspace.keyspace_name,
            keyspace_type: keyspace.keyspace_type,
        };
        hash_by_keyspace_id.insert(keyspace.keyspace_id, temp);
    }
    let mut hash_by_table_id = HashMap::new();
    for table in master_dumpentities_parse.tables {
        let temp = TablesNoId {
            keyspace_id: table.keyspace_id,
            table_name: table.table_name,
            state: table.state,
        };
        hash_by_table_id.insert(table.table_id, temp);
    }
    let mut hash_by_tablet_id = HashMap::new();
    for tablet in master_dumpentities_parse.tablets {
        let temp = TabletsNoId {
            table_id: tablet.table_id,
            state: tablet.state,
            replicas: tablet.replicas,
            leader: tablet.leader,
        };
        hash_by_tablet_id.insert(tablet.tablet_id, temp);
    }

    let mut got_underreplication: Option<bool> = None;
    for under_replicated_tablet in &master_healthcheck_parse.under_replicated_tablets {
        if got_underreplication.is_none() {
            println!("\nUnder replicated tablets");
            println!("{:<5} {:<20} {:<30} {:<10} {:<50}", "Type", "Keyspacename", "Tablename", "State", "Tablet replicas");
            got_underreplication = Some(true);
        };
        let tablet = hash_by_tablet_id.get(under_replicated_tablet);
        let table = hash_by_table_id.get(&tablet.unwrap().table_id);
        let keyspace = hash_by_keyspace_id.get(&table.unwrap().keyspace_id);
        print!("{:<5} {:<20} {:<30} {:<10} ", keyspace.unwrap().keyspace_type, keyspace.unwrap().keyspace_name, table.unwrap().table_name, table.unwrap().state);
        for replica in tablet.unwrap().replicas.as_ref().unwrap() {
                if &replica.server_uuid == tablet.unwrap().leader.as_ref().unwrap() {
                    print!("LEADER,{}:{} ", replica.replica_type, replica.addr);
                } else {
                    print!("{}:{} ", replica.replica_type, replica.addr);
                }
        }
        println!("");
    }

    /*
        let master_metrics_data = reqwest::blocking::get(format!("http://{}/metrics", master_to_use.as_ref().unwrap()))
            .unwrap_or_else(|e| {
                eprintln!("Error reading from URL: {}", e);
                process::exit(1);
            })
            .text().unwrap();
        //println!("{:?}", master_metrics_data);
        let master_metrics_parse: Vec<Metrics> = serde_json::from_str(&master_metrics_data)
            .unwrap_or_else(|e| {
                eprintln!("Error parsing response: {}", e);
                process::exit(1);
            });
        let metricstypes = vec!["cluster", "server", "table", "tablet"];
        for metrictype in metricstypes.iter() {
            for metric in &master_metrics_parse {
            //if metric.metrics_type == "server" {
                for m in &metric.metrics {
                    match m {
                        NamedMetrics::MetricValue {name, value} => {
                            //println!("Value: {}", name)
                            if value > &0 && &metric.metrics_type == metrictype {
                                println!("{} {} {}", metric.metrics_type, name, value);
                            }
                        },
                        NamedMetrics::MetricLatency {name, total_count, min, mean, percentile_75, percentile_95, percentile_99, percentile_99_9, percentile_99_99, max, total_sum} => {
                            //println!("Latency: {}", name)
                            if total_count > &0 && &metric.metrics_type == metrictype {
                                println!("{} {} {}", metric.metrics_type, name, total_count);
                            }
                        },
                    }
                }
                //println!("{:?}", metric.metrics);
            }
            //println!("{:?}", metric.metrics_type);
        }
        //println!("{:?}", master_metrics_parse);
        */
}