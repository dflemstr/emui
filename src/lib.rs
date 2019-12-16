#![no_std]

use core::marker;

pub mod children;
pub mod layout;
pub mod renderer;
pub mod theme;
pub mod util;
pub mod widget;

pub mod widgets;

pub struct Emui<C, D>
where
    C: embedded_graphics::pixelcolor::PixelColor,
    D: embedded_graphics::Drawing<C>,
{
    layout: layout::Layout,
    drawing: D,
    phantom: marker::PhantomData<C>,
}

impl<C, D> Emui<C, D>
where
    C: embedded_graphics::pixelcolor::PixelColor,
    D: embedded_graphics::Drawing<C>,
{
    pub fn new(drawing: D) -> Self {
        let layout = layout::Layout::new();
        let phantom = marker::PhantomData;
        Self {
            layout,
            drawing,
            phantom,
        }
    }

    pub fn update<'a, T, W>(&'a mut self, width: u32, height: u32, theme: &'a T, widget: &mut W)
    where
        T: theme::Theme<'a, Color = C>,
        T::Font: embedded_graphics::fonts::Font<'a, C>,
        W: widget::Node<'a, T>,
    {
        self.layout.update_tree(width, height, theme, widget);
        renderer::Renderer::new(theme, &self.layout, &mut self.drawing).render_tree(widget);
    }

    pub fn drawing(&self) -> &D {
        &self.drawing
    }

    pub fn drawing_mut(&mut self) -> &mut D {
        &mut self.drawing
    }

    pub fn into_drawing(self) -> D {
        self.drawing
    }
}
