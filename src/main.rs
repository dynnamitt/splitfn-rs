use chrono::{DateTime, Utc};
use clap::Parser;
use std::process::ExitCode; // 0.4.15

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
    base_path: Option<std::path::PathBuf>,
}

fn main() -> Result<(), ExitCode> {
    let args = Cli::parse();

    let p = &args.path;
    let bp = &args.base_path;

    // test 4 , even better idiomatically
    let p_len: Option<u64> = p.is_file().then(|| p.metadata().unwrap().len());
    let p_created: Option<DateTime<Utc>> = p.metadata().unwrap().created().map(|d| d.into()).ok();

    println!("p: {:?}, b: {:?}", p, bp);
    println!("cre: {:?}, len: {:?}", p_created, p_len);

    Ok(())
}
