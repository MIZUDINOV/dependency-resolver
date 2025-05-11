pub mod registry;

use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

// Ошибки библиотеки
#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("Version not found: {0}")]
    VersionNotFound(String),
    #[error("No solution found")]
    NoSolution,
}

// Описание пакета (упрощённый аналог package.json)
#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub dependencies: HashMap<String, String>, // Имя пакета → SemVer-строка ("^1.0.0")
}

// Результат разрешения
#[derive(Debug, Serialize)]
pub struct ResolutionResult {
    pub packages: HashMap<String, Version>, // Имя пакета → точная версия
}
