use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RequestPayload {
    LoadScenario {
        scenario: String,
    },
    NewRun {
        #[serde(deserialize_with = "deserialize_u64")]
        seed: u64,
    },
    EncounterStep,
    StartEncounter,
    CaptureAttempt,
    StartBattle,
    BattleAction {
        action: String,
    },
    BattleStep,
    EndBattle,
    GetSnapshot,
    GetReport,
    SaveReplay,
    LoadReplay {
        replay: String,
    },
    ReplayToEnd,
    Shutdown,
}

fn deserialize_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    struct U64Visitor;
    impl<'de> Visitor<'de> for U64Visitor {
        type Value = u64;
        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a non-negative integer")
        }
        fn visit_u64<E>(self, value: u64) -> Result<u64, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
        fn visit_i64<E>(self, value: i64) -> Result<u64, E>
        where
            E: de::Error,
        {
            if value < 0 {
                return Err(E::custom("negative value not allowed"));
            }
            Ok(value as u64)
        }
        fn visit_f64<E>(self, value: f64) -> Result<u64, E>
        where
            E: de::Error,
        {
            if value < 0.0 || value.fract() != 0.0 {
                return Err(E::custom("non-integer or negative float"));
            }
            Ok(value as u64)
        }
    }
    deserializer.deserialize_any(U64Visitor)
}
