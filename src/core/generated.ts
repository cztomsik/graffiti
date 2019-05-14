export type FfiMsg =
  | { tag: 'GetEvents'; value: boolean }
  | { tag: 'CreateWindow' }
  | { tag: 'UpdateScene'; value: FfiMsg_UpdateScene }

export interface FfiMsg_UpdateScene {
  window: WindowId
  msgs: Array<UpdateSceneMsg>
}

export module FfiMsg {
  export const GetEvents = (value: boolean): FfiMsg => ({ tag: 'GetEvents', value })

  export const CreateWindow: FfiMsg = { tag: 'CreateWindow' }

  export const UpdateScene = (value: FfiMsg_UpdateScene): FfiMsg => ({ tag: 'UpdateScene', value })
}

export type FfiResult =
  | { tag: 'Nothing' }
  | { tag: 'Error'; value: string }
  | { tag: 'Events'; value: Array<Event> }
  | { tag: 'WindowId'; value: WindowId }

export module FfiResult {
  export const Nothing: FfiResult = { tag: 'Nothing' }

  export const Error = (value: string): FfiResult => ({ tag: 'Error', value })

  export const Events = (value: Array<Event>): FfiResult => ({ tag: 'Events', value })

  export const WindowId = (value: WindowId): FfiResult => ({ tag: 'WindowId', value })
}

export type Event = { tag: 'WindowEvent'; value: Event_WindowEvent }

export interface Event_WindowEvent {
  window: WindowId
  event: WindowEvent
}

export module Event {
  export const WindowEvent = (value: Event_WindowEvent): Event => ({ tag: 'WindowEvent', value })
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
  export const MouseMove = (value: WindowEvent_MouseMove): WindowEvent => ({ tag: 'MouseMove', value })

  export const MouseDown = (value: WindowEvent_MouseDown): WindowEvent => ({ tag: 'MouseDown', value })

  export const MouseUp = (value: WindowEvent_MouseUp): WindowEvent => ({ tag: 'MouseUp', value })

  export const Scroll = (value: WindowEvent_Scroll): WindowEvent => ({ tag: 'Scroll', value })

  export const KeyDown = (value: number): WindowEvent => ({ tag: 'KeyDown', value })

  export const KeyPress = (value: number): WindowEvent => ({ tag: 'KeyPress', value })

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
  | { tag: 'SetStyleProp'; value: UpdateSceneMsg_SetStyleProp }

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

export interface UpdateSceneMsg_SetStyleProp {
  surface: SurfaceId
  prop: StyleProp
}

export module UpdateSceneMsg {
  export const Alloc: UpdateSceneMsg = { tag: 'Alloc' }

  export const AppendChild = (value: UpdateSceneMsg_AppendChild): UpdateSceneMsg => ({ tag: 'AppendChild', value })

  export const InsertBefore = (value: UpdateSceneMsg_InsertBefore): UpdateSceneMsg => ({ tag: 'InsertBefore', value })

  export const RemoveChild = (value: UpdateSceneMsg_RemoveChild): UpdateSceneMsg => ({ tag: 'RemoveChild', value })

  export const SetStyleProp = (value: UpdateSceneMsg_SetStyleProp): UpdateSceneMsg => ({ tag: 'SetStyleProp', value })
}

export type StyleProp =
  | { tag: 'Size'; value: Size }
  | { tag: 'Flex'; value: Flex }
  | { tag: 'Flow'; value: Flow }
  | { tag: 'Padding'; value: Dimensions }
  | { tag: 'Margin'; value: Dimensions }
  | { tag: 'BorderRadius'; value: (BorderRadius) | undefined }
  | { tag: 'Border'; value: (Border) | undefined }
  | { tag: 'BoxShadow'; value: (BoxShadow) | undefined }
  | { tag: 'BackgroundColor'; value: (Color) | undefined }
  | { tag: 'Image'; value: (Image) | undefined }
  | { tag: 'Text'; value: (Text) | undefined }
  | { tag: 'Overflow'; value: Overflow }

export module StyleProp {
  export const Size = (value: Size): StyleProp => ({ tag: 'Size', value })

  export const Flex = (value: Flex): StyleProp => ({ tag: 'Flex', value })

  export const Flow = (value: Flow): StyleProp => ({ tag: 'Flow', value })

  export const Padding = (value: Dimensions): StyleProp => ({ tag: 'Padding', value })

  export const Margin = (value: Dimensions): StyleProp => ({ tag: 'Margin', value })

  export const BorderRadius = (value: (BorderRadius) | undefined): StyleProp => ({ tag: 'BorderRadius', value })

  export const Border = (value: (Border) | undefined): StyleProp => ({ tag: 'Border', value })

  export const BoxShadow = (value: (BoxShadow) | undefined): StyleProp => ({ tag: 'BoxShadow', value })

  export const BackgroundColor = (value: (Color) | undefined): StyleProp => ({ tag: 'BackgroundColor', value })

  export const Image = (value: (Image) | undefined): StyleProp => ({ tag: 'Image', value })

  export const Text = (value: (Text) | undefined): StyleProp => ({ tag: 'Text', value })

  export const Overflow = (value: Overflow): StyleProp => ({ tag: 'Overflow', value })
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

export const Color = (p0: number, p1: number, p2: number, p3: number): Color => [p0, p1, p2, p3]

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
  alignSelf: FlexAlign
  justifyContent: JustifyContent
}

export interface Flex {
  flexGrow: number
  flexShrink: number
  flexBasis: Dimension
}

export type Dimension = { tag: 'Auto' } | { tag: 'Point'; value: number } | { tag: 'Percent'; value: number }

export module Dimension {
  export const Auto: Dimension = { tag: 'Auto' }

  export const Point = (value: number): Dimension => ({ tag: 'Point', value })

  export const Percent = (value: number): Dimension => ({ tag: 'Percent', value })
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

export const Size = (p0: Dimension, p1: Dimension): Size => [p0, p1]

export interface Rect {
  0: number
  1: number
  2: number
  3: number
  length: 4
}

export const Rect = (p0: number, p1: number, p2: number, p3: number): Rect => [p0, p1, p2, p3]

export interface Dimensions {
  0: Dimension
  1: Dimension
  2: Dimension
  3: Dimension
  length: 4
}

export const Dimensions = (p0: Dimension, p1: Dimension, p2: Dimension, p3: Dimension): Dimensions => [p0, p1, p2, p3]

export interface Vector2f {
  0: number
  1: number
  length: 2
}

export const Vector2f = (p0: number, p1: number): Vector2f => [p0, p1]

export interface BorderRadius {
  0: number
  1: number
  2: number
  3: number
  length: 4
}

export const BorderRadius = (p0: number, p1: number, p2: number, p3: number): BorderRadius => [p0, p1, p2, p3]

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
