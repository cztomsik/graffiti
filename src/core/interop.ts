// generated

    

export enum EventKind { 
    MouseMove,
    MouseDown,
    MouseUp,
    Scroll,
    KeyDown,
    KeyPress,
    KeyUp,
    Focus,
    Blur,
    Resize,
    Close, 
}
export enum Display { 
    None,
    Flex,
    Block, 
}
export enum Align { 
    Auto,
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly, 
}
export enum FlexWrap { 
    NoWrap,
    Wrap,
    WrapReverse, 
}
export enum FlexDirection { 
    Column,
    ColumnReverse,
    Row,
    RowReverse, 
}
export enum TextAlign { 
    Left,
    Center,
    Right, 
}

    
export const Pos = (x,y) => [x,y]
export const Bounds = (a,b) => [a,b]
export const Color = (r,g,b,a) => [r,g,b,a]
export const Event = (kind,target,key) => [kind,target,key]
export const StyleChange = (surface,prop) => [surface,prop]
export const Text = (font_size,line_height,align,text) => [font_size,line_height,align,text]
export const BorderRadius = (top,right,bottom,left) => [top,right,bottom,left]
export const BoxShadow = (color,offset,blur,spread) => [color,offset,blur,spread]

    
export module ApiMsg {
        
    export const GetEvents = (poll) => [0, poll]
    export const UpdateStyles = (window,changes) => [1, window,changes]
    export const UpdateScene = (window,changes) => [2, window,changes]
    export const GetBounds = (window,surface) => [3, window,surface]
    export const CreateWindow = (title,width,height) => [4, title,width,height]
    export const ResizeWindow = (window) => [5, window]
    export const DestroyWindow = (window) => [6, window]
      
}
    
export module ApiResponse {
        
    export const Events = (events) => [0, events]
    export const Nothing = () => [1, ]
    export const Bounds = (bounds) => [2, bounds]
      
}
    
export module SceneChange {
        
    export const Alloc = () => [0, ]
    export const InsertAt = (parent,child,index) => [1, parent,child,index]
    export const RemoveChild = (parent,child) => [2, parent,child]
      
}
    
export module StyleProp {
        
    export const Display = (value) => [0, value]
    export const Width = (value) => [1, value]
    export const Height = (value) => [2, value]
    export const MinWidth = (value) => [3, value]
    export const MinHeight = (value) => [4, value]
    export const MaxWidth = (value) => [5, value]
    export const MaxHeight = (value) => [6, value]
    export const Top = (value) => [7, value]
    export const Right = (value) => [8, value]
    export const Bottom = (value) => [9, value]
    export const Left = (value) => [10, value]
    export const MarginTop = (value) => [11, value]
    export const MarginRight = (value) => [12, value]
    export const MarginBottom = (value) => [13, value]
    export const MarginLeft = (value) => [14, value]
    export const PaddingTop = (value) => [15, value]
    export const PaddingRight = (value) => [16, value]
    export const PaddingBottom = (value) => [17, value]
    export const PaddingLeft = (value) => [18, value]
    export const FlexGrow = (value) => [19, value]
    export const FlexShrink = (value) => [20, value]
    export const FlexBasis = (value) => [21, value]
    export const FlexDirection = (value) => [22, value]
    export const FlexWrap = (value) => [23, value]
    export const AlignSelf = (value) => [24, value]
    export const AlignContent = (value) => [25, value]
    export const AlignItems = (value) => [26, value]
    export const JustifyContent = (value) => [27, value]
    export const Color = (value) => [28, value]
    export const BackgroundColor = (value) => [29, value]
    export const BorderTopLeftRadius = (value) => [30, value]
    export const BorderTopRightRadius = (value) => [31, value]
    export const BorderBottomLeftRadius = (value) => [32, value]
    export const BorderBottomRightRadius = (value) => [33, value]
    export const BoxShadow = (value) => [34, value]
    export const Text = (value) => [35, value]
      
}
    
export module Dimension {
        
    export const Undefined = () => [0, ]
    export const Auto = () => [1, ]
    export const Px = (value) => [2, value]
    export const Percent = (value) => [3, value]
      
}
    
  