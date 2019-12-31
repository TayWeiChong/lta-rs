//! <p align="center">
//!     <img width="333" height="117" src="https://raw.githubusercontent.com/BudiNverse/lta-rs/master/logo.png">
//! </p>
//! <p align="center">
//!     <a href="https://github.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/badge/-lta--rs-blueviolet.svg?style=flat-square"/>
//!     </a>
//!     <a href="https://github.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/github/license/BudiNverse/lta-rs?style=flat-square"/>
//!     </a>
//!     <a href="https://crates.io/crates/lta">
//!         <img src="https://img.shields.io/crates/v/lta?style=flat-square"/>
//!     </a>
//!     <a href="https://travis-ci.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/travis/com/BudiNverse/lta-rs?style=flat-square"/>
//!     </a>
//!     <a href="https://github.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/badge/rust-1.3.9-blueviolet.svg?style=flat-square"/>
//!     </a>
//!     <a href="https://github.com/BudiNverse/lta-rs">
//!         <img src="https://img.shields.io/crates/d/lta?style=flat-square"/>
//!     </a>
//! </p>
//!
//!
//! # lta
//! lta-rs is a lta datamall client library written in pure safe rust. lta-rs is used to interact with the lta-datamall
//!
//! ## Design Decisions
//! - Made sure that Rust structs are as close to the original response as possible to make sure that people can reference the original docs if there are any issues
//! - Simple and no additional baggage. Only the client is included. E.g If anyone wants to add concurrency, they have to do it on their own
//! - Predictable API usage
//!
//! ## Usage
//! Put this in you `Cargo.toml`
//! ```toml
//! [dependencies]
//! # features available: async, blocking, all. If you only need blocking requests, choose blocking vice versa.
//! lta = { version = "0.3.0-beta", features = ["all"] }
//! ```
//!
//! Initialise API key
//! ```rust
//! use lta::{
//!     utils::{Client, LTAResult},
//!     models::traffic::erp_rates::ErpRate,
//!     blocking::{
//!         traffic::get_erp_rates,
//!         lta_client::LTAClient
//!     }
//! };
//!
//! fn main() -> LTAResult<()> {
//!     let api_key = std::env::var("API_KEY").unwrap();
//!     let client = LTAClient::with_api_key(api_key);
//!     let erp_rates: Vec<ErpRate> = get_erp_rates(&client)?;
//!     println!("{:?}", erp_rates);
//!     Ok(())
//! }
//! ```

#[cfg(feature = "blocking")]
pub use lta_blocking as blocking;

#[cfg(feature = "async")]
pub use lta_async as r#async;

pub use lta_models as models;
pub use lta_utils_commons as utils;
pub use utils::chrono;
pub use utils::reqwest;

#[rustfmt::skip]
pub mod prelude {
    pub use crate::utils::{LTAResult, Client};
}

#[cfg(test)]
mod tests {
    use crate::models::prelude::*;
    use crate::blocking::lta_client::LTAClient;
    use crate::blocking::{bus, crowd, taxi, traffic, train};
    use crate::utils::{Client, LTAResult};
    use std::env;
    use std::fmt::Debug;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    #[ignore]
    #[rustfmt::skip]
    fn dump_json() -> Result<(), Box<dyn std::error::Error>>{
        let api_key = env::var("API_KEY").expect("`API_KEY` not present as env var!");
        let client = LTAClient::with_api_key(api_key);
        let urls_with_query = [
            (lta_models::bus::bus_arrival::URL, &[("BusStopCode", "83139"), ("", ""), ("", "")], "bus_arrival.json"),
            (lta_models::traffic::bike_parking::URL, &[("Lat", "1.364897"), ("Long", "103.766094"), ("Dist", "15.0")], "bike_parking.json"),
        ];

        let urls = [
            (crate::models::bus::bus_routes::URL, "bus_route.json"),
            (crate::models::bus::bus_services::URL, "bus_services.json"),
            (crate::models::bus::bus_stops::URL, "bus_stops.json"),
            (crate::models::taxi::taxi_avail::URL, "taxi_avail.json"),
            (crate::models::traffic::carpark_avail::URL, "carpark_avail.json"),
            (crate::models::traffic::erp_rates::URL, "erp_rates.json"),
            (crate::models::traffic::est_travel_time::URL, "est_travel_time.json"),
            (crate::models::traffic::faulty_traffic_lights::URL, "faulty_traffic_lights.json"),
            (crate::models::train::train_service_alert::URL, "train_service_alert.json"),
            (crate::models::crowd::passenger_vol::URL_BY_BUS_STOPS, "passenger_vol_bus_stops.json"),
            (crate::models::crowd::passenger_vol::URL_BY_OD_BUS_STOPS, "passenger_vol_od_bus_stops.json"),
            (crate::models::crowd::passenger_vol::URL_BY_OD_TRAIN, "passenger_vol_od_train.json"),
            (crate::models::crowd::passenger_vol::URL_BY_TRAIN, "passenger_vol_train.json"),
        ];
        let mut results: Vec<(String, &str)> = Vec::with_capacity(15);

        for url in urls.iter() {
            let rb = client.get_req_builder(url.0);
            let data = rb
                .send()
                .map(|res| res.text().unwrap())?;

            println!("{}", &data);
            results.push((data, url.1))
        }

        for url in urls_with_query.iter() {
            let rb = client.get_req_builder(url.0);
            let data = rb
                .query(url.1)
                .send()
                .map(|res| res.text().unwrap())?;

            println!("{}", &data);
            results.push((data, url.2))
        }
        results.into_iter().for_each(|f| {
            let mut file = File::create(format!("./dumped_data/{}", f.1)).unwrap();
            file.write_all(f.0.as_bytes()).unwrap();
        });

        Ok(())
    }

