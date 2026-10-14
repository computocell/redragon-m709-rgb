use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::types::{MouseConfig, ProfileConfig};

#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("TOML deserialization error: {0}")]
    TomlDeserialize(#[from] toml::de::Error),
    #[error("Profile not found: {0}")]
    NotFound(String),
}

/// Represents an exported profile file in TOML format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileFile {
    pub name: String,
    pub description: Option<String>,
    pub profile: ProfileConfig,
}

pub struct ProfileManager {
    base_dir: PathBuf,
}

impl ProfileManager {
    /// Initializes ProfileManager with standard XDG config directory
    pub fn default_dir() -> PathBuf {
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(xdg).join("redragon-mouse")
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config").join("redragon-mouse")
        } else {
            PathBuf::from(".config").join("redragon-mouse")
        }
    }

    pub fn new() -> Result<Self, ProfileError> {
        let base_dir = Self::default_dir();
        fs::create_dir_all(&base_dir)?;
        Ok(Self { base_dir })
    }

    pub fn with_dir<P: AsRef<Path>>(path: P) -> Result<Self, ProfileError> {
        let base_dir = path.as_ref().to_path_buf();
        fs::create_dir_all(&base_dir)?;
        Ok(Self { base_dir })
    }

    pub fn dir(&self) -> &Path {
        &self.base_dir
    }

    /// Lists all `.toml` profile names in the configuration directory
    pub fn list_profiles(&self) -> Result<Vec<String>, ProfileError> {
        let mut profiles = Vec::new();
        if self.base_dir.exists() {
            for entry in fs::read_dir(&self.base_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("toml") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        profiles.push(stem.to_string());
                    }
                }
            }
        }
        profiles.sort();
        Ok(profiles)
    }

    /// Saves a single profile configuration with a given name
    pub fn save_profile(
        &self,
        name: &str,
        description: Option<&str>,
        profile: &ProfileConfig,
    ) -> Result<PathBuf, ProfileError> {
        let file = ProfileFile {
            name: name.to_string(),
            description: description.map(|d| d.to_string()),
            profile: profile.clone(),
        };

        let content = toml::to_string_pretty(&file)?;
        let path = self.base_dir.join(format!("{}.toml", name));
        fs::write(&path, content)?;
        Ok(path)
    }

    /// Loads a profile configuration by name
    pub fn load_profile(&self, name: &str) -> Result<ProfileConfig, ProfileError> {
        let path = self.base_dir.join(format!("{}.toml", name));
        if !path.exists() {
            return Err(ProfileError::NotFound(name.to_string()));
        }

        let content = fs::read_to_string(&path)?;
        let file: ProfileFile = toml::from_str(&content)?;
        Ok(file.profile)
    }

    /// Saves full backup of all 5 profiles and active profile
    pub fn save_full_backup(&self, config: &MouseConfig) -> Result<PathBuf, ProfileError> {
        let path = self.base_dir.join("hardware_backup.toml");
        let content = toml::to_string_pretty(config)?;
        fs::write(&path, content)?;
        Ok(path)
    }

    /// Loads full backup of all 5 profiles and active profile
    pub fn load_full_backup(&self) -> Result<MouseConfig, ProfileError> {
        let path = self.base_dir.join("hardware_backup.toml");
        if !path.exists() {
            return Err(ProfileError::NotFound("hardware_backup".to_string()));
        }
        let content = fs::read_to_string(&path)?;
        let config: MouseConfig = toml::from_str(&content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_save_and_load() {
        let unique_dir = std::env::temp_dir().join(format!("redragon_test_{}", std::process::id()));
        let manager = ProfileManager::with_dir(&unique_dir).unwrap();

        let mut prof = ProfileConfig::default();
        prof.brightness = 1;
        prof.speed = 5;

        manager.save_profile("test_gaming", Some("Gaming preset"), &prof).unwrap();

        let list = manager.list_profiles().unwrap();
        assert_eq!(list, vec!["test_gaming"]);

        let loaded = manager.load_profile("test_gaming").unwrap();
        assert_eq!(loaded.brightness, 1);
        assert_eq!(loaded.speed, 5);

        let _ = std::fs::remove_dir_all(unique_dir);
    }
}
