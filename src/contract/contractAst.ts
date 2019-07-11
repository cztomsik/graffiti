import { EntryT, EntryType, T, Variant as V } from 'ts-rust-bridge-codegen'

const { Tuple, Enum, Struct, Union, Newtype, Alias } = EntryType

const SurfaceId = Alias('SurfaceId', T.Scalar.USIZE)

const Dimension = Union(
  'Dimension',
  [
    V.Unit('Auto'),
    V.NewType('Point', T.Scalar.F32),
    V.NewType('Percent', T.Scalar.F32)
  ],
  { tagAnnotation: false }
)

const Size = Tuple('Size', [T.RefTo(Dimension), T.RefTo(Dimension)])

const Rect = Tuple('Rect', [
  T.Scalar.F32,
  T.Scalar.F32,
  T.Scalar.F32,
  T.Scalar.F32
])

const Dimensions = Tuple('Dimensions', [
  T.RefTo(Dimension),
  T.RefTo(Dimension),
  T.RefTo(Dimension),
  T.RefTo(Dimension)
])

const FlexAlign = Enum('FlexAlign', {
  variants: [
    'Auto',
    'FlexStart',
    'Center',
    'FlexEnd',
    'Stretch',
    'Baseline',
    'SpaceBetween',
    'SpaceAround'
  ]
})

const JustifyContent = Enum('JustifyContent', {
  variants: [
    'FlexStart',
    'Center',
    'FlexEnd',
    'SpaceBetween',
    'SpaceAround',
    'SpaceEvenly'
  ]
})

const FlexDirection = Enum('FlexDirection', {
  variants: ['Column', 'ColumnReverse', 'Row', 'RowReverse']
})

const FlexWrap = Enum('FlexWrap', {
  variants: ['NoWrap', 'Wrap', 'WrapReverse']
})

const Flow = Struct('Flow', {
  flexDirection: T.RefTo(FlexDirection),
  flexWrap: T.RefTo(FlexWrap),
  alignContent: T.RefTo(FlexAlign),
  alignItems: T.RefTo(FlexAlign),
  alignSelf: T.RefTo(FlexAlign),
  justifyContent: T.RefTo(JustifyContent)
})

const Flex = Struct('Flex', {
  flexGrow: T.Scalar.F32,
  flexShrink: T.Scalar.F32,
  flexBasis: T.RefTo(Dimension)
})

const Color = Tuple('Color', new Array(4).fill(T.Scalar.U8))

const Vector2f = Tuple('Vector2f', [T.Scalar.F32, T.Scalar.F32])

const BorderRadius = Tuple('BorderRadius', new Array(4).fill(T.Scalar.F32))

const BoxShadow = Struct('BoxShadow', {
  color: T.RefTo(Color),
  offset: T.RefTo(Vector2f),
  blur: T.Scalar.F32,
  spread: T.Scalar.F32
})

const BorderStyle = Enum('BorderStyle', { variants: ['None', 'Solid'] })

const BorderSide = Struct('BorderSide', {
  width: T.Scalar.F32,
  style: T.RefTo(BorderStyle),
  color: T.RefTo(Color)
})

const Border = Struct('Border', {
  top: T.RefTo(BorderSide),
  right: T.RefTo(BorderSide),
  bottom: T.RefTo(BorderSide),
  left: T.RefTo(BorderSide)
})

const Image = Struct('Image', { url: T.Scalar.Str })

const TextAlign = Enum('TextAlign', { variants: ['Left', 'Center', 'Right'] })

// TODO: font family/query, weight, size
const Text = Struct('Text', {
  color: T.RefTo(Color),
  fontSize: T.Scalar.F32,
  lineHeight: T.Scalar.F32,
  align: T.RefTo(TextAlign),
  text: T.Scalar.Str
})

const Overflow = Enum('Overflow', {
  variants: [
    'Visible',
    'Hidden',
    'Scroll'
  ]
})

