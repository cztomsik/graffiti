export type FfiMsg =
  | { tag: 'GetEvents'; value: boolean }
  | { tag: 'CreateWindow' }
  | { tag: 'UpdateScene'; value: FfiMsg_UpdateScene }

export interface FfiMsg_UpdateScene {
  window: WindowId
  msgs: Array<UpdateSceneMsg>
}

export module FfiMsg {
  export const GetEvents = (value: boolean): FfiMsg => ({
    tag: 'GetEvents',
    value
  })

  export const CreateWindow: FfiMsg = { tag: 'CreateWindow' }

  export const UpdateScene = (value: FfiMsg_UpdateScene): FfiMsg => ({
    tag: 'UpdateScene',
    value
  })
}

export type FfiResult =
  | { tag: 'Nothing' }
  | { tag: 'Events'; value: Array<Event> }
  | { tag: 'WindowId'; value: WindowId }

export module FfiResult {
  export const Nothing: FfiResult = { tag: 'Nothing' }

  export const Events = (value: Array<Event>): FfiResult => ({
    tag: 'Events',
    value
  })

  export const WindowId = (value: WindowId): FfiResult => ({
    tag: 'WindowId',
    value
  })
}

export type Event = { tag: 'WindowEvent'; value: Event_WindowEvent }

export interface Event_WindowEvent {
  window: WindowId
  event: WindowEvent
}

export module Event {
  export const WindowEvent = (value: Event_WindowEvent): Event => ({
    tag: 'WindowEvent',
    value
  })
}

export type WindowEvent =
  | { tag: 'MouseMove'; value: WindowEvent_MouseMove }
  | { tag: 'MouseDown'; value: WindowEvent_MouseDown }
  | { tag: 'MouseUp'; value: WindowEvent_MouseUp }
  | { tag: 'Scroll'; value: WindowEvent_Scroll }
  | { tag: 'KeyDown'; value: number }
  | { tag: 'KeyPress'; value: number }
  | { tag: 'KeyUp'; value: number }
  | { tag: 'Focus' }
  | { tag: 'Blur' }
  | { tag: 'Resize' }
  | { tag: 'Close' }
  | { tag: 'Unknown' }

export interface WindowEvent_MouseMove {
  target: number
}

export interface WindowEvent_MouseDown {
  target: number
}

export interface WindowEvent_MouseUp {
  target: number
}

export interface WindowEvent_Scroll {
  target: number
}

export module WindowEvent {
  export const MouseMove = (value: WindowEvent_MouseMove): WindowEvent => ({
    tag: 'MouseMove',
    value
  })

  export const MouseDown = (value: WindowEvent_MouseDown): WindowEvent => ({
    tag: 'MouseDown',
    value
  })

  export const MouseUp = (value: WindowEvent_MouseUp): WindowEvent => ({
    tag: 'MouseUp',
    value
  })

  export const Scroll = (value: WindowEvent_Scroll): WindowEvent => ({
    tag: 'Scroll',
    value
  })

  export const KeyDown = (value: number): WindowEvent => ({
    tag: 'KeyDown',
    value
  })

  export const KeyPress = (value: number): WindowEvent => ({
    tag: 'KeyPress',
    value
  })

  export const KeyUp = (value: number): WindowEvent => ({ tag: 'KeyUp', value })

  export const Focus: WindowEvent = { tag: 'Focus' }

  export const Blur: WindowEvent = { tag: 'Blur' }

  export const Resize: WindowEvent = { tag: 'Resize' }

  export const Close: WindowEvent = { tag: 'Close' }

  export const Unknown: WindowEvent = { tag: 'Unknown' }
}

export type UpdateSceneMsg =
  | { tag: 'Alloc' }
  | { tag: 'AppendChild'; value: UpdateSceneMsg_AppendChild }
  | { tag: 'InsertBefore'; value: UpdateSceneMsg_InsertBefore }
  | { tag: 'RemoveChild'; value: UpdateSceneMsg_RemoveChild }
  | { tag: 'SetBorderRadius'; value: UpdateSceneMsg_SetBorderRadius }
  | { tag: 'SetOverflow'; value: UpdateSceneMsg_SetOverflow }
  | { tag: 'SetSize'; value: UpdateSceneMsg_SetSize }
  | { tag: 'SetFlex'; value: UpdateSceneMsg_SetFlex }
  | { tag: 'SetFlow'; value: UpdateSceneMsg_SetFlow }
  | { tag: 'SetPadding'; value: UpdateSceneMsg_SetPadding }
  | { tag: 'SetMargin'; value: UpdateSceneMsg_SetMargin }
  | { tag: 'SetBoxShadow'; value: UpdateSceneMsg_SetBoxShadow }
  | { tag: 'SetBackgroundColor'; value: UpdateSceneMsg_SetBackgroundColor }
  | { tag: 'SetImage'; value: UpdateSceneMsg_SetImage }
  | { tag: 'SetText'; value: UpdateSceneMsg_SetText }
  | { tag: 'SetBorder'; value: UpdateSceneMsg_SetBorder }

