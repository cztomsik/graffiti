import {
  FfiMsg,
  WindowId,
  UpdateSceneMsg,
  FfiMsg_UpdateScene,
  FfiResult,
  Event,
  WindowEvent,
  Event_WindowEvent,
  WindowEvent_MouseMove,
  WindowEvent_MouseDown,
  WindowEvent_MouseUp,
  WindowEvent_Scroll,
  SurfaceId,
  UpdateSceneMsg_InsertAt,
  UpdateSceneMsg_RemoveChild,
  StyleProp,
  UpdateSceneMsg_SetStyleProp,
  Size,
  Flex,
  Flow,
  Dimensions,
  BorderRadius,
  Border,
  BoxShadow,
  Color,
  Image,
  Text,
  Overflow,
  FlexDirection,
  FlexWrap,
  FlexAlign,
  JustifyContent,
  Dimension,
  Rect,
  Vector2f,
  TextAlign,
  BorderSide,
  BorderStyle
} from './generated'

import {
  read_u32,
  read_bool,
  read_seq,
  read_str,
  read_u64,
  read_u16,
  read_opt,
  read_u8,
  read_f32,
  Sink,
  Deserializer
} from 'ts-binary'

const readVecUpdateSceneMsg = (sink: Sink): Array<UpdateSceneMsg> => read_seq(sink, readUpdateSceneMsg)

const readVecEvent = (sink: Sink): Array<Event> => read_seq(sink, readEvent)

const readOptBorderRadius = (sink: Sink): (BorderRadius) | undefined => read_opt(sink, readBorderRadius)

const readOptBorder = (sink: Sink): (Border) | undefined => read_opt(sink, readBorder)

const readOptBoxShadow = (sink: Sink): (BoxShadow) | undefined => read_opt(sink, readBoxShadow)

const readOptColor = (sink: Sink): (Color) | undefined => read_opt(sink, readColor)

const readOptImage = (sink: Sink): (Image) | undefined => read_opt(sink, readImage)

const readOptText = (sink: Sink): (Text) | undefined => read_opt(sink, readText)

export const readFfiMsg = (sink: Sink): FfiMsg => {
  switch (read_u32(sink)) {
    case 0:
      return FfiMsg.GetEvents(read_bool(sink))
    case 1:
      return FfiMsg.CreateWindow
    case 2:
      return FfiMsg.UpdateScene(readFfiMsg_UpdateScene(sink))
  }
  throw new Error('bad variant index for FfiMsg')
}

const readFfiMsg_UpdateScene = (sink: Sink): FfiMsg_UpdateScene => {
  const window = readWindowId(sink)
  const msgs = readVecUpdateSceneMsg(sink)
  return { window, msgs }
}

export const readFfiResult = (sink: Sink): FfiResult => {
  switch (read_u32(sink)) {
    case 0:
      return FfiResult.Nothing
    case 1:
      return FfiResult.Error(read_str(sink))
    case 2:
      return FfiResult.Events(readVecEvent(sink))
    case 3:
      return FfiResult.WindowId(readWindowId(sink))
  }
  throw new Error('bad variant index for FfiResult')
}

export const readEvent = (sink: Sink): Event => {
  switch (read_u32(sink)) {
    case 0:
      return Event.WindowEvent(readEvent_WindowEvent(sink))
  }
  throw new Error('bad variant index for Event')
}

const readEvent_WindowEvent = (sink: Sink): Event_WindowEvent => {
  const window = readWindowId(sink)
  const event = readWindowEvent(sink)
  return { window, event }
}

export const readWindowEvent = (sink: Sink): WindowEvent => {
  switch (read_u32(sink)) {
    case 0:
      return WindowEvent.MouseMove(readWindowEvent_MouseMove(sink))
    case 1:
      return WindowEvent.MouseDown(readWindowEvent_MouseDown(sink))
    case 2:
      return WindowEvent.MouseUp(readWindowEvent_MouseUp(sink))
    case 3:
      return WindowEvent.Scroll(readWindowEvent_Scroll(sink))
    case 4:
      return WindowEvent.KeyDown(read_u16(sink))
    case 5:
      return WindowEvent.KeyPress(read_u16(sink))
    case 6:
      return WindowEvent.KeyUp(read_u16(sink))
    case 7:
      return WindowEvent.Focus
    case 8:
      return WindowEvent.Blur
    case 9:
      return WindowEvent.Resize
    case 10:
      return WindowEvent.Close
    case 11:
      return WindowEvent.Unknown
  }
  throw new Error('bad variant index for WindowEvent')
}