    fn run_test_and_print<F, T>(f: F)
    where
        F: FnOnce(&LTAClient) -> LTAResult<T>,
        T: Debug,
    {
        let api_key = env::var("API_KEY").unwrap();
        let client = LTAClient::with_api_key(api_key);
        let res = f(&client).unwrap();
        println!("{:?}", res);
    }

    #[test]
    fn concurrent() {
        use std::sync::Arc;
        use std::thread::spawn;

        let api_key = env::var("API_KEY").unwrap();
        let c1 = Arc::new(LTAClient::with_api_key(api_key));
        let c2 = c1.clone();
        let child = spawn(move || traffic::get_carpark_avail(&c1).unwrap());
        let _vms = traffic::get_vms_emas(&c2).unwrap();
        child.join().unwrap();
    }

    #[tokio::test]
    async fn fut() -> LTAResult<()> {
        use crate::r#async as lta_async;

        let api_key = env::var("API_KEY").expect("API_KEY must be set!");
        let client = lta_async::lta_client::LTAClient::with_api_key(api_key);
        let f1 = lta_async::bus::get_arrival(&client, 83139, None).await?;
        let f2 = lta_async::bus::get_arrival(&client, 83139, None).await?;

        println!("{:?} \n{:?}", f1, f2);
        Ok(())
    }

    #[test]
    fn get_arrivals() {
        run_test_and_print(|c| bus::get_arrival(c, 83139, None))
    }

    #[test]
    fn get_bus_services() {
        run_test_and_print(bus::get_bus_services);
    }

    #[test]
    fn get_bus_routes() {
        run_test_and_print(bus::get_bus_routes);
    }

    #[test]
    fn get_bus_stops() {
        run_test_and_print(bus::get_bus_stops);
    }

    #[test]
    #[ignore]
    fn get_passenger_vol() {
        run_test_and_print(|c| crowd::get_passenger_vol_by(c, VolType::OdBusStop, None));
    }

    #[test]
    fn get_taxi_avail() {
        run_test_and_print(taxi::get_taxi_avail);
    }

    #[test]
    fn get_erp_rates() {
        run_test_and_print(traffic::get_erp_rates);
    }

    #[test]
    fn get_cp_avail() {
        run_test_and_print(traffic::get_carpark_avail);
    }

    #[test]
    fn get_est_travel_time() {
        run_test_and_print(traffic::get_est_travel_time);
    }

    #[test]
    fn get_faulty_traffic_lights() {
        run_test_and_print(traffic::get_faulty_traffic_lights);
    }

    #[test]
    fn get_road_details() {
        run_test_and_print(|c| traffic::get_road_details(c, RoadDetailsType::RoadWorks));
    }

    #[test]
    fn get_traffic_images() {
        run_test_and_print(traffic::get_traffic_images);
    }

    #[test]
    fn get_traffic_incidents() {
        run_test_and_print(traffic::get_traffic_incidents);
    }

    #[test]
    fn get_traffic_speed_band() {
        run_test_and_print(traffic::get_traffic_speed_band);
    }

    #[test]
    fn get_vms() {
        run_test_and_print(traffic::get_vms_emas);
    }

    #[test]
    fn get_bike_parking() {
        run_test_and_print(|c| traffic::get_bike_parking(c, 1.364897, 103.766094, Some(15.0)));
    }

    #[test]
    fn get_train_service_alerts() {
        run_test_and_print(train::get_train_service_alert);
    }
}
