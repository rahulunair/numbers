use anyhow::Result;
use clap::Clap;
use serde_derive::Deserialize;

const NUMBERS_URL: &str = "http://numbersapi.com/";

#[derive(Clap)]
#[clap(version = "0.1.0", author = "mail@rahul.onl")]
struct NumbersArgs {
    #[clap(short, long)]
    fetch: bool,
    #[clap(short, long, default_value = "42")]
    number: usize,
    #[clap(short, long, default_value = "random")]
    kind: String,
}
struct NumbersApiClient {
    url: String,
}

#[derive(Deserialize)]
struct NumbersJsonData {
    text: String,
    number: f64,
    found: bool,
    typ: String,
}
struct NumberRespose {}
impl NumbersApiClient {
    fn default(number: &str, typ: &str) -> Self {
        let url = format!("{}/{}/{}", NUMBERS_URL, typ, number);
        Self { url }
    }
}

fn main() {
    let client = NumbersArgs::parse();
    if client.fetch {
        println!("yay");
    }
}
