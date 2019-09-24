use futures::future::*;
use influxdb::client::InfluxDbClient;
use influxdb::query::{InfluxDbQuery, Timestamp};

fn main() {
    let client = InfluxDbClient::new("http://localhost:8086", "test");

    let write_query =
        InfluxDbQuery::write_query(Timestamp::NOW, "weather").add_field("temperature", 82);

    tokio::spawn(
        lazy(move || client.query(&write_query))
            .map(|x| {
                println!("{:?}", &x);
            })
            .map_err(|e| {
                println!("{:?}", &e);
            }),
    );
}
