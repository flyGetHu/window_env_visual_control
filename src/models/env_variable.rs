use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnvScope {
    User,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVariable {
    pub name: String,
    pub value: String,
    pub scope: EnvScope,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Local>,
    pub updated_at: chrono::DateTime<chrono::Local>,
}

impl EnvVariable {
    pub fn new(name: String, value: String, scope: EnvScope) -> Self {
        let now = chrono::Local::now();
        Self {
            name,
            value,
            scope,
            description: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_value(&mut self, new_value: String) {
        self.value = new_value;
        self.updated_at = chrono::Local::now();
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
        self.updated_at = chrono::Local::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVariables {
    pub variables: HashMap<String, EnvVariable>,
}

impl EnvVariables {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn add(&mut self, variable: EnvVariable) {
        self.variables.insert(variable.name.clone(), variable);
    }

    pub fn remove(&mut self, name: &str) -> Option<EnvVariable> {
        self.variables.remove(name)
    }

    pub fn get(&self, name: &str) -> Option<&EnvVariable> {
        self.variables.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut EnvVariable> {
        self.variables.get_mut(name)
    }

    pub fn len(&self) -> usize {
        self.variables.len()
    }

    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &EnvVariable> {
        self.variables.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut EnvVariable> {
        self.variables.values_mut()
    }

    pub fn filter_by_scope(&self, scope: EnvScope) -> Vec<&EnvVariable> {
        self.variables
            .values()
            .filter(|v| v.scope == scope)
            .collect()
    }

    pub fn search(&self, query: &str) -> Vec<&EnvVariable> {
        let query = query.to_lowercase();
        self.variables
            .values()
            .filter(|v| {
                v.name.to_lowercase().contains(&query)
                    || v.value.to_lowercase().contains(&query)
                    || v.description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&query))
                        .unwrap_or(false)
            })
            .collect()
    }
}