const readWindowEvent_MouseMove = (sink: Sink): WindowEvent_MouseMove => {
  const target = read_u64(sink)
  return { target }
}

const readWindowEvent_MouseDown = (sink: Sink): WindowEvent_MouseDown => {
  const target = read_u64(sink)
  return { target }
}

const readWindowEvent_MouseUp = (sink: Sink): WindowEvent_MouseUp => {
  const target = read_u64(sink)
  return { target }
}

const readWindowEvent_Scroll = (sink: Sink): WindowEvent_Scroll => {
  const target = read_u64(sink)
  return { target }
}

export const readUpdateSceneMsg = (sink: Sink): UpdateSceneMsg => {
  switch (read_u32(sink)) {
    case 0:
      return UpdateSceneMsg.Alloc
    case 1:
      return UpdateSceneMsg.InsertAt(readUpdateSceneMsg_InsertAt(sink))
    case 2:
      return UpdateSceneMsg.RemoveChild(readUpdateSceneMsg_RemoveChild(sink))
    case 3:
      return UpdateSceneMsg.SetStyleProp(readUpdateSceneMsg_SetStyleProp(sink))
  }
  throw new Error('bad variant index for UpdateSceneMsg')
}

const readUpdateSceneMsg_InsertAt = (sink: Sink): UpdateSceneMsg_InsertAt => {
  const parent = readSurfaceId(sink)
  const child = readSurfaceId(sink)
  const index = read_u64(sink)
  return { parent, child, index }
}

const readUpdateSceneMsg_RemoveChild = (sink: Sink): UpdateSceneMsg_RemoveChild => {
  const parent = readSurfaceId(sink)
  const child = readSurfaceId(sink)
  return { parent, child }
}

const readUpdateSceneMsg_SetStyleProp = (sink: Sink): UpdateSceneMsg_SetStyleProp => {
  const surface = readSurfaceId(sink)
  const prop = readStyleProp(sink)
  return { surface, prop }
}

export const readStyleProp = (sink: Sink): StyleProp => {
  switch (read_u32(sink)) {
    case 0:
      return StyleProp.Size(readSize(sink))
    case 1:
      return StyleProp.Flex(readFlex(sink))
    case 2:
      return StyleProp.Flow(readFlow(sink))
    case 3:
      return StyleProp.Padding(readDimensions(sink))
    case 4:
      return StyleProp.Margin(readDimensions(sink))
    case 5:
      return StyleProp.BorderRadius(readOptBorderRadius(sink))
    case 6:
      return StyleProp.Border(readOptBorder(sink))
    case 7:
      return StyleProp.BoxShadow(readOptBoxShadow(sink))
    case 8:
      return StyleProp.BackgroundColor(readOptColor(sink))
    case 9:
      return StyleProp.Image(readOptImage(sink))
    case 10:
      return StyleProp.Text(readOptText(sink))
    case 11:
      return StyleProp.Overflow(readOverflow(sink))
  }
  throw new Error('bad variant index for StyleProp')
}

export const readWindowId: Deserializer<WindowId> = read_u16

export const readSurfaceId: Deserializer<SurfaceId> = read_u64

export const readColor = (sink: Sink): Color => Color(read_u8(sink), read_u8(sink), read_u8(sink), read_u8(sink))

const FlexDirectionReverseMap: FlexDirection[] = [
  FlexDirection.Column,
  FlexDirection.ColumnReverse,
  FlexDirection.Row,
  FlexDirection.RowReverse
]

export const readFlexDirection = (sink: Sink): FlexDirection => FlexDirectionReverseMap[read_u32(sink)]

const FlexWrapReverseMap: FlexWrap[] = [FlexWrap.NoWrap, FlexWrap.Wrap, FlexWrap.WrapReverse]

export const readFlexWrap = (sink: Sink): FlexWrap => FlexWrapReverseMap[read_u32(sink)]

