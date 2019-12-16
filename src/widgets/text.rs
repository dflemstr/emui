use crate::layout;
use crate::theme;
use crate::util;
use crate::widget;

#[derive(Debug, Default)]
pub struct State {
    layout: layout::State,
}

#[derive(Debug)]
pub struct Text<'a> {
    state: &'a mut State,
    text: &'a str,
}

impl<'a> Text<'a> {
    pub fn new(state: &'a mut State, text: &'a str) -> Self {
        Self { state, text }
    }
}

impl<'a, T> widget::Node<'a, T> for Text<'a>
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

impl<'a, T> widget::Widget<'a, T> for Text<'a>
where
    T: theme::Theme<'a>,
{
    type Output = util::FontIter<'a, T>;

    fn draw(&self, context: &widget::DrawContext<T>) -> Self::Output {
        use embedded_graphics::fonts::Font;
        use embedded_graphics::style::WithStyle;
        use embedded_graphics::transform::Transform;

        let x = context.position.x;
        let y = context.position.y;
        T::Font::render_str(&self.text)
            .translate(embedded_graphics::geometry::Point::new(x, y))
            .stroke(Some(context.theme.text_color()))
            .into_iter()
    }

    fn layout_style(&self, theme: &T) -> layout::Style {
        use embedded_graphics::fonts::Font;
        use embedded_graphics::geometry::Dimensions;

        let size = T::Font::render_str(&self.text).size();
        layout::Style {
            align_self: stretch::style::AlignSelf::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(size.width as f32),
                height: stretch::style::Dimension::Points(size.height as f32),
            },
            margin: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(theme.spacing()),
                end: stretch::style::Dimension::Points(theme.spacing()),
                top: stretch::style::Dimension::Points(theme.spacing()),
                bottom: stretch::style::Dimension::Points(theme.spacing()),
            },
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