const StyleProp = Union(
  'StyleProp',
  [
    V.NewType('Size', T.RefTo(Size)),
    V.NewType('Flex', T.RefTo(Flex)),
    V.NewType('Flow', T.RefTo(Flow)),
    V.NewType('Padding', T.RefTo(Dimensions)),
    V.NewType('Margin', T.RefTo(Dimensions)),
    V.NewType('BorderRadius', T.Option(T.RefTo(BorderRadius))),
    V.NewType('Border', T.Option(T.RefTo(Border))),
    V.NewType('BoxShadow', T.Option(T.RefTo(BoxShadow))),
    V.NewType('BackgroundColor', T.Option(T.RefTo(Color))),
    V.NewType('Image', T.Option(T.RefTo(Image))),
    V.NewType('Text', T.Option(T.RefTo(Text))),
    V.NewType('Overflow', T.RefTo(Overflow)),
  ],
  { tagAnnotation: false }
)

const UpdateSceneMsg = Union(
  'UpdateSceneMsg',
  [
    V.Unit('Alloc'),
    V.Struct('InsertAt', {
      parent: T.RefTo(SurfaceId),
      child: T.RefTo(SurfaceId),
      index: T.Scalar.USIZE
    }),
    V.Struct('RemoveChild', {
      parent: T.RefTo(SurfaceId),
      child: T.RefTo(SurfaceId)
    }),
    V.Struct('SetStyleProp', {
      surface: T.RefTo(SurfaceId),
      prop: T.RefTo(StyleProp)
    }),
  ],
  { tagAnnotation: false }
)

const WindowId = Alias('WindowId', T.Scalar.U16)

// WIP
const WindowEvent = Union(
  'WindowEvent',
  [
    V.Struct('MouseMove', {
      target: T.Scalar.USIZE
    }),
    V.Struct('MouseDown', {
      target: T.Scalar.USIZE
    }),
    V.Struct('MouseUp', {
      target: T.Scalar.USIZE
    }),
    V.Struct('Scroll', {
      target: T.Scalar.USIZE
    }),

    V.NewType('KeyDown', T.Scalar.U16),
    V.NewType('KeyPress', T.Scalar.U16),
    V.NewType('KeyUp', T.Scalar.U16),

    V.Unit('Focus'),
    V.Unit('Blur'),

    V.Unit('Resize'),
    V.Unit('Close'),

    // TODO: temp
    V.Unit('Unknown')
  ],
  { tagAnnotation: false }
)

const Event = Union(
  'Event',
  [
    V.Struct('WindowEvent', {
      window: T.RefTo(WindowId),
      event: T.RefTo(WindowEvent)
    })
  ],
  { tagAnnotation: false }
)

const FfiResult = Union(
  'FfiResult',
  [
    V.Unit('Nothing'),
    V.NewType('Error', T.Scalar.Str),
    V.NewType('Events', T.Vec(T.RefTo(Event))),
    V.NewType('WindowId', T.RefTo(WindowId))
  ],
  { tagAnnotation: false }
)

const FfiMsg = Union(
  'FfiMsg',
  [
    V.NewType('GetEvents', T.Scalar.Bool),
    V.Unit('CreateWindow'),
    V.Struct('UpdateScene', {
      window: T.RefTo(WindowId),
      msgs: T.Vec(T.RefTo(UpdateSceneMsg))
    })
  ],
  { tagAnnotation: false }
)

export const exampleEntries: EntryT[] = [
  FfiMsg,
  FfiResult,
  Event,
  WindowEvent,
  UpdateSceneMsg,
  StyleProp,
  WindowId,
  SurfaceId,
  Color,
  FlexDirection,
  FlexWrap,
  FlexAlign,
  JustifyContent,
  Flow,
  Flex,
  Dimension,
  Overflow,
  Size,
  Rect,
  Dimensions,
  Vector2f,
  BorderRadius,
  BoxShadow,
  Image,
  TextAlign,
  Text,
  Border,
  BorderSide,
  BorderStyle
]
