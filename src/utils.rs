pub fn length_of<T: ?Sized + AsRef<str>>(x: &T) -> usize {
    x.as_ref().len()
}
