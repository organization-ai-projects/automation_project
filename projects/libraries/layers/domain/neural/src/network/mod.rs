pub mod activation;
pub mod layer;
pub mod layer_config;
pub mod network_error;
pub mod neural_network;
pub mod weight_init;

pub use activation::Activation;
pub use layer::Layer;
pub use layer_config::LayerConfig;
pub use network_error::NetworkError;
pub use neural_network::SimpleNeuralNet;
pub use weight_init::WeightInit;

#[cfg(test)]
mod tests;
