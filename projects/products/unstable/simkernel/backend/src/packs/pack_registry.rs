#![allow(dead_code)]
use crate::packs::pack::Pack;
use crate::packs_builtin::pack_digital_pet::DigitalPetPack;
use crate::packs_builtin::pack_hospital::HospitalPack;
use crate::packs_builtin::pack_monster_catcher::MonsterCatcherPack;
use crate::packs_builtin::pack_theme_park::ThemeParkPack;
use std::collections::BTreeMap;

pub struct PackRegistry {
    packs: BTreeMap<String, Box<dyn Pack>>,
}

impl PackRegistry {
    pub fn new() -> Self {
        let mut r = Self {
            packs: BTreeMap::new(),
        };
        r.register(Box::new(HospitalPack));
        r.register(Box::new(ThemeParkPack));
        r.register(Box::new(MonsterCatcherPack));
        r.register(Box::new(DigitalPetPack));
        r
    }

    pub fn register(&mut self, pack: Box<dyn Pack>) {
        self.packs.insert(pack.kind().as_str().to_string(), pack);
    }

    pub fn get_pack(&self, kind: &str) -> Option<&dyn Pack> {
        self.packs.get(kind).map(|p| p.as_ref())
    }

    pub fn list_packs(&self) -> Vec<String> {
        self.packs.keys().cloned().collect()
    }
}

impl Default for PackRegistry {
    fn default() -> Self {
        Self::new()
    }
}
