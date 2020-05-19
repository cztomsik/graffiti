// hand-written to keep deps (and compile-time) low

#![allow(non_upper_case_globals)]

use std::os::raw::{c_float, c_void};

pub enum YGNode {}
pub type YGNodeRef = *mut YGNode;

pub type YGMeasureFunc = Option<
    unsafe extern "C" fn(
        node: YGNodeRef,
        width: c_float,
        widthMode: YGMeasureMode,
        height: c_float,
        heightMode: YGMeasureMode,
    ) -> YGSize,
>;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum YGMeasureMode {
    Undefined = 0,
    Exactly = 1,
    AtMost = 2,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum YGDirection {
    Inherit = 0,
    LTR = 1,
    RTL = 2,
}

pub const YGUndefined: c_float = std::f32::NAN;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum YGEdge {
    Left = 0,
    Top = 1,
    Right = 2,
    Bottom = 3,
    Start = 4,
    End = 5,
    Horizontal = 6,
    Vertical = 7,
    All = 8,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum YGAlign {
    Auto = 0,
    FlexStart = 1,
    Center = 2,
    FlexEnd = 3,
    Stretch = 4,
    Baseline = 5,
    SpaceBetween = 6,
    SpaceAround = 7,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum YGFlexDirection {
    Column = 0,
    ColumnReverse = 1,
    Row = 2,
    RowReverse = 3,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum YGJustify {
    FlexStart = 0,
    Center = 1,
    FlexEnd = 2,
    SpaceBetween = 3,
    SpaceAround = 4,
    SpaceEvenly = 5,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum YGOverflow {
    Visible = 0,
    Hidden = 1,
    Scroll = 2,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum YGDisplay {
    Flex = 0,
    None = 1,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum YGPositionType {
    Relative = 0,
    Absolute = 1,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum YGWrap {
    NoWrap = 0,
    Wrap = 1,
    WrapReverse = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct YGSize {
    pub width: c_float,
    pub height: c_float,
}

extern "C" {
    pub fn YGNodeNew() -> YGNodeRef;
    pub fn YGNodeFree(node: YGNodeRef);
    pub fn YGNodeFreeRecursive(node: YGNodeRef);
    pub fn YGNodeReset(node: YGNodeRef);
    pub fn YGNodeInsertChild(node: YGNodeRef, child: YGNodeRef, index: u32);
    pub fn YGNodeRemoveChild(node: YGNodeRef, child: YGNodeRef);
    pub fn YGNodeRemoveAllChildren(node: YGNodeRef);

    pub fn YGNodeStyleSetWidth(node: YGNodeRef, width: c_float);
    pub fn YGNodeStyleSetWidthPercent(node: YGNodeRef, width: c_float);
    pub fn YGNodeStyleSetWidthAuto(node: YGNodeRef);
    pub fn YGNodeStyleSetHeight(node: YGNodeRef, height: c_float);
    pub fn YGNodeStyleSetHeightPercent(node: YGNodeRef, height: c_float);
    pub fn YGNodeStyleSetHeightAuto(node: YGNodeRef);
    pub fn YGNodeStyleSetMinWidth(node: YGNodeRef, minWidth: c_float);
    pub fn YGNodeStyleSetMinWidthPercent(node: YGNodeRef, minWidth: c_float);
    pub fn YGNodeStyleSetMinHeight(node: YGNodeRef, minHeight: c_float);
    pub fn YGNodeStyleSetMinHeightPercent(node: YGNodeRef, minHeight: c_float);
    pub fn YGNodeStyleSetMaxWidth(node: YGNodeRef, maxWidth: c_float);
    pub fn YGNodeStyleSetMaxWidthPercent(node: YGNodeRef, maxWidth: c_float);
    pub fn YGNodeStyleSetMaxHeight(node: YGNodeRef, maxHeight: c_float);
    pub fn YGNodeStyleSetMaxHeightPercent(node: YGNodeRef, maxHeight: c_float);

    pub fn YGNodeCalculateLayout(
        node: YGNodeRef,
        availableWidth: c_float,
        availableHeight: c_float,
        ownerDirection: YGDirection,
    );

    pub fn YGNodeMarkDirty(node: YGNodeRef);
    pub fn YGNodeMarkDirtyAndPropogateToDescendants(node: YGNodeRef);
    pub fn YGNodeSetContext(node: YGNodeRef, context: *mut c_void);
    pub fn YGNodeGetContext(node: YGNodeRef) -> *mut c_void;
    pub fn YGNodeSetMeasureFunc(node: YGNodeRef, measureFunc: YGMeasureFunc);

    pub fn YGNodeStyleSetFlexDirection(node: YGNodeRef, flexDirection: YGFlexDirection);
    pub fn YGNodeStyleSetFlexWrap(node: YGNodeRef, flexWrap: YGWrap);

    pub fn YGNodeStyleSetFlexGrow(node: YGNodeRef, flexGrow: c_float);
    pub fn YGNodeStyleSetFlexShrink(node: YGNodeRef, flexShrink: c_float);
    pub fn YGNodeStyleSetFlexBasis(node: YGNodeRef, flexBasis: c_float);
    pub fn YGNodeStyleSetFlexBasisPercent(node: YGNodeRef, flexBasis: c_float);
    pub fn YGNodeStyleSetFlexBasisAuto(node: YGNodeRef);

    pub fn YGNodeStyleSetJustifyContent(node: YGNodeRef, justifyContent: YGJustify);
    pub fn YGNodeStyleSetAlignContent(node: YGNodeRef, alignContent: YGAlign);
    pub fn YGNodeStyleSetAlignItems(node: YGNodeRef, alignItems: YGAlign);
    pub fn YGNodeStyleSetAlignSelf(node: YGNodeRef, alignSelf: YGAlign);

    pub fn YGNodeStyleSetOverflow(node: YGNodeRef, overflow: YGOverflow);
    pub fn YGNodeStyleSetDisplay(node: YGNodeRef, display: YGDisplay);

    pub fn YGNodeStyleSetPositionType(node: YGNodeRef, positionType: YGPositionType);
    pub fn YGNodeStyleSetPosition(node: YGNodeRef, edge: YGEdge, position: c_float);
    pub fn YGNodeStyleSetPositionPercent(node: YGNodeRef, edge: YGEdge, position: c_float);

    pub fn YGNodeStyleSetMargin(node: YGNodeRef, edge: YGEdge, margin: c_float);
    pub fn YGNodeStyleSetMarginPercent(node: YGNodeRef, edge: YGEdge, margin: c_float);
    pub fn YGNodeStyleSetMarginAuto(node: YGNodeRef, edge: YGEdge);

    pub fn YGNodeStyleSetPadding(node: YGNodeRef, edge: YGEdge, padding: c_float);
    pub fn YGNodeStyleSetPaddingPercent(node: YGNodeRef, edge: YGEdge, padding: c_float);

    pub fn YGNodeStyleSetBorder(node: YGNodeRef, edge: YGEdge, border: c_float);

    pub fn YGNodeLayoutGetLeft(node: YGNodeRef) -> c_float;
    pub fn YGNodeLayoutGetTop(node: YGNodeRef) -> c_float;
    pub fn YGNodeLayoutGetWidth(node: YGNodeRef) -> c_float;
    pub fn YGNodeLayoutGetHeight(node: YGNodeRef) -> c_float;
}
