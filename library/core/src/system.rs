use crate::{
    chunk::{
        base_tile::BaseTile,
        entity::{ChunkBundle, ModifiedLayer, ZOrder},
        render::{mesh::ChunkMesh, GridTopology},
        BaseChunk, ChunkRender,
    },
    lib::*,
    tilemap::{BaseTilemap, TilemapAutoSpawn, TilemapEvents},
    SpawnedChunks,
};

pub(crate) fn tilemap_update_events<T: TilemapEvents + WorldQuery + Component>(
    mut tilemap_query: Query<&mut T>,
) {
    for mut tilemap in tilemap_query.iter_mut() {
        tilemap.update_events();
    }
}

pub(crate) fn tilemap_chunk_spawned<
    T: BaseTile,
    C: BaseChunk<T> + ChunkRender<T>,
    M: BaseTilemap<T, C> + TilemapEvents + WorldQuery + Component,
>(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut tilemap_query: Query<(Entity, &mut M)>,
    mut spawned_chunks: ResMut<SpawnedChunks>,
) {
    for (map_entity, mut tilemap) in tilemap_query.iter_mut() {
        let chunk_spawned_events = tilemap.chunk_spawned_events();
        let mut reader = chunk_spawned_events.get_reader();
        let reader_len = reader.iter(&chunk_spawned_events).count();
        let mut points: Vec<Point2> = Vec::new();
        for event in reader.iter(&chunk_spawned_events) {
            points.push(event.point)
        }
        for point in points.into_iter() {
            let spawned_map = spawned_chunks
                .0
                .entry(map_entity)
                .or_insert_with(|| HashSet::default());
            if spawned_map.contains(&point) {
                continue;
            } else {
                spawned_map.insert(point);
            }

            let layers = tilemap.layers();
            let layers_len = tilemap.layers().len();
            let chunk_dimensions = tilemap.chunk_dimensions();
            let tile_dimensions = tilemap.tile_dimensions();
            let texture_dimensions = tilemap.texture_dimensions();
            let texture_atlas = tilemap.texture_atlas().clone_weak();
            let pipeline_handle = tilemap.topology().to_pipeline_handle();
            let topology = tilemap.topology();
            let chunk = if let Some(chunk) = tilemap.get_chunk_mut(&point) {
                chunk
            } else {
                warn!("Can not get chunk at {}, skipping", &point);
                continue;
            };
            let mut entities = Vec::with_capacity(reader_len);
            for sprite_order in 0..layers_len {
                if layers.get(sprite_order).is_none() {
                    continue;
                }
                let mut mesh = Mesh::from(&ChunkMesh::new(chunk_dimensions.into()));
                let (indexes, colors) = if let Some(parts) =
                    chunk.tiles_to_renderer_parts(sprite_order, chunk_dimensions.into())
                {
                    parts
                } else {
                    warn!("Can not split tiles to data for the renderer");
                    continue;
                };
                mesh.set_attribute(ChunkMesh::ATTRIBUTE_TILE_INDEX, indexes);
                mesh.set_attribute(ChunkMesh::ATTRIBUTE_TILE_COLOR, colors);
                let mesh_handle = meshes.add(mesh);
                chunk.set_mesh(sprite_order, mesh_handle.clone());

                use DimensionKind::*;
                let (tile_width, tile_height) = match tile_dimensions {
                    Dimension2(d2) => (
                        d2.width * texture_dimensions.width,
                        d2.height * texture_dimensions.height,
                    ),
                    Dimension3(d3) => (
                        d3.width * texture_dimensions.width,
                        d3.height * texture_dimensions.height,
                    ),
                };
                let (chunk_width, chunk_height) = match chunk_dimensions {
                    Dimension2(d2) => (
                        d2.width * texture_dimensions.width,
                        d2.height * texture_dimensions.height,
                    ),
                    Dimension3(d3) => (
                        d3.width * texture_dimensions.width,
                        d3.height * texture_dimensions.height,
                    ),
                };
                use GridTopology::*;
                let translation_x = match topology {
                    HexX | HexEvenCols | HexOddCols => {
                        (((chunk.point().x * tile_width as i32) as f32 * 0.75) as i32
                            * chunk_width as i32) as f32
                    }
                    HexY => {
                        (chunk.point().x * tile_width as i32 * chunk_width as i32) as f32
                            + (chunk.point().y as f32 * chunk_height as f32 * 0.5)
                                * tile_width as f32
                    }
                    Square | HexEvenRows | HexOddRows => {
                        (chunk.point().x * tile_width as i32 * chunk_width as i32) as f32
                    }
                };
                let translation_y = match topology {
                    HexX => {
                        (chunk.point().y * tile_height as i32 * chunk_height as i32) as f32
                            + (chunk.point().x as f32 * chunk_width as f32 * 0.5)
                                * tile_height as f32
                    }
                    HexY | HexEvenRows | HexOddRows => {
                        (((chunk.point().y * tile_height as i32) as f32 * 0.75) as i32
                            * chunk_height as i32) as f32
                    }
                    Square | HexEvenCols | HexOddCols => {
                        (chunk.point().y * tile_height as i32 * chunk_height as i32) as f32
                    }
                };
                let translation = Vec3::new(translation_x, translation_y, sprite_order as f32);
                let pipeline = RenderPipeline::new(pipeline_handle.clone_weak().typed());
                let entity = if let Some(entity) = commands
                    .spawn(ChunkBundle {
                        point,
                        sprite_order: ZOrder(sprite_order),
                        texture_atlas: texture_atlas.clone_weak(),
                        mesh: mesh_handle.clone_weak(),
                        transform: Transform::from_translation(translation),
                        render_pipelines: RenderPipelines::from_pipelines(vec![pipeline]),
                        draw: Default::default(),
                        visible: Visible {
                            // TODO: this would be nice as a config parameter to make
                            // RapierRenderPlugin's output visible.
                            is_visible: true,
                            is_transparent: true,
                        },
                        main_pass: MainPass,
                        global_transform: Default::default(),
                        modified_layer: Default::default(),
                    })
                    .current_entity()
                {
                    entity
                } else {
                    error!(
                        "Chunk entity does not exist unexpectedly, can not run the tilemap system"
                    );
                    return;
                };

                info!("Chunk {} spawned", point);

                chunk.set_entity(sprite_order, entity);
                entities.push(entity);
            }
            commands.push_children(map_entity, &entities);
        }
    }
}

