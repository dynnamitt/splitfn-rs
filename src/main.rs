use chrono::{DateTime, Utc};
use clap::Parser;
use std::io;
use std::{
    path::{Path, PathBuf, MAIN_SEPARATOR},
    time::SystemTime,
};

#[derive(Parser)]
struct Cli {
    base_path: Option<PathBuf>,
}

const BAD_FILE_NAME: &str = "[Name-less]";
// const FALLBACK

#[derive(Debug, Default, serde::Serialize)]
struct ReOrganized<'a> {
    stem: &'a str,
    size: Option<u64>,
    created: Option<DateTime<Utc>>,
    parent_parts: Vec<&'a str>,
    ext: Option<&'a str>,
}
impl<'a> ReOrganized<'a> {
    fn new(p: &'a Path, base: Option<&PathBuf>) -> Self {
        // pass error
        //
        let created: Option<SystemTime> = p.metadata().and_then(|m| m.created()).ok();

        let stripped_p = base.and_then(|x| p.strip_prefix(x).ok());
        let min_p = stripped_p.unwrap_or(p);
        let stem = min_p
            .file_stem()
            .and_then(|f_st| f_st.to_str())
            .unwrap_or(BAD_FILE_NAME);

        let size = p
            .is_file()
            .then(|| p.metadata().ok().map(|meta| meta.len()))
            .flatten();

        let parent = min_p.parent().and_then(|par| par.to_str());
        let parent = parent.map(|par_p| par_p.split(MAIN_SEPARATOR).collect());
        let parent_parts = parent.unwrap_or(vec![]);

        let ext = p.extension().and_then(|ext| ext.to_str());

        Self {
            stem,
            size,
            created: created.map(|st| st.into()),
            parent_parts,
            ext,
        }
    }
}

fn main() {
    let args = Cli::parse();

    let base = &args.base_path;

    for line in io::stdin().lines() {
        if let Ok(line) = line {
            let p = PathBuf::from(line);
            let my_meta = ReOrganized::new(p.as_path(), base.as_ref());

            let serialized = serde_json::to_string(&my_meta).unwrap();
            println!("{}", serialized);
            // println!("{:?}", my_meta);
        } else {
            unimplemented!();
        }

        // let ro = ReOrganized::new(p, base.as_ref());
    }
}
