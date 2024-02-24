use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub workweek: WorkWeekConfig,
}

#[derive(Debug, Deserialize)]
pub struct WorkWeekConfig {
    pub hours: u32,
}

#[derive(Debug)]
pub enum ConfigError {
    OpenFile(PathBuf, std::io::Error),
    InvalidFile(toml::de::Error),
}

pub fn read_config_from_str(str: &str) -> Result<Config, ConfigError> {
    Ok(toml::from_str(&str).map_err(|e| ConfigError::InvalidFile(e))?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_workweek_hours() {
        let config = read_config_from_str(
            r"
            [workweek]
            hours = 30
            ",
        )
        .unwrap();

        assert_eq!(config.workweek.hours, 30);
    }
}
