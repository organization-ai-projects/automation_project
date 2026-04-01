#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    RunRequested,
    RunCompleted,
    ToggleGravity,
    ToggleElectromagnetism,
    ToggleStrongNuclear,
    ToggleWeakNuclear,
    ToggleDarkMatter,
    ToggleDarkEnergy,
    ToggleThermodynamics,
    SetSeed(u64),
    SetTicks(u64),
    SetTicksPerEra(u64),
}
