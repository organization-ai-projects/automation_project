// projects/products/code_agent_sandbox/src/engine/path_rights.rs
pub const READ: u8 = 0b0001; // Bit 0
pub const WRITE: u8 = 0b0010; // Bit 1
pub const FORBIDDEN: u8 = 0b0100; // Bit 2

#[derive(Debug)]
pub struct PathRights {
    pub path: &'static str,
    pub rights: u8, // Bitmask des droits
}

pub const PATH_RIGHTS: &[PathRights] = &[
    PathRights {
        path: "src/**",
        rights: READ | WRITE,
    },
    PathRights {
        path: "tests/**",
        rights: READ | WRITE,
    },
    PathRights {
        path: "examples/**",
        rights: READ | WRITE,
    },
    PathRights {
        path: "benches/**",
        rights: READ | WRITE,
    },
    PathRights {
        path: "Cargo.toml",
        rights: READ | WRITE,
    },
    PathRights {
        path: "Cargo.lock",
        rights: READ | WRITE,
    },
    PathRights {
        path: "README.md",
        rights: READ | WRITE,
    },
    PathRights {
        path: ".git/**",
        rights: FORBIDDEN,
    },
    PathRights {
        path: "**/.env",
        rights: FORBIDDEN,
    },
    PathRights {
        path: "**/.env.*",
        rights: FORBIDDEN,
    },
    PathRights {
        path: "**/id_rsa",
        rights: FORBIDDEN,
    },
    PathRights {
        path: "**/id_ed25519",
        rights: FORBIDDEN,
    },
    PathRights {
        path: "**/secrets/**",
        rights: FORBIDDEN,
    },
    PathRights {
        path: "**/target/**",
        rights: FORBIDDEN,
    },
];
