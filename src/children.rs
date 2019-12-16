use crate::theme;
use crate::widget;

pub trait Children<'a, T>
where
    T: theme::Theme<'a>,
{
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn visit_children<V>(&self, visitor: V) -> V::Output
    where
        V: widget::ChildVisitor<'a, T>;

    fn visit_children_mut<V>(&mut self, visitor: V) -> V::Output
    where
        V: widget::MutChildVisitor<'a, T>;
}

macro_rules! tuple_nodes {
    ($($W:ident),*) => {
        impl<'a, T, $($W),*> Children<'a, T> for ($($W,)*)
        where
            T: theme::Theme<'a>,
            $($W: widget::Node<'a, T>,)*
        {
            fn len(&self) -> usize {
                #[allow(dead_code)]
                enum Widgets { $($W,)* __CountWidgetsLast }
                Widgets::__CountWidgetsLast as usize
            }

            #[allow(non_snake_case)]
            fn visit_children<V>(&self, mut visitor: V) -> V::Output where V: widget::ChildVisitor<'a, T> {
                match *self {
                    ($(ref $W,)*) => {
                        $(visitor.accept_child($W);)*
                        visitor.end()
                    }
                }
            }
            #[allow(non_snake_case)]
            fn visit_children_mut<V>(&mut self, mut visitor: V) -> V::Output where V: widget::MutChildVisitor<'a, T> {
                match *self {
                    ($(ref mut $W,)*) => {
                        $(visitor.accept_child_mut($W);)*
                        visitor.end()
                    }
                }
            }
        }
    }
}

tuple_nodes!(N1);
tuple_nodes!(N1, N2);
tuple_nodes!(N1, N2, N3);
tuple_nodes!(N1, N2, N3, N4);
tuple_nodes!(N1, N2, N3, N4, N5);
tuple_nodes!(N1, N2, N3, N4, N5, N6);
tuple_nodes!(N1, N2, N3, N4, N5, N6, N7);
tuple_nodes!(N1, N2, N3, N4, N5, N6, N7, N8);
tuple_nodes!(N1, N2, N3, N4, N5, N6, N7, N8, N9);
tuple_nodes!(N1, N2, N3, N4, N5, N6, N7, N8, N9, N10);
tuple_nodes!(N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11);
tuple_nodes!(N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12);
tuple_nodes!(N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13);
tuple_nodes!(N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14);
tuple_nodes!(N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15);
tuple_nodes!(N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16);
tuple_nodes!(N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17);
tuple_nodes!(N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18);
tuple_nodes!(N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19);
tuple_nodes!(
    N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20
);
tuple_nodes!(
    N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21
);
tuple_nodes!(
    N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21,
    N22
);
tuple_nodes!(
    N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21,
    N22, N23
);
tuple_nodes!(
    N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21,
    N22, N23, N24
);
tuple_nodes!(
    N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21,
    N22, N23, N24, N25
);
tuple_nodes!(
    N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21,
    N22, N23, N24, N25, N26
);
tuple_nodes!(
    N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21,
    N22, N23, N24, N25, N26, N27
);
tuple_nodes!(
    N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21,
    N22, N23, N24, N25, N26, N27, N28
);
tuple_nodes!(
    N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21,
    N22, N23, N24, N25, N26, N27, N28, N29
);
tuple_nodes!(
    N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21,
    N22, N23, N24, N25, N26, N27, N28, N29, N30
);
tuple_nodes!(
    N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21,
    N22, N23, N24, N25, N26, N27, N28, N29, N30, N31
);
tuple_nodes!(
    N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21,
    N22, N23, N24, N25, N26, N27, N28, N29, N30, N31, N32
);
