//! The tilemap events.

use crate::lib::*;

#[derive(Debug)]
pub struct TilemapChunkSpawned {
    pub point: Point2,
}

#[derive(Debug)]
pub struct TilemapChunkModified {
    pub layer: HashSet<Entity>,
}

#[derive(Debug)]
pub struct TilemapChunkDespawned {
    pub point: Point2,
}
