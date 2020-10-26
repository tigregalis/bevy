use crate::{Font, FontAtlasSet, GlyphLayout};
use ab_glyph::{PxScale, ScaleFont};
use bevy_asset::Assets;
use bevy_math::{Mat4, Vec2, Vec3};
use bevy_render::{
    color::Color,
    draw::{Draw, DrawContext, DrawError, Drawable},
    mesh,
    pipeline::PipelineSpecialization,
    prelude::Msaa,
    renderer::{
        AssetRenderResourceBindings, BindGroup, BufferUsage, RenderResourceBindings,
        RenderResourceId,
    },
};
use bevy_sprite::{TextureAtlas, TextureAtlasSprite};

#[derive(Clone, Debug)]
pub struct TextStyle {
    pub font_size: f32,
    pub color: Color,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            font_size: 12.0,
        }
    }
}

pub struct DrawableText<'a> {
    pub font: &'a Font,
    pub font_atlas_set: &'a FontAtlasSet,
    pub texture_atlases: &'a Assets<TextureAtlas>,
    pub render_resource_bindings: &'a mut RenderResourceBindings,
    pub asset_render_resource_bindings: &'a mut AssetRenderResourceBindings,
    pub position: Vec3,
    pub container_size: Vec2,
    pub style: &'a TextStyle,
    pub text: &'a str,
    pub msaa: &'a Msaa,
    pub glyph_layout: &'a GlyphLayout,
}

impl<'a> Drawable for DrawableText<'a> {
    fn draw(&mut self, draw: &mut Draw, context: &mut DrawContext) -> Result<(), DrawError> {
        // TODO:
        // - Call `outline_glyph only` once and store the `OutlineGlyph`s
        // - Provide other options for features (fixed / max width, h-align, multi-style text using sections)
        // - Test performance
        // - Likely performance win from caching
        // - Consider batching (e.g. as a resource) for performance
        // - Consider using higher-level `glyph_brush` API (with batching)
        context.set_pipeline(
            draw,
            &bevy_sprite::SPRITE_SHEET_PIPELINE_HANDLE,
            &PipelineSpecialization {
                sample_count: self.msaa.samples,
                ..Default::default()
            },
        )?;

        let render_resource_context = &**context.render_resource_context;
        if let Some(RenderResourceId::Buffer(quad_vertex_buffer)) = render_resource_context
            .get_asset_resource(&bevy_sprite::QUAD_HANDLE, mesh::VERTEX_BUFFER_ASSET_INDEX)
        {
            draw.set_vertex_buffer(0, quad_vertex_buffer, 0);
        }
        let mut indices = 0..0;
        if let Some(RenderResourceId::Buffer(quad_index_buffer)) = render_resource_context
            .get_asset_resource(&bevy_sprite::QUAD_HANDLE, mesh::INDEX_BUFFER_ASSET_INDEX)
        {
            draw.set_index_buffer(quad_index_buffer, 0);
            if let Some(buffer_info) = render_resource_context.get_buffer_info(quad_index_buffer) {
                indices = 0..(buffer_info.size / 4) as u32;
            } else {
                panic!("expected buffer type");
            }
        }

        // set global bindings
        context.set_bind_groups_from_bindings(draw, &mut [self.render_resource_bindings])?;

        // NOTE: this uses ab_glyph apis directly. it _might_ be a good idea to add our own layer on top
        let font = &self.font.font;
        let scale = PxScale::from(self.style.font_size);
        let scaled_font = ab_glyph::Font::as_scaled(&font, scale);
        let glyphs = &self.glyph_layout.glyphs;
        // vertical axis of glyph position (up = -) is opposite that of transform (up = +)
        // caret (transform) is the bottom left corner of the text rectangle
        // max_y (glyph position) is the height of the text rectangle, i.e. the bottom edge of the text rectangle
        // caret + |max_y| therefore is the top left corner of the text rectangle
        let caret = self.position;
        let mut max_y: f32 = 0.0;
        for glyph in glyphs.iter() {
            max_y = max_y.max(glyph.position.y - scaled_font.descent());
        }
        max_y = max_y.floor();

        // set local per-character bindings
        for glyph in glyphs.iter() {
            if let Some(glyph_atlas_info) = self
                .font_atlas_set
                .get_glyph_atlas_info(self.style.font_size, &glyph.id)
            {
                if let Some(outlined) = scaled_font.outline_glyph(glyph.clone()) {
                    let texture_atlas = self
                        .texture_atlases
                        .get(&glyph_atlas_info.texture_atlas)
                        .unwrap();
                    let glyph_rect = texture_atlas.textures[glyph_atlas_info.char_index as usize];
                    let glyph_width = glyph_rect.width();
                    let glyph_height = glyph_rect.height();
                    let atlas_render_resource_bindings = self
                        .asset_render_resource_bindings
                        .get_mut(&glyph_atlas_info.texture_atlas)
                        .unwrap();
                    context.set_bind_groups_from_bindings(
                        draw,
                        &mut [atlas_render_resource_bindings],
                    )?;

                    let bounds = outlined.px_bounds();
                    let x = bounds.min.x + glyph_width / 2.0;
                    // the 0.5 accounts for odd-numbered heights (bump up by 1 pixel)
                    let y = max_y - bounds.max.y + glyph_height / 2.0 + 0.5;
                    let transform = Mat4::from_translation(caret + Vec3::new(x, y, 0.0));

                    let sprite = TextureAtlasSprite {
                        index: glyph_atlas_info.char_index,
                        color: self.style.color,
                    };

                    let transform_buffer = context
                        .shared_buffers
                        .get_buffer(&transform, BufferUsage::UNIFORM)
                        .unwrap();
                    let sprite_buffer = context
                        .shared_buffers
                        .get_buffer(&sprite, BufferUsage::UNIFORM)
                        .unwrap();
                    let sprite_bind_group = BindGroup::build()
                        .add_binding(0, transform_buffer)
                        .add_binding(1, sprite_buffer)
                        .finish();
                    context.create_bind_group_resource(2, &sprite_bind_group)?;
                    draw.set_bind_group(2, &sprite_bind_group);
                    draw.draw_indexed(indices.clone(), 0, 0..1);
                }
            }
        }

        Ok(())
    }
}
