use crate::children;
use crate::layout;
use crate::theme;
use crate::widget;
use core::marker;

pub type Main<'a, T, X> = Container<'a, MainBase, T, X>;

pub type Row<'a, T, X> = Container<'a, RowBase, T, X>;

pub type Column<'a, T, X> = Container<'a, ColumnBase, T, X>;

#[derive(Debug, Default)]
pub struct State {
    layout: layout::State,
}

#[derive(Debug)]
pub struct Container<'a, B, T, X> {
    state: &'a mut State,
    children: X,
    phantom: marker::PhantomData<(B, T)>,
}

pub trait Base {
    const FLEX_DIRECTION: stretch::style::FlexDirection;
    const SIZE: stretch::geometry::Size<stretch::style::Dimension>;
    const HORZ_SPACE_FACTOR: f32;
    const VERT_SPACE_FACTOR: f32;
}

#[derive(Debug)]
pub enum MainBase {}

#[derive(Debug)]
pub enum RowBase {}

#[derive(Debug)]
pub enum ColumnBase {}

impl Base for MainBase {
    const FLEX_DIRECTION: stretch::style::FlexDirection = stretch::style::FlexDirection::Column;
    const SIZE: stretch::geometry::Size<stretch::style::Dimension> = stretch::geometry::Size {
        width: stretch::style::Dimension::Percent(1.0),
        height: stretch::style::Dimension::Percent(1.0),
    };
    const HORZ_SPACE_FACTOR: f32 = 1.0;
    const VERT_SPACE_FACTOR: f32 = 1.0;
}

impl Base for RowBase {
    const FLEX_DIRECTION: stretch::style::FlexDirection = stretch::style::FlexDirection::Row;
    const SIZE: stretch::geometry::Size<stretch::style::Dimension> = stretch::geometry::Size {
        width: stretch::style::Dimension::Auto,
        height: stretch::style::Dimension::Auto,
    };
    const HORZ_SPACE_FACTOR: f32 = -1.0;
    const VERT_SPACE_FACTOR: f32 = -1.0;
}

impl Base for ColumnBase {
    const FLEX_DIRECTION: stretch::style::FlexDirection = stretch::style::FlexDirection::Column;
    const SIZE: stretch::geometry::Size<stretch::style::Dimension> = stretch::geometry::Size {
        width: stretch::style::Dimension::Auto,
        height: stretch::style::Dimension::Auto,
    };
    const HORZ_SPACE_FACTOR: f32 = -1.0;
    const VERT_SPACE_FACTOR: f32 = -1.0;
}

impl<'a, B, T, X> Container<'a, B, T, X>
where
    B: Base,
    T: theme::Theme<'a>,
    X: children::Children<'a, T>,
{
    pub fn new(state: &'a mut State, children: X) -> Self {
        let phantom = marker::PhantomData;
        Self {
            state,
            phantom,
            children,
        }
    }
}

impl<'a, B, T, X> widget::Node<'a, T> for Container<'a, B, T, X>
where
    B: Base,
    T: theme::Theme<'a>,
    X: children::Children<'a, T>,
{
    fn visit<V>(&self, visitor: V) -> V::Output
    where
        V: widget::Visitor<'a, T>,
    {
        let visitor = visitor.accept_node(self, self.children.len());
        self.children.visit_children(visitor)
    }

    fn visit_mut<V>(&mut self, visitor: V) -> V::Output
    where
        V: widget::MutVisitor<'a, T>,
    {
        let visitor = visitor.accept_node_mut(self, self.children.len());
        self.children.visit_children_mut(visitor)
    }
}

impl<'a, B, T, X> widget::Widget<'a, T> for Container<'a, B, T, X>
where
    B: Base,
    T: theme::Theme<'a>,
    X: children::Children<'a, T>,
{
    type Output = core::iter::Empty<embedded_graphics::drawable::Pixel<T::Color>>;

    fn draw(&self, _context: &widget::DrawContext<T>) -> Self::Output {
        core::iter::empty()
    }

    fn layout_style(&self, theme: &T) -> layout::Style {
        layout::Style {
            flex_shrink: 1.0,
            flex_grow: 1.0,
            flex_basis: stretch::style::Dimension::Auto,
            flex_wrap: stretch::style::FlexWrap::Wrap,
            align_items: stretch::style::AlignItems::Stretch,
            align_content: stretch::style::AlignContent::Stretch,
            flex_direction: B::FLEX_DIRECTION,
            size: B::SIZE,
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(theme.spacing() * B::HORZ_SPACE_FACTOR),
                end: stretch::style::Dimension::Points(theme.spacing() * B::HORZ_SPACE_FACTOR),
                top: stretch::style::Dimension::Points(theme.spacing() * B::VERT_SPACE_FACTOR),
                bottom: stretch::style::Dimension::Points(theme.spacing() * B::VERT_SPACE_FACTOR),
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
