use crate::{
    lib::*,
    tile::{RawTile, Tile},
};

pub(crate) struct Chunk {
    /// The point coordinate of the chunk.
    point: Point2,
    /// The sprite layers of the chunk.
    sprite_layers: Vec<Option<SpriteLayer<RawTile>>>,
    /// Contains a map of all collision entities.
    user_data: u128,
}

impl BaseChunk<RawTile> for Chunk {
    fn point(&self) -> Point2 {
        self.point
    }

    fn set_entity(&mut self, sprite_order: usize, entity: Entity) {
        if let Some(layer) = self.sprite_layers.get_mut(sprite_order) {
            if let Some(layer) = layer.as_mut() {
                layer.entity = Some(entity);
            } else {
                error!("can not add entity to sprite layer {}", sprite_order);
            }
        } else {
            error!("sprite layer {} does not exist", sprite_order);
        }
    }

    /// Gets the layers entity, if any. Useful for despawning.
    fn get_entity(&self, sprite_order: usize) -> Option<Entity> {
        self.sprite_layers
            .get(sprite_order)
            .and_then(|o| o.as_ref().and_then(|layer| layer.entity))
    }

    fn get_entities(&self) -> Vec<Entity> {
        let mut entities = Vec::new();
        for sprite_layer in &self.sprite_layers {
            if let Some(layer) = sprite_layer {
                if let Some(entity) = layer.entity {
                    entities.push(entity);
                }
            }
        }
        entities
    }

    fn get_tile(&self, sprite_order: usize, index: usize) -> Option<&RawTile> {
        self.sprite_layers.get(sprite_order).and_then(|layer| {
            layer
                .as_ref()
                .and_then(|layer| layer.inner.as_ref().get_tile(index))
        })
    }

    fn get_tile_mut(&mut self, sprite_order: usize, index: usize) -> Option<&mut RawTile> {
        self.sprite_layers.get_mut(sprite_order).and_then(|layer| {
            layer
                .as_mut()
                .and_then(|layer| layer.inner.as_mut().get_tile_mut(index))
        })
    }

    fn get_layer(&self, index: usize) -> Option<SpriteLayer<RawTile>> {
        self.sprite_layers.get(index).and_then(|layer| *layer)
    }
}

impl ChunkRender<RawTile> for Chunk {
    /// Sets the mesh for the chunk layer to use.
    fn set_mesh(&mut self, sprite_order: usize, mesh: Handle<Mesh>) {
        if let Some(layer) = self.sprite_layers.get_mut(sprite_order) {
            if let Some(layer) = layer.as_mut() {
                layer.inner.as_mut().set_mesh(mesh)
            } else {
                error!("can not set mesh to sprite layer {}", sprite_order);
            }
        } else {
            error!("sprite layer {} does not exist", sprite_order);
        }
    }
}

impl Chunk {
    /// A newly constructed chunk from a point and the maximum number of layers.
    pub(crate) fn new(point: Point2, layers: &[Option<LayerKind>], dimensions: Dimension2) -> Self {
        let mut chunk = Self {
            point,
            sprite_layers: vec![None; layers.len()],
            user_data: 0,
            // #[cfg(feature = "bevy_rapier2d")]
            // collision_entities: HashMap::default(),
        };
        for (sprite_order, kind) in layers.iter().enumerate() {
            if let Some(kind) = kind {
                chunk.add_layer(kind, sprite_order, dimensions)
            }
        }
        chunk
    }

    /// Adds a layer from a layer kind, the z layer, and dimensions of the
    /// chunk.
    pub(crate) fn add_layer(
        &mut self,
        kind: &LayerKind,
        sprite_order: usize,
        dimensions: Dimension2,
    ) {
        match kind {
            LayerKind::Dense => {
                let tiles = vec![
                    RawTile {
                        index: 0,
                        color: Color::rgba(0.0, 0.0, 0.0, 0.0)
                    };
                    dimensions.area() as usize
                ];
                if let Some(layer) = self.sprite_layers.get_mut(sprite_order) {
                    *layer = Some(SpriteLayer {
                        inner: LayerKindInner::Dense(DenseLayer::new(tiles)),
                        entity: None,
                    });
                } else {
                    error!("sprite layer {} is out of bounds", sprite_order);
                }
            }
            LayerKind::Sparse => {
                if let Some(layer) = self.sprite_layers.get_mut(sprite_order) {
                    *layer = Some(SpriteLayer {
                        inner: LayerKindInner::Sparse(SparseLayer::new(HashMap::default())),
                        entity: None,
                    });
                } else {
                    error!("sprite layer {} is out of bounds", sprite_order);
                }
            }
        }
    }

    /// Returns the point of the location of the chunk.
    pub(crate) fn point(&self) -> Point2 {
        self.point
    }

    /// Moves a layer from a z layer to another.
    pub(crate) fn move_layer(&mut self, from_z: usize, to_z: usize) {
        // TODO: rename to swap and include it in the greater api
        if self.sprite_layers.get(to_z).is_some() {
            error!(
                "sprite layer {} unexpectedly exists and can not be moved",
                to_z
            );
            return;
        }

        self.sprite_layers.swap(from_z, to_z);
    }

    /// Removes a layer from the specified layer.
    pub(crate) fn remove_layer(&mut self, sprite_order: usize) {
        self.sprite_layers.get_mut(sprite_order).take();
    }

    /// Sets a single raw tile to be added to a z layer and index.
    pub(crate) fn set_tile<P: Into<Point2>>(&mut self, index: usize, tile: Tile<P>) {
        if let Some(layer) = self.sprite_layers.get_mut(tile.sprite_order) {
            if let Some(layer) = layer.as_mut() {
                let raw_tile = RawTile {
                    index: tile.sprite_index,
                    color: tile.tint,
                };
                layer.inner.as_mut().set_tile(index, raw_tile);
            } else {
                error!("can not set tile to sprite layer {}", tile.sprite_order);
            }
        } else {
            error!("sprite layer {} does not exist", tile.sprite_order);
        }
    }

    /// Removes a tile from a sprite layer with a given index and z order.
    pub(crate) fn remove_tile(&mut self, index: usize, sprite_order: usize) {
        if let Some(layer) = self.sprite_layers.get_mut(sprite_order) {
            if let Some(layer) = layer.as_mut() {
                layer.inner.as_mut().remove_tile(index);
            } else {
                error!("can not remove tile on sprite layer {}", sprite_order);
            }
        } else {
            error!("sprite layer {} does not exist", sprite_order);
        }
    }
}
