// generated

    
use crate::api::{ApiMsg,ApiResponse};
use crate::commons::{Pos,Bounds,Color};
use crate::viewport::{SceneChange,Event,EventKind};
use crate::style::{StyleChange,StyleProp};
use crate::box_layout::{Display,Dimension,Align,FlexWrap,FlexDirection};
use crate::text_layout::{Text,TextAlign};
use crate::render::{BorderRadius,BoxShadow};

    
interop! {
      
  ApiMsg { 
    0 GetEvents { poll },
    1 UpdateStyles { window, changes },
    2 UpdateScene { window, changes },
    3 GetBounds { window, surface },
    4 CreateWindow { title, width, height },
    5 ResizeWindow { window },
    6 DestroyWindow { window } 
  }
  ApiResponse { 
    0 Events { events },
    1 Nothing {  },
    2 Bounds { bounds } 
  }
  SceneChange { 
    0 Alloc {  },
    1 InsertAt { parent, child, index },
    2 RemoveChild { parent, child } 
  }
  StyleProp { 
    0 Display { value },
    1 Width { value },
    2 Height { value },
    3 MinWidth { value },
    4 MinHeight { value },
    5 MaxWidth { value },
    6 MaxHeight { value },
    7 Top { value },
    8 Right { value },
    9 Bottom { value },
    10 Left { value },
    11 MarginTop { value },
    12 MarginRight { value },
    13 MarginBottom { value },
    14 MarginLeft { value },
    15 PaddingTop { value },
    16 PaddingRight { value },
    17 PaddingBottom { value },
    18 PaddingLeft { value },
    19 FlexGrow { value },
    20 FlexShrink { value },
    21 FlexBasis { value },
    22 FlexDirection { value },
    23 FlexWrap { value },
    24 AlignSelf { value },
    25 AlignContent { value },
    26 AlignItems { value },
    27 JustifyContent { value },
    28 Color { value },
    29 BackgroundColor { value },
    30 BorderTopLeftRadius { value },
    31 BorderTopRightRadius { value },
    32 BorderBottomLeftRadius { value },
    33 BorderBottomRightRadius { value },
    34 BoxShadow { value },
    35 Text { value } 
  }
  Dimension { 
    0 Undefined {  },
    1 Auto {  },
    2 Px { value },
    3 Percent { value } 
  }

      
  Pos [x,y]
  Bounds [a,b]
  Color [r,g,b,a]
  Event [kind,target,key]
  StyleChange [surface,prop]
  Text [font_size,line_height,align,text]
  BorderRadius [top,right,bottom,left]
  BoxShadow [color,offset,blur,spread]

      
  EventKind(u8)
  Display(u8)
  Align(u8)
  FlexWrap(u8)
  FlexDirection(u8)
  TextAlign(u8)
    
}
    