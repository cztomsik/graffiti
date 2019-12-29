// generated

    
use crate::api::{ApiMsg,ApiResponse};
use crate::commons::{Pos,Bounds,Color,BoxShadow,Border,BorderSide,BorderRadius,BorderStyle,Image};
use crate::viewport::{SceneChange,Event,EventKind};
use crate::box_layout::{DimensionProp,Dimension,AlignProp,Align,FlexWrap,FlexDirection};
use crate::text_layout::{Text,TextAlign};

    
interop! {
      
  ApiMsg { 
    0 GetEvents { poll },
    1 UpdateScene { window, changes },
    2 GetBounds { window, surface },
    3 CreateWindow { title, width, height },
    4 ResizeWindow { window },
    5 DestroyWindow { window } 
  }
  ApiResponse { 
    0 Events { events },
    1 Nothing {  },
    2 Bounds { bounds } 
  }
  SceneChange { 
    0 Alloc {  },
    1 InsertAt { parent, child, index },
    2 RemoveChild { parent, child },
    3 Dimension { surface, prop, value },
    4 Align { surface, prop, value },
    5 FlexWrap { surface, value },
    6 FlexDirection { surface, value },
    7 BackgroundColor { surface, value },
    8 Border { surface, value },
    9 BoxShadow { surface, value },
    10 TextColor { surface, value },
    11 BorderRadius { surface, value },
    12 Image { surface, value },
    13 Text { surface, text } 
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
  BoxShadow [color,offset,blur,spread]
  Border [top,right,bottom,left]
  BorderSide [width,style,color]
  BorderRadius [top,right,bottom,left]
  Image [url]
  Event [kind,target,key]
  Text [font_size,line_height,align,text]

      
  BorderStyle(u8)
  EventKind(u8)
  DimensionProp(u8)
  AlignProp(u8)
  Align(u8)
  FlexWrap(u8)
  FlexDirection(u8)
  TextAlign(u8)
    
}
    