<p align="center">
  <img width="300" height="300" src="./logo.png">
</p>
<p align="center">
  <a href="https://github.com/BudiNverse/lta-rs">
    <img src="https://img.shields.io/badge/-lta--rs-blueviolet.svg?style=flat-square"/>
  </a>
  <a href="https://github.com/BudiNverse/lta-rs">
    <img src="https://img.shields.io/github/license/BudiNverse/lta-rs?style=flat-square"/>
  </a>
  <a href="https://crates.io/crates/lta">
    <img src="https://img.shields.io/crates/v/lta?style=flat-square"/>
  </a>
  <a href="https://travis-ci.com/BudiNverse/lta-rs">
    <img src="https://img.shields.io/travis/com/BudiNverse/lta-rs?style=flat-square"/>
  </a>
  <a href="https://github.com/BudiNverse/lta-rs">
    <img src="https://img.shields.io/badge/rust-1.3.6-blueviolet.svg?style=flat-square"/>
  </a>
  <a href="https://github.com/BudiNverse/lta-rs">
    <img src="https://img.shields.io/crates/d/lta?style=flat-square"/>
  </a>
</p>

# lta-rs
> lta-rs is a lta datamall client library written in pure safe rust. lta-rs is used to interact with the [lta-datamall](https://www.mytransport.sg/content/mytransport/home/dataMall.html)

## lta-rs in action

### Cargo.toml setup
There are various versions available. If you omit `branch = "version_no"`, you are taking it from master branch
The library is also available on crates.io
```toml
[dependencies]
lta = "0.3.0-async-preview-1"
```

### API key setup
You can get your API key from [here](https://www.mytransport.sg/content/mytransport/home/dataMall/request-for-api.html)

```rust
extern crate lta;

use lta::lta_client::*;

fn main() {
    let api_key = "MY_API_KEY";
    let client = LTAClient::with_api_key(api_key);
}

```

### Examples

Getting bus timings
```rust
use lta::bus::get_arrival;
use lta::lta_config::*;

fn get_arrivals(client: &LTAClient) {
    let resp: Result<BusArrivalResp, Error> = get_arrival(client, 83139, "15");
    match resp {
        Ok(bus_arrival_resp) => println!("{:?}", bus_arrival_resp),
        Err(e) => println!("{:?}", e)
    };
}
```

Getting anything else
```rust
// All the APIs in this library are designed to be used like this
// `module::get_something`
// All of them return Result<Vec<T>, Error>
// The example below is bus::get_bus_services()
// and traffic::get_erp_rates()
// Do note that the API is similar across all the APIs except for
// bus::get_arrival
use lta::bus::get_bus_services;
use lta::traffic::get_erp_rates;
use lta::lta_config::*;

fn get_bus_services(client: &LTAClient) {
    let resp: Result<Vec<BusService>, Error> = get_bus_services(client);
    match resp {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{:?}", e)
    };
}

fn get_erp_rates(client: &LTAClient) {
    let resp: Result<Vec<ErpRate>, Error> = get_erp_rates(client);
    match resp {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{:?}", e)
    };
}
```


### Async Example
```rust
use lta::r#async::lta_client::LTAClient;
use lta::r#async::{bus::get_arrival, traffic::get_erp_rates};
use lta::utils::commons::Client;
use tokio::prelude::Future;

fn async_example(client: &LTAClient) -> impl Future<Item = (), Error = ()> {
    type Req = (Vec<ErpRate>, BusArrivalResp);
    let fut = get_erp_rates(client);
    let fut2 = get_arrival(client, 83139, "15");
    fut.join(fut2)
        .map(|(a, b): Req| {
            println!("{:?}", a);
            println!("{:?}", b);
        })
        .map_err(|e| println!("Request failed ${:?}", e))
}

fn run_async() {
    let api_key = env::var("API_KEY").unwrap();
    let client = &LTAClient::with_api_key(api_key);
    let fut = async_example(client);
    tokio::run(fut);
}
```

### Custom Client
There are some instance where you might need to customise the client more due to certain limitations.
```rust
use std::time::Duration;
use lta::reqwest::ClientBuilder;
use lta::lta_client::LTAClient;
use lta::utils::commons::Client;

fn my_custom_client() -> LTAClient {
    let client = ClientBuilder::new()
        .gzip(true)
        .connect_timeout(Some(Duration::new(420,0)))
        .build()
        .unwrap();

    LTAClient::new(Some("api_key".to_string()), client)     
}
 ```

### Getting help
- You can get help via github issues. I will try my best to respond to your queries :smile:

### Design decisions
- Made sure that Rust structs are as close to the original response as possible to make sure that people can reference the original docs if there are any issues 
- Simple and no additional baggage. Only the client is included. E.g If anyone wants to add concurrency, they have to do it on their own
- Predictable API usage

### Changelog
Version 0.1
- All endpoints that are available from lta datamall website
- Configuration using API

Version 0.2 **[ Breaking Changes ]**
- Changed all API to take in `&LTAClient` rather than using a global `LTAClient`

Version 0.2.1
- Updated dependencies to latest version as of `21 July 2019`

Version 0.2.2 **[ Broken get_bus_stops, yanked from crates.io ]**
- Updated `LTAClient::with_api_key` to create a LTAClient

Version 0.2.3
- Hotfix for broken `lta::bus::get_bus_stops` which will panic due to typo in serde rename

Version 0.3.0-async-preview-1 **[ Breaking Changes ]**
- Client trait, now has 2 clients, one with async capabilities
- Currently using `futures-preview = "0.3.0-alpha.17"` and `tokio = "0.1.22"` 

### Todo (excluding bugs from issues)
- [x] Proper date types using chrono library
- [x] Utils cleanup
- [x] Host on crates.io
- [ ] Static website to showcase project
- [x] Documentation
- [x] More idiomatic Rust code
- [x] Asynchronous requests 
- [x] Travis CI
- [x] Documentation for async
- [ ] `std::future`
- [ ] Customisable `Client`

### License
lta-rs is licensed under MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

### Frequently Asked Questions

> Where do I get the official docs from lta?

You can get them [here](https://www.mytransport.sg/content/dam/datamall/datasets/LTA_DataMall_API_User_Guide.pdf)

> Why are some of the datatypes different from the lta documentation?

Some of the datatypes returned are not ideal such as returning `lat` and `lang` as `string` rather than `number`. Some of the types are also converted to enums to reduce the number of stringly typed stuff

> My application panicked.

Check if your API key is valid, if it is and your application still panics because of this library, create a github issue

> Is this project affiliated to LTA or any government body?

No.


### Common Technical Questions
- [EOF while parsing a value](https://github.com/BudiNverse/lta-rs/issues/1)
- [API key not init!](https://github.com/BudiNverse/lta-rs/issues/2)
- [No such known host](https://github.com/BudiNverse/lta-rs/issues/3)