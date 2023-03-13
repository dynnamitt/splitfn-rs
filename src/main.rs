use clap::Parser;
use std::{os::unix::prelude::MetadataExt, process::ExitCode};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
    base_path: Option<std::path::PathBuf>,
}

fn main() -> Result<(), ExitCode> {
    let args = Cli::parse();

    let p = &args.path;
    let bp = &args.base_path;

    // test 1
    let p_size: Option<u64> = if p.is_file() {
        // size
        match p.metadata() {
            Ok(m) => Some(m.len()),
            _ => None,
        }
    } else {
        None
    };

    // test 2 , better idiomatically
    let p_len: Option<u64> = p.is_file().then(|| match p.metadata() {
        Ok(m) => m.len(),
        _ => 0, // silly right
    });

    // test 3 , even better idiomatically
    let p_len2 = p.is_file().then(|| p.metadata().and_then(|x| Ok(x.len())));

    println!("p: {:?} len:{:?}, b: {:?}", p, p_size, bp);

    Ok(())
}
