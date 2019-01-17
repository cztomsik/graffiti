use crate::resources::OpResource;
use std::cell::RefCell;
use std::rc::Rc;
use yoga::{Context, Direction, FlexStyle, MeasureMode, Node as YogaNode, NodeRef, Size};

pub struct Surface {
    pub yoga_node: YogaNode,
    pub brush: Option<Rc<OpResource>>,
    pub clip: Option<Rc<OpResource>>,
    pub children: Vec<Rc<RefCell<Surface>>>,
}

impl Surface {
    pub fn new() -> Self {
        Surface {
            yoga_node: YogaNode::new(),
            brush: Option::None,
            clip: Option::None,
            children: Vec::new(),
        }
    }

    pub fn append_child(&mut self, child: Rc<RefCell<Surface>>) {
        self.insert_at(child, self.children.len() as u32)
    }

    fn insert_at(&mut self, child: Rc<RefCell<Surface>>, index: u32) {
        debug!("insert_at {:?}", index);
        self.yoga_node
            .insert_child(&mut (child.borrow_mut().yoga_node), index);
        self.children.push(child)
    }

    pub fn insert_before(&mut self, child: Rc<RefCell<Surface>>, before: Rc<RefCell<Surface>>) {
        let index = self
            .children
            .iter()
            .position(|it| it.as_ptr() == before.as_ptr())
            .unwrap();

        self.insert_at(child, index as u32)
    }

    pub fn remove_child(&mut self, child: Rc<RefCell<Surface>>) {
        self.yoga_node
            .remove_child(&mut (child.borrow_mut().yoga_node));

        let index = self
            .children
            .iter()
            .position(|a| Rc::ptr_eq(a, &child))
            .unwrap();
        self.children.remove(index);
    }

    pub fn set_brush(&mut self, brush: Option<Rc<OpResource>>) {
        self.brush = brush;
    }

    pub fn set_clip(&mut self, clip: Option<Rc<OpResource>>) {
        self.clip = clip;
    }

    pub fn apply_flex_style(&mut self, style: Rc<Vec<FlexStyle>>) {
        style.iter().for_each(|s| self.yoga_node.apply_style(s));
    }

    pub fn set_measure(&mut self, measure: Box<Measure>) {
        self.yoga_node.set_context(Some(Context::new(measure)));
        self.yoga_node.set_measure_func(Some(call_measure));
        self.mark_dirty();
    }

    pub fn mark_dirty(&mut self) {
        self.yoga_node.mark_dirty();
    }

    pub fn calculate_layout(&mut self, available_width: f32, available_height: f32) {
        self.yoga_node
            .calculate_layout(available_width, available_height, Direction::LTR);
    }
}

pub trait Measure {
    fn measure(&self, w: f32, wm: MeasureMode, h: f32, hm: MeasureMode) -> Size;
}

extern "C" fn call_measure(
    node_ref: NodeRef,
    w: f32,
    wm: MeasureMode,
    h: f32,
    hm: MeasureMode,
) -> Size {
    info!("measure");

    let measure: &Box<Measure> = YogaNode::get_context(&node_ref)
        .unwrap()
        .downcast_ref::<Box<Measure>>()
        .unwrap();

    measure.measure(w, wm, h, hm)
}
