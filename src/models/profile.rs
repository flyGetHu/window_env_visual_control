use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Local};

use super::env_variable::{EnvVariable, EnvScope};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvProfile {
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub variables: HashMap<String, String>,
    pub scope: EnvScope,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl EnvProfile {
    pub fn new(name: String, scope: EnvScope) -> Self {
        let now = Local::now();
        Self {
            name,
            description: None,
            enabled: false,
            variables: HashMap::new(),
            scope,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.updated_at = Local::now();
    }

    pub fn add_variable(&mut self, name: String, value: String) {
        self.variables.insert(name, value);
        self.updated_at = Local::now();
    }

    pub fn remove_variable(&mut self, name: &str) -> Option<String> {
        let result = self.variables.remove(name);
        if result.is_some() {
            self.updated_at = Local::now();
        }
        result
    }

    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.variables.get(name)
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
        self.updated_at = Local::now();
    }

    pub fn iter_variables(&self) -> impl Iterator<Item = (&String, &String)> {
        self.variables.iter()
    }

    pub fn len(&self) -> usize {
        self.variables.len()
    }

    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }

    pub fn to_env_variables(&self) -> Vec<EnvVariable> {
        self.variables
            .iter()
            .map(|(name, value)| EnvVariable::new(
                name.clone(),
                value.clone(),
                self.scope.clone(),
            ))
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvProfiles {
    pub profiles: HashMap<String, EnvProfile>,
}

impl EnvProfiles {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
        }
    }

    pub fn add(&mut self, profile: EnvProfile) {
        self.profiles.insert(profile.name.clone(), profile);
    }

    pub fn remove(&mut self, name: &str) -> Option<EnvProfile> {
        self.profiles.remove(name)
    }

    pub fn get(&self, name: &str) -> Option<&EnvProfile> {
        self.profiles.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut EnvProfile> {
        self.profiles.get_mut(name)
    }

    pub fn len(&self) -> usize {
        self.profiles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &EnvProfile> {
        self.profiles.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut EnvProfile> {
        self.profiles.values_mut()
    }

    pub fn enabled_profiles(&self) -> Vec<&EnvProfile> {
        self.profiles
            .values()
            .filter(|p| p.enabled)
            .collect()
    }

    pub fn disabled_profiles(&self) -> Vec<&EnvProfile> {
        self.profiles
            .values()
            .filter(|p| !p.enabled)
            .collect()
    }

    pub fn search(&self, query: &str) -> Vec<&EnvProfile> {
        let query = query.to_lowercase();
        self.profiles
            .values()
            .filter(|p| {
                p.name.to_lowercase().contains(&query)
                    || p.description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&query))
                        .unwrap_or(false)
            })
            .collect()
    }

    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let profiles: Self = serde_yaml::from_str(&content)?;
        Ok(profiles)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_yaml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}