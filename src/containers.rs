#[derive(Debug, Clone, PartialEq)]
pub struct Readonly<T>(T);
impl<T> Readonly<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }

    pub fn item(&self) -> &T {
        &self.0
    }
}
