use pubgrub::{
    resolve,
    DependencyProvider,
    DefaultStringReporter,
    Ranges,
    SemanticVersion,
    Dependencies,
    DependencyConstraints,
    PackageResolutionStatistics,
    PubGrubError,
};
use pubgrub::Reporter;
use std::collections::HashMap;
use std::str::FromStr;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PackageName(String);

impl fmt::Display for PackageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PackageVersion(SemanticVersion);

impl fmt::Display for PackageVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
struct Package {
    version: SemanticVersion,
    dependencies: Vec<(PackageName, Ranges<SemanticVersion>)>,
}

#[derive(Debug, Default)]
struct MockRepo {
    packages: HashMap<PackageName, Vec<Package>>,
}

#[derive(Debug)]
struct RepoError(String);

impl fmt::Display for RepoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for RepoError {}

impl DependencyProvider for MockRepo {
    type P = PackageName;
    type V = SemanticVersion;
    type VS = Ranges<SemanticVersion>;
    type M = String;
    type Err = RepoError;
    type Priority = usize;

    fn choose_version(&self, name: &Self::P, range: &Self::VS) -> Result<Option<Self::V>, Self::Err> {
        let candidates = self.packages.get(name).ok_or_else(|| RepoError("Package not found".to_string()))?;
        let mut versions: Vec<_> = candidates.iter().map(|pkg| pkg.version.clone()).collect();
        versions.sort();
        versions.reverse();
        for v in versions {
            if range.contains(&v) {
                return Ok(Some(v));
            }
        }
        Ok(None)
    }

    fn prioritize(&self, _name: &Self::P, range: &Self::VS, _stats: &PackageResolutionStatistics) -> Self::Priority {
        // Приоритет: чем меньше диапазон, тем выше приоритет (меньше вариантов)
        // Используем количество сегментов диапазона
        range.iter().count()
    }

    fn get_dependencies(&self, name: &Self::P, version: &Self::V) -> Result<Dependencies<Self::P, Self::VS, Self::M>, Self::Err> {
        let candidates = self.packages.get(name).ok_or_else(|| RepoError("Package not found".to_string()))?;
        for pkg in candidates {
            if &pkg.version == version {
                let mut deps = DependencyConstraints::default();
                for (dep_name, dep_range) in &pkg.dependencies {
                    deps.insert(dep_name.clone(), dep_range.clone());
                }
                return Ok(Dependencies::Available(deps));
            }
        }
        Ok(Dependencies::Unavailable("Version not found".to_string()))
    }
}

fn version(v: &str) -> SemanticVersion {
    SemanticVersion::from_str(v).unwrap()
}

fn range(s: &str) -> Ranges<SemanticVersion> {
    Ranges::singleton(version(s))
}

fn main() {
    let mut repo = MockRepo::default();

    let foo = PackageName("foo".into());
    let bar = PackageName("bar".into());

    repo.packages.insert(
        foo.clone(),
        vec![Package {
            version: version("1.0.0"),
            dependencies: vec![(bar.clone(), range("2.0.0"))],
        }],
    );

    repo.packages.insert(
        bar.clone(),
        vec![
            Package {
                version: version("1.0.0"),
                dependencies: vec![],
            },
            Package {
                version: version("2.0.0"),
                dependencies: vec![],
            },
        ],
    );

    match resolve(&repo, foo.clone(), version("1.0.0")) {
        Ok(solution) => {
            println!("Resolved dependencies:");
            for (pkg, ver) in solution {
                println!("  {}: {}", pkg, ver);
            }
        }
        Err(PubGrubError::NoSolution(mut tree)) => {
            tree.collapse_no_versions();
            eprintln!("{}", <DefaultStringReporter as Reporter<PackageName, Ranges<SemanticVersion>, String>>::report(&tree));
        }
        Err(e) => panic!("{:?}", e),
    }
}
