// generated

    
use crate::commons::{ElementChild,Pos,Bounds,Color};
use crate::app::{WindowEvent};
use crate::viewport::{SceneChange,Event};
use crate::box_layout::{Display,Overflow,Dimension,Align,FlexWrap,FlexDirection};
use crate::text_layout::{Text,TextAlign};
use crate::render::{Transform,BoxShadow};
use crate::interop::{AppMsg,AppResponse};

    
interop! {
      
  ElementChild { 
    0 Element { id },
    1 Text { id } 
  }
  SceneChange { 
    0 Realloc { elements_count, texts_count },
    1 InsertAt { parent, child, index },
    2 RemoveChild { parent, child },
    3 SetText { id, text },
    4 Display { element, value },
    5 Overflow { element, value },
    6 Width { element, value },
    7 Height { element, value },
    8 MinWidth { element, value },
    9 MinHeight { element, value },
    10 MaxWidth { element, value },
    11 MaxHeight { element, value },
    12 Top { element, value },
    13 Right { element, value },
    14 Bottom { element, value },
    15 Left { element, value },
    16 MarginTop { element, value },
    17 MarginRight { element, value },
    18 MarginBottom { element, value },
    19 MarginLeft { element, value },
    20 PaddingTop { element, value },
    21 PaddingRight { element, value },
    22 PaddingBottom { element, value },
    23 PaddingLeft { element, value },
    24 FlexGrow { element, value },
    25 FlexShrink { element, value },
    26 FlexBasis { element, value },
    27 FlexDirection { element, value },
    28 FlexWrap { element, value },
    29 AlignSelf { element, value },
    30 AlignContent { element, value },
    31 AlignItems { element, value },
    32 JustifyContent { element, value },
    33 Color { element, value },
    34 BackgroundColor { element, value },
    35 BorderTopLeftRadius { element, value },
    36 BorderTopRightRadius { element, value },
    37 BorderBottomLeftRadius { element, value },
    38 BorderBottomRightRadius { element, value },
    39 BoxShadow { element, value },
    40 Transform { element, value } 
  }
  Event { 
    0 MouseMove { target },
    1 MouseDown { target },
    2 MouseUp { target },
    3 Scroll { target },
    4 KeyDown { target, key },
    5 KeyPress { target, key },
    6 KeyUp { target, key },
    7 Resize { target },
    8 Close { target } 
  }
  Dimension { 
    0 Undefined {  },
    1 Auto {  },
    2 Px { value },
    3 Percent { value } 
  }
  Transform { 
    0 Scale { x, y } 
  }
  AppMsg { 
    0 GetEvents { poll },
    1 CreateWindow { title, width, height },
    2 ResizeWindow { window, width, height },
    3 UpdateScene { window, changes },
    4 GetOffsetBounds { window, element },
    5 DestroyWindow { window } 
  }
  AppResponse { 
    0 WindowId { id },
    1 Ack {  },
    2 Events { events },
    3 Bounds { bounds } 
  }

      
  Pos [x,y]
  Bounds [a,b]
  Color [r,g,b,a]
  WindowEvent [window,event]
  Text [font_size,line_height,align,text]
  BoxShadow [color,offset,blur,spread]

      
  Display(u8)
  Overflow(u8)
  Align(u8)
  FlexWrap(u8)
  FlexDirection(u8)
  TextAlign(u8)
    
}
    