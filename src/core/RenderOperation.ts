import { N } from './nativeApi'

// r g b a, all values are between (0,1)
export type BridgeColor = [number, number, number, number]

// pub enum RenderOperation {
//   // this was hack at first but it could be useful for hitSlop (hitBox can be bigger than clipBox)
//   HitTest(u32),
//   SaveRect,
//   PushScrollClip,
//   PushBorderRadiusClip(f32),
//   PopClip,
//   Rectangle(RectangleDisplayItem),
//   Border(BorderDisplayItem),
//   Text(TextDisplayItem, Vec<GlyphInstance>),
//   PopStackingContext,
//   PushStackingContext(PushStackingContextDisplayItem),
// }

type HitTest = { HitTest: number }
type SaveRect = { SaveRect: null }
type PushScrollClip = { PushScrollClip: number }
type PushBorderRadiusClip = { PushBorderRadiusClip: number }
type PopClip = { PopClip: null }
type Rectangle = { Rectangle: RectangleDisplayItem }
type Border = { Border: BorderDisplayItem }
type Text = { Text: TextPayload }
type Image = { Image: N.ImageId }
type PopStackingContext = { PopStackingContext: null }
type PushStackingContext = {
  PushStackingContext: PushStackingContextDisplayItem
}

export type RenderOperation =
  | HitTest
  | SaveRect
  | PushScrollClip
  | PushBorderRadiusClip
  | PopClip
  | Rectangle
  | Border
  | Text
  | Image
  | PopStackingContext
  | PushStackingContext

export const RenderOp = {
  HitTest: (id: number): HitTest => ({ HitTest: id }),
  SaveRect: (): SaveRect => ({ SaveRect: null }),
  PushScrollClip: (id: number): PushScrollClip => ({ PushScrollClip: id }),
  PushBorderRadiusClip: (radius: number): PushBorderRadiusClip => ({
    PushBorderRadiusClip: radius
  }),
  PopClip: (): PopClip => ({ PopClip: null }),
  Rectangle: (color: BridgeColor): Rectangle => ({ Rectangle: { color } }),
  Border: (params: BorderDisplayItem): Border => ({ Border: params }),
  Text: (textParams: TextDisplayItem, glyphs: GlyphInstance[]): Text => ({
    Text: [textParams, glyphs]
  }),
  PopStackingContext: (): PopStackingContext => ({ PopStackingContext: null }),
  PushStackingContext: (ctx: StackingContext): PushStackingContext => ({
    PushStackingContext: { stacking_context: ctx }
  }),
  Image: (id: N.ImageId): Image => ({ Image: id })
}

// PushStackingContext-------
type PushStackingContextDisplayItem = {
  stacking_context: StackingContext
}

type StackingContext = {
  transform_style: TransformStyle
  mix_blend_mode: MixBlendMode
  raster_space: 'Screen' | { Local: number }
}

export enum TransformStyle {
  Flat = 'Flat',
  Preserve3D = 'Preserve3D'
}

export enum MixBlendMode {
  Normal = 'Normal',
  Multiply = 'Multiply ',
  Screen = 'Screen',
  Overlay = 'Overlay ',
  Darken = 'Darken',
  Lighten = 'Lighten ',
  ColorDodge = 'ColorDodge ',
  ColorBurn = 'ColorBurn',
  HardLight = 'HardLight ',
  SoftLight = 'SoftLight ',
  Difference = 'Difference ',
  Exclusion = 'Exclusion ',
  Hue = 'Hue',
  Saturation = 'Saturation ',
  Color = 'Color',
  Luminosity = 'Luminosity '
}

// Text ---------------------
type TextPayload = [TextDisplayItem, GlyphInstance[]]

// index: GlyphIndex, point: LayoutPoint [x, y]
export type GlyphInstance = [number, [number, number]]
type TextDisplayItem = {
  font_key: [
    // namespace
    number,
    // font size
    number
  ]
  color: BridgeColor
}

// Rectangle -----------------
type RectangleDisplayItem = {
  color: BridgeColor
}

// Border --------------------
export type BorderDisplayItem = {
  // [TopWidth, RightWidth, BottomWidth, LeftWidth]
  widths: [number, number, number, number]
  details: BorderDetails
}

export enum BorderStyle {
  None = 'None',
  Solid = 'Solid',
  Double = 'Double',
  Dotted = 'Dotted',
  Dashed = 'Dashed',
  Hidden = 'Hidden',
  Groove = 'Groove',
  Ridge = 'Ridge',
  Inset = 'Inset',
  Outset = 'Outset'
}

type BorderSide = { color: BridgeColor; style: BorderStyle }

// width, height
type LayoutSize = [number, number]

type BorderDetails = {
  Normal: {
    left: BorderSide
    right: BorderSide
    top: BorderSide
    bottom: BorderSide
    radius: {
      top_left: LayoutSize
      top_right: LayoutSize
      bottom_left: LayoutSize
      bottom_right: LayoutSize
    }
    do_aa: boolean
  }
}
// https://doc.servo.org/webrender_api/struct.NinePatchBorder.html
//| { NinePatch(NinePatchBorder) }
