use crate::{
    chunk::{base_tile::BaseTile, layer::SpriteLayer},
    lib::*,
};

/// Raw tile that is stored in the chunks.
pub mod base_tile;
/// Chunk entity.
pub(crate) mod entity;
/// Sparse and dense chunk layers.
pub mod layer;
/// Meshes for rendering to vertices.

/// Files and helpers for rendering.
pub mod render;

pub trait BaseChunk<T: BaseTile> {
    fn point(&self) -> Point2;

    fn set_entity(&mut self, sprite_order: usize, entity: Entity);

    fn get_entity(&self, sprite_order: usize) -> Option<Entity>;

    fn get_entities(&self) -> Vec<Entity>;

    fn get_tile(&self, sprite_order: usize, index: usize) -> Option<&T>;

    fn get_tile_mut(&mut self, sprite_order: usize, index: usize) -> Option<&mut T>;

    fn get_layer(&self, index: usize) -> Option<SpriteLayer<T>>;
}

pub trait ChunkRender<T: BaseTile>: BaseChunk<T> {
    fn set_mesh(&mut self, sprite_order: usize, mesh: Handle<Mesh>);

    fn tiles_to_renderer_parts(
        &self,
        sprite_order: usize,
        dimensions: DimensionKind,
    ) -> Option<(Vec<f32>, Vec<[f32; 4]>)> {
        let area = dimensions.area() as usize;
        self.get_layer(sprite_order)
            .and_then(|sprite_layer| Some(sprite_layer.inner.as_ref().tiles_to_attributes(area)))
    }
}
