//! This will provide the logic for loading and parsing the `SOVEREIGN.ini` file

use configparser::ini::Ini;
use std::collections::HashMap;
use anyhow::Result;

pub struct SovereignConfig {
    pub args: HashMap<String, Option<String>>,
}

impl SovereignConfig {
    /// Loads an INI file from the specified path and parses it into a `SovereignConfig`.
    ///
    /// # Arguments
    /// * `path` - A string slice that holds the path to the .ini file.
    ///
    /// # Filtering Logic
    /// * Skips sections named "Launcher" or "default".
    /// * Discards keys with values set to "false" or that are empty.
    /// * Automatically prepends "--" to every key to match CLI standards.
    pub fn load(path: &str) -> Result<Self> {
        let mut ini = Ini::new();
        ini.load(path).map_err(|e| anyhow::anyhow!(e))?;

        let mut args = HashMap::new();
        if let Some(sections) = ini.get_map() {
            for (section, properties) in sections {
                // Ignore non-gameplay sections
                if section == "Launcher" || section == "default" { continue; }
                
                for (key, value) in properties {
                    if let Some(ref v) = value {
                        let v_low = v.to_lowercase();
                        if v_low == "false" || v.is_empty() { continue; }
                    }
                    args.insert(format!("--{}", key), value);
                }
            }
        }
        Ok(SovereignConfig { args })
    }
}