export interface UpdateSceneMsg_AppendChild {
  parent: SurfaceId
  child: SurfaceId
}

export interface UpdateSceneMsg_InsertBefore {
  parent: SurfaceId
  child: SurfaceId
  before: SurfaceId
}

export interface UpdateSceneMsg_RemoveChild {
  parent: SurfaceId
  child: SurfaceId
}

export interface UpdateSceneMsg_SetBorderRadius {
  surface: SurfaceId
  borderRadius: (BorderRadius) | undefined
}

export interface UpdateSceneMsg_SetOverflow {
  surface: SurfaceId
  overflow: Overflow
}

export interface UpdateSceneMsg_SetSize {
  surface: SurfaceId
  size: Size
}

export interface UpdateSceneMsg_SetFlex {
  surface: SurfaceId
  flex: Flex
}

export interface UpdateSceneMsg_SetFlow {
  surface: SurfaceId
  flow: Flow
}

export interface UpdateSceneMsg_SetPadding {
  surface: SurfaceId
  padding: Dimensions
}

export interface UpdateSceneMsg_SetMargin {
  surface: SurfaceId
  margin: Dimensions
}

export interface UpdateSceneMsg_SetBoxShadow {
  surface: SurfaceId
  boxShadow: (BoxShadow) | undefined
}

export interface UpdateSceneMsg_SetBackgroundColor {
  surface: SurfaceId
  color: (Color) | undefined
}

export interface UpdateSceneMsg_SetImage {
  surface: SurfaceId
  image: (Image) | undefined
}

export interface UpdateSceneMsg_SetText {
  surface: SurfaceId
  text: (Text) | undefined
}

export interface UpdateSceneMsg_SetBorder {
  surface: SurfaceId
  border: (Border) | undefined
}

export module UpdateSceneMsg {
  export const Alloc: UpdateSceneMsg = { tag: 'Alloc' }

  export const AppendChild = (
    value: UpdateSceneMsg_AppendChild
  ): UpdateSceneMsg => ({ tag: 'AppendChild', value })

  export const InsertBefore = (
    value: UpdateSceneMsg_InsertBefore
  ): UpdateSceneMsg => ({ tag: 'InsertBefore', value })

  export const RemoveChild = (
    value: UpdateSceneMsg_RemoveChild
  ): UpdateSceneMsg => ({ tag: 'RemoveChild', value })

  export const SetBorderRadius = (
    value: UpdateSceneMsg_SetBorderRadius
  ): UpdateSceneMsg => ({ tag: 'SetBorderRadius', value })

  export const SetOverflow = (
    value: UpdateSceneMsg_SetOverflow
  ): UpdateSceneMsg => ({ tag: 'SetOverflow', value })

  export const SetSize = (value: UpdateSceneMsg_SetSize): UpdateSceneMsg => ({
    tag: 'SetSize',
    value
  })

  export const SetFlex = (value: UpdateSceneMsg_SetFlex): UpdateSceneMsg => ({
    tag: 'SetFlex',
    value
  })

  export const SetFlow = (value: UpdateSceneMsg_SetFlow): UpdateSceneMsg => ({
    tag: 'SetFlow',
    value
  })

  export const SetPadding = (
    value: UpdateSceneMsg_SetPadding
  ): UpdateSceneMsg => ({ tag: 'SetPadding', value })

  export const SetMargin = (
    value: UpdateSceneMsg_SetMargin
  ): UpdateSceneMsg => ({ tag: 'SetMargin', value })

  export const SetBoxShadow = (
    value: UpdateSceneMsg_SetBoxShadow
  ): UpdateSceneMsg => ({ tag: 'SetBoxShadow', value })

  export const SetBackgroundColor = (
    value: UpdateSceneMsg_SetBackgroundColor
  ): UpdateSceneMsg => ({ tag: 'SetBackgroundColor', value })

  export const SetImage = (value: UpdateSceneMsg_SetImage): UpdateSceneMsg => ({
    tag: 'SetImage',
    value
  })

  export const SetText = (value: UpdateSceneMsg_SetText): UpdateSceneMsg => ({
    tag: 'SetText',
    value
  })

  export const SetBorder = (
    value: UpdateSceneMsg_SetBorder
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

export enum Overflow {
  Visible = 'Visible',
  Hidden = 'Hidden',
  Scroll = 'Scroll'
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
