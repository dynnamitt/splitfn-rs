use chrono::{DateTime, Utc};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
    base_path: Option<std::path::PathBuf>,
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    let p = &args.path;
    let base = &args.base_path;

    // why as_ref()
    let stripped_p = base.as_ref().map(|x| p.strip_prefix(x).ok()).flatten();

    // cannot pass error up (so unwrap sadly)
    let p_len: Option<u64> = p.is_file().then(|| p.metadata().unwrap().len());

    // pass error to main
    let p_created: DateTime<Utc> = p.metadata()?.created().map(|d| d.into())?;

    let p_ext = p.extension();

    println!("p: {:?}, b: {:?}, stripped_p: {:?}", p, base, stripped_p);
    println!("cre: {}, len: {:?}", p_created, p_len);
    println!("{:?}", p_ext);

    Ok(())
}
