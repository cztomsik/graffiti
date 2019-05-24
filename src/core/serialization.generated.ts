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
  UpdateSceneMsg_AppendChild,
  UpdateSceneMsg_InsertBefore,
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
  write_u32,
  write_bool,
  write_seq,
  write_str,
  write_u64,
  write_u16,
  write_opt,
  write_u8,
  write_f32,
  Sink,
  Serializer
} from 'ts-binary'

const writeVecUpdateSceneMsg = (sink: Sink, val: Array<UpdateSceneMsg>): Sink =>
  write_seq(sink, val, writeUpdateSceneMsg)

const writeVecEvent = (sink: Sink, val: Array<Event>): Sink => write_seq(sink, val, writeEvent)

const writeOptBorderRadius = (sink: Sink, val: (BorderRadius) | undefined): Sink =>
  write_opt(sink, val, writeBorderRadius)

const writeOptBorder = (sink: Sink, val: (Border) | undefined): Sink => write_opt(sink, val, writeBorder)

const writeOptBoxShadow = (sink: Sink, val: (BoxShadow) | undefined): Sink => write_opt(sink, val, writeBoxShadow)

const writeOptColor = (sink: Sink, val: (Color) | undefined): Sink => write_opt(sink, val, writeColor)

const writeOptImage = (sink: Sink, val: (Image) | undefined): Sink => write_opt(sink, val, writeImage)

const writeOptText = (sink: Sink, val: (Text) | undefined): Sink => write_opt(sink, val, writeText)

export const writeFfiMsg = (sink: Sink, val: FfiMsg): Sink => {
  switch (val.tag) {
    case 'GetEvents':
      return write_bool(write_u32(sink, 0), val.value)
    case 'CreateWindow':
      return write_u32(sink, 1)
    case 'UpdateScene':
      return writeFfiMsg_UpdateScene(write_u32(sink, 2), val.value)
  }
}

const writeFfiMsg_UpdateScene = (sink: Sink, { window, msgs }: FfiMsg_UpdateScene): Sink =>
  writeVecUpdateSceneMsg(writeWindowId(sink, window), msgs)

export const writeFfiResult = (sink: Sink, val: FfiResult): Sink => {
  switch (val.tag) {
    case 'Nothing':
      return write_u32(sink, 0)
    case 'Error':
      return write_str(write_u32(sink, 1), val.value)
    case 'Events':
      return writeVecEvent(write_u32(sink, 2), val.value)
    case 'WindowId':
      return writeWindowId(write_u32(sink, 3), val.value)
  }
}

export const writeEvent = (sink: Sink, val: Event): Sink => {
  switch (val.tag) {
    case 'WindowEvent':
      return writeEvent_WindowEvent(write_u32(sink, 0), val.value)
  }
}

const writeEvent_WindowEvent = (sink: Sink, { window, event }: Event_WindowEvent): Sink =>
  writeWindowEvent(writeWindowId(sink, window), event)

export const writeWindowEvent = (sink: Sink, val: WindowEvent): Sink => {
  switch (val.tag) {
    case 'MouseMove':
      return writeWindowEvent_MouseMove(write_u32(sink, 0), val.value)
    case 'MouseDown':
      return writeWindowEvent_MouseDown(write_u32(sink, 1), val.value)
    case 'MouseUp':
      return writeWindowEvent_MouseUp(write_u32(sink, 2), val.value)
    case 'Scroll':
      return writeWindowEvent_Scroll(write_u32(sink, 3), val.value)
    case 'KeyDown':
      return write_u16(write_u32(sink, 4), val.value)
    case 'KeyPress':
      return write_u16(write_u32(sink, 5), val.value)
    case 'KeyUp':
      return write_u16(write_u32(sink, 6), val.value)
    case 'Focus':
      return write_u32(sink, 7)
    case 'Blur':
      return write_u32(sink, 8)
    case 'Resize':
      return write_u32(sink, 9)
    case 'Close':
      return write_u32(sink, 10)
    case 'Unknown':
      return write_u32(sink, 11)
  }
}

const writeWindowEvent_MouseMove = (sink: Sink, { target }: WindowEvent_MouseMove): Sink => write_u64(sink, target)

