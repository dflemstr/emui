use crate::layout;
use crate::theme;
use crate::util;
use crate::widget;
use core::marker;

#[derive(Debug, Default)]
pub struct State {
    layout: layout::State,
}

#[derive(Debug)]
pub struct Frame<'a, N, T> {
    state: &'a mut State,
    phantom: marker::PhantomData<T>,
    label: Option<&'a str>,
    child: N,
}

impl<'a, N, T> Frame<'a, N, T>
where
    N: widget::Node<'a, T>,
    T: theme::Theme<'a>,
{
    pub fn new(state: &'a mut State, label: &'a str, child: N) -> Self {
        let phantom = marker::PhantomData;
        let label = Some(label);
        Self {
            state,
            phantom,
            label,
            child,
        }
    }
}

impl<'a, N, T> widget::Node<'a, T> for Frame<'a, N, T>
where
    N: widget::Node<'a, T>,
    T: theme::Theme<'a>,
{
    fn visit<V>(&self, visitor: V) -> V::Output
    where
        V: widget::Visitor<'a, T>,
    {
        use widget::ChildVisitor;

        let mut visitor = visitor.accept_node(self, 1);
        visitor.accept_child(&self.child);
        visitor.end()
    }

    fn visit_mut<V>(&mut self, visitor: V) -> V::Output
    where
        V: widget::MutVisitor<'a, T>,
    {
        use widget::MutChildVisitor;

        let mut visitor = visitor.accept_node_mut(self, 1);
        visitor.accept_child_mut(&mut self.child);
        visitor.end()
    }
}

impl<'a, N, T> widget::Widget<'a, T> for Frame<'a, N, T>
where
    N: widget::Node<'a, T>,
    T: theme::Theme<'a>,
{
    type Output =
        core::iter::Chain<util::RectIter<T::Color>, util::OptionalIter<util::FontIter<'a, T>>>;

    fn draw(&self, context: &widget::DrawContext<T>) -> Self::Output {
        use embedded_graphics::fonts::Font;
        use embedded_graphics::geometry::Dimensions;
        use embedded_graphics::style::WithStyle;
        use embedded_graphics::transform::Transform;

        let x1 = context.position.x;
        let y1 = context.position.y;
        let x2 = x1 + context.size.width as i32 - 1;
        let y2 = y1 + context.size.height as i32 - 1;

        let rect_iter = embedded_graphics::egrectangle!(
            (x1, y1),
            (x2, y2),
            stroke = Some(context.theme.border_color())
        )
        .into_iter();

        let text_iter = if let Some(label) = self.label {
            let text = T::Font::render_str(label);
            let text_size = text.size();
            let tx = x1 + 2 * context.theme.spacing() as i32;
            let ty = y1 - text_size.height as i32 / 2;
            Some(
                text.translate(embedded_graphics::geometry::Point::new(tx, ty))
                    .fill(Some(context.theme.background_color()))
                    .stroke(Some(context.theme.text_color()))
                    .into_iter(),
            )
        } else {
            None
        };

        rect_iter.chain(text_iter.into_iter().flatten())
    }

    fn layout_style(&self, theme: &T) -> layout::Style {
        layout::Style {
            flex_shrink: 1.0,
            flex_grow: 1.0,
            flex_basis: stretch::style::Dimension::Auto,
            flex_wrap: stretch::style::FlexWrap::Wrap,
            align_items: stretch::style::AlignItems::Stretch,
            align_content: stretch::style::AlignContent::Stretch,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Auto,
                height: stretch::style::Dimension::Auto,
            },
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(3.0 * theme.spacing()),
                end: stretch::style::Dimension::Points(-theme.spacing()),
                top: stretch::style::Dimension::Points(3.0 * theme.spacing()),
                bottom: stretch::style::Dimension::Points(-theme.spacing()),
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
