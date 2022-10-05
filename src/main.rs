#![warn(clippy::nursery, clippy::pedantic)]

use std::{env, fs};

pub mod args;

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum SystemManager {
    Pacman,
    Apt,
    Pip,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum LanguageManager {
    Cargo,
    Yarn,
    Npm,
    Poetry,
}

#[derive(Debug, Clone)]
enum PackageManager {
    System(SystemManager),
    Language(LanguageManager),
}

fn acquire_dir_files() -> Option<Vec<String>> {
    if let Ok(path) = env::current_dir() {
        Some(
            fs::read_dir(path)
                .unwrap()
                .map(|res| res.map(|e| e.path()))
                .map(|e| e.unwrap().into_os_string().into_string().unwrap())
                .collect::<Vec<String>>(),
        )
    } else {
        None
    }
}

fn main() {
    let manager =
        acquire_dir_files().map_or(PackageManager::System(SystemManager::Pacman), |dir_files| {
            // For now, the first file it comes across will be the configured package manager.
            // Later, configuration options will be implemented for certain paths.
            let matcher = |v: &Vec<String>, s: &str| -> bool { v.contains(&s.to_string()) };

            if matcher(&dir_files, "Cargo.toml") {
                PackageManager::Language(LanguageManager::Cargo)
            } else if matcher(&dir_files, "Yarn.lock") {
                PackageManager::Language(LanguageManager::Yarn)
            } else if matcher(&dir_files, "package-lock.json") {
                PackageManager::Language(LanguageManager::Npm)
            } else {
                PackageManager::Language(LanguageManager::Poetry)
            }
        });

    println!("{:?}", manager);

    std::process::exit(0)
}
