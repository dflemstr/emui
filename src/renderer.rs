use crate::layout;
use crate::theme;
use crate::widget;

pub struct Renderer<'a, T, D> {
    theme: &'a T,
    layout: &'a layout::Layout,
    drawing: &'a mut D,
}

impl<'a, T, D> Renderer<'a, T, D>
where
    T: theme::Theme<'a>,
    D: embedded_graphics::Drawing<T::Color>,
{
    pub fn new(theme: &'a T, layout: &'a layout::Layout, drawing: &'a mut D) -> Self {
        Self {
            theme,
            layout,
            drawing,
        }
    }

    pub fn render_tree<W>(self, widget: &W)
    where
        W: widget::Node<'a, T>,
    {
        let renderer = self;
        let offset = layout::Point { x: 0.0, y: 0.0 };
        widget.visit(RendererVisitor { offset, renderer });
    }

    fn render<N>(&mut self, offset: layout::Point, node: &N) -> layout::Point
    where
        N: widget::Widget<'a, T>,
    {
        let geometry = self.layout.get_geometry(node.layout_state()).unwrap();
        let theme = self.theme;
        let position = embedded_graphics::geometry::Point::new(
            (offset.x + geometry.position.x) as i32,
            (offset.y + geometry.position.y) as i32,
        );
        let size = embedded_graphics::geometry::Size::new(
            geometry.size.width.max(0.0) as u32,
            geometry.size.height.max(0.0) as u32,
        );
        let draw_context = widget::DrawContext {
            theme,
            position,
            size,
        };
        self.drawing.draw(node.draw(&draw_context));
        geometry.position
    }
}

struct RendererVisitor<'a, T, D>
where
    T: theme::Theme<'a>,
    D: embedded_graphics::Drawing<T::Color>,
{
    offset: layout::Point,
    renderer: Renderer<'a, T, D>,
}

struct RendererChildVisitor<'a, T, D>
where
    T: theme::Theme<'a>,
    D: embedded_graphics::Drawing<T::Color>,
{
    offset: layout::Point,
    renderer: Option<Renderer<'a, T, D>>,
}

impl<'a, T, D> widget::Visitor<'a, T> for RendererVisitor<'a, T, D>
where
    T: theme::Theme<'a>,
    D: embedded_graphics::Drawing<T::Color>,
{
    type Output = Renderer<'a, T, D>;
    type ChildVisitor = RendererChildVisitor<'a, T, D>;

    fn accept_leaf<N>(mut self, node: &N) -> Self::Output
    where
        N: widget::Widget<'a, T>,
    {
        self.renderer.render(self.offset, node);
        self.renderer
    }

    fn accept_node<N>(mut self, node: &N, _child_count: usize) -> Self::ChildVisitor
    where
        N: widget::Widget<'a, T>,
    {
        let offset = self.renderer.render(self.offset, node);

        let renderer = Some(self.renderer);
        RendererChildVisitor { offset, renderer }
    }
}

impl<'a, T, D> widget::ChildVisitor<'a, T> for RendererChildVisitor<'a, T, D>
where
    T: theme::Theme<'a>,
    D: embedded_graphics::Drawing<T::Color>,
{
    type Output = Renderer<'a, T, D>;

    fn accept_child<W>(&mut self, widget: &W)
    where
        W: widget::Node<'a, T>,
    {
        let offset = self.offset;
        let renderer = self.renderer.take().unwrap();
        self.renderer = Some(widget.visit(RendererVisitor { offset, renderer }));
    }

    fn end(mut self) -> Self::Output {
        self.renderer.take().unwrap()
    }
}
