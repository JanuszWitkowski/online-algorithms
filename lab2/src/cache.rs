pub trait Cache {
    fn access(&self, x: usize) -> usize;
    fn name(&self) -> &'static str;
}
