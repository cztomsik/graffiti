// quick PoC of the new API
//
// STATUS:
// x types
// x safety
// x downcasting, auto-upcasting
// - implement all of the previous functionality
// - finish bindings refactor
//
// this took me a while to figure out so it's worth explaining a bit
// rust generally does not like mutable shared references nor OO things like inheritance
// id trees are efficient but you have to explicitly drop each node
// rc-tree is nicer but it has some overhead and it's not OO either
// and things like node.xxx_subdata() are problematic if you need to borrow() again without panic
// so the idea here is to have "store" of slotmaps and have generic NodeRef<>
// which carries node type with its signature, can deref to parent impls and
// also can be downcasted safely because it's just a number (and Rc<> to a store)
// NodeRefs are meant to be used in Rc<> so each node is dropped automatically
// but all the data are stored separately which means it should be much faster to go through
// the tree (and do queries and selector matching for example)
//
// BTW: we want Rc<NodeRef<T>> because we want to have one shared SlotMap<u32, Rc<dyn Any>>
//      for interop/bindings (JS and FFI) and that's also a reason why we want downcasting
//      otherwise the API looked awkward (the idea right now is to have api similar to GTK naming)

// TODO
#![allow(unused)]

use std::ops::Deref;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};
use std::fmt::{Debug, Formatter, Error};
use std::cell::{Cell, RefCell, Ref};
use std::any::TypeId;
use crate::util::{Atom, SlotMap};

#[allow(unused)]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    Element = 1,
    Attribute = 2,
    Text = 3,
    CdataSection = 4,
    EntityReference = 5,
    Entity = 6,
    ProcessingInstruction = 7,
    Comment = 8,
    Document = 9,
    DocumentType = 10,
    DocumentFragment = 11,
    Notation = 12,
}

type NodeId = u32;

pub struct Node;
pub struct ParentNode;
pub struct Document;
pub struct Element;
pub struct CharacterData;
pub struct Text;
pub struct Comment;

macro_rules! impl_deref {
    ($A: ident, $B: ident) => {
        impl Deref for NodeRef<$A> {
            type Target = NodeRef<$B>;

            fn deref(&self) -> &Self::Target {
                self.cast::<$B>()
            }
        }
    };
}

impl_deref!(ParentNode, Node);
impl_deref!(Document, ParentNode);
impl_deref!(Element, ParentNode);
impl_deref!(CharacterData, Node);
impl_deref!(Text, CharacterData);
impl_deref!(Comment, CharacterData);

#[repr(transparent)]
pub struct NodeRef<T>((Rc<RefCell<Store>>, NodeId), PhantomData<T>);

impl <T> Debug for NodeRef<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> { fmt.debug_tuple("NodeRef").field(&self.0.1).finish() }
}

impl <T> PartialEq for NodeRef<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::as_ptr(&self.0 .0) == Rc::as_ptr(&other.0 .0) && self.0.1 == other.0.1
    }
}

impl <T: 'static> NodeRef<T> {
    // private "transmute", should be safe because of repr(transparent)
    fn cast<'a, X>(&'a self) -> &'a NodeRef<X> { unsafe { &*(self as *const Self as *const NodeRef<X>) } }
}

impl NodeRef<Node> {
    fn store(&self) -> Ref<Store> { self.0.0.borrow() }
    fn id(&self) -> NodeId { self.0.1 }

    fn downcast<T: 'static>(&self) -> Option<&NodeRef<T>> {
        /*
        const NODE: TypeId = TypeId::of::<Node>();
        const PARENT: TypeId = TypeId::of::<ParentNode>();
        const DOC: TypeId = TypeId::of::<Document>();
        const EL: TypeId = TypeId::of::<Element>();
        const CDATA: TypeId = TypeId::of::<CharacterData>();
        const TXT: TypeId = TypeId::of::<Text>();
        const CMT: TypeId = TypeId::of::<Comment>();

        {
        use NodeType::*;        

        match (TypeId::of::<T>(), self.node_type()) {
            (NODE, _) | (PARENT, Element | Document) | (DOC, Document) | (EL, Element) | (CDATA, Text | Comment) | (TXT, Text) | (CMT, Comment) => Some(self.cast()),
            _ => None
        }
        }
        */
        todo!()
    }

    pub fn node_type(&self) -> NodeType { self.store().nodes[self.id()].node_type }
    pub fn parent_node(&self) -> Option<&NodeRef<ParentNode>> { todo!() /*self.store().nodes[self.id()].parent_node*/ }
    pub fn parent_element(&self) -> Option<&NodeRef<Element>> { match self.parent_node() { Some(p) if self.node_type() == NodeType::Element => Some(p.cast()), _ => None } }
}

