use bevy_asset::Handle;
use bevy_math::Size;

use crate::{Font, TextAlignment, TextStyle};

#[derive(Debug, Clone)]
pub enum Text {
    Basic(BasicText),
    Rich(RichText),
}

impl Default for Text {
    fn default() -> Self {
        Self::Basic(BasicText::default())
    }
}

#[derive(Debug, Default, Clone)]
pub struct BasicText {
    pub section: TextSection,
    pub alignment: TextAlignment,
}

#[derive(Debug, Default, Clone)]
pub struct RichText {
    pub sections: Vec<TextSection>,
    pub alignment: TextAlignment,
}

impl Text {
    pub fn section(&self) -> &TextSection {
        match self {
            Text::Basic(BasicText { section, .. }) => section,
            Text::Rich(RichText { sections, .. }) => &sections[0],
        }
    }

    pub fn section_mut(&mut self) -> &mut TextSection {
        match self {
            Text::Basic(BasicText { section, .. }) => section,
            Text::Rich(RichText { sections, .. }) => &mut sections[0],
        }
    }

    pub fn sections(&self) -> &[TextSection] {
        match self {
            Text::Basic(BasicText { section, .. }) => std::slice::from_ref(section),
            Text::Rich(RichText { sections, .. }) => sections,
        }
    }

    pub fn sections_mut(&mut self) -> &mut [TextSection] {
        match self {
            Text::Basic(BasicText { section, .. }) => std::slice::from_mut(section),
            Text::Rich(RichText { sections, .. }) => sections,
        }
    }

    pub fn alignment(&self) -> &TextAlignment {
        match self {
            Text::Basic(BasicText { alignment, .. }) => alignment,
            Text::Rich(RichText { alignment, .. }) => alignment,
        }
    }

    pub fn alignment_mut(&mut self) -> &mut TextAlignment {
        match self {
            Text::Basic(BasicText { alignment, .. }) => alignment,
            Text::Rich(RichText { alignment, .. }) => alignment,
        }
    }
}

impl From<BasicText> for Text {
    fn from(inner: BasicText) -> Self {
        Text::Basic(inner)
    }
}

impl From<RichText> for Text {
    fn from(inner: RichText) -> Self {
        Text::Rich(inner)
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
