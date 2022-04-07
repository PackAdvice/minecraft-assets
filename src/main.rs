use crate::version::Version;
use std::env;

mod assets;
mod version;

fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(version) => println!("{:?}", Version::new(version)?),
        None => println!(concat!("Usage: ", env!("CARGO_BIN_NAME"), " <Version>")),
    }
    Ok(())
}
