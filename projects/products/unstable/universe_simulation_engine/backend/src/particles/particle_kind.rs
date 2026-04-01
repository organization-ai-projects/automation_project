use serde::{Deserialize, Serialize};

use crate::math::constants;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParticleKind {
    UpQuark,
    DownQuark,
    CharmQuark,
    StrangeQuark,
    TopQuark,
    BottomQuark,
    Electron,
    Muon,
    Tau,
    ElectronNeutrino,
    MuonNeutrino,
    TauNeutrino,
    Photon,
    Gluon,
    WBoson,
    ZBoson,
    Higgs,
    Proton,
    Neutron,
    Hydrogen,
    Helium,
    Lithium,
    Carbon,
    Nitrogen,
    Oxygen,
    Iron,
    DarkMatterParticle,
}

impl ParticleKind {
    pub fn mass_kg(&self) -> f64 {
        match self {
            Self::UpQuark => 3.9e-30,
            Self::DownQuark => 8.5e-30,
            Self::CharmQuark => 2.28e-27,
            Self::StrangeQuark => 1.7e-28,
            Self::TopQuark => 3.08e-25,
            Self::BottomQuark => 7.47e-27,
            Self::Electron => constants::ELECTRON_MASS,
            Self::Muon => 1.883_531_6e-28,
            Self::Tau => 3.167_54e-27,
            Self::ElectronNeutrino | Self::MuonNeutrino | Self::TauNeutrino => 1e-37,
            Self::Photon | Self::Gluon => 0.0,
            Self::WBoson => 1.433e-25,
            Self::ZBoson => 1.625e-25,
            Self::Higgs => 2.228e-25,
            Self::Proton => constants::PROTON_MASS,
            Self::Neutron => constants::NEUTRON_MASS,
            Self::Hydrogen => constants::PROTON_MASS + constants::ELECTRON_MASS,
            Self::Helium => 6.646_476e-27,
            Self::Lithium => 1.152_7e-26,
            Self::Carbon => 1.994e-26,
            Self::Nitrogen => 2.325_9e-26,
            Self::Oxygen => 2.656_8e-26,
            Self::Iron => 9.274e-26,
            Self::DarkMatterParticle => 1e-25,
        }
    }

    pub fn charge(&self) -> f64 {
        let e = constants::ELEMENTARY_CHARGE;
        match self {
            Self::UpQuark | Self::CharmQuark | Self::TopQuark => 2.0 / 3.0 * e,
            Self::DownQuark | Self::StrangeQuark | Self::BottomQuark => -1.0 / 3.0 * e,
            Self::Electron | Self::Muon | Self::Tau => -e,
            Self::Proton | Self::Hydrogen => e,
            Self::WBoson => e,
            Self::ElectronNeutrino
            | Self::MuonNeutrino
            | Self::TauNeutrino
            | Self::Photon
            | Self::Gluon
            | Self::ZBoson
            | Self::Higgs
            | Self::Neutron
            | Self::Helium
            | Self::Lithium
            | Self::Carbon
            | Self::Nitrogen
            | Self::Oxygen
            | Self::Iron
            | Self::DarkMatterParticle => 0.0,
        }
    }

    pub fn is_stable(&self) -> bool {
        matches!(
            self,
            Self::Electron
                | Self::Proton
                | Self::Photon
                | Self::ElectronNeutrino
                | Self::MuonNeutrino
                | Self::TauNeutrino
                | Self::Hydrogen
                | Self::Helium
                | Self::Lithium
                | Self::Carbon
                | Self::Nitrogen
                | Self::Oxygen
                | Self::Iron
                | Self::DarkMatterParticle
        )
    }

    pub fn interacts_electromagnetically(&self) -> bool {
        self.charge().abs() > 0.0
    }

    pub fn interacts_strongly(&self) -> bool {
        matches!(
            self,
            Self::UpQuark
                | Self::DownQuark
                | Self::CharmQuark
                | Self::StrangeQuark
                | Self::TopQuark
                | Self::BottomQuark
                | Self::Gluon
                | Self::Proton
                | Self::Neutron
        )
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::UpQuark => "Up Quark",
            Self::DownQuark => "Down Quark",
            Self::CharmQuark => "Charm Quark",
            Self::StrangeQuark => "Strange Quark",
            Self::TopQuark => "Top Quark",
            Self::BottomQuark => "Bottom Quark",
            Self::Electron => "Electron",
            Self::Muon => "Muon",
            Self::Tau => "Tau",
            Self::ElectronNeutrino => "Electron Neutrino",
            Self::MuonNeutrino => "Muon Neutrino",
            Self::TauNeutrino => "Tau Neutrino",
            Self::Photon => "Photon",
            Self::Gluon => "Gluon",
            Self::WBoson => "W Boson",
            Self::ZBoson => "Z Boson",
            Self::Higgs => "Higgs Boson",
            Self::Proton => "Proton",
            Self::Neutron => "Neutron",
            Self::Hydrogen => "Hydrogen",
            Self::Helium => "Helium",
            Self::Lithium => "Lithium",
            Self::Carbon => "Carbon",
            Self::Nitrogen => "Nitrogen",
            Self::Oxygen => "Oxygen",
            Self::Iron => "Iron",
            Self::DarkMatterParticle => "Dark Matter",
        }
    }
}