pub(crate) fn tilemap_chunk_modified<
    T: BaseTile,
    C: BaseChunk<T>,
    M: BaseTilemap<T, C> + TilemapEvents + WorldQuery + Component,
>(
    tilemap_query: Query<&M>,
    mut layer_query: Query<&mut ModifiedLayer>,
) {
    for tilemap in tilemap_query.iter() {
        let mut reader = tilemap.chunk_modified_events().get_reader();
        for event in reader.iter(&tilemap.chunk_modified_events()) {
            let layer = &event.layer;
            for entity in layer.into_iter() {
                let mut modified_layer = if let Ok(layer) = layer_query.get_mut(*entity) {
                    layer
                } else {
                    warn!("Chunk layer does not exist, skipping");
                    continue;
                };
                modified_layer.0 += 1;
            }
        }
    }
}

pub(crate) fn tilemap_chunk_despawned<
    T: BaseTile,
    C: BaseChunk<T>,
    M: BaseTilemap<T, C> + TilemapEvents + WorldQuery + Component,
>(
    commands: &mut Commands,
    mut tilemap_query: Query<&mut M>,
) {
    for mut tilemap in tilemap_query.iter_mut() {
        let mut reader = tilemap.chunk_despawned_events().get_reader();
        let mut points: Vec<Point2> = Vec::new();
        for event in reader.iter(tilemap.chunk_despawned_events()) {
            points.push(event.point)
        }
        for point in points.into_iter() {
            let chunk = if let Some(chunk) = tilemap.get_chunk_mut(&point) {
                chunk
            } else {
                warn!("Can not get chunk at {}, skipping", &point);
                continue;
            };
            let entities = chunk.get_entities();
            for entity in entities.into_iter() {
                commands.despawn_recursive(entity);
            }
            info!("Chunk {} despawned", point);
        }
    }
}

