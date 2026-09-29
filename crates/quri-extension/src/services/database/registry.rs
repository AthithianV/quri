#![allow(dead_code)]

use super::{
    mysql::MySqlPlugin, postgres::PostgresPlugin, sqlite::SqlitePlugin, traits::DatabasePlugin,
};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone, Default)]
pub struct DatabasePluginRegistry {
    plugins: HashMap<&'static str, Arc<dyn DatabasePlugin>>,
}

impl DatabasePluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn register<P>(&mut self, plugin: P)
    where
        P: DatabasePlugin + 'static,
    {
        self.plugins.insert(plugin.db_type(), Arc::new(plugin));
    }

    pub fn get(&self, db_type: &str) -> Option<Arc<dyn DatabasePlugin>> {
        self.plugins.get(db_type).cloned()
    }

    pub fn available_plugins(&self) -> Vec<&'static str> {
        let mut plugins = self.plugins.keys().copied().collect::<Vec<_>>();
        plugins.sort_unstable();
        plugins
    }
}

pub fn create_plugin_registry() -> DatabasePluginRegistry {
    let mut registry = DatabasePluginRegistry::new();

    registry.register(PostgresPlugin);
    registry.register(MySqlPlugin);
    registry.register(SqlitePlugin);

    registry
}
