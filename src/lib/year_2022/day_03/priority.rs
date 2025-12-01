pub trait Priority {
    fn priority(&self) -> usize;
}
impl Priority for char {
    fn priority(&self) -> usize {
        if self.is_ascii_uppercase() {
            (*self as usize) - ('A' as usize) + 27
        } else {
            (*self as usize) - ('a' as usize) + 1
        }
    }
}
