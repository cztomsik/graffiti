use crate::resources::OpResource;
use std::cell::RefCell;
use std::rc::Rc;
use yoga::{Direction, FlexStyle, Node as YogaNode};

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
            .remove_child(&mut (child.borrow_mut().yoga_node))
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

    pub fn calculate_layout(&mut self, width: f32, height: f32) {
        self.yoga_node
            .calculate_layout(width, height, Direction::LTR);
    }
}