impl NodeRef<ParentNode> {
    pub fn first_child(&self) -> Option<&NodeRef<Node>> { todo!() /*self.store().nodes[self.id()].first_child*/ }
    pub fn next_sibling(&self) -> Option<&NodeRef<Node>> { todo!() /*self.store().nodes[self.id()].next_sibling*/ }

    pub fn insert_child(&self, child: &NodeRef<Node>, index: u32) { todo!() }
    pub fn remove_child(&self, child: &NodeRef<Node>) { /*assert_eq!()*/ }

    //pub fn query_selector(&self, selector: &str) -> Option<&NodeRef<Node>> { todo!() }
    //pub fn query_selector_all(&self, selector: &str) -> Vec<&NodeRef<Node>> { todo!() }
}

impl Document {
    pub fn new() -> NodeRef<Document> {
        let store = Store::default();
        
        create_node(&Rc::new(RefCell::new(store)), NodeType::Document)
    }
}

impl NodeRef<Document> {
    pub fn create_element(&self, local_name: &str) -> NodeRef<Element> { todo!() }
    pub fn create_text_node(&self, data: &str) -> NodeRef<Text> { todo!() }
    pub fn create_comment(&self, data: &str) -> NodeRef<Comment> { todo!() }
}

impl NodeRef<Element> {
    pub fn local_name(&self) -> String { todo!() }
    pub fn attribute_names(&self) -> Vec<String> { todo!() }
    pub fn attribute(&self, att: &str) -> String { todo!() }
    pub fn set_attribute(&self, att: &str, val: &str) { todo!() }
    pub fn remove_attribute(&self, att: &str) { todo!() }
    pub fn matches(&self, selector: &str) -> bool { todo!() }
    //pub fn style() -> Rc<CssStyleDeclaration> { todo!() }
}

impl NodeRef<CharacterData> {
    pub fn data(&self) -> String { todo!() }
    pub fn set_data(&self, data: &str) { todo!() }
}


#[derive(Default)]
struct Store {
    nodes: SlotMap<NodeId, NodeData>,
    refs: SlotMap<NodeId, Rc<NodeRef<Node>>>,
}

struct NodeData {
    node_type: NodeType,
    parent_node: Option<NodeId>,
    next_sibling: Option<NodeId>,
    first_child: Option<NodeId>
}

fn create_node<T>(store: &Rc<RefCell<Store>>, node_type: NodeType) -> NodeRef<T> {
    let id = store.borrow_mut().nodes.insert(NodeData {
        node_type,
        parent_node: None,
        next_sibling: None,
        first_child: None
    });
    
    NodeRef((store.clone(), id), PhantomData)
}


mod tests {
    use super::*;

    #[test]
    fn test() {
        let doc = Document::new();
        assert_eq!(doc.node_type(), NodeType::Document);
        assert_eq!(doc.first_child(), None);
        
        let div = doc.create_element("div");
        assert_eq!(div.node_type(), NodeType::Element);
        assert_eq!(div.local_name(), "div");
        //assert_eq!(doc.first_child(), None);

        let hello = doc.create_text_node("hello");
        assert_eq!(hello.node_type(), NodeType::Text);
        assert_eq!(hello.data(), "hello");
        hello.set_data("hello world");

        let comment = doc.create_comment("test");
        assert_eq!(comment.node_type(), NodeType::Comment);
        assert_eq!(comment.data(), "test");
        comment.set_data("test2");

        div.insert_child(&hello, 0);
        div.insert_child(&comment, 0);

        div.remove_child(&comment);
    }
}
