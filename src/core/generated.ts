export type FfiMsg =
  | { tag: 'GetNextEvent'; value: boolean }
  | { tag: 'CreateWindow' }
  | { tag: 'UpdateScene'; value: FfiMsgUpdateScene }

export interface FfiMsgUpdateScene {
  window: WindowId
  msgs: Array<UpdateSceneMsg>
}

export module FfiMsg {
  export const GetNextEvent = (value: boolean): FfiMsg => ({
    tag: 'GetNextEvent',
    value
  })

  export const CreateWindow: FfiMsg = { tag: 'CreateWindow' }

  export const UpdateScene = (value: FfiMsgUpdateScene): FfiMsg => ({
    tag: 'UpdateScene',
    value
  })
}

export type FfiResult =
  | { tag: 'Nothing' }
  | { tag: 'Event'; value: Event }
  | { tag: 'WindowId'; value: WindowId }

export module FfiResult {
  export const Nothing: FfiResult = { tag: 'Nothing' }

  export const Event = (value: Event): FfiResult => ({ tag: 'Event', value })

  export const WindowId = (value: WindowId): FfiResult => ({
    tag: 'WindowId',
    value
  })
}

export type Event = { tag: 'WindowEvent'; value: EventWindowEvent }

export interface EventWindowEvent {
  window: WindowId
  event: WindowEvent
}

export module Event {
  export const WindowEvent = (value: EventWindowEvent): Event => ({
    tag: 'WindowEvent',
    value
  })
}

export type WindowEvent =
  | { tag: 'MouseMove'; value: WindowEventMouseMove }
  | { tag: 'MouseDown'; value: WindowEventMouseDown }
  | { tag: 'MouseUp'; value: WindowEventMouseUp }
  | { tag: 'Scroll'; value: WindowEventScroll }
  | { tag: 'KeyDown' }
  | { tag: 'KeyPress'; value: number }
  | { tag: 'KeyUp' }
  | { tag: 'Focus' }
  | { tag: 'Blur' }
  | { tag: 'Resize' }
  | { tag: 'Close' }
  | { tag: 'Unknown' }

export interface WindowEventMouseMove {
  target: number
}

export interface WindowEventMouseDown {
  target: number
}

export interface WindowEventMouseUp {
  target: number
}

export interface WindowEventScroll {
  target: number
}

export module WindowEvent {
  export const MouseMove = (value: WindowEventMouseMove): WindowEvent => ({
    tag: 'MouseMove',
    value
  })

  export const MouseDown = (value: WindowEventMouseDown): WindowEvent => ({
    tag: 'MouseDown',
    value
  })

  export const MouseUp = (value: WindowEventMouseUp): WindowEvent => ({
    tag: 'MouseUp',
    value
  })

  export const Scroll = (value: WindowEventScroll): WindowEvent => ({
    tag: 'Scroll',
    value
  })

  export const KeyDown: WindowEvent = { tag: 'KeyDown' }

  export const KeyPress = (value: number): WindowEvent => ({
    tag: 'KeyPress',
    value
  })

  export const KeyUp: WindowEvent = { tag: 'KeyUp' }

  export const Focus: WindowEvent = { tag: 'Focus' }

  export const Blur: WindowEvent = { tag: 'Blur' }

  export const Resize: WindowEvent = { tag: 'Resize' }

  export const Close: WindowEvent = { tag: 'Close' }

  export const Unknown: WindowEvent = { tag: 'Unknown' }
}

export type UpdateSceneMsg =
  | { tag: 'Alloc' }
  | { tag: 'AppendChild'; value: UpdateSceneMsgAppendChild }
  | { tag: 'InsertBefore'; value: UpdateSceneMsgInsertBefore }
  | { tag: 'RemoveChild'; value: UpdateSceneMsgRemoveChild }
  | { tag: 'SetBorderRadius'; value: UpdateSceneMsgSetBorderRadius }
  | { tag: 'SetSize'; value: UpdateSceneMsgSetSize }
  | { tag: 'SetFlex'; value: UpdateSceneMsgSetFlex }
  | { tag: 'SetFlow'; value: UpdateSceneMsgSetFlow }
  | { tag: 'SetPadding'; value: UpdateSceneMsgSetPadding }
  | { tag: 'SetMargin'; value: UpdateSceneMsgSetMargin }
  | { tag: 'SetBoxShadow'; value: UpdateSceneMsgSetBoxShadow }
  | { tag: 'SetBackgroundColor'; value: UpdateSceneMsgSetBackgroundColor }
  | { tag: 'SetImage'; value: UpdateSceneMsgSetImage }
  | { tag: 'SetText'; value: UpdateSceneMsgSetText }
  | { tag: 'SetBorder'; value: UpdateSceneMsgSetBorder }

