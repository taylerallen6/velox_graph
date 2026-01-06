use serde::{Deserialize, Serialize};

// const SETTINGS_FILE_NAME: &str = "vdb_settings.toml";

// INFO: TOML serialized
#[derive(Serialize, Deserialize, Debug)]
pub struct VeloxGraghSettings {
    version: String,
}

impl VeloxGraghSettings {
    pub(crate) fn new() -> VeloxGraghSettings {
        VeloxGraghSettings {
            version: String::from("4.0"),
        }
    }
}
