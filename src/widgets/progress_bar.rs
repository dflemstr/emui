use crate::layout;
use crate::theme;
use crate::util;
use crate::widget;
use embedded_graphics::egrectangle;

#[derive(Debug, Default)]
pub struct State {
    layout: layout::State,
}

#[derive(Debug)]
pub struct ProgressBar<'a> {
    state: &'a mut State,
    value: u32,
    max: u32,
}

impl<'a> ProgressBar<'a> {
    pub fn new(state: &'a mut State, value: u32, max: u32) -> Self {
        Self { state, value, max }
    }
}

impl<'a, T> widget::Node<'a, T> for ProgressBar<'a>
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

impl<'a, T> widget::Widget<'a, T> for ProgressBar<'a>
where
    T: theme::Theme<'a>,
{
    type Output = core::iter::Chain<util::RectIter<T::Color>, util::RectIter<T::Color>>;

    fn draw(&self, context: &widget::DrawContext<T>) -> Self::Output {
        let x1 = context.position.x;
        let y1 = context.position.y;
        let xval = x1 + ((self.value * (context.size.width - 1)) / self.max) as i32;
        let x2 = x1 + context.size.width as i32 - 1;
        let y2 = y1 + context.size.height as i32 - 1;

        egrectangle!(
            (x1, y1),
            (xval, y2),
            fill = Some(context.theme.fill_color())
        )
        .into_iter()
        .chain(egrectangle!(
            (x1, y1),
            (x2, y2),
            stroke = Some(context.theme.border_color())
        ))
    }

    fn layout_style(&self, theme: &T) -> layout::Style {
        layout::Style {
            flex_shrink: 1.0,
            flex_grow: 1.0,
            flex_basis: stretch::style::Dimension::Auto,
            min_size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(20.0),
                height: stretch::style::Dimension::Points(3.0),
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
