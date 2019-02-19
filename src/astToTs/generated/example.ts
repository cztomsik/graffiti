export type Msg =
  | { tag: 'Hello' }
  | { tag: 'World'; value: string }
  | { tag: 'You'; value: [number, boolean] }
  | { tag: 'All'; value: MsgAll }
export type X =
  | { tag: 'AppendChild'; value: XAppendChild }
  | { tag: 'RemoveChild'; value: XRemoveChild }
export type SurfaceId = number & { type: 'SurfaceId' }

export interface MsgAll {
  people: string
}

export interface XAppendChild {
  parent: SurfaceId
  child: SurfaceId
}

export interface XRemoveChild {
  parent: SurfaceId
  child: SurfaceId
}

export interface SurfaceCanHave {
  borderRadius: number
  boxShadow: (BoxShadow) | undefined
  backgroundColor: (Color) | undefined
  backgroundImage: (Image) | undefined
  text: (Text) | undefined
  border: (Border) | undefined
}

export interface Color {
  0: number
  1: number
  2: number
  3: number
  length: 4
}

export interface Vector2f {
  0: number
  1: number
  length: 2
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

export interface Text {
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
  color: Color
  style: BorderStyle
}

export enum BorderStyle {
  None,
  Solid
}

export function mkSurfaceId(val: number): number & { type: 'SurfaceId' } {
  return val as any
}

export function mkColor(p0: number, p1: number, p2: number, p3: number): Color {
  return [p0, p1, p2, p3]
}

export function mkVector2f(p0: number, p1: number): Vector2f {
  return [p0, p1]
}
