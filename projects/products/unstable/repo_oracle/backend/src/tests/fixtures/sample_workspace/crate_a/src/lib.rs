pub fn greet() -> String {
    "hello".to_string()
}

pub struct Config {
    pub name: String,
}

pub trait Processor {
    fn process(&self);
}
