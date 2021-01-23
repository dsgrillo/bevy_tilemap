use crate::{chunk::Chunk2D, lib::*, tile::RawTile};

pub struct Tilemap2D {
    /// The type of grid to use.
    topology: GridTopology,
    /// An optional field which can contain the tilemaps dimensions in chunks.
    dimensions: Option<Dimension2>,
    /// A chunks dimensions in tiles.
    chunk_dimensions: Dimension2,
    /// A tiles dimensions in pixels.
    tile_dimensions: Dimension2,
    /// The layers that are currently set in the tilemap in order from lowest
    /// to highest.
    layers: Vec<Option<TilemapLayer>>,
    /// Auto flags used for different automated features.
    auto_flags: AutoFlags,
    /// Dimensions of chunks to spawn from camera transform.
    auto_spawn: Option<Dimension2>,
    // /// Rapier physics scale for colliders and rigid bodies created
    // /// for layers with colliders.
    // #[cfg(feature = "bevy_rapier2d")]
    // physics_scale: f32,
    /// Custom flags.
    custom_flags: Vec<u32>,
    #[cfg_attr(feature = "serde", serde(skip))]
    /// The handle of the texture atlas.
    texture_atlas: Handle<TextureAtlas>,
    /// A map of all the chunks at points.
    chunks: HashMap<Point2, Chunk2D>,
    #[cfg_attr(feature = "serde", serde(skip))]
    /// A map of all currently spawned entities.
    entities: HashMap<usize, Vec<Entity>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    /// The events of the tilemap.
    chunk_events: Events<TilemapChunkEvent>,
    // #[cfg(feature = "bevy_rapier2d")]
    // #[cfg_attr(feature = "serde", serde(skip))]
    // /// The collision events of the tilemap.
    // collision_events: Events<TilemapCollisionEvent>,
    /// A set of all spawned chunks.
    spawned: HashSet<(i32, i32)>,
}

impl Tilemap<RawTile, Chunk2D> for Tilemap2D {
    type Error = ();

    fn dimensions(&self) -> Option<DimensionKind> {
        unimplemented!()
    }

    fn texture_atlas(&self) -> &Handle<TextureAtlas> {
        unimplemented!()
    }

    fn texture_dimensions(&self) -> Dimension2 {
        unimplemented!()
    }

    fn tile_dimensions(&self) -> DimensionKind {
        unimplemented!()
    }

    fn layers(&self) -> Vec<Option<TileLayer>> {
        unimplemented!()
    }

    fn topology(&self) -> GridTopology {
        unimplemented!()
    }

    fn chunk_dimensions(&self) -> DimensionKind {
        unimplemented!()
    }

    fn spawn_chunk<P: Into<Point2>>(&mut self, point: P) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn despawn_chunk<P: Into<Point2>>(&mut self, point: P) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn get_chunk(&self, point: &Point2) -> Option<&Chunk2D> {
        unimplemented!()
    }

    fn get_chunk_mut(&mut self, point: &Point2) -> Option<&mut Chunk2D> {
        unimplemented!()
    }

    fn point_to_chunk_point<P: Into<Point2>>(&self, point: P) -> (i32, i32) {
        unimplemented!()
    }
}
