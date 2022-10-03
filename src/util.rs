pub enum AddUpdate {
    Exists,
    Added,
    Updated,
}
impl AddUpdate {
    pub(crate) fn exists(&self) -> bool {
        match self {
            Self::Exists => true,
            _ => false,
        }
    }
    pub(crate) fn added(&self) -> bool {
        match self {
            Self::Added => true,
            _ => false,
        }
    }
    pub(crate) fn updated(&self) -> bool {
        match self {
            Self::Updated => true,
            _ => false,
        }
    }
}
