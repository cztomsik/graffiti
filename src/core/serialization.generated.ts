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
  BorderRadius,
  UpdateSceneMsg_SetBorderRadius,
  Overflow,
  UpdateSceneMsg_SetOverflow,
  Size,
  UpdateSceneMsg_SetSize,
  Flex,
  UpdateSceneMsg_SetFlex,
  Flow,
  UpdateSceneMsg_SetFlow,
  Dimensions,
  UpdateSceneMsg_SetPadding,
  UpdateSceneMsg_SetMargin,
  BoxShadow,
  UpdateSceneMsg_SetBoxShadow,
  Color,
  UpdateSceneMsg_SetBackgroundColor,
  Image,
  UpdateSceneMsg_SetImage,
  Text,
  UpdateSceneMsg_SetText,
  Border,
  UpdateSceneMsg_SetBorder,
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
  write_u64,
  write_u16,
  write_opt,
  write_u8,
  write_f32,
  write_str,
  Sink,
  SerFunc
} from 'ts-rust-bridge-bincode'

const writeVecUpdateSceneMsg = (sink: Sink, val: Array<UpdateSceneMsg>): Sink =>
  write_seq(sink, val, writeUpdateSceneMsg)

const writeVecEvent = (sink: Sink, val: Array<Event>): Sink =>
  write_seq(sink, val, writeEvent)

const writeOptBorderRadius = (
  sink: Sink,
  val: (BorderRadius) | undefined
): Sink => write_opt(sink, val, writeBorderRadius)

const writeOptBoxShadow = (sink: Sink, val: (BoxShadow) | undefined): Sink =>
  write_opt(sink, val, writeBoxShadow)

const writeOptColor = (sink: Sink, val: (Color) | undefined): Sink =>
  write_opt(sink, val, writeColor)

const writeOptImage = (sink: Sink, val: (Image) | undefined): Sink =>
  write_opt(sink, val, writeImage)

const writeOptText = (sink: Sink, val: (Text) | undefined): Sink =>
  write_opt(sink, val, writeText)

const writeOptBorder = (sink: Sink, val: (Border) | undefined): Sink =>
  write_opt(sink, val, writeBorder)

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

const writeFfiMsg_UpdateScene = (
  sink: Sink,
  { window, msgs }: FfiMsg_UpdateScene
): Sink => writeVecUpdateSceneMsg(writeWindowId(sink, window), msgs)

export const writeFfiResult = (sink: Sink, val: FfiResult): Sink => {
  switch (val.tag) {
    case 'Nothing':
      return write_u32(sink, 0)
    case 'Events':
      return writeVecEvent(write_u32(sink, 1), val.value)
    case 'WindowId':
      return writeWindowId(write_u32(sink, 2), val.value)
  }
}

export const writeEvent = (sink: Sink, val: Event): Sink => {
  switch (val.tag) {
    case 'WindowEvent':
      return writeEvent_WindowEvent(write_u32(sink, 0), val.value)
  }
}

const writeEvent_WindowEvent = (
  sink: Sink,
  { window, event }: Event_WindowEvent
): Sink => writeWindowEvent(writeWindowId(sink, window), event)

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
      return write_u32(sink, 4)
    case 'KeyPress':
      return write_u16(write_u32(sink, 5), val.value)
    case 'KeyUp':
      return write_u32(sink, 6)
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

const writeWindowEvent_MouseMove = (
  sink: Sink,
  { target }: WindowEvent_MouseMove
): Sink => write_u64(sink, target)

const writeWindowEvent_MouseDown = (
  sink: Sink,
  { target }: WindowEvent_MouseDown
): Sink => write_u64(sink, target)

const writeWindowEvent_MouseUp = (
  sink: Sink,
  { target }: WindowEvent_MouseUp
): Sink => write_u64(sink, target)

