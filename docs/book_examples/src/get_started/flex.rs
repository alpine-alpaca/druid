use druid::{
    widget::{Flex, Label},
    Widget,
};

// ANCHOR: flex
fn build_ui() -> impl Widget<()> {
    Flex::row()
        .with_flex_child(
            Flex::column()
                .with_flex_child(Label::new("top left"), 1.0)
                .with_flex_child(Label::new("bottom left"), 1.0),
            1.0,
        )
        .with_flex_child(
            Flex::column()
                .with_flex_child(Label::new("top right"), 1.0)
                .with_flex_child(Label::new("bottom right"), 1.0),
            1.0,
        )
}
// ANCHOR_END: flex