const FlexAlignReverseMap: FlexAlign[] = [
  FlexAlign.Auto,
  FlexAlign.FlexStart,
  FlexAlign.Center,
  FlexAlign.FlexEnd,
  FlexAlign.Stretch,
  FlexAlign.Baseline,
  FlexAlign.SpaceBetween,
  FlexAlign.SpaceAround
]

export const readFlexAlign = (sink: Sink): FlexAlign => FlexAlignReverseMap[read_u32(sink)]

const JustifyContentReverseMap: JustifyContent[] = [
  JustifyContent.FlexStart,
  JustifyContent.Center,
  JustifyContent.FlexEnd,
  JustifyContent.SpaceBetween,
  JustifyContent.SpaceAround,
  JustifyContent.SpaceEvenly
]

export const readJustifyContent = (sink: Sink): JustifyContent => JustifyContentReverseMap[read_u32(sink)]

export const readFlow = (sink: Sink): Flow => {
  const flexDirection = readFlexDirection(sink)
  const flexWrap = readFlexWrap(sink)
  const alignContent = readFlexAlign(sink)
  const alignItems = readFlexAlign(sink)
  const alignSelf = readFlexAlign(sink)
  const justifyContent = readJustifyContent(sink)
  return { flexDirection, flexWrap, alignContent, alignItems, alignSelf, justifyContent }
}

export const readFlex = (sink: Sink): Flex => {
  const flexGrow = read_f32(sink)
  const flexShrink = read_f32(sink)
  const flexBasis = readDimension(sink)
  return { flexGrow, flexShrink, flexBasis }
}

export const readDimension = (sink: Sink): Dimension => {
  switch (read_u32(sink)) {
    case 0:
      return Dimension.Auto
    case 1:
      return Dimension.Point(read_f32(sink))
    case 2:
      return Dimension.Percent(read_f32(sink))
  }
  throw new Error('bad variant index for Dimension')
}

const OverflowReverseMap: Overflow[] = [Overflow.Visible, Overflow.Hidden, Overflow.Scroll]

export const readOverflow = (sink: Sink): Overflow => OverflowReverseMap[read_u32(sink)]

export const readSize = (sink: Sink): Size => Size(readDimension(sink), readDimension(sink))

export const readRect = (sink: Sink): Rect => Rect(read_f32(sink), read_f32(sink), read_f32(sink), read_f32(sink))

export const readDimensions = (sink: Sink): Dimensions =>
  Dimensions(readDimension(sink), readDimension(sink), readDimension(sink), readDimension(sink))

export const readVector2f = (sink: Sink): Vector2f => Vector2f(read_f32(sink), read_f32(sink))

export const readBorderRadius = (sink: Sink): BorderRadius =>
  BorderRadius(read_f32(sink), read_f32(sink), read_f32(sink), read_f32(sink))

export const readBoxShadow = (sink: Sink): BoxShadow => {
  const color = readColor(sink)
  const offset = readVector2f(sink)
  const blur = read_f32(sink)
  const spread = read_f32(sink)
  return { color, offset, blur, spread }
}

export const readImage = (sink: Sink): Image => {
  const url = read_str(sink)
  return { url }
}

const TextAlignReverseMap: TextAlign[] = [TextAlign.Left, TextAlign.Center, TextAlign.Right]

export const readTextAlign = (sink: Sink): TextAlign => TextAlignReverseMap[read_u32(sink)]

export const readText = (sink: Sink): Text => {
  const color = readColor(sink)
  const fontSize = read_f32(sink)
  const lineHeight = read_f32(sink)
  const align = readTextAlign(sink)
  const text = read_str(sink)
  return { color, fontSize, lineHeight, align, text }
}

export const readBorder = (sink: Sink): Border => {
  const top = readBorderSide(sink)
  const right = readBorderSide(sink)
  const bottom = readBorderSide(sink)
  const left = readBorderSide(sink)
  return { top, right, bottom, left }
}

export const readBorderSide = (sink: Sink): BorderSide => {
  const width = read_f32(sink)
  const style = readBorderStyle(sink)
  const color = readColor(sink)
  return { width, style, color }
}

const BorderStyleReverseMap: BorderStyle[] = [BorderStyle.None, BorderStyle.Solid]

export const readBorderStyle = (sink: Sink): BorderStyle => BorderStyleReverseMap[read_u32(sink)]
