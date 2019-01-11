use webrender::api::{
    BorderDisplayItem, GlyphInstance, PushStackingContextDisplayItem, RectangleDisplayItem,
    TextDisplayItem,
};

// operation to be performed during render, like builder.push_rect(), or api.hit_test()
// a bit like cpu opcode, it can have some parameters but it also can be relative to current
// state (registers) and/or provided layout rects (memory)
// we are really doing some kind of very simple virtual machine
#[derive(Deserialize, Debug, Clone)]
pub enum RenderOperation {
    // this was hack at first but it could be useful for hitSlop (hitBox can be bigger than clipBox)
    HitTest(u32),
    SaveRect,
    PushScrollClip(u64),
    PushBorderRadiusClip(f32),
    PopClip,
    Rectangle(RectangleDisplayItem),
    Border(BorderDisplayItem),
    Text(TextDisplayItem, Vec<GlyphInstance>),
    PopStackingContext,
    PushStackingContext(PushStackingContextDisplayItem),
}

// TODO: move some portion of Window.render here
