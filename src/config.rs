use std::collections::HashMap;
use tracing::Level;
use crate::consts::env::{DEFAULT_SENSOR_DATUM_PREFIX, ENV_SENSOR_DATUM_PREFIX};

pub struct Settings {
    pub bind_address: String,
    pub log_level: Level,
    pub redis_url: String,
    pub sensor_datum_prefix: String,
}

impl Settings {
    pub fn from_env_vars(vars: &HashMap<String, String>) -> Result<Self, anyhow::Error> {
        let log_level = vars.get(crate::consts::env::LOG_LEVEL_ENV_VAR)
            .map(|s| s.as_str())
            .unwrap_or(crate::consts::env::DEFAULT_LOG_LEVEL)
            .parse()?;
        let bind_address = vars.get(crate::consts::env::BIND_ADDRESS_ENV_VAR)
            .cloned()
            .unwrap_or_else(|| crate::consts::env::DEFAULT_BIND_ADDRESS.to_string());
        let redis_url = vars.get(crate::consts::env::REDIS_URL_ENV_VAR)
            .cloned()
            .unwrap_or_else(|| crate::consts::env::DEFAULT_REDIS_URL.to_string());
        let sensor_datum_prefix = vars.get(ENV_SENSOR_DATUM_PREFIX).cloned().unwrap_or_else(|| DEFAULT_SENSOR_DATUM_PREFIX.to_string());
        Ok(Self { 
            bind_address, 
            log_level, 
            redis_url, 
            sensor_datum_prefix 
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tracing::Level;

    #[test]
    fn settings_from_env_vars_defaults() {
        let vars = HashMap::new();
        let settings = Settings::from_env_vars(&vars).unwrap();
        assert_eq!(settings.bind_address, "0.0.0.0:20120");
        assert_eq!(settings.log_level, Level::INFO);
        assert_eq!(settings.redis_url, "redis://localhost:6379");
        assert_eq!(settings.sensor_datum_prefix, DEFAULT_SENSOR_DATUM_PREFIX);
    }

    #[test]
    fn settings_from_env_vars_custom() {
        let mut vars = HashMap::new();
        vars.insert("LOG_LEVEL".to_string(), "DEBUG".to_string());
        vars.insert("BIND_ADDRESS".to_string(), "127.0.0.1:12345".to_string());
        vars.insert("REDIS_URL".to_string(), "redis://custom:1234".to_string());
        let settings = Settings::from_env_vars(&vars).unwrap();
        assert_eq!(settings.bind_address, "127.0.0.1:12345");
        assert_eq!(settings.log_level, Level::DEBUG);
        assert_eq!(settings.redis_url, "redis://custom:1234");
        assert_eq!(settings.sensor_datum_prefix, DEFAULT_SENSOR_DATUM_PREFIX);
    }

    #[test]
    fn settings_from_env_vars_invalid_log_level() {
        let mut vars = HashMap::new();
        vars.insert("LOG_LEVEL".to_string(), "INVALID".to_string());
        let result = Settings::from_env_vars(&vars);
        assert!(result.is_err());
    }

    #[test]
    fn test_sensor_datum_prefix_custom() {
        let mut vars = HashMap::new();
        vars.insert(ENV_SENSOR_DATUM_PREFIX.to_string(), "customprefix".to_string());
        let settings = Settings::from_env_vars(&vars).unwrap();
        assert_eq!(settings.sensor_datum_prefix, "customprefix");
    }
}