const writeWindowEvent_Scroll = (
  sink: Sink,
  { target }: WindowEvent_Scroll
): Sink => write_u64(sink, target)

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
    case 'SetBorderRadius':
      return writeUpdateSceneMsg_SetBorderRadius(write_u32(sink, 4), val.value)
    case 'SetOverflow':
      return writeUpdateSceneMsg_SetOverflow(write_u32(sink, 5), val.value)
    case 'SetSize':
      return writeUpdateSceneMsg_SetSize(write_u32(sink, 6), val.value)
    case 'SetFlex':
      return writeUpdateSceneMsg_SetFlex(write_u32(sink, 7), val.value)
    case 'SetFlow':
      return writeUpdateSceneMsg_SetFlow(write_u32(sink, 8), val.value)
    case 'SetPadding':
      return writeUpdateSceneMsg_SetPadding(write_u32(sink, 9), val.value)
    case 'SetMargin':
      return writeUpdateSceneMsg_SetMargin(write_u32(sink, 10), val.value)
    case 'SetBoxShadow':
      return writeUpdateSceneMsg_SetBoxShadow(write_u32(sink, 11), val.value)
    case 'SetBackgroundColor':
      return writeUpdateSceneMsg_SetBackgroundColor(
        write_u32(sink, 12),
        val.value
      )
    case 'SetImage':
      return writeUpdateSceneMsg_SetImage(write_u32(sink, 13), val.value)
    case 'SetText':
      return writeUpdateSceneMsg_SetText(write_u32(sink, 14), val.value)
    case 'SetBorder':
      return writeUpdateSceneMsg_SetBorder(write_u32(sink, 15), val.value)
  }
}

const writeUpdateSceneMsg_AppendChild = (
  sink: Sink,
  { parent, child }: UpdateSceneMsg_AppendChild
): Sink => writeSurfaceId(writeSurfaceId(sink, parent), child)

const writeUpdateSceneMsg_InsertBefore = (
  sink: Sink,
  { parent, child, before }: UpdateSceneMsg_InsertBefore
): Sink =>
  writeSurfaceId(writeSurfaceId(writeSurfaceId(sink, parent), child), before)

const writeUpdateSceneMsg_RemoveChild = (
  sink: Sink,
  { parent, child }: UpdateSceneMsg_RemoveChild
): Sink => writeSurfaceId(writeSurfaceId(sink, parent), child)

const writeUpdateSceneMsg_SetBorderRadius = (
  sink: Sink,
  { surface, borderRadius }: UpdateSceneMsg_SetBorderRadius
): Sink => writeOptBorderRadius(writeSurfaceId(sink, surface), borderRadius)

const writeUpdateSceneMsg_SetOverflow = (
  sink: Sink,
  { surface, overflow }: UpdateSceneMsg_SetOverflow
): Sink => writeOverflow(writeSurfaceId(sink, surface), overflow)

const writeUpdateSceneMsg_SetSize = (
  sink: Sink,
  { surface, size }: UpdateSceneMsg_SetSize
): Sink => writeSize(writeSurfaceId(sink, surface), size)

const writeUpdateSceneMsg_SetFlex = (
  sink: Sink,
  { surface, flex }: UpdateSceneMsg_SetFlex
): Sink => writeFlex(writeSurfaceId(sink, surface), flex)

const writeUpdateSceneMsg_SetFlow = (
  sink: Sink,
  { surface, flow }: UpdateSceneMsg_SetFlow
): Sink => writeFlow(writeSurfaceId(sink, surface), flow)

const writeUpdateSceneMsg_SetPadding = (
  sink: Sink,
  { surface, padding }: UpdateSceneMsg_SetPadding
): Sink => writeDimensions(writeSurfaceId(sink, surface), padding)

const writeUpdateSceneMsg_SetMargin = (
  sink: Sink,
  { surface, margin }: UpdateSceneMsg_SetMargin
): Sink => writeDimensions(writeSurfaceId(sink, surface), margin)

const writeUpdateSceneMsg_SetBoxShadow = (
  sink: Sink,
  { surface, boxShadow }: UpdateSceneMsg_SetBoxShadow
): Sink => writeOptBoxShadow(writeSurfaceId(sink, surface), boxShadow)

const writeUpdateSceneMsg_SetBackgroundColor = (
  sink: Sink,
  { surface, color }: UpdateSceneMsg_SetBackgroundColor
): Sink => writeOptColor(writeSurfaceId(sink, surface), color)

const writeUpdateSceneMsg_SetImage = (
  sink: Sink,
  { surface, image }: UpdateSceneMsg_SetImage
): Sink => writeOptImage(writeSurfaceId(sink, surface), image)

const writeUpdateSceneMsg_SetText = (
  sink: Sink,
  { surface, text }: UpdateSceneMsg_SetText
): Sink => writeOptText(writeSurfaceId(sink, surface), text)

const writeUpdateSceneMsg_SetBorder = (
  sink: Sink,
  { surface, border }: UpdateSceneMsg_SetBorder
): Sink => writeOptBorder(writeSurfaceId(sink, surface), border)

export const writeWindowId: SerFunc<WindowId> = write_u16

