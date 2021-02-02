#![allow(clippy::all)]
use bevy::{
    asset::LoadState,
    prelude::*,
    sprite::{TextureAtlas, TextureAtlasBuilder},
    utils::HashSet,
    window::WindowMode,
};
use bevy_tilemap::prelude::*;
use bevy::utils::HashMap;

const TILE_SIZE: f32 = 32.0;

const CHUNK_WIDTH: u32 = TILE_SIZE as u32; //16;
const CHUNK_HEIGHT: u32 = TILE_SIZE as u32; //16;
const TILEMAP_WIDTH: i32 = CHUNK_WIDTH as i32 * 4;
const TILEMAP_HEIGHT: i32 = CHUNK_HEIGHT as i32 * 4;

#[derive(Default, Clone)]
struct TileSpriteHandles {
    handles: Vec<HandleUntyped>,
    atlas_loaded: bool,
}

#[derive(Default, Copy, Clone, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Default)]
struct Render {
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    position: Position,
    render: Render,
}

#[derive(Default, Clone)]
struct GameState {
    map_loaded: bool,
    spawned: bool,
    collisions: HashSet<(i32, i32)>,
}

impl GameState {

}

fn setup(asset_server: Res<AssetServer>,
          mut tile_sprite_handles: ResMut<TileSpriteHandles>) {
    tile_sprite_handles.handles = asset_server.load_folder("textures").unwrap();


}

fn load(
    commands: &mut Commands,
    mut sprite_handles: ResMut<TileSpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    asset_server: Res<AssetServer>,
) {
    if sprite_handles.atlas_loaded {
        return;
    }

    // Lets load all our textures from our folder!
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        for handle in sprite_handles.handles.iter() {
            let texture = textures.get(handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), &texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let atlas_handle = texture_atlases.add(texture_atlas);

        // These are fairly advanced configurations just to quickly showcase
        // them.
        let tilemap = Tilemap::builder()
            // .dimensions(TILEMAP_WIDTH as u32, TILEMAP_HEIGHT as u32)
            // .chunk_dimensions(CHUNK_WIDTH, CHUNK_HEIGHT)
            .tile_dimensions(32, 32)
            .auto_chunk()
            .auto_spawn(2, 2)
            .z_layers(2)
            .texture_atlas(atlas_handle)
            .finish()
            .unwrap();

        let tilemap_components = TilemapBundle {
            tilemap,
            transform: Default::default(),
            global_transform: Default::default(),
        };

        // commands.spawn(Camera2dBundle::default());
        commands
            .spawn(tilemap_components)
            .with(Timer::from_seconds(0.075, true));

        let texture_handle = asset_server.load("textures/0.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), /*3*/ 6, 4);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let player = Player {
            offset: Vec2{x: 0.0, y: 0.0}, //FIXME: check were is the error with offset calculations
            ..Default::default()
        };

        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3 {z: 0.1, x: player.offset.x, y: player.offset.y},
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Animation {
                state: "default".to_string(),
                animation_state: AnimationState {
                    timer: Timer::from_seconds(0.1, true),
                    frames: vec![0],
                    frame: 0,
                },
                animations: vec![
                    ("default".to_string(), vec![0]),
                    ("walk_down".to_string(), vec![2, 0, 1, 0 /*     3,  4, 5*/]),
                    ("walk_up".to_string(), vec![8, 6, 7, 6 /*      9,  10, 11*/]),
                    (
                        "walk_right".to_string(),
                        vec![14, 12, 13, 12 /*  15, 16, 17*/],
                    ),
                    (
                        "walk_left".to_string(),
                        vec![20, 18, 19, 18 /* 21,  22, 23*/],
                    ),
                    ("idle_down".to_string(), vec![0]),
                    ("idle_up".to_string(), vec![6]),
                    ("idle_right".to_string(), vec![12]),
                    ("idle_left".to_string(), vec![18]),
                ]
                    .into_iter()
                    .collect(),
            })
            .with(player).with_children(|parent| {
                parent.spawn(Camera2dBundle::default());
            });

        sprite_handles.atlas_loaded = true;
    }
}

