use crate::lib::*;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
/// The kinds of errors that can occur.
pub enum ErrorKind {
    /// If the coordinate or index is out of bounds.
    DimensionError(DimensionError),
    /// If a layer already exists this error is returned.
    LayerExists(usize),
    /// If a layer does not already exist this error is returned.
    LayerDoesNotExist(usize),
    /// Texture atlas was not set
    MissingTextureAtlas,
    /// The tile dimensions were not set.
    MissingTileDimensions,
    /// The chunk does not exist.
    MissingChunk,
    /// The chunk already exists.
    ChunkAlreadyExists(Point2),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use ErrorKind::*;
        match self {
            DimensionError(err) => ::std::fmt::Debug::fmt(&err, f),
            LayerExists(n) => write!(
                f,
                "layer {} already exists, try `remove_layer` or `move_layer` first",
                n
            ),
            LayerDoesNotExist(n) => write!(f, "layer {} does not exist, try `add_layer` first", n),
            MissingTextureAtlas => write!(
                f,
                "texture atlas is missing, must use `TilemapBuilder::texture_atlas`"
            ),
            MissingTileDimensions => {
                write!(f, "tile dimensions are missing, it is required to set it")
            }
            MissingChunk => write!(f, "the chunk does not exist, try `add_chunk` first"),
            ChunkAlreadyExists(p) => write!(
                f,
                "the chunk {} already exists, if this was intentional run `remove_chunk` first",
                p
            ),
        }
    }
}

impl Error for ErrorKind {}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
/// The error type for operations when interacting with the tilemap.
pub struct TilemapError(pub Box<ErrorKind>);

impl Display for TilemapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl Error for TilemapError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

impl From<ErrorKind> for TilemapError {
    fn from(kind: ErrorKind) -> TilemapError {
        TilemapError(Box::new(kind))
    }
}

impl From<DimensionError> for TilemapError {
    fn from(err: DimensionError) -> TilemapError {
        TilemapError(Box::new(ErrorKind::DimensionError(err)))
    }
}
