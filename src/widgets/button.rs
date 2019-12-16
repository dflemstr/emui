use crate::layout;
use crate::theme;
use crate::util;
use crate::widget;

#[derive(Debug, Default)]
pub struct State {
    layout: layout::State,
    pressed: bool,
}

#[derive(Debug)]
pub struct Button<'a> {
    state: &'a mut State,
    text: &'a str,
}

impl<'a> Button<'a> {
    pub fn new(state: &'a mut State, text: &'a str) -> Self {
        Self { state, text }
    }
}

impl<'a, T> widget::Node<'a, T> for Button<'a>
where
    T: theme::Theme<'a>,
{
    fn visit<V>(&self, visitor: V) -> V::Output
    where
        V: widget::Visitor<'a, T>,
    {
        visitor.accept_leaf(self)
    }

    fn visit_mut<V>(&mut self, visitor: V) -> V::Output
    where
        V: widget::MutVisitor<'a, T>,
    {
        visitor.accept_leaf_mut(self)
    }
}

impl<'a, T> widget::Widget<'a, T> for Button<'a>
where
    T: theme::Theme<'a>,
{
    type Output = core::iter::Chain<util::FontIter<'a, T>, util::RectIter<T::Color>>;

    fn draw(&self, context: &widget::DrawContext<T>) -> Self::Output {
        use embedded_graphics::fonts::Font;
        use embedded_graphics::geometry::Dimensions;
        use embedded_graphics::style::WithStyle;
        use embedded_graphics::transform::Transform;

        let x1 = context.position.x;
        let y1 = context.position.y;
        let x2 = x1 + context.size.width as i32 - 1;
        let y2 = y1 + context.size.height as i32 - 1;

        let text = T::Font::render_str(&self.text);
        let text_size = text.size();
        let xt = x1 + (context.size.width as i32 - text_size.width as i32) / 2;
        let yt = y1 + (context.size.height as i32 - text_size.height as i32) / 2;

        text.translate(embedded_graphics::geometry::Point::new(xt, yt))
            .stroke(Some(context.theme.text_color()))
            .into_iter()
            .chain(embedded_graphics::egrectangle!(
                (x1, y1),
                (x2, y2),
                fill = if self.state.pressed {
                    Some(context.theme.border_color())
                } else {
                    None
                },
                stroke = Some(context.theme.border_color()),
            ))
    }

    fn layout_style(&self, theme: &T) -> layout::Style {
        layout::Style {
            flex_shrink: 1.0,
            flex_grow: 1.0,
            ..theme.base_style()
        }
    }

    fn layout_state(&self) -> &layout::State {
        &self.state.layout
    }

    fn layout_state_mut(&mut self) -> &mut layout::State {
        &mut self.state.layout
    }
}
