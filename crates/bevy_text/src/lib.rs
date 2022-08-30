mod error;
mod font;
mod font_atlas;
mod font_atlas_set;
mod font_loader;
mod glyph_brush;
mod pipeline;
mod text;
mod text2d;

pub use error::*;
pub use font::*;
pub use font_atlas::*;
pub use font_atlas_set::*;
pub use font_loader::*;
pub use glyph_brush::*;
pub use pipeline::*;
pub use text::*;
pub use text2d::*;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        Font, HorizontalAlign, Text, Text2dBundle, TextAlignment, TextError, TextSection,
        TextStyle, VerticalAlign,
    };
}

use bevy_app::prelude::*;
use bevy_asset::AddAsset;
use bevy_ecs::{entity::Entity, schedule::ParallelSystemDescriptorCoercion, system::Resource};
use bevy_render::{RenderApp, RenderStage};
use bevy_sprite::SpriteSystem;
use bevy_window::ModifiesWindows;

pub type DefaultTextPipeline = TextPipeline<Entity>;

#[derive(Default)]
pub struct TextPlugin;

/// `TextPlugin` settings
#[derive(Resource)]
pub struct TextSettings {
    /// Maximum number of font atlases supported in a FontAtlasSet
    pub max_font_atlases: usize,
    /// Allows font size to be set dynamically exceeding the amount set in max_font_atlases.
    /// Note each font size has to be generated which can have a strong performance impact.
    pub allow_dynamic_font_size: bool,
}

impl Default for TextSettings {
    fn default() -> Self {
        Self {
            max_font_atlases: 16,
            allow_dynamic_font_size: false,
        }
    }
}

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Font>()
            .add_asset::<FontAtlasSet>()
            .register_type::<Text>()
            .register_type::<TextSection>()
            .register_type::<TextAlignment>()
            .register_type::<VerticalAlign>()
            .register_type::<HorizontalAlign>()
            .init_asset_loader::<FontLoader>()
            .init_resource::<TextSettings>()
            .insert_resource(DefaultTextPipeline::default())
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_text2d_layout.after(ModifiesWindows),
            );

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_system_to_stage(
                RenderStage::Extract,
                extract_text2d_sprite.after(SpriteSystem::ExtractSprites),
            );
        }
    }
}
