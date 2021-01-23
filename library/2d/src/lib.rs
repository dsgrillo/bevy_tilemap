#[no_implicit_prelude]
pub mod chunk;
#[no_implicit_prelude]
pub mod tile;
#[no_implicit_prelude]
pub mod tilemap;

/// A custom prelude around everything that we only need to use.
#[no_implicit_prelude]
mod lib {
    extern crate bevy_app;
    extern crate bevy_asset;
    extern crate bevy_ecs;
    extern crate bevy_log;
    extern crate bevy_math;
    #[cfg(feature = "bevy_rapier2d")]
    extern crate bevy_rapier2d;
    extern crate bevy_reflect;
    extern crate bevy_render;
    extern crate bevy_sprite;
    extern crate bevy_tilemap_engine;
    extern crate bevy_tilemap_types;
    extern crate bevy_transform;
    extern crate bevy_utils;
    extern crate bevy_window;
    pub extern crate bitflags;
    #[cfg(feature = "serde")]
    extern crate serde;
    extern crate std;

    pub(crate) use bevy_app::{
        stage as app_stage, AppBuilder, Events, Plugin, PluginGroup, PluginGroupBuilder,
    };
    pub(crate) use bevy_asset::{AddAsset, Assets, Handle, HandleUntyped};
    pub(crate) use bevy_ecs::{
        Bundle, Changed, Commands, Entity, IntoSystem, Query, Res, ResMut, Resources, SystemStage,
    };
    pub(crate) use bevy_log::{error, info, warn};
    pub(crate) use bevy_math::Vec3;
    #[cfg(feature = "bevy_rapier2d")]
    pub(crate) use bevy_rapier2d::rapier::{
        dynamics::RigidBodyBuilder,
        geometry::{ColliderBuilder, InteractionGroups},
    };
    pub(crate) use bevy_reflect::{TypeUuid, Uuid};
    pub(crate) use bevy_render::{
        camera::Camera,
        color::Color,
        draw::{Draw, Visible},
        mesh::{Indices, Mesh},
        pipeline::{
            BlendDescriptor, BlendFactor, BlendOperation, ColorStateDescriptor, ColorWrite,
            CompareFunction, CullMode, DepthStencilStateDescriptor, FrontFace, PipelineDescriptor,
            PrimitiveTopology, RasterizationStateDescriptor, RenderPipeline, RenderPipelines,
            StencilStateDescriptor, StencilStateFaceDescriptor,
        },
        render_graph::{base::MainPass, RenderGraph},
        shader::{Shader, ShaderStage, ShaderStages},
        texture::TextureFormat,
    };
    pub(crate) use bevy_sprite::TextureAtlas;
    pub(crate) use bevy_tilemap_engine::{
        chunk::{
            base_tile::BaseTile,
            layer::{DenseLayer, LayerKind, LayerKindInner, SparseLayer, SpriteLayer},
            render::GridTopology,
            Chunk, ChunkRender,
        },
        tilemap::{TileLayer, Tilemap},
    };
    pub(crate) use bevy_tilemap_types::{
        dimension::{Dimension2, DimensionError, DimensionKind},
        point::Point2,
    };
    pub(crate) use bevy_transform::{
        components::{GlobalTransform, Parent, Transform},
        hierarchy::{BuildChildren, DespawnRecursiveExt},
    };
    pub(crate) use bevy_utils::{HashMap, HashSet};
    pub(crate) use bevy_window::WindowResized;

    pub(crate) use crate::bitflags::*;

    #[cfg(feature = "serde")]
    pub(crate) use serde::{Deserialize, Serialize};

    pub(crate) use std::{
        boxed::Box,
        clone::Clone,
        cmp::Ord,
        convert::{AsMut, AsRef, From, Into},
        default::Default,
        error::Error,
        fmt::{Debug, Display, Formatter, Result as FmtResult},
        iter::{Extend, IntoIterator, Iterator},
        option::Option::{self, *},
        result::Result::{self, *},
        vec::Vec,
    };

    // Macros
    pub(crate) use std::{vec, write};

    #[cfg(debug_assertions)]
    #[allow(unused_imports)]
    pub(crate) use bevy_log::debug;

    #[cfg(debug_assertions)]
    #[allow(unused_imports)]
    pub(crate) use std::println;
}
