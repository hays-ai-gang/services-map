use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use crate::model::{ServiceDefinition, ServicesMap};

pub struct AppState {
    services: RwLock<Vec<ServiceDefinition>>,
    file_path: PathBuf,
}

impl AppState {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        let file_path = file_path.into();
        let services = Self::load_from_file(&file_path).unwrap_or_default();
        Self {
            services: RwLock::new(services),
            file_path,
        }
    }

    fn load_from_file(path: &Path) -> Option<Vec<ServiceDefinition>> {
        let data = std::fs::read_to_string(path).ok()?;
        let map: ServicesMap = serde_json::from_str(&data).ok()?;
        Some(map.services)
    }

    fn save(&self) {
        let services = self.services.read().unwrap();
        let map = ServicesMap {
            services: services.clone(),
        };
        if let Ok(json) = serde_json::to_string_pretty(&map) {
            let _ = std::fs::write(&self.file_path, json);
        }
    }

    pub fn get_all(&self, filters: &HashMap<String, String>) -> ServicesMap {
        let services = self.services.read().unwrap();
        let filtered: Vec<ServiceDefinition> = if filters.is_empty() {
            services.clone()
        } else {
            services
                .iter()
                .filter(|s| filters.iter().all(|(k, v)| s.metadata.get(k) == Some(v)))
                .cloned()
                .collect()
        };
        ServicesMap { services: filtered }
    }

    pub fn replace_all(&self, map: ServicesMap) {
        let mut services = self.services.write().unwrap();
        *services = map.services;
        drop(services);
        self.save();
    }

    pub fn get_by_name(&self, name: &str) -> Option<ServiceDefinition> {
        let services = self.services.read().unwrap();
        services.iter().find(|s| s.name == name).cloned()
    }

    pub fn create(&self, service: ServiceDefinition) -> Result<(), String> {
        let mut services = self.services.write().unwrap();
        if services.iter().any(|s| s.name == service.name) {
            let name = service.name;
            return Err(format!("Service '{name}' already exists"));
        }
        services.push(service);
        drop(services);
        self.save();
        Ok(())
    }

    pub fn update(&self, name: &str, service: ServiceDefinition) -> Result<(), String> {
        let mut services = self.services.write().unwrap();
        let idx = services
            .iter()
            .position(|s| s.name == name)
            .ok_or_else(|| format!("Service '{name}' not found"))?;
        services[idx] = service;
        drop(services);
        self.save();
        Ok(())
    }

    pub fn delete(&self, name: &str) -> Result<(), String> {
        let mut services = self.services.write().unwrap();
        let idx = services
            .iter()
            .position(|s| s.name == name)
            .ok_or_else(|| format!("Service '{name}' not found"))?;
        services.remove(idx);
        drop(services);
        self.save();
        Ok(())
    }
}
