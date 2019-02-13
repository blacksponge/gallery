use std::fs;
use std::path::PathBuf;
use crate::config::Config;

pub fn get_album_canonical_path(album_path: PathBuf, config: &Config) -> PathBuf {
    let mut canonical_path = fs::canonicalize(&config.storage_path).unwrap();
    canonical_path.push(album_path);
    canonical_path
}

pub fn is_path_album(path: &PathBuf, config: &Config) -> bool {
    get_album_canonical_path(path.to_path_buf(), config).is_dir()
}

pub fn trim_one_char(s: &String) -> String {
    if s.len() < 2 {
        return s.to_string();
    } else {
        return s[1..(s.len() - 1)].to_string();
    }
}

pub fn get_breadcrumb(path: &PathBuf, config: &Config) -> Vec<(String, String)> {
    let mut bc: Vec<_> = path.iter()
        .filter_map(|e| e.to_str())
        .scan(String::from(""), |state, e| {
            state.push_str("/");
            state.push_str(e);
            Some((state.clone(), String::from(e)))
        })
        .collect();
    bc.insert(0, (String::from("/"), config.gallery_name.clone()));
    bc.pop();
    bc
}
