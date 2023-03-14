use chrono::{DateTime, Utc};
use clap::Parser;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

#[derive(Parser)]
struct Cli {
    path: PathBuf,
    base_path: Option<PathBuf>,
}

const BAD_FILE_NAME: &str = "[Name-less]";

#[derive(Debug)]
struct ReOrganized<'a> {
    stem: &'a str,
    size: Option<u64>,
    created: DateTime<Utc>,
    parent_parts: Option<Vec<&'a str>>,
    ext: Option<&'a str>,
}
impl<'a> ReOrganized<'a> {
    fn new(p: &'a Path, base: Option<&PathBuf>) -> Result<Self, std::io::Error> {
        // pass error
        //
        // TODO: make this better
        let created: DateTime<Utc> = p.metadata()?.created().map(|d| d.into())?;

        let stripped_p = base.and_then(|x| p.strip_prefix(x).ok());
        let min_p = stripped_p.unwrap_or(p);
        let stem = min_p
            .file_stem()
            .and_then(|f_st| f_st.to_str())
            .unwrap_or(BAD_FILE_NAME);

        // TODO: not sure if this unwrap is smart
        // Is there a better fallback-val ??
        let size = p.is_file().then(|| p.metadata().unwrap().len());

        let parent_path = min_p.parent().and_then(|par| par.to_str());
        let parent_parts = parent_path.map(|par_p| par_p.split(MAIN_SEPARATOR).collect());
        let ext = p.extension().and_then(|ext| ext.to_str());

        Ok(Self {
            stem,
            size,
            created,
            parent_parts,
            ext,
        })
    }
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    let p = &args.path;
    let base = &args.base_path;

    let ro = ReOrganized::new(p, base.as_ref())?;

    println!("{:?}", ro);

    Ok(())
}
