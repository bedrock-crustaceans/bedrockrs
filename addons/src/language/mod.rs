use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use crate::error::AddonError;
use crate::error::AddonError::{FormatError, IOError, JsonError};
use crate::language::code::LanguageCode;

mod code;

#[derive(Debug, Clone)]
pub struct Languages {
    texts: HashMap<String, HashMap<String, String>>,
    codes: Vec<LanguageCode>,
}

impl Languages {
    pub fn import(languages_path: PathBuf) -> Result<Self, AddonError> {
        let mut texts = HashMap::new();
        let codes = vec![];

        if languages_path.is_dir() {
            let languages_file_path = languages_path.join("languages.json");

            let languages_file = fs::read_to_string(&languages_file_path)
                .map_err(|e| IOError(Arc::new(e), languages_file_path.clone()))?;
            let languages_file: Vec<LanguageCode> = serde_json::from_str(&languages_file)
                .map_err(|e| JsonError(Arc::new(e), languages_file_path))?;

            // Parse all the language data in all language files
            for language in languages_file {
                let language_code = match language {
                    LanguageCode::VanillaCode(v) => v,
                    LanguageCode::CustomCode(v) => v.x,
                };

                let language_path = languages_path.join(format!("{language_code}.lang"));

                let language = fs::read_to_string(&language_path)
                    .map_err(|e| IOError(Arc::new(e), language_path.clone()))?;

                let mut language_data = HashMap::new();

                'language_lines: for (line_index, line) in language.lines().enumerate() {
                    // If line is empty continue
                    if line.trim().is_empty() {
                        continue 'language_lines;
                    }

                    // Translation comments start with `##` ignore them
                    if line.starts_with("##") {
                        continue 'language_lines;
                    };

                    // Remove possible inline comments
                    let line = match line.split_once("\t##") {
                        None => line,
                        Some((v, _)) => v,
                    };

                    // Split identifier and value
                    let (id, val) = match line.split_once("=") {
                        None => Err(FormatError {
                            message: String::from("Language data must be split by a \"=\""),
                            path: language_path.clone(),
                            line: Some(line_index),
                            column: None,
                        }),
                        Some(v) => Ok(v),
                    }?;

                    language_data.insert(id.to_string(), val.to_string());
                }

                texts.insert(language_code, language_data);
            }
        }

        Ok(Self { texts, codes })
    }
}