export const writeSurfaceId: SerFunc<SurfaceId> = write_u64

export const writeColor = (sink: Sink, val: Color): Sink =>
  write_u8(write_u8(write_u8(write_u8(sink, val[0]), val[1]), val[2]), val[3])

const FlexDirectionMap: { [key: string]: number } = {
  Column: 0,
  ColumnReverse: 1,
  Row: 2,
  RowReverse: 3
}

export const writeFlexDirection = (sink: Sink, val: FlexDirection): Sink =>
  write_u32(sink, FlexDirectionMap[val])

const FlexWrapMap: { [key: string]: number } = {
  NoWrap: 0,
  Wrap: 1,
  WrapReverse: 2
}

export const writeFlexWrap = (sink: Sink, val: FlexWrap): Sink =>
  write_u32(sink, FlexWrapMap[val])

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

export const writeFlexAlign = (sink: Sink, val: FlexAlign): Sink =>
  write_u32(sink, FlexAlignMap[val])

const JustifyContentMap: { [key: string]: number } = {
  FlexStart: 0,
  Center: 1,
  FlexEnd: 2,
  SpaceBetween: 3,
  SpaceAround: 4,
  SpaceEvenly: 5
}

export const writeJustifyContent = (sink: Sink, val: JustifyContent): Sink =>
  write_u32(sink, JustifyContentMap[val])

export const writeFlow = (
  sink: Sink,
  { flexDirection, flexWrap, alignContent, alignItems, justifyContent }: Flow
): Sink =>
  writeJustifyContent(
    writeFlexAlign(
      writeFlexAlign(
        writeFlexWrap(writeFlexDirection(sink, flexDirection), flexWrap),
        alignContent
      ),
      alignItems
    ),
    justifyContent
  )

export const writeFlex = (
  sink: Sink,
  { flexGrow, flexShrink, flexBasis }: Flex
): Sink =>
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

const OverflowMap: { [key: string]: number } = {
  Visible: 0,
  Hidden: 1,
  Scroll: 2
}

export const writeOverflow = (sink: Sink, val: Overflow): Sink =>
  write_u32(sink, OverflowMap[val])

export const writeSize = (sink: Sink, val: Size): Sink =>
  writeDimension(writeDimension(sink, val[0]), val[1])

export const writeRect = (sink: Sink, val: Rect): Sink =>
  write_f32(
    write_f32(write_f32(write_f32(sink, val[0]), val[1]), val[2]),
    val[3]
  )

export const writeDimensions = (sink: Sink, val: Dimensions): Sink =>
  writeDimension(
    writeDimension(
      writeDimension(writeDimension(sink, val[0]), val[1]),
      val[2]
    ),
    val[3]
  )

export const writeVector2f = (sink: Sink, val: Vector2f): Sink =>
  write_f32(write_f32(sink, val[0]), val[1])

export const writeBorderRadius = (sink: Sink, val: BorderRadius): Sink =>
  write_f32(
    write_f32(write_f32(write_f32(sink, val[0]), val[1]), val[2]),
    val[3]
  )

export const writeBoxShadow = (
  sink: Sink,
  { color, offset, blur, spread }: BoxShadow
): Sink =>
  write_f32(
    write_f32(writeVector2f(writeColor(sink, color), offset), blur),
    spread
  )

export const writeImage = (sink: Sink, { url }: Image): Sink =>
  write_str(sink, url)

const TextAlignMap: { [key: string]: number } = { Left: 0, Center: 1, Right: 2 }

export const writeTextAlign = (sink: Sink, val: TextAlign): Sink =>
  write_u32(sink, TextAlignMap[val])

export const writeText = (
  sink: Sink,
  { color, fontSize, lineHeight, align, text }: Text
): Sink =>
  write_str(
    writeTextAlign(
      write_f32(write_f32(writeColor(sink, color), fontSize), lineHeight),
      align
    ),
    text
  )

export const writeBorder = (
  sink: Sink,
  { top, right, bottom, left }: Border
): Sink =>
  writeBorderSide(
    writeBorderSide(writeBorderSide(writeBorderSide(sink, top), right), bottom),
    left
  )

export const writeBorderSide = (
  sink: Sink,
  { width, style, color }: BorderSide
): Sink => writeColor(writeBorderStyle(write_f32(sink, width), style), color)

const BorderStyleMap: { [key: string]: number } = { None: 0, Solid: 1 }

export const writeBorderStyle = (sink: Sink, val: BorderStyle): Sink =>
  write_u32(sink, BorderStyleMap[val])
