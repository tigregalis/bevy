use bevy_asset::Handle;
use bevy_math::Size;

use crate::{Font, TextAlignment, TextStyle};

#[derive(Debug, Default, Clone)]
pub struct Text {
    pub sections: TextType,
    pub alignment: TextAlignment,
}

#[derive(Debug, Clone)]
pub enum TextType {
    Simple(TextSection),
    Rich(Vec<TextSection>),
}

impl Default for TextType {
    fn default() -> Self {
        Self::Simple(Default::default())
    }
}

impl TextType {
    pub fn to_text_sections(&self) -> &[TextSection] {
        match self {
            TextType::Simple(section) => std::slice::from_ref(section),
            TextType::Rich(sections) => &sections,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct TextSection {
    pub value: String,
    pub font: Handle<Font>,
    pub style: TextStyle,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct CalculatedSize {
    pub size: Size,
}
