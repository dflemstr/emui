use crate::theme;
pub(crate) type FontIter<'a, T> = <<T as theme::Theme<'a>>::Font as IntoIterator>::IntoIter;
pub(crate) type RectIter<C> =
    <embedded_graphics::primitives::Rectangle<C> as IntoIterator>::IntoIter;
pub(crate) type OptionalIter<I> = core::iter::Flatten<core::option::IntoIter<I>>;