const writeWindowEvent_MouseDown = (sink: Sink, { target }: WindowEvent_MouseDown): Sink => write_u64(sink, target)

const writeWindowEvent_MouseUp = (sink: Sink, { target }: WindowEvent_MouseUp): Sink => write_u64(sink, target)

const writeWindowEvent_Scroll = (sink: Sink, { target }: WindowEvent_Scroll): Sink => write_u64(sink, target)

export const writeUpdateSceneMsg = (sink: Sink, val: UpdateSceneMsg): Sink => {
  switch (val.tag) {
    case 'Alloc':
      return write_u32(sink, 0)
    case 'AppendChild':
      return writeUpdateSceneMsg_AppendChild(write_u32(sink, 1), val.value)
    case 'InsertBefore':
      return writeUpdateSceneMsg_InsertBefore(write_u32(sink, 2), val.value)
    case 'RemoveChild':
      return writeUpdateSceneMsg_RemoveChild(write_u32(sink, 3), val.value)
    case 'SetStyleProp':
      return writeUpdateSceneMsg_SetStyleProp(write_u32(sink, 4), val.value)
  }
}

const writeUpdateSceneMsg_AppendChild = (sink: Sink, { parent, child }: UpdateSceneMsg_AppendChild): Sink =>
  writeSurfaceId(writeSurfaceId(sink, parent), child)

const writeUpdateSceneMsg_InsertBefore = (sink: Sink, { parent, child, before }: UpdateSceneMsg_InsertBefore): Sink =>
  writeSurfaceId(writeSurfaceId(writeSurfaceId(sink, parent), child), before)

const writeUpdateSceneMsg_RemoveChild = (sink: Sink, { parent, child }: UpdateSceneMsg_RemoveChild): Sink =>
  writeSurfaceId(writeSurfaceId(sink, parent), child)

const writeUpdateSceneMsg_SetStyleProp = (sink: Sink, { surface, prop }: UpdateSceneMsg_SetStyleProp): Sink =>
  writeStyleProp(writeSurfaceId(sink, surface), prop)

export const writeStyleProp = (sink: Sink, val: StyleProp): Sink => {
  switch (val.tag) {
    case 'Size':
      return writeSize(write_u32(sink, 0), val.value)
    case 'Flex':
      return writeFlex(write_u32(sink, 1), val.value)
    case 'Flow':
      return writeFlow(write_u32(sink, 2), val.value)
    case 'Padding':
      return writeDimensions(write_u32(sink, 3), val.value)
    case 'Margin':
      return writeDimensions(write_u32(sink, 4), val.value)
    case 'BorderRadius':
      return writeOptBorderRadius(write_u32(sink, 5), val.value)
    case 'Border':
      return writeOptBorder(write_u32(sink, 6), val.value)
    case 'BoxShadow':
      return writeOptBoxShadow(write_u32(sink, 7), val.value)
    case 'BackgroundColor':
      return writeOptColor(write_u32(sink, 8), val.value)
    case 'Image':
      return writeOptImage(write_u32(sink, 9), val.value)
    case 'Text':
      return writeOptText(write_u32(sink, 10), val.value)
    case 'Overflow':
      return writeOverflow(write_u32(sink, 11), val.value)
  }
}

export const writeWindowId: Serializer<WindowId> = write_u16

export const writeSurfaceId: Serializer<SurfaceId> = write_u64

export const writeColor = (sink: Sink, val: Color): Sink =>
  write_u8(write_u8(write_u8(write_u8(sink, val[0]), val[1]), val[2]), val[3])

const FlexDirectionMap: { [key: string]: number } = { Column: 0, ColumnReverse: 1, Row: 2, RowReverse: 3 }

export const writeFlexDirection = (sink: Sink, val: FlexDirection): Sink => write_u32(sink, FlexDirectionMap[val])

const FlexWrapMap: { [key: string]: number } = { NoWrap: 0, Wrap: 1, WrapReverse: 2 }

export const writeFlexWrap = (sink: Sink, val: FlexWrap): Sink => write_u32(sink, FlexWrapMap[val])

