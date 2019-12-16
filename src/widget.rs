use crate::layout;
use crate::theme;

pub trait Node<'a, T>
where
    T: theme::Theme<'a>,
{
    fn visit<V>(&self, visitor: V) -> V::Output
    where
        V: Visitor<'a, T>;

    fn visit_mut<V>(&mut self, visitor: V) -> V::Output
    where
        V: MutVisitor<'a, T>;
}

pub trait Widget<'a, T>
where
    T: theme::Theme<'a>,
{
    type Output: Iterator<Item = embedded_graphics::drawable::Pixel<T::Color>>;

    fn draw(&self, context: &DrawContext<T>) -> Self::Output;

    fn layout_style(&self, theme: &T) -> layout::Style;

    fn layout_state(&self) -> &layout::State;

    fn layout_state_mut(&mut self) -> &mut layout::State;
}

pub trait Visitor<'a, T>: Sized
where
    T: theme::Theme<'a>,
{
    type Output;
    type ChildVisitor: ChildVisitor<'a, T, Output = Self::Output>;

    fn accept_leaf<N>(self, node: &N) -> Self::Output
    where
        N: Widget<'a, T>;

    fn accept_node<N>(self, node: &N, child_count: usize) -> Self::ChildVisitor
    where
        N: Widget<'a, T>;
}

pub trait MutVisitor<'a, T>: Sized
where
    T: theme::Theme<'a>,
{
    type Output;
    type MutChildVisitor: MutChildVisitor<'a, T, Output = Self::Output>;

    fn accept_leaf_mut<N>(self, node: &mut N) -> Self::Output
    where
        N: Widget<'a, T>;

    fn accept_node_mut<N>(self, node: &mut N, child_count: usize) -> Self::MutChildVisitor
    where
        N: Widget<'a, T>;
}

pub trait ChildVisitor<'a, T>
where
    T: theme::Theme<'a>,
{
    type Output;

    fn accept_child<W>(&mut self, widget: &W)
    where
        W: Node<'a, T>;

    fn end(self) -> Self::Output;
}

pub trait MutChildVisitor<'a, T>
where
    T: theme::Theme<'a>,
{
    type Output;

    fn accept_child_mut<W>(&mut self, widget: &mut W)
    where
        W: Node<'a, T>;

    fn end(self) -> Self::Output;
}

pub struct DrawContext<'a, T> {
    pub theme: &'a T,
    pub position: embedded_graphics::geometry::Point,
    pub size: embedded_graphics::geometry::Size,
}