export interface UpdateSceneMsgAppendChild {
  parent: SurfaceId
  child: SurfaceId
}

export interface UpdateSceneMsgInsertBefore {
  parent: SurfaceId
  child: SurfaceId
  before: SurfaceId
}

export interface UpdateSceneMsgRemoveChild {
  parent: SurfaceId
  child: SurfaceId
}

export interface UpdateSceneMsgSetBorderRadius {
  surface: SurfaceId
  borderRadius: (BorderRadius) | undefined
}

export interface UpdateSceneMsgSetSize {
  surface: SurfaceId
  size: Size
}

export interface UpdateSceneMsgSetFlex {
  surface: SurfaceId
  flex: Flex
}

export interface UpdateSceneMsgSetFlow {
  surface: SurfaceId
  flow: Flow
}

export interface UpdateSceneMsgSetPadding {
  surface: SurfaceId
  padding: Dimensions
}

export interface UpdateSceneMsgSetMargin {
  surface: SurfaceId
  margin: Dimensions
}

export interface UpdateSceneMsgSetBoxShadow {
  surface: SurfaceId
  boxShadow: (BoxShadow) | undefined
}

export interface UpdateSceneMsgSetBackgroundColor {
  surface: SurfaceId
  color: (Color) | undefined
}

export interface UpdateSceneMsgSetImage {
  surface: SurfaceId
  image: (Image) | undefined
}

export interface UpdateSceneMsgSetText {
  surface: SurfaceId
  text: (Text) | undefined
}

export interface UpdateSceneMsgSetBorder {
  surface: SurfaceId
  border: (Border) | undefined
}

export module UpdateSceneMsg {
  export const Alloc: UpdateSceneMsg = { tag: 'Alloc' }

  export const AppendChild = (
    value: UpdateSceneMsgAppendChild
  ): UpdateSceneMsg => ({ tag: 'AppendChild', value })

  export const InsertBefore = (
    value: UpdateSceneMsgInsertBefore
  ): UpdateSceneMsg => ({ tag: 'InsertBefore', value })

  export const RemoveChild = (
    value: UpdateSceneMsgRemoveChild
  ): UpdateSceneMsg => ({ tag: 'RemoveChild', value })

  export const SetBorderRadius = (
    value: UpdateSceneMsgSetBorderRadius
  ): UpdateSceneMsg => ({ tag: 'SetBorderRadius', value })

  export const SetSize = (value: UpdateSceneMsgSetSize): UpdateSceneMsg => ({
    tag: 'SetSize',
    value
  })

  export const SetFlex = (value: UpdateSceneMsgSetFlex): UpdateSceneMsg => ({
    tag: 'SetFlex',
    value
  })

  export const SetFlow = (value: UpdateSceneMsgSetFlow): UpdateSceneMsg => ({
    tag: 'SetFlow',
    value
  })

  export const SetPadding = (
    value: UpdateSceneMsgSetPadding
  ): UpdateSceneMsg => ({ tag: 'SetPadding', value })

  export const SetMargin = (
    value: UpdateSceneMsgSetMargin
  ): UpdateSceneMsg => ({ tag: 'SetMargin', value })

  export const SetBoxShadow = (
    value: UpdateSceneMsgSetBoxShadow
  ): UpdateSceneMsg => ({ tag: 'SetBoxShadow', value })

  export const SetBackgroundColor = (
    value: UpdateSceneMsgSetBackgroundColor
  ): UpdateSceneMsg => ({ tag: 'SetBackgroundColor', value })

  export const SetImage = (value: UpdateSceneMsgSetImage): UpdateSceneMsg => ({
    tag: 'SetImage',
    value
  })

