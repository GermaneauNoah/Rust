use clap::Parser;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum StorageType {
    File,
    Memory,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum LanguageType {
    Fr,
    En,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ServiceType {
    Stdio,
    Udp,
    Web,
}

pub const PSEUDO_CANDIDATS: [&str; 2] = ["blanc", "nul"];

#[derive(Debug, Clone)]
pub struct Configuration {
    candidats: Vec<String>,
    storage: StorageType,
    language: LanguageType,
    service: ServiceType,
    port: u16,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Parameters {
    #[arg(
        short = 'c',
        long = "candidates",
        value_name = "CANDIDAT",
        num_args = 1..,
        required = true
    )]
    pub candidates: Vec<String>,

    #[arg(
        short = 's',
        long = "storage",
        value_name = "STORAGE",
        default_value = "memory"
    )]
    pub storage: StorageType,

    #[arg(
        short = 'l',
        long = "language",
        value_name = "LANGUAGE",
        default_value = "fr"
    )]
    pub language: LanguageType,

    #[arg(
        short = 'S',
        long = "service",
        value_name = "SERVICE",
        default_value = "stdio"
    )]
    pub service: ServiceType,

    #[arg(
        short = 'p',
        long = "port",
        value_name = "PORT",
        default_value = "8080"
    )]
    pub port: u16,
}

pub fn charger_configuration() -> Configuration {
    let params = Parameters::parse();
    Configuration::new(params.candidates, params.storage, params.language, params.service, params.port)
}

impl Configuration {
    pub fn new(candidats: Vec<String>, storage: StorageType, language: LanguageType, service: ServiceType, port: u16) -> Self {
        let mut uniques: Vec<String> = Vec::new();
        for candidat in candidats {
            let candidat = candidat.trim().to_lowercase();
            if candidat.is_empty() {
                continue;
            }
            if PSEUDO_CANDIDATS.iter().any(|p| *p == candidat) {
                continue;
            }
            if !uniques.contains(&candidat) {
                uniques.push(candidat);
            }
        }
        Self { candidats: uniques, storage, language, service, port }
    }

    pub fn storage(&self) -> StorageType {
        self.storage
    }

    pub fn language(&self) -> LanguageType {
        self.language
    }

    pub fn service(&self) -> ServiceType {
        self.service
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn candidats_reels(&self) -> Vec<String> {
        self.candidats.clone()
    }

    pub fn candidats_affichage(&self) -> Vec<String> {
        let mut liste = self.candidats.clone();
        for pseudo in PSEUDO_CANDIDATS {
            let pseudo = pseudo.to_string();
            if !liste.contains(&pseudo) {
                liste.push(pseudo);
            }
        }
        liste
    }
}
