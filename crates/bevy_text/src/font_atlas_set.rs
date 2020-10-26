use crate::{Font, FontAtlas};
use ab_glyph::{Glyph, ScaleFont, GlyphId};
use bevy_asset::{Assets, Handle};
use bevy_core::FloatOrd;
use bevy_math::{Size, Vec2};
use bevy_render::texture::Texture;
use bevy_sprite::TextureAtlas;
use bevy_type_registry::TypeUuid;
use bevy_utils::HashMap;

// work around rust's f32 order/hash limitations
type FontSizeKey = FloatOrd;

#[derive(Default, TypeUuid)]
#[uuid = "73ba778b-b6b5-4f45-982d-d21b6b86ace2"]
pub struct FontAtlasSet {
    font: Handle<Font>,
    font_atlases: HashMap<FontSizeKey, Vec<FontAtlas>>,
}

#[derive(Debug)]
pub struct GlyphAtlasInfo {
    pub texture_atlas: Handle<TextureAtlas>,
    pub char_index: u32,
}

impl FontAtlasSet {
    pub fn new(font: Handle<Font>) -> Self {
        Self {
            font,
            font_atlases: HashMap::default(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&FontSizeKey, &Vec<FontAtlas>)> {
        self.font_atlases.iter()
    }

    pub fn has_char(&self, character: char, font_size: f32) -> bool {
        self.font_atlases
            .get(&FloatOrd(font_size))
            .map_or(false, |font_atlas| {
                font_atlas
                    .iter()
                    .any(|atlas| atlas.get_char_index(character).is_some())
            })
    }

    pub fn add_glyphs_to_atlas(
        &mut self,
        fonts: &Assets<Font>,
        texture_atlases: &mut Assets<TextureAtlas>,
        textures: &mut Assets<Texture>,
        font_size: f32,
        text: &str,
        glyph_layout: &GlyphLayout,
    ) -> Option<Size> {
        let font = fonts.get(&self.font)?;
        let scaled_font = ab_glyph::Font::as_scaled(&font.font, font_size);
        let font_atlases = self
            .font_atlases
            .entry(FloatOrd(font_size))
            .or_insert_with(|| {
                vec![FontAtlas::new(
                    textures,
                    texture_atlases,
                    Vec2::new(512.0, 512.0),
                )]
            });
        for character in text.chars() {
            if character.is_control() {
                continue;
            }
            let glyph = scaled_font.scaled_glyph(character);
            let glyph_id = glyph.id;
            if !font_atlases
                .iter()
                .any(|atlas| atlas.get_char_index(character).is_some())
            {
                if let Some(outlined_glyph) = scaled_font.outline_glyph(glyph.clone()) {
                    let glyph_texture = Font::get_outlined_glyph_texture(outlined_glyph);
                    // there is a mismatch between the size of this texture
                    // and the size of the outlined glyph
                    let add_char_to_font_atlas = |atlas: &mut FontAtlas| -> bool {
                        atlas.add_char(textures, texture_atlases, character, &glyph_texture, glyph_id)
                    };
                    if !font_atlases.iter_mut().any(add_char_to_font_atlas) {
                        font_atlases.push(FontAtlas::new(
                            textures,
                            texture_atlases,
                            Vec2::new(512.0, 512.0),
                        ));
                        if !font_atlases.last_mut().unwrap().add_char(
                            textures,
                            texture_atlases,
                            character,
                            &glyph_texture,
                            glyph_id,
                        ) {
                            panic!("could not add character to newly created FontAtlas");
                        }
                    }
                }
            }
        }
        // TODO: split this off into a separate method
        let glyphs = &glyph_layout.glyphs;
        let mut max_x: f32 = 0.0;
        let mut max_y: f32 = 0.0;
        for glyph in glyphs.iter() {
            max_x = max_x.max(glyph.position.x + scaled_font.h_advance(glyph.id));
            max_y = max_y.max(glyph.position.y - scaled_font.descent());
        }
        let size = Size::new(max_x, max_y);
        Some(size)
    }

    pub fn get_char(&self, font_size: f32, glyph_id: &GlyphId) -> Option<&char> {
        self
            .font_atlases
            .get(&FloatOrd(font_size))
            .and_then(|font_atlas| {
                font_atlas
                    .iter()
                    .find_map(|atlas| {
                        atlas.glyph_id_to_char.get(&glyph_id)
                    })
            })
    }

    pub fn get_glyph_atlas_info(&self, font_size: f32, glyph_id: &GlyphId) -> Option<GlyphAtlasInfo> {
        self.font_atlases
            .get(&FloatOrd(font_size))
            .and_then(|font_atlas| {
                font_atlas
                    .iter()
                    .find_map(|atlas| {
                        let character = *atlas.glyph_id_to_char.get(glyph_id)?;
                        atlas
                            .get_char_index(character)
                            .map(|char_index| (char_index, atlas.texture_atlas.clone_weak()))
                    })
                    .map(|(char_index, texture_atlas)| GlyphAtlasInfo {
                        texture_atlas,
                        char_index,
                    })
            })
    }
}

#[derive(Clone, Debug, Default)]
pub struct GlyphLayout {
    pub glyphs: Vec<Glyph>
}