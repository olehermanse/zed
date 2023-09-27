use std::marker::PhantomData;

use gpui2::geometry::{Length, Size};
use gpui2::{hsla, Hsla};

use crate::prelude::*;
use crate::theme;

#[derive(Default, PartialEq)]
pub enum SplitDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Element)]
pub struct Pane<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
    size: Size<Length>,
    fill: Hsla,
}

impl<V: 'static> Pane<V> {
    pub fn new(scroll_state: ScrollState, size: Size<Length>) -> Self {
        Self {
            view_type: PhantomData,
            scroll_state,
            size,
            fill: hsla(0., 1., 0.5, 1.),
        }
    }

    pub fn fill(mut self, fill: Hsla) -> Self {
        self.fill = fill;
        self
    }

    fn render(&mut self, view: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .flex()
            .flex_initial()
            .fill(self.fill)
            .w(self.size.width)
            .h(self.size.height)
            .overflow_y_scroll(self.scroll_state.clone())
    }
}

#[derive(Element)]
pub struct PaneGroup<V: 'static> {
    view_type: PhantomData<V>,
    groups: Vec<PaneGroup<V>>,
    panes: Vec<Pane<V>>,
    split_direction: SplitDirection,
}

impl<V: 'static> PaneGroup<V> {
    pub fn new_groups(groups: Vec<PaneGroup<V>>, split_direction: SplitDirection) -> Self {
        Self {
            view_type: PhantomData,
            groups,
            panes: Vec::new(),
            split_direction,
        }
    }

    pub fn new_panes(panes: Vec<Pane<V>>, split_direction: SplitDirection) -> Self {
        Self {
            view_type: PhantomData,
            groups: Vec::new(),
            panes,
            split_direction,
        }
    }

    fn render(&mut self, view: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        if !self.panes.is_empty() {
            let el = div()
                .flex()
                .flex_1()
                .gap_px()
                .w_full()
                .h_full()
                .fill(theme.lowest.accent.default.background)
                .children(self.panes.iter_mut().map(|pane| pane.render(view, cx)));

            if self.split_direction == SplitDirection::Horizontal {
                return el;
            } else {
                return el.flex_col();
            }
        }

        if !self.groups.is_empty() {
            let el = div()
                .flex()
                .flex_1()
                .gap_px()
                .w_full()
                .h_full()
                .fill(theme.lowest.accent.default.background)
                .children(self.groups.iter_mut().map(|group| group.render(view, cx)));

            if self.split_direction == SplitDirection::Horizontal {
                return el;
            } else {
                return el.flex_col();
            }
        }

        unreachable!()
    }
}