fn build_random_dungeon(
    mut game_state: ResMut<GameState>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Tilemap>,
) {
    if game_state.map_loaded {
        return;
    }

    for mut map in query.iter_mut() {
        // Then we need to find out what the handles were to our textures we are going to use.
        let floor_sprite: Handle<Texture> = asset_server.get_handle("textures/square-floor6.png");
        let wall_sprite: Handle<Texture> = asset_server.get_handle("textures/square-wall.png");
        let texture_atlas = texture_atlases.get(map.texture_atlas()).unwrap();
        let floor_idx = texture_atlas.get_texture_index(&floor_sprite).unwrap();
        let wall_idx = texture_atlas.get_texture_index(&wall_sprite).unwrap();

        // Now we fill the entire space with floors.
        let mut tiles = Vec::new();
        for y in 0..TILEMAP_HEIGHT {
            for x in 0..TILEMAP_WIDTH {
                let y = y - TILEMAP_HEIGHT / 2;
                let x = x - TILEMAP_WIDTH / 2;
                // By default tile sets the Z order at 0. Lower means that tile
                // will render lower than others. 0 is the absolute bottom
                // level which is perfect for backgrounds.
                let tile = Tile {
                    point: (x, y),
                    sprite_index: floor_idx,
                    ..Default::default()
                };
                tiles.push(tile);
            }
        }

        for x in 0..TILEMAP_WIDTH {
            let x = x - TILEMAP_WIDTH / 2;
            let tile_a = (x, -TILEMAP_HEIGHT / 2);
            let tile_b = (x, TILEMAP_HEIGHT / 2 - 1);
            tiles.push(Tile {
                point: tile_a,
                sprite_index: wall_idx,
                ..Default::default()
            });
            tiles.push(Tile {
                point: tile_b,
                sprite_index: wall_idx,
                ..Default::default()
            });
            game_state.collisions.insert(tile_a);
            game_state.collisions.insert(tile_b);
        }

        // Then the wall tiles on the Y axis.
        for y in 0..TILEMAP_HEIGHT {
            let y = y - TILEMAP_HEIGHT / 2;
            let tile_a = (-TILEMAP_WIDTH / 2, y);
            let tile_b = (TILEMAP_WIDTH / 2 - 1, y);
            tiles.push(Tile {
                point: tile_a,
                sprite_index: wall_idx,
                ..Default::default()
            });
            tiles.push(Tile {
                point: tile_b,
                sprite_index: wall_idx,
                ..Default::default()
            });
            game_state.collisions.insert(tile_a);
            game_state.collisions.insert(tile_b);
        }

        // Lets just generate some random walls to sparsely place around the dungeon!
        // let range = (TILEMAP_WIDTH * TILEMAP_HEIGHT) as usize / 5;
        // let mut rng = rand::thread_rng();
        // for _ in 0..range {
        //     let x = rng.gen_range((-TILEMAP_WIDTH / 2)..(TILEMAP_WIDTH / 2));
        //     let y = rng.gen_range((-TILEMAP_HEIGHT / 2)..(TILEMAP_HEIGHT / 2));
        //     let coord = (x, y, 0i32);
        //     if coord != (0, 0, 0) {
        //         tiles.push(Tile {
        //             point: (x, y),
        //             sprite_index: wall_idx,
        //             ..Default::default()
        //         });
        //         game_state.collisions.insert((x, y));
        //     }
        // }

        // The above should give us a neat little randomized dungeon! However,
        // we are missing a hero! First, we need to add a layer. We must make
        // this layer `Sparse` else we will lose efficiency with our data!
        //
        // You might've noticed that we didn't create a layer for z_layer 0 but
        // yet it still works and exists. By default if a layer doesn't exist
        // and tiles need to be written there then a Dense layer is created
        // automatically.
        map.add_layer(
            TilemapLayer {
                kind: LayerKind::Sparse,
                ..Default::default()
            },
            1,
        )
        .unwrap();

        // Now lets add in a dwarf friend!
        // let dwarf_sprite: Handle<Texture> = asset_server.get_handle("textures/square-dwarf.png");
        // let dwarf_sprite_index = texture_atlas.get_texture_index(&dwarf_sprite).unwrap();
        // // We add in a Z order of 1 to place the tile above the background on Z
        // // order 0.
        // let dwarf_tile = Tile {
        //     point: (0, 0),
        //     sprite_index: dwarf_sprite_index,
        //     z_order: 1,
        //     ..Default::default()
        // };
        // tiles.push(dwarf_tile);

        // commands.spawn(PlayerBundle {
        //     player: Player {},
        //     position: Position { x: 0, y: 0 },
        //     render: Render {
        //         sprite_index: dwarf_sprite_index,
        //         z_order: 1,
        //     },
        // });

        // Now we pass all the tiles to our map.
        map.insert_tiles(tiles).unwrap();

        game_state.map_loaded = true;
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Endless Dungeon".to_string(),
            width: 512.,
            height: 512.,
            vsync: false,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .init_resource::<TileSpriteHandles>()
        .init_resource::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapDefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(load.system())
        .add_system(build_random_dungeon.system())
        .add_system(player_movement_system.system())
        .add_system(sprite_animation_system.system())
        .run()
}


fn sprite_animation_system(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut TextureAtlasSprite/*, &mut Transform*/)>,
) {
    for (mut animation, mut sprite/*, mut transform*/) in query.iter_mut() {
        animation.update(time.delta_seconds());
        sprite.index = animation.animation_state.frames[animation.animation_state.frame] as u32;
        // transform.translation.y += animation.animation_state.y_offset;
    }
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform, &mut Animation)>,
) {
    for (mut player, mut transform, mut animation) in query.iter_mut() {
        let mut moving = true;
        if !player.started_walking {
            if keyboard_input.pressed(KeyCode::Left) {
                player.walk(Direction::Left);
                animation.set_state("walk_left".to_string());
            } else if keyboard_input.pressed(KeyCode::Right) {
                player.walk(Direction::Right);
                animation.set_state("walk_right".to_string());
            } else if keyboard_input.pressed(KeyCode::Up) {
                player.walk(Direction::Up);
                animation.set_state("walk_up".to_string());
            } else if keyboard_input.pressed(KeyCode::Down) {
                player.walk(Direction::Down);
                animation.set_state("walk_down".to_string());
            } else {
                moving = false;
            }
        }
        let signal_x = {
            if transform.translation.x >= 0.0 {
                1.0
            } else {
                -1.0
            }
        };
        let signal_y = {
            if transform.translation.y >= 0.0 {
                1.0
            } else {
                -1.0
            }
        };

        if player.started_walking {
            transform.translation.x += time.delta_seconds()
                * (player.position.x * TILE_SIZE - player.position_start.x * TILE_SIZE)
                * player.velocity;
            transform.translation.y += time.delta_seconds()
                * (player.position.y * TILE_SIZE - player.position_start.y * TILE_SIZE)
                * player.velocity;
        }


        if player.started_walking &&
            (((transform.translation.x.abs() + player.offset.x) / TILE_SIZE * signal_x) as i32) == (player.position.x as i32)  &&
            (((transform.translation.y.abs() + player.offset.y) / TILE_SIZE * signal_y) as i32) == (player.position.y as i32)
        {
            println!("Player position {:?}", player.position);
            player.position_start = player.position.clone();
            player.started_walking = false;
        }

        if !moving {
            animation.set_state(match player.direction {
                Direction::Down     => "idle_down".to_string(),
                Direction::Up       => "idle_up".to_string(),
                Direction::Left     => "idle_left".to_string(),
                Direction::Right    => "idle_right".to_string(),
            });
        }
    }
}


