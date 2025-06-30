use std::collections::HashMap;
use tracing::Level;

pub struct Settings {
    pub bind_address: String,
    pub log_level: Level,
    pub redis_url: String,
}

impl Settings {
    pub fn from_env_vars(vars: &HashMap<String, String>) -> Result<Self, anyhow::Error> {
        let log_level = vars.get("LOG_LEVEL")
            .map(|s| s.as_str())
            .unwrap_or("INFO")
            .parse()?;
        let bind_address = vars.get("BIND_ADDRESS")
            .cloned()
            .unwrap_or_else(|| "0.0.0.0:20120".to_string());
        let redis_url = vars.get("REDIS_URL")
            .cloned()
            .unwrap_or_else(|| "redis://localhost:6379".to_string());
        Ok(Self { bind_address, log_level, redis_url })
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
    }

    #[test]
    fn settings_from_env_vars_invalid_log_level() {
        let mut vars = HashMap::new();
        vars.insert("LOG_LEVEL".to_string(), "INVALID".to_string());
        let result = Settings::from_env_vars(&vars);
        assert!(result.is_err());
    }
}
