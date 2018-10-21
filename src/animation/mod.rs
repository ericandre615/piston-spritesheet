use serde_json;

use std::error::Error;
use std::fs::File;
use std::collections::HashMap;
use std::result::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Animation {
    pub name: String,
    pub framerate: u32,
    pub frames: Vec<(f64, f64)>
}

impl Animation {
    pub fn new(name: String, framerate: u32, frames: Vec<(f64, f64)>) -> Animation {
        Animation {
            name,
            framerate,
            frames,
        }
    }

    pub fn load_from_json(filepath: &'static str) -> Result<HashMap<String, Animation>, Box<Error>> {
        let file = File::open(filepath)?;
        let json: HashMap<String, Animation> = serde_json::from_reader(file)?;

        Ok(json)
    }
}
