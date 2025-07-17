#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkTagType {
    pub link: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkTagTypeWithSub {
    pub link: String,
    pub title: String,
    pub sub: Vec<LinkTagType>,
}