const FlexAlignMap: { [key: string]: number } = {
  Auto: 0,
  FlexStart: 1,
  Center: 2,
  FlexEnd: 3,
  Stretch: 4,
  Baseline: 5,
  SpaceBetween: 6,
  SpaceAround: 7
}

export const writeFlexAlign = (sink: Sink, val: FlexAlign): Sink => write_u32(sink, FlexAlignMap[val])

const JustifyContentMap: { [key: string]: number } = {
  FlexStart: 0,
  Center: 1,
  FlexEnd: 2,
  SpaceBetween: 3,
  SpaceAround: 4,
  SpaceEvenly: 5
}

export const writeJustifyContent = (sink: Sink, val: JustifyContent): Sink => write_u32(sink, JustifyContentMap[val])

export const writeFlow = (
  sink: Sink,
  { flexDirection, flexWrap, alignContent, alignItems, alignSelf, justifyContent }: Flow
): Sink =>
  writeJustifyContent(
    writeFlexAlign(
      writeFlexAlign(
        writeFlexAlign(writeFlexWrap(writeFlexDirection(sink, flexDirection), flexWrap), alignContent),
        alignItems
      ),
      alignSelf
    ),
    justifyContent
  )

export const writeFlex = (sink: Sink, { flexGrow, flexShrink, flexBasis }: Flex): Sink =>
  writeDimension(write_f32(write_f32(sink, flexGrow), flexShrink), flexBasis)

export const writeDimension = (sink: Sink, val: Dimension): Sink => {
  switch (val.tag) {
    case 'Auto':
      return write_u32(sink, 0)
    case 'Point':
      return write_f32(write_u32(sink, 1), val.value)
    case 'Percent':
      return write_f32(write_u32(sink, 2), val.value)
  }
}

const OverflowMap: { [key: string]: number } = { Visible: 0, Hidden: 1, Scroll: 2 }

export const writeOverflow = (sink: Sink, val: Overflow): Sink => write_u32(sink, OverflowMap[val])

export const writeSize = (sink: Sink, val: Size): Sink => writeDimension(writeDimension(sink, val[0]), val[1])

export const writeRect = (sink: Sink, val: Rect): Sink =>
  write_f32(write_f32(write_f32(write_f32(sink, val[0]), val[1]), val[2]), val[3])

export const writeDimensions = (sink: Sink, val: Dimensions): Sink =>
  writeDimension(writeDimension(writeDimension(writeDimension(sink, val[0]), val[1]), val[2]), val[3])

export const writeVector2f = (sink: Sink, val: Vector2f): Sink => write_f32(write_f32(sink, val[0]), val[1])

export const writeBorderRadius = (sink: Sink, val: BorderRadius): Sink =>
  write_f32(write_f32(write_f32(write_f32(sink, val[0]), val[1]), val[2]), val[3])

export const writeBoxShadow = (sink: Sink, { color, offset, blur, spread }: BoxShadow): Sink =>
  write_f32(write_f32(writeVector2f(writeColor(sink, color), offset), blur), spread)

export const writeImage = (sink: Sink, { url }: Image): Sink => write_str(sink, url)

const TextAlignMap: { [key: string]: number } = { Left: 0, Center: 1, Right: 2 }

export const writeTextAlign = (sink: Sink, val: TextAlign): Sink => write_u32(sink, TextAlignMap[val])

export const writeText = (sink: Sink, { color, fontSize, lineHeight, align, text }: Text): Sink =>
  write_str(writeTextAlign(write_f32(write_f32(writeColor(sink, color), fontSize), lineHeight), align), text)

export const writeBorder = (sink: Sink, { top, right, bottom, left }: Border): Sink =>
  writeBorderSide(writeBorderSide(writeBorderSide(writeBorderSide(sink, top), right), bottom), left)

export const writeBorderSide = (sink: Sink, { width, style, color }: BorderSide): Sink =>
  writeColor(writeBorderStyle(write_f32(sink, width), style), color)

const BorderStyleMap: { [key: string]: number } = { None: 0, Solid: 1 }

export const writeBorderStyle = (sink: Sink, val: BorderStyle): Sink => write_u32(sink, BorderStyleMap[val])
