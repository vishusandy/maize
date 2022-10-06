pub enum AddUpdate {
    Exists,
    Added,
    Updated,
}

impl AddUpdate {
    pub(crate) fn exists(&self) -> bool {
        matches!(self, Self::Exists)
    }
    pub(crate) fn added(&self) -> bool {
        matches!(self, Self::Added)
    }
    pub(crate) fn updated(&self) -> bool {
        matches!(self, Self::Updated)
    }
}
