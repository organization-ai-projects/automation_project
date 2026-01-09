pub mod config;
pub mod logger;
pub mod supervisor;

// Point d'entrée pour initialiser les composants du watcher
pub fn initialize() {
    println!("Watcher initialized");
}
