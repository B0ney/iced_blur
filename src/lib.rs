use iced_core::{Element, Length};

mod shader;

pub fn blur(passes: u32, offset: f32) -> Blur {
    Blur::new(passes, offset)
}

pub struct Blur {
    width: Length,
    height: Length,
    passes: u32,
    offset: f32,
}

impl Blur {
    pub fn new(passes: u32, offset: f32) -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            passes,
            offset,
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
}

impl<'a, Message, Theme, Renderer> From<Blur> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_core::Renderer + iced_widget::shader::Renderer,
{
    fn from(value: Blur) -> Self {
        let Blur {
            width,
            height,
            passes,
            offset,
        } = value;

        Element::new(
            iced_widget::shader(shader::Shader::new(passes, offset))
                .width(width)
                .height(height),
        )
    }
}
