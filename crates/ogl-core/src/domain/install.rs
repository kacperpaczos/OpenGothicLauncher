use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// All Gothic game variants that OpenGothicLauncher can detect and launch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GothicGame {
    Gothic1,
    Gothic2,
    Gothic2NotR,
    ChroniclesOfMyrtana,
    Gothic3,
}

impl GothicGame {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Gothic1 => "Gothic",
            Self::Gothic2 => "Gothic II",
            Self::Gothic2NotR => "Gothic II: Night of the Raven",
            Self::ChroniclesOfMyrtana => "The Chronicles of Myrtana: Archolos",
            Self::Gothic3 => "Gothic 3",
        }
    }

    pub fn profile_id(&self) -> String {
        format!("{:?}", self)
    }

    pub fn all_variants() -> Vec<GothicGame> {
        vec![
            GothicGame::Gothic1,
            GothicGame::Gothic2,
            GothicGame::Gothic2NotR,
            GothicGame::ChroniclesOfMyrtana,
            GothicGame::Gothic3,
        ]
    }
}

impl std::str::FromStr for GothicGame {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Gothic1" => Ok(Self::Gothic1),
            "Gothic2" => Ok(Self::Gothic2),
            "Gothic2NotR" => Ok(Self::Gothic2NotR),
            "ChroniclesOfMyrtana" => Ok(Self::ChroniclesOfMyrtana),
            "Gothic3" => Ok(Self::Gothic3),
            _ => Err(()),
        }
    }
}

/// A validated installation found on disk.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GothicInstall {
    pub game: GothicGame,
    pub root_path: PathBuf,
}
