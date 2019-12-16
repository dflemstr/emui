//! Layout algorithms and data structures.
//!
//! The core data structure in this
use crate::theme;
use crate::widget;
use core::fmt;

pub struct Layout {
    stretch: stretch::Stretch,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Geometry {
    pub position: Point,
    pub size: Size,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Debug, Default)]
pub struct State {
    node: Option<stretch::node::Node>,
}

pub type Style = stretch::style::Style;

struct StretchVisitor<'a, T> {
    stretch: &'a mut stretch::Stretch,
    theme: &'a T,
}

struct StretchChildVisitor<'a, T> {
    index: usize,
    child_count: usize,
    old_child_count: usize,
    parent: stretch::node::Node,
    stretch: &'a mut stretch::Stretch,
    theme: &'a T,
}

impl Layout {
    pub fn new() -> Self {
        let stretch = stretch::Stretch::new();
        Self { stretch }
    }

    pub fn get_geometry(&self, data: &State) -> Option<Geometry> {
        data.node.map(|n| {
            let layout = self.stretch.layout(n).unwrap();

            let stretch::geometry::Point { x, y } = layout.location;
            let position = Point { x, y };

            let stretch::geometry::Size { width, height } = layout.size;
            let size = Size { width, height };

            Geometry { position, size }
        })
    }

    pub fn update_tree<'a, T, N>(&mut self, width: u32, height: u32, theme: &T, widget: &mut N)
    where
        T: theme::Theme<'a>,
        N: widget::Node<'a, T>,
    {
        let stretch = &mut self.stretch;
        let root = widget.visit_mut(StretchVisitor { stretch, theme });
        let width = stretch::number::Number::Defined(width as f32);
        let height = stretch::number::Number::Defined(height as f32);
        stretch
            .compute_layout(root, stretch::geometry::Size { width, height })
            .unwrap();
    }
}

impl fmt::Debug for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Layout").finish()
    }
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, 'b, T> widget::MutVisitor<'b, T> for StretchVisitor<'a, T>
where
    T: theme::Theme<'b>,
{
    type Output = stretch::node::Node;
    type MutChildVisitor = StretchChildVisitor<'a, T>;

    fn accept_leaf_mut<W>(self, widget: &mut W) -> Self::Output
    where
        W: widget::Widget<'b, T>,
    {
        let style = widget.layout_style(self.theme);
        let node = *widget
            .layout_state_mut()
            .node
            .get_or_insert_with(|| self.stretch.new_node(style, &[]).unwrap());

        if &style != self.stretch.style(node).unwrap() {
            self.stretch.set_style(node, style).unwrap();
        }

        node
    }

    fn accept_node_mut<W>(self, widget: &mut W, child_count: usize) -> Self::MutChildVisitor
    where
        W: widget::Widget<'b, T>,
    {
        let style = widget.layout_style(self.theme);

        let index = 0;
        let parent = *widget
            .layout_state_mut()
            .node
            .get_or_insert_with(|| self.stretch.new_node(style, &[]).unwrap());
        let stretch = self.stretch;
        let theme = self.theme;
        let old_child_count = stretch.child_count(parent).unwrap();

        if child_count < old_child_count {
            for i in (child_count..old_child_count).rev() {
                stretch.remove_child_at_index(parent, i).unwrap();
            }
        }

        StretchChildVisitor {
            index,
            child_count,
            old_child_count,
            parent,
            stretch,
            theme,
        }
    }
}

impl<'a, 'b, T> widget::MutChildVisitor<'b, T> for StretchChildVisitor<'a, T>
where
    T: theme::Theme<'b>,
{
    type Output = stretch::node::Node;

    fn accept_child_mut<N>(&mut self, node: &mut N)
    where
        N: widget::Node<'b, T>,
    {
        let stretch = &mut self.stretch;
        let theme = self.theme;
        let node = node.visit_mut(StretchVisitor { stretch, theme });

        if self.index < self.old_child_count {
            if node
                != self
                    .stretch
                    .child_at_index(self.parent, self.index)
                    .unwrap()
            {
                self.stretch
                    .replace_child_at_index(self.parent, self.index, node)
                    .unwrap();
            }
        } else {
            self.stretch.add_child(self.parent, node).unwrap();
        }

        assert!(self.index < self.child_count);
        self.index += 1;
    }

    fn end(self) -> Self::Output {
        self.parent
    }
}
