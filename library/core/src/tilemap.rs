use crate::{
    chunk::{base_tile::BaseTile, layer::LayerKind, render::GridTopology, BaseChunk},
    event::{TilemapChunkDespawned, TilemapChunkModified, TilemapChunkSpawned},
    lib::*,
};

pub struct TileLayer {
    pub kind: LayerKind,
    pub user_data: u128,
}

pub trait BaseTilemap<T: BaseTile, C: BaseChunk<T>> {
    type Error: Display + Debug + Error;
    // type Dimension: Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Hash + Debug;
    // type Point;
    //
    // fn set_texture_atlas(&self) -> &Handle<TextureAtlas>;
    //
    // fn set_tile_origin(&self) -> TileOrigin;

    fn dimensions(&self) -> Option<DimensionKind>;

    //
    fn texture_atlas(&self) -> &Handle<TextureAtlas>;

    //
    fn texture_dimensions(&self) -> Dimension2;

    //
    fn tile_dimensions(&self) -> DimensionKind;

    // fn insert_tiles<P, I>(&mut self, tiles: I) -> Result<(), Self::Error>
    // where
    //     P: Into<Self::Point>,
    //     I: IntoIterator<Item = Tile<P>>;
    //
    // fn insert_tile<P: Into<Self::Point>>(&mut self, tile: Tile<P>) -> Result<(), Self::Error>;
    //
    // fn clear_tiles<P, I>(&mut self, points: I) -> Result<(), Self::Error>
    // where
    //     P: Into<Self::Point>,
    //     I: IntoIterator<Item = (P, usize)>;
    //
    // fn clear_tile<P>(&mut self, point: P, sprite_order: usize) -> Result<(), Self::Error>;
    //
    // fn clear(&mut self, sprite_order: usize) -> Result<(), Self::Error>;
    //
    // fn get_tile<P>(&mut self, point: P, sprite_order: usize) -> Option<&Rawtile>
    // where
    //     P: Into<Self::Point>;
    //
    // fn get_tile_mut<P>(&mut self, point: P, sprite_order: usize) -> Option<&mut RawTile>
    // where
    //     P: Into<Self::Point>;
    //
    // fn set_layer(&mut self, layer: TileLayer) -> Result<(), Self::Error>;
    //
    // fn move_layer(&mut self, from_sprite_order: usize, to_sprite_order: usize) -> Result<(), Self::Error>;
    //
    // fn remove_layer(&mut self, sprite_order: usize) -> Result<(), Self::Error>;

    //
    fn layers(&self) -> Vec<Option<TileLayer>>;

    //
    fn topology(&self) -> GridTopology;

    //
    fn chunk_dimensions(&self) -> DimensionKind;

    // fn insert_chunk<T, P: Into<Point2>>(&mut self, point: P) -> Result<(), Self::Error>;
    //
    // fn contains_chunk<T, P: Into<Point2>>(&mut self, point: P) -> bool;

    fn spawn_chunk<P: Into<Point2>>(&mut self, point: P) -> Result<(), Self::Error>;
    //
    // fn spawn_chunk_containing_point<P: Into<Point2>>(
    //     &mut self,
    //     point: P,
    // ) -> Result<(), Self::Error>;

    fn despawn_chunk<P: Into<Point2>>(&mut self, point: P) -> Result<(), Self::Error>;

    // fn remove_chunk<P: Into<Point2>>(&mut self, point: P) -> Result<(), Self::Error>;

    //
    // fn chunks(&self) -> &HashMap<Point2, C>;

    //
    // fn chunks_mut(&mut self) -> &mut HashMap<Point2, C>;

    fn get_chunk(&self, point: &Point2) -> Option<&C>;

    fn get_chunk_mut(&mut self, point: &Point2) -> Option<&mut C>;

    fn point_to_chunk_point<P: Into<Point2>>(&self, point: P) -> (i32, i32);
}

pub trait TilemapEvents {
    fn chunk_spawned_events(&self) -> &Events<TilemapChunkSpawned>;

    fn chunk_modified_events(&self) -> &Events<TilemapChunkModified>;

    fn chunk_despawned_events(&self) -> &Events<TilemapChunkDespawned>;

    fn update_events(&mut self);
}

pub trait TilemapAutoSpawn {
    fn set_auto_spawn(&mut self, dimension: Dimension2);

    fn auto_spawn(&self) -> Option<Dimension2>;
}
