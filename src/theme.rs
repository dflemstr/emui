use core::marker;

pub trait Theme<'a> {
    type Color: embedded_graphics::pixelcolor::PixelColor;
    type Font: embedded_graphics::fonts::Font<'a, Self::Color>
        + embedded_graphics::transform::Transform
        + IntoIterator<Item = embedded_graphics::drawable::Pixel<Self::Color>>;

    fn base_style(&self) -> stretch::style::Style {
        stretch::style::Style {
            margin: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(self.spacing()),
                end: stretch::style::Dimension::Points(self.spacing()),
                top: stretch::style::Dimension::Points(self.spacing()),
                bottom: stretch::style::Dimension::Points(self.spacing()),
            },
            ..Default::default()
        }
    }

    fn spacing(&self) -> f32;

    fn background_color(&self) -> Self::Color;

    fn text_color(&self) -> Self::Color;

    fn border_color(&self) -> Self::Color;

    fn fill_color(&self) -> Self::Color;
}

#[derive(Debug)]
pub struct SimpleTheme<C, F> {
    pub spacing: f32,
    pub background_color: C,
    pub text_color: C,
    pub border_color: C,
    pub fill_color: C,
    pub phantom: marker::PhantomData<F>,
}

impl<'a, C, F> Theme<'a> for SimpleTheme<C, F>
where
    C: embedded_graphics::pixelcolor::PixelColor,
    F: embedded_graphics::fonts::Font<'a, C>
        + embedded_graphics::transform::Transform
        + IntoIterator<Item = embedded_graphics::drawable::Pixel<C>>,
{
    type Color = C;
    type Font = F;

    fn spacing(&self) -> f32 {
        self.spacing
    }

    fn background_color(&self) -> Self::Color {
        self.background_color
    }

    fn text_color(&self) -> Self::Color {
        self.text_color
    }

    fn border_color(&self) -> Self::Color {
        self.border_color
    }

    fn fill_color(&self) -> Self::Color {
        self.fill_color
    }
}
