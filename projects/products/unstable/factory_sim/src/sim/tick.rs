/// A single logical time unit for the simulation.
/// No wall-clock time is involved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tick(pub u64);

impl Tick {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(self) -> u64 {
        self.0
    }

    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

impl std::fmt::Display for Tick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tick({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_increments() {
        let t = Tick::new(3);
        assert_eq!(t.next().value(), 4);
    }

    #[test]
    fn ordering_holds() {
        assert!(Tick::new(0) < Tick::new(1));
    }
}
