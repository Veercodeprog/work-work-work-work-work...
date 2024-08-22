trait reversible {
    fn reverse(&self) -> Self;
}
impl<T: Clone> Reversible for Vec<T> {
    fn reverse(&self) -> Self {
        let mut reversed_vec = self.clone();
        reversed_vec.reverse();
        reversed_vec
    }
}
