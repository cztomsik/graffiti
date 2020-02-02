// generated

    

export enum Display { 
    None,
    Block,
    Flex, 
}
export enum Overflow { 
    Visible,
    Hidden,
    Scroll, 
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
export const WindowEvent = (window,event) => [window,event]
export const Text = (font_size,line_height,align,text) => [font_size,line_height,align,text]
export const BoxShadow = (color,offset,blur,spread) => [color,offset,blur,spread]

    
export module ElementChild {
        
    export const Element = (id) => [0, id]
    export const Text = (id) => [1, id]

        
    export const TAGS = {
      Element: 0,
      Text: 1,
}
      }
    
export module SceneChange {
        
    export const Realloc = (elements_count,texts_count) => [0, elements_count,texts_count]
    export const InsertAt = (parent,child,index) => [1, parent,child,index]
    export const RemoveChild = (parent,child) => [2, parent,child]
    export const SetText = (id,text) => [3, id,text]
    export const Display = (element,value) => [4, element,value]
    export const Overflow = (element,value) => [5, element,value]
    export const Width = (element,value) => [6, element,value]
    export const Height = (element,value) => [7, element,value]
    export const MinWidth = (element,value) => [8, element,value]
    export const MinHeight = (element,value) => [9, element,value]
    export const MaxWidth = (element,value) => [10, element,value]
    export const MaxHeight = (element,value) => [11, element,value]
    export const Top = (element,value) => [12, element,value]
    export const Right = (element,value) => [13, element,value]
    export const Bottom = (element,value) => [14, element,value]
    export const Left = (element,value) => [15, element,value]
    export const MarginTop = (element,value) => [16, element,value]
    export const MarginRight = (element,value) => [17, element,value]
    export const MarginBottom = (element,value) => [18, element,value]
    export const MarginLeft = (element,value) => [19, element,value]
    export const PaddingTop = (element,value) => [20, element,value]
    export const PaddingRight = (element,value) => [21, element,value]
    export const PaddingBottom = (element,value) => [22, element,value]
    export const PaddingLeft = (element,value) => [23, element,value]
    export const FlexGrow = (element,value) => [24, element,value]
    export const FlexShrink = (element,value) => [25, element,value]
    export const FlexBasis = (element,value) => [26, element,value]
    export const FlexDirection = (element,value) => [27, element,value]
    export const FlexWrap = (element,value) => [28, element,value]
    export const AlignSelf = (element,value) => [29, element,value]
    export const AlignContent = (element,value) => [30, element,value]
    export const AlignItems = (element,value) => [31, element,value]
    export const JustifyContent = (element,value) => [32, element,value]
    export const Color = (element,value) => [33, element,value]
    export const BackgroundColor = (element,value) => [34, element,value]
    export const BorderTopLeftRadius = (element,value) => [35, element,value]
    export const BorderTopRightRadius = (element,value) => [36, element,value]
    export const BorderBottomLeftRadius = (element,value) => [37, element,value]
    export const BorderBottomRightRadius = (element,value) => [38, element,value]
    export const BoxShadow = (element,value) => [39, element,value]
    export const Transform = (element,value) => [40, element,value]

        
    export const TAGS = {
      Realloc: 0,
      InsertAt: 1,
      RemoveChild: 2,
      SetText: 3,
      Display: 4,
      Overflow: 5,
      Width: 6,
      Height: 7,
      MinWidth: 8,
      MinHeight: 9,
      MaxWidth: 10,
      MaxHeight: 11,
      Top: 12,
      Right: 13,
      Bottom: 14,
      Left: 15,
      MarginTop: 16,
      MarginRight: 17,
      MarginBottom: 18,
      MarginLeft: 19,
      PaddingTop: 20,
      PaddingRight: 21,
      PaddingBottom: 22,
      PaddingLeft: 23,
      FlexGrow: 24,
      FlexShrink: 25,
      FlexBasis: 26,
      FlexDirection: 27,
      FlexWrap: 28,
      AlignSelf: 29,
      AlignContent: 30,
      AlignItems: 31,
      JustifyContent: 32,
      Color: 33,
      BackgroundColor: 34,
      BorderTopLeftRadius: 35,
      BorderTopRightRadius: 36,
      BorderBottomLeftRadius: 37,
      BorderBottomRightRadius: 38,
      BoxShadow: 39,
      Transform: 40,
}
      }
    
export module Event {
        
    export const MouseMove = (target) => [0, target]
    export const MouseDown = (target) => [1, target]
    export const MouseUp = (target) => [2, target]
    export const Scroll = (target) => [3, target]
    export const KeyDown = (target,key) => [4, target,key]
    export const KeyPress = (target,key) => [5, target,key]
    export const KeyUp = (target,key) => [6, target,key]
    export const Resize = (target) => [7, target]
    export const Close = (target) => [8, target]

        
    export const TAGS = {
      MouseMove: 0,
      MouseDown: 1,
      MouseUp: 2,
      Scroll: 3,
      KeyDown: 4,
      KeyPress: 5,
      KeyUp: 6,
      Resize: 7,
      Close: 8,
}
      }
    
export module Dimension {
        
    export const Undefined = () => [0, ]
    export const Auto = () => [1, ]
    export const Px = (value) => [2, value]
    export const Percent = (value) => [3, value]

        
    export const TAGS = {
      Undefined: 0,
      Auto: 1,
      Px: 2,
      Percent: 3,
}
      }
    
export module Transform {
        
    export const Scale = (x,y) => [0, x,y]

        
    export const TAGS = {
      Scale: 0,
}
      }
    
export module AppMsg {
        
    export const GetEvents = (poll) => [0, poll]
    export const CreateWindow = (title,width,height) => [1, title,width,height]
    export const ResizeWindow = (window,width,height) => [2, window,width,height]
    export const UpdateScene = (window,changes) => [3, window,changes]
    export const GetOffsetBounds = (window,element) => [4, window,element]
    export const DestroyWindow = (window) => [5, window]

        
    export const TAGS = {
      GetEvents: 0,
      CreateWindow: 1,
      ResizeWindow: 2,
      UpdateScene: 3,
      GetOffsetBounds: 4,
      DestroyWindow: 5,
}
      }
    
export module AppResponse {
        
    export const WindowId = (id) => [0, id]
    export const Ack = () => [1, ]
    export const Events = (events) => [2, events]
    export const Bounds = (bounds) => [3, bounds]

        
    export const TAGS = {
      WindowId: 0,
      Ack: 1,
      Events: 2,
      Bounds: 3,
}
      }
    
  