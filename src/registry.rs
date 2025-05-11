use super::{Package, Version};
use std::collections::HashMap;

pub struct MockRegistry {
    packages: HashMap<String, Vec<Package>>,
}

impl MockRegistry {
    pub fn new() -> Self {
        let mut registry = MockRegistry {
            packages: HashMap::new(),
        };

        registry.add_package(Package {
            name: "react".to_string(),
            version: Version::parse("19.0.0").unwrap(),
            dependencies: HashMap::from([("lodash".to_string(), "^4.17.0".to_string())]),
        });

        registry.add_package(Package {
            name: "lodash".to_string(),
            version: Version::parse("4.17.21").unwrap(),
            dependencies: HashMap::new(),
        });

        registry
    }

    // Добавление пакета
    fn add_package(&mut self, pkg: Package) {
        self.packages.entry(pkg.name.clone()).or_default().push(pkg);
    }

    // Получение всех версий пакета
    pub fn get_versions(&mut self, name: &str) -> Vec<Version> {
        self.packages
            .get(name)
            .map(|pkgs| pkgs.iter().map(|pkg| pkg.version.clone()).collect())
            .unwrap_or_default()
    }

    // Получение конкретной версии пакета
    pub fn get_package(&self, name: &str, version: &Version) -> Option<&Package> {
        self.packages
            .get(name)?
            .iter()
            .find(|pkg| &pkg.version == version)
    }
}
