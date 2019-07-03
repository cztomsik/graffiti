use crate::text::TextLayout;
use font_kit::source::SystemSource;
use crate::SceneListener;

pub struct SimpleTextLayout {
    font: font_kit::font::Font
}

impl SimpleTextLayout {
    pub fn new() -> Self {
        let font = SystemSource::new()
            .select_by_postscript_name("ArialMT")
            .unwrap()
            .load()
            .unwrap();

        SimpleTextLayout {
            font
        }
    }
}

impl SceneListener for SimpleTextLayout {

}

impl TextLayout for SimpleTextLayout {

}

#[cfg(test)]
mod tests {
    use super::SimpleTextLayout;

    #[test]
    fn test_new() {
        let text_layout = SimpleTextLayout::new();
    }
}
