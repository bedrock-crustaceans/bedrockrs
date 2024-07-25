use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::AddonError;

#[derive(Debug, Clone)]
pub struct Languages(HashMap<String, (Option<String>, HashMap<String, String>)>);

impl Languages {
    pub fn import(languages_path: PathBuf) -> Result<Self, AddonError> {
        todo!()

        // let data = match fs::read(language_file_path) {
        //     Ok(v) => v,
        //     Err(e) => {
        //         todo!()
        //     }
        // };
        //
        // let lines = match String::from_utf8(data) {
        //     Ok(v) => v,
        //     Err(e) => {
        //         todo!()
        //     }
        // };
        //
        // let mut map = HashMap::new();
        //
        // let line_iter = lines.lines();
        //
        // 'lines: for line in line_iter {
        //     // If line is empty continue
        //     if line.trim().is_empty() {
        //         continue 'lines;
        //     }
        //
        //     // Translation comments start with `##` ignore them
        //     if line.starts_with("##") {
        //         continue 'lines;
        //     };
        //
        //     // Remove possible inline comments
        //     let line = match line.split_once("\t##") {
        //         None => line,
        //         Some((v, _)) => v,
        //     };
        //
        //     // Split identifier and value
        //     let (id, val) = match line.split_once("=") {
        //         None => {
        //             todo!()
        //         }
        //         Some((i, v)) => (String::from(i), String::from(v)),
        //     };
        //
        //     map.insert(id, val);
        // }
        //
        // Ok(Self(map))
    }

    pub fn languages(&self) -> Vec<String> {
        self.0.keys().map(|f| f.clone()).collect()
    }
}