/// The chunk update system that is used to set attributes of the tiles and
/// tints if they need updating.
pub(crate) fn tilemap_chunk_update<
    T: BaseTile,
    C: BaseChunk<T> + ChunkRender<T>,
    M: BaseTilemap<T, C> + WorldQuery + Component,
>(
    mut meshes: ResMut<Assets<Mesh>>,
    map_query: Query<&M>,
    mut chunk_query: Query<(&Parent, &Point2, &ZOrder, &Handle<Mesh>), Changed<ModifiedLayer>>,
) {
    for (parent, point, sprite_order, mesh_handle) in chunk_query.iter_mut() {
        let tilemap = if let Ok(tilemap) = map_query.get(**parent) {
            tilemap
        } else {
            error!("`Tilemap` is missing, can not update chunk");
            return;
        };
        let chunk = if let Some(chunk) = tilemap.get_chunk(point) {
            chunk
        } else {
            error!("`Chunk` is missing, can not update chunk");
            return;
        };
        let mesh = if let Some(mesh) = meshes.get_mut(mesh_handle) {
            mesh
        } else {
            error!("`Mesh` is missing, can not update chunk");
            return;
        };
        let (indexes, colors) = if let Some((index, colors)) =
            chunk.tiles_to_renderer_parts(sprite_order.0, tilemap.chunk_dimensions())
        {
            (index, colors)
        } else {
            error!("Tiles are missing, can not update chunk");
            return;
        };
        mesh.set_attribute(ChunkMesh::ATTRIBUTE_TILE_INDEX, indexes);
        mesh.set_attribute(ChunkMesh::ATTRIBUTE_TILE_COLOR, colors);
    }
}

/// Actual method used to spawn chunks.
fn auto_spawn<T: BaseTile, C: BaseChunk<T>, M: BaseTilemap<T, C> + WorldQuery + Component>(
    camera_transform: &Transform,
    tilemap_transform: &Transform,
    tilemap: &mut Mut<M>,
    spawn_dimensions: Dimension2,
    spawned_chunks: &mut HashSet<Point2>,
) {
    let translation = camera_transform.translation - tilemap_transform.translation;
    let tile_width = tilemap.tile_dimensions().width() * tilemap.texture_dimensions().width;
    let tile_height = tilemap.tile_dimensions().height() * tilemap.texture_dimensions().height;
    let point_x = translation.x / tile_width as f32;
    let point_y = translation.y / tile_height as f32;
    let (chunk_x, chunk_y) = tilemap.point_to_chunk_point((point_x as i32, point_y as i32));
    let mut new_spawned: Vec<Point2> = Vec::new();
    let spawn_width = spawn_dimensions.width as i32;
    let spawn_height = spawn_dimensions.height as i32;
    for y in -spawn_width as i32..spawn_width + 1 {
        for x in -spawn_height..spawn_height + 1 {
            let chunk_x = x + chunk_x;
            let chunk_y = y + chunk_y;
            if let Some(dimension) = tilemap.dimensions() {
                let width = (dimension.width() / tilemap.chunk_dimensions().width()) as i32 / 2;
                if chunk_x < -width || chunk_x > width {
                    continue;
                }
            }
            if let Some(map_dimension) = tilemap.dimensions() {
                let height =
                    (map_dimension.height() / tilemap.chunk_dimensions().width()) as i32 / 2;
                if chunk_y < -height || chunk_y > height {
                    continue;
                }
            }

            if let Err(e) = tilemap.spawn_chunk(Point2::new(chunk_x, chunk_y)) {
                warn!("{}", e);
            }
            new_spawned.push(Point2::new(chunk_x, chunk_y));
        }
    }

    for point in spawned_chunks.iter() {
        if !new_spawned.contains(&point.into()) {
            if let Err(e) = tilemap.despawn_chunk(point) {
                error!("{}", e);
            }
        }
    }
    spawned_chunks.clear();

    for new_point in new_spawned.into_iter() {
        spawned_chunks.insert(new_point);
    }
}

