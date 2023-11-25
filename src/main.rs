use chrono::{DateTime, Utc};
use clap::Parser;
use std::fs::DirEntry;
use std::io;
use std::{
    path::{Path, PathBuf, MAIN_SEPARATOR},
    time::SystemTime,
};

mod exts;
use exts::IMG_EXT;

#[derive(Parser)]
struct Cli {
    base_path: Option<PathBuf>,
}

const BAD_FILE_NAME: &str = "[Name-less]";

// TODO: regex
const SEP: &str = "[\\.\\-_]"; // TODO: escape like r"" in python

// const FALLBACK
//
fn media_file(ent: &Result<DirEntry, std::io::Error>) -> bool {
    ent.as_ref().map_or(false, |dir_ent| {
        dir_ent
            .path()
            .extension()
            .and_then(|ext| ext.to_str())
            .map_or(false, |ext| IMG_EXT.contains(&ext))
    })
}

#[derive(Debug, Default, serde::Serialize)]
struct ReOrganized<'a> {
    stem: &'a str,
    size: Option<u64>,
    created: Option<DateTime<Utc>>,
    contains: Option<usize>,
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

        let contains = p
            .is_dir()
            .then(|| {
                p.read_dir()
                    .map(|rdir| rdir.filter(media_file).count())
                    .ok()
            })
            .flatten();
        // let files_below = files_below.and_then

        let parent = min_p.parent().and_then(|par| par.to_str());
        let parent = parent.map(|par_p| par_p.split(MAIN_SEPARATOR).collect());
        let parent_parts = parent.unwrap_or_default();

        let ext = p.extension().and_then(|ext| ext.to_str());

        Self {
            stem,
            size,
            contains,
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basics() {
        let base = PathBuf::from("/b");
        let f_path = PathBuf::from("/b/c/zzz.yyy.zz");
        let metas = ReOrganized::new(f_path.as_path(), Some(&base));

        assert!(metas.ext == Some("zz"));
        assert!(metas.stem == "zzz.yyy");
        assert!(metas.created == None);
        assert!(metas.size == None);
        assert_eq!(metas.parent_parts, vec!["c"]);
    }
    #[test]
    fn regex() {
        // TODO: implement REGEX_TRIO w SEP-regex ala python mclass project
        assert!(false);
    }
}