struct AnimationState {
    timer: Timer,
    frames: Vec<usize>,
    frame: usize,
}

impl AnimationState {
    fn update(&mut self, delta: f32) {
        self.timer.tick(delta);

        if self.timer.just_finished() {
            // println!("updating animation frame: {}/{}", self.frame, self.frames.capacity());
            if self.frames.capacity() > (self.frame + 1) as usize {
                self.frame += 1;
            } else {
                self.frame = 0;
            }
        }
    }

    fn set_frames(&mut self, frames: Vec<usize>) {
        self.frames = frames;
        self.frame = 0;
    }
}

struct Animation {
    state: String,
    animation_state: AnimationState,
    animations: HashMap<String, Vec<usize>>,
}

impl Animation {
    fn update(&mut self, delta: f32) {
        self.animation_state.update(delta);
    }

    fn set_state(&mut self, state: String) {
        if self.state == &*state {
            return;
        }

        self.state = state;
        self.animation_state.set_frames(
            self.animations.get(&self.state).unwrap().clone(), //TODO: how to keep a single reference?
        );
    }
}


#[derive(Debug)]
struct Player {
    direction: Direction,
    position: Vec2,
    position_start: Vec2,
    started_walking: bool,
    is_stoped: bool,
    velocity: f32,
    offset: Vec2,
}

impl Player {
    fn walk(&mut self, direction: Direction) {
        if self.started_walking {
            return;
        }

        self.started_walking = true;
        self.direction = direction;

        let new_position = match self.direction {
            Direction::Left => Vec2 { x: -1.0, y: 0.0 },
            Direction::Right => Vec2 { x: 1.0, y: 0.0 },
            Direction::Up => Vec2 { x: 0.0, y: 1.0 },
            Direction::Down => Vec2 { x: 0.0, y: -1.0 },
        };

        self.position_start = self.position.clone();
        self.position = self.position + new_position;
    }
}

impl Default for Player {
    fn default() -> Player {
        Player {
            direction: Direction::Down,
            position: Vec2 { x: 0.0, y: 0.0 },
            position_start: Vec2 { x: 0.0, y: 0.0 },
            started_walking: false,
            is_stoped: true,
            velocity: 2.0,
            offset: Vec2 {x: 0.0, y: 0.0}
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