  export const SetText = (value: UpdateSceneMsgSetText): UpdateSceneMsg => ({
    tag: 'SetText',
    value
  })

  export const SetBorder = (
    value: UpdateSceneMsgSetBorder
  ): UpdateSceneMsg => ({ tag: 'SetBorder', value })
}

export type WindowId = number

export type SurfaceId = number

export interface Color {
  0: number
  1: number
  2: number
  3: number
  length: 4
}

export module Color {
  export const mk = (p0: number, p1: number, p2: number, p3: number): Color => [
    p0,
    p1,
    p2,
    p3
  ]
}

export enum FlexDirection {
  Column = 'Column',
  ColumnReverse = 'ColumnReverse',
  Row = 'Row',
  RowReverse = 'RowReverse'
}

export enum FlexWrap {
  NoWrap = 'NoWrap',
  Wrap = 'Wrap',
  WrapReverse = 'WrapReverse'
}

export enum FlexAlign {
  Auto = 'Auto',
  FlexStart = 'FlexStart',
  Center = 'Center',
  FlexEnd = 'FlexEnd',
  Stretch = 'Stretch',
  Baseline = 'Baseline',
  SpaceBetween = 'SpaceBetween',
  SpaceAround = 'SpaceAround'
}

export enum JustifyContent {
  FlexStart = 'FlexStart',
  Center = 'Center',
  FlexEnd = 'FlexEnd',
  SpaceBetween = 'SpaceBetween',
  SpaceAround = 'SpaceAround',
  SpaceEvenly = 'SpaceEvenly'
}

export interface Flow {
  flexDirection: FlexDirection
  flexWrap: FlexWrap
  alignContent: FlexAlign
  alignItems: FlexAlign
  justifyContent: JustifyContent
}

export interface Flex {
  flexGrow: number
  flexShrink: number
  flexBasis: Dimension
}

export type Dimension =
  | { tag: 'Auto' }
  | { tag: 'Point'; value: number }
  | { tag: 'Percent'; value: number }

export module Dimension {
  export const Auto: Dimension = { tag: 'Auto' }

  export const Point = (value: number): Dimension => ({ tag: 'Point', value })

  export const Percent = (value: number): Dimension => ({
    tag: 'Percent',
    value
  })
}

export interface Size {
  0: Dimension
  1: Dimension
  length: 2
}

export module Size {
  export const mk = (p0: Dimension, p1: Dimension): Size => [p0, p1]
}

export interface Rect {
  0: number
  1: number
  2: number
  3: number
  length: 4
}

export module Rect {
  export const mk = (p0: number, p1: number, p2: number, p3: number): Rect => [
    p0,
    p1,
    p2,
    p3
  ]
}

export interface Dimensions {
  0: Dimension
  1: Dimension
  2: Dimension
  3: Dimension
  length: 4
}

export module Dimensions {
  export const mk = (
    p0: Dimension,
    p1: Dimension,
    p2: Dimension,
    p3: Dimension
  ): Dimensions => [p0, p1, p2, p3]
}

export interface Vector2f {
  0: number
  1: number
  length: 2
}

export module Vector2f {
  export const mk = (p0: number, p1: number): Vector2f => [p0, p1]
}

export interface BorderRadius {
  0: number
  1: number
  2: number
  3: number
  length: 4
}

export module BorderRadius {
  export const mk = (
    p0: number,
    p1: number,
    p2: number,
    p3: number
  ): BorderRadius => [p0, p1, p2, p3]
}

export interface BoxShadow {
  color: Color
  offset: Vector2f
  blur: number
  spread: number
}

export interface Image {
  url: string
}

export enum TextAlign {
  Left = 'Left',
  Center = 'Center',
  Right = 'Right'
}

export interface Text {
  color: Color
  fontSize: number
  lineHeight: number
  align: TextAlign
  text: string
}

export interface Border {
  top: BorderSide
  right: BorderSide
  bottom: BorderSide
  left: BorderSide
}

export interface BorderSide {
  width: number
  style: BorderStyle
  color: Color
}

export enum BorderStyle {
  None = 'None',
  Solid = 'Solid'
}
