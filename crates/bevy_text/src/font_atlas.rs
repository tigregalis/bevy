use bevy_asset::{Assets, Handle};
use bevy_math::Vec2;
use bevy_render::texture::{Texture, TextureFormat};
use bevy_sprite::{DynamicTextureAtlasBuilder, TextureAtlas};
use bevy_utils::HashMap;
use glyph_brush_layout::ab_glyph::GlyphId;

pub struct FontAtlas {
    pub dynamic_texture_atlas_builder: DynamicTextureAtlasBuilder,
    pub glyph_to_index: HashMap<char, u32>,
    // we can easily get the glyph id from a char using `scaled_font.glyph_id(c)`
    // but there is no easy way to do the reverse, so we map these
    pub glyph_id_to_char: HashMap<GlyphId, char>,
    pub texture_atlas: Handle<TextureAtlas>,
}

impl FontAtlas {
    pub fn new(
        textures: &mut Assets<Texture>,
        texture_atlases: &mut Assets<TextureAtlas>,
        size: Vec2,
    ) -> FontAtlas {
        let atlas_texture = textures.add(Texture::new_fill(
            size,
            &[0, 0, 0, 0],
            TextureFormat::Rgba8UnormSrgb,
        ));
        let texture_atlas = TextureAtlas::new_empty(atlas_texture, size);
        Self {
            texture_atlas: texture_atlases.add(texture_atlas),
            glyph_to_index: HashMap::default(),
            glyph_id_to_char: HashMap::default(),
            dynamic_texture_atlas_builder: DynamicTextureAtlasBuilder::new(size, 1),
        }
    }

    pub fn get_char_index(&self, character: char) -> Option<u32> {
        self.glyph_to_index.get(&character).cloned()
    }

    pub fn add_char(
        &mut self,
        textures: &mut Assets<Texture>,
        texture_atlases: &mut Assets<TextureAtlas>,
        character: char,
        texture: &Texture,
        glyph_id: GlyphId,
    ) -> bool {
        let texture_atlas = texture_atlases.get_mut(&self.texture_atlas).unwrap();
        if let Some(index) =
            self.dynamic_texture_atlas_builder
                .add_texture(texture_atlas, textures, texture)
        {
            self.glyph_to_index.insert(character, index);
            self.glyph_id_to_char.insert(glyph_id, character);
            true
        } else {
            false
        }
    }
}
