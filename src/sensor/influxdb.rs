use influxdb::{Client, ReadQuery};
use serde::Deserialize;

use super::super::settings::InfluxDB;

#[tokio::main]
pub async fn run(settings: &InfluxDB) {
    let client = Client::new(&settings.hostname, &settings.database);

    #[derive(Deserialize, Debug)]
    struct Temperature {
        temperature: f32,
    }

    let query_t1_sl = ReadQuery::new(
        "SELECT temperature FROM sensor WHERE time > now() - 1h AND userdata = '25' ORDER BY DESC LIMIT 1",
    );
    let query_t1_sr = ReadQuery::new(
        "SELECT temperature FROM sensor WHERE time > now() - 1h AND userdata = '26' ORDER BY DESC LIMIT 1",
    );

    let result_t1_sl = client
        .json_query(query_t1_sl)
        .await
        .and_then(|mut db_result| db_result.deserialize_next::<Temperature>());

    let result_t1_sr = client
        .json_query(query_t1_sr)
        .await
        .and_then(|mut db_result| db_result.deserialize_next::<Temperature>());

    let temp = (result_t1_sl.unwrap().series[0].values[0].temperature
        + result_t1_sr.unwrap().series[0].values[0].temperature)
        / 2.0;

    println!("{:.1}", temp);
}

pub fn query_device(device_name: String, field: String, _threshold: f32) {
    #[derive(Deserialize, Debug)]
    struct Temperature {
        _temperature: f32,
    }

    let _query = ReadQuery::new(
        format!("SELECT {field} FROM sensor WHERE time > now() - 1h AND userdata = '{device_name}' ORDER BY DESC LIMIT 1"),
    );

    //if device
}
