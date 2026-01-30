// projects/products/code_agent_sandbox/src/engine/path_rights.rs
pub(crate) const READ: u8 = 0b0001; // Bit 0
pub(crate) const WRITE: u8 = 0b0010; // Bit 1
pub(crate) const FORBIDDEN: u8 = 0b0100; // Bit 2

#[derive(Debug)]
pub(crate) struct PathRights {
    pub(crate) path: &'static str,
    pub(crate) rights: u8,
}

pub(crate) const PATH_RIGHTS: &[PathRights] = &[
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
