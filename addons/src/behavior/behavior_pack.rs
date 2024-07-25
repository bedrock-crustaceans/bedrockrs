use bedrockrs_core::SemVer;
use image::RgbaImage;
use uuid::Uuid;

use crate::language::Languages;

//     fn import(path: impl AsRef<Path>) -> Result<Self, AddonError>
//     where
//         Self: Sized,
//     {
//         // Convert the given path into a PathBuf
//         let directory: PathBuf = path.as_ref().to_path_buf();
//
//         let manifest_content =
//             std::fs::read(directory.join("manifest.json")).map_err(|e| todo!())?;
//
//         let text = String::from_utf8(manifest_content).map_err(|e| todo!())?;
//
//         let json: Value = match serde_json::from_str(&text) {
//             Ok(v) => v,
//             Err(e) => {
//                 todo!()
//             }
//         };
//
//         let main_obj = match json {
//             Value::Object(ref v) => v.clone(),
//             other => {
//                 todo!()
//             }
//         };
//
//         let format_version: u32 = match main_obj.get("format_version") {
//             Some(Value::Number(ref v)) => match v.as_u64() {
//                 None => {
//                     todo!()
//                 }
//                 Some(v) => match v.try_into() {
//                     Ok(v) => v,
//                     Err(e) => {
//                         todo!()
//                     }
//                 },
//             },
//             Some(other) => {
//                 todo!()
//             }
//             None => {
//                 todo!()
//             }
//         };
//
//         let header_obj = match main_obj.get("header") {
//             Some(Value::Object(ref v)) => v.clone(),
//             Some(other) => {
//                 todo!()
//             }
//             None => {
//                 todo!()
//             }
//         };
//
//         let name = match header_obj.get("name") {
//             Some(Value::String(v)) => v.clone(),
//             Some(other) => {
//                 todo!()
//             }
//             None => {
//                 todo!()
//             }
//         };
//
//         let description = match header_obj.get("description") {
//             Some(Value::String(v)) => v.clone(),
//             Some(other) => {
//                 todo!()
//             }
//             None => {
//                 todo!()
//             }
//         };
//
//         println!("\"format_version\": {format_version:?}");
//         println!("\"name\":           {name:?}");
//         println!("\"description\":    {description:?}");
//         println!("======================================");
//         println!("{json:#?}");
//
//         todo!()
//     }
//
//     fn export(path: impl AsRef<Path>) -> Result<Self, AddonError>
//     where
//         Self: Sized,
//     {
//         todo!()
//     }
// }
