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

/// Metadata for a Gothic game variant, used for UI display.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GameMetadata {
    pub title: String,
    pub subtitle: Option<String>,
    pub description: String,
    pub banner_url: String,
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

    pub fn metadata(&self) -> GameMetadata {
        match self {
            Self::Gothic1 => GameMetadata {
                title: "Gothic".to_string(),
                subtitle: None,
                description: "Gothic to kultowe RPG akcji z 2001 roku, osadzone w mrocznym świecie fantasy. Wcielasz się w Bezimiennego, skazańca wrzuconego do Kolonii Karnej — gigantycznego więzienia otoczonego magiczną barierą.".to_string(),
                banner_url: "/banner-archolos.png".to_string(), // Keep placeholders for now
            },
            Self::Gothic2 => GameMetadata {
                title: "Gothic II".to_string(),
                subtitle: None,
                description: "Gothic II kontynuuje przygodę Bezimiennego po upadku Bariery. Miasto Khorinis i otaczające je tereny są zagrożone przez armię ciemności.".to_string(),
                banner_url: "/banner-archolos.png".to_string(),
            },
            Self::Gothic2NotR => GameMetadata {
                title: "Gothic II".to_string(),
                subtitle: Some("Night of the Raven".to_string()),
                description: "Night of the Raven — dodatek do Gothic II, który rozszerza świat gry o Jharkendar. Nowe potwory, przedmioty i questline'y czynią tę wersję definitywnym doświadczeniem Gothic II.".to_string(),
                banner_url: "/banner-archolos.png".to_string(),
            },
            Self::ChroniclesOfMyrtana => GameMetadata {
                title: "Archolos".to_string(),
                subtitle: Some("The Chronicles of Myrtana".to_string()),
                description: "The Chronicles of Myrtana: Archolos to pełnoprawna gra RPG stworzona na silniku Gothic II. Oferuje ponad 100 godzin rozgrywki z nową fabułą i ogromnym światem.".to_string(),
                banner_url: "/banner-archolos.png".to_string(),
            },
            Self::Gothic3 => GameMetadata {
                title: "Gothic 3".to_string(),
                subtitle: None,
                description: "Trzecia część serii Gothic, w której Bezimienny trafia na kontynent Myrtana. Wyzwól krainę spod panowania orków lub sprzymierz się z nimi.".to_string(),
                banner_url: "/banner-archolos.png".to_string(),
            },
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
