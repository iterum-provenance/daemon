// use serde::{Deserialize, Serialize};
// use std::error::Error;
// use std::fs;

// fn read_file_to_struct<T: Deserialize>(path: String) -> Result<T, Box<Error>> {
//     let string: String = fs::read_to_string(&path)?;
//     let item = serde_json::from_str(&string)?;
//     Ok(item)
// }
