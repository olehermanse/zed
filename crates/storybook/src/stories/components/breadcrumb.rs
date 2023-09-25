use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use ui::Breadcrumb;

use crate::story::Story;

#[derive(Element, Default)]
pub struct BreadcrumbStory {}

impl BreadcrumbStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container()
            .child(Story::title_for::<_, Breadcrumb>())
            .child(Story::label("Default"))
            .child(Breadcrumb::new())
    }
}
