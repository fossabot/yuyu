use anyhow::Result;
use clap::{App, Arg};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("yuyu")
        .version("1.0")
        .arg(Arg::with_name("url").required(true))
        .get_matches();

    let url = matches.value_of("url").unwrap();

    yuyu_core::start(url).await?;

    Ok(())
}
