pub trait Rng {
    /// Fills the provided slice with random bytes.
    fn fill(&mut self, dest: &mut [u8]);
}
