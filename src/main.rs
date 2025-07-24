use std::fs::read_to_string;

use crate::config::Config;
use sonic_rs::{Deserialize, Serialize};
use uuid::Uuid;

mod config;
mod memcached;
mod session;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = toml::from_str::<Config>(read_to_string("config.toml").unwrap().as_str());
    println!("{:?}", config);
    Ok(())

    // println!("Hello, world!");
    // let client = memcache::Client::with_pool_size("memcache://127.0.1:11211", 10).unwrap();

    // let result = client.flush().unwrap();
    // println!("flush result : {:?}", result);

    // let test_message = sonic_rs::json!(TestStruct {
    //     name: "test".to_string(),
    //     value: Uuid::now_v7(),
    // });
    // let result = client.set("hello", test_message.to_string(), 10);
    // println!("set result : {:?}", result);

    // let result = client.get::<String>("hello").unwrap().unwrap();
    // let result = sonic_rs::from_str::<TestStruct>(result.as_str());
    // println!("get result : {:?}", result);

    // let result = client.get::<String>("world");
    // println!("get result : {:?}", result);
}
