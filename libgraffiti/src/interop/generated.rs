// generated

    
use crate::api::{ApiMsg};
use crate::commons::{Pos,Color,BoxShadow,Border,BorderSide,BorderRadius,BorderStyle,Image};
use crate::window::{SceneChange,Event,EventKind};
use crate::box_layout::{DimensionProp,Dimension,AlignProp,Align,FlexWrap,FlexDirection};
use crate::text_layout::{Text,TextAlign};

    
interop! {
      
  ApiMsg { 
    CreateWindow { width, height },
    GetEvents { poll },
    UpdateScene { window, changes } 
  }
  SceneChange { 
    Alloc {  },
    InsertAt { parent, child, index },
    RemoveChild { parent, child },
    Dimension { surface, prop, value },
    Align { surface, prop, value },
    FlexWrap { surface, value },
    FlexDirection { surface, value },
    BackgroundColor { surface, value },
    Border { surface, value },
    BoxShadow { surface, value },
    TextColor { surface, value },
    BorderRadius { surface, value },
    Image { surface, value },
    Text { surface, text } 
  }
  Dimension { 
    Undefined {  },
    Auto {  },
    Points { value },
    Percent { value } 
  }

      
  Pos [x,y]
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
    