/// On window size change, the radius of chunks changes if needed.
pub(crate) fn tilemap_chunk_auto_radius<
    T: BaseTile,
    C: BaseChunk<T>,
    M: BaseTilemap<T, C> + TilemapAutoSpawn + WorldQuery + Component,
>(
    window_resized_events: Res<Events<WindowResized>>,
    mut tilemap_query: Query<(Entity, &mut M, &Transform)>,
    camera_query: Query<(&Camera, &Transform)>,
    mut spawned_chunks: ResMut<SpawnedChunks>,
) {
    let mut window_reader = window_resized_events.get_reader();
    for event in window_reader.iter(&window_resized_events) {
        for (map_entity, mut tilemap, tilemap_transform) in tilemap_query.iter_mut() {
            let window_width = event.width as u32;
            let window_height = event.height as u32;
            let tile_width = tilemap.tile_dimensions().width() * tilemap.texture_dimensions().width;
            let tile_height =
                tilemap.tile_dimensions().height() * tilemap.texture_dimensions().height;
            let chunk_px_width = tilemap.chunk_dimensions().width() * tile_width;
            let chunk_px_height = tilemap.chunk_dimensions().height() * tile_height;
            let chunks_wide = (window_width as f32 / chunk_px_width as f32).ceil() as u32 + 1;
            let chunks_high = (window_height as f32 / chunk_px_height as f32).ceil() as u32 + 1;
            let spawn_dimensions = Dimension2::new(chunks_wide, chunks_high);
            tilemap.set_auto_spawn(spawn_dimensions);
            let entity_spawned_chunks = match spawned_chunks.0.get_mut(&map_entity) {
                Some(s) => s,
                None => {
                    error!("`Tilemap` is missing from spawned chunks");
                    continue;
                }
            };
            for (_camera, camera_transform) in camera_query.iter() {
                auto_spawn(
                    camera_transform,
                    &tilemap_transform,
                    &mut tilemap,
                    spawn_dimensions,
                    entity_spawned_chunks,
                );
            }
        }
    }
}

/// Spawns and despawns chunks automatically based on a camera's position.
pub(crate) fn tilemap_chunk_auto_spawn<
    T: BaseTile,
    C: BaseChunk<T>,
    M: BaseTilemap<T, C> + TilemapAutoSpawn + WorldQuery + Component,
>(
    mut tilemap_query: Query<(Entity, &mut M, &Transform)>,
    camera_query: Query<(&Camera, &Transform), Changed<Transform>>,
    mut spawned_chunks: ResMut<SpawnedChunks>,
) {
    // For the transform, get chunk coord.
    for (map_entity, mut tilemap, tilemap_transform) in tilemap_query.iter_mut() {
        for (_camera, camera_transform) in camera_query.iter() {
            let spawn_dimensions = if let Some(dimensions) = tilemap.auto_spawn() {
                dimensions
            } else {
                continue;
            };
            let entity_spawned_chunks = match spawned_chunks.0.get_mut(&map_entity) {
                Some(s) => s,
                None => {
                    error!("`Tilemap` is missing from spawned chunks");
                    continue;
                }
            };
            auto_spawn(
                camera_transform,
                &tilemap_transform,
                &mut tilemap,
                spawn_dimensions,
                entity_spawned_chunks,
            );
        }
    }
}
