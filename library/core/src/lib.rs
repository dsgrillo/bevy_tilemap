#[no_implicit_prelude]
pub mod chunk;
#[no_implicit_prelude]
pub mod event;
#[no_implicit_prelude]
pub mod system;
#[no_implicit_prelude]
pub mod tilemap;

use lib::*;

pub struct SpawnedChunks(pub HashMap<Entity, HashSet<Point2>>);

#[no_implicit_prelude]
mod lib {
    extern crate bevy_app;
    extern crate bevy_asset;
    extern crate bevy_ecs;
    extern crate bevy_log;
    extern crate bevy_math;
    extern crate bevy_reflect;
    extern crate bevy_render;
    extern crate bevy_sprite;
    extern crate bevy_tilemap_types;
    extern crate bevy_transform;
    extern crate bevy_utils;
    extern crate bevy_window;
    extern crate std;

    pub(crate) use bevy_app::{Events, Plugin, PluginGroup, PluginGroupBuilder};
    pub(crate) use bevy_asset::{AddAsset, Assets, Handle, HandleUntyped};
    pub(crate) use bevy_ecs::{
        Bundle, Changed, Commands, Component, Entity, IntoSystem, Mut, Query, Res, ResMut,
        Resources, SystemStage, WorldQuery,
    };
    pub(crate) use bevy_log::{error, info, warn};
    pub(crate) use bevy_math::Vec3;
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
}
