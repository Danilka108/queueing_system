#[derive(Debug)]
pub enum ShaderKind {
    Vertex,
    Geometry,
    Fragment,
}

impl std::fmt::Display for ShaderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vertex => write!(f, "vertex"),
            Self::Geometry => write!(f, "geometry"),
            Self::Fragment => write!(f, "fragment"),
        }
    }
}
