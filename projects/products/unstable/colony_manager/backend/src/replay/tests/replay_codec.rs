use std::{env, fs, process};

use crate::{
    replay::{replay_codec::ReplayCodec, replay_file::ReplayFile},
    rng::{rng_draw::RngDraw, seed::Seed},
};

#[test]
fn replay_codec_preserves_large_u64_draws() -> Result<(), String> {
    let path = env::temp_dir().join(format!(
        "colony_manager_replay_codec_{}.json",
        process::id()
    ));
    let replay = ReplayFile {
        seed: Seed(42),
        ticks: 2,
        scenario_name: "hauling_basic".to_string(),
        rng_draws: vec![
            RngDraw {
                raw_value: u64::MAX - 3,
                resolved_index: 0,
            },
            RngDraw {
                raw_value: 18_446_744_073_709_551_000,
                resolved_index: 1,
            },
        ],
    };

    ReplayCodec::save(&replay, &path).map_err(|e| e.to_string())?;
    let loaded = ReplayCodec::load(&path).map_err(|e| e.to_string())?;
    if fs::remove_file(&path).is_err() {
        // Best-effort cleanup for temp artifact.
    }

    if loaded.rng_draws != replay.rng_draws {
        return Err("replay codec lost RNG draw precision".to_string());
    }
    Ok(())
}
