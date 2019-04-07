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

const UpdateSceneMsg = Union(
  'UpdateSceneMsg',
  [
    V.Unit('Alloc'),
    V.Struct('AppendChild', {
      parent: T.RefTo(SurfaceId),
      child: T.RefTo(SurfaceId)
    }),
    V.Struct('InsertBefore', {
      parent: T.RefTo(SurfaceId),
      child: T.RefTo(SurfaceId),
      before: T.RefTo(SurfaceId)
    }),
    V.Struct('RemoveChild', {
      parent: T.RefTo(SurfaceId),
      child: T.RefTo(SurfaceId)
    }),
    V.Struct('SetBorderRadius', {
      surface: T.RefTo(SurfaceId),
      borderRadius: T.Option(T.RefTo(BorderRadius))
    }),
    V.Struct('SetSize', {
      surface: T.RefTo(SurfaceId),
      size: T.RefTo(Size)
    }),
    V.Struct('SetFlex', {
      surface: T.RefTo(SurfaceId),
      flex: T.RefTo(Flex)
    }),
    V.Struct('SetFlow', {
      surface: T.RefTo(SurfaceId),
      flow: T.RefTo(Flow)
    }),
    V.Struct('SetPadding', {
      surface: T.RefTo(SurfaceId),
      padding: T.RefTo(Dimensions)
    }),
    V.Struct('SetMargin', {
      surface: T.RefTo(SurfaceId),
      margin: T.RefTo(Dimensions)
    }),
    V.Struct('SetBoxShadow', {
      surface: T.RefTo(SurfaceId),
      boxShadow: T.Option(T.RefTo(BoxShadow))
    }),
    V.Struct('SetBackgroundColor', {
      surface: T.RefTo(SurfaceId),
      color: T.Option(T.RefTo(Color))
    }),
    V.Struct('SetImage', {
      surface: T.RefTo(SurfaceId),
      image: T.Option(T.RefTo(Image))
    }),
    V.Struct('SetText', {
      surface: T.RefTo(SurfaceId),
      text: T.Option(T.RefTo(Text))
    }),
    V.Struct('SetBorder', {
      surface: T.RefTo(SurfaceId),
      border: T.Option(T.RefTo(Border))
    })
  ],
  { tagAnnotation: false }
)

const WindowId = Alias('WindowId', T.Scalar.U16)

const FfiMsg = Union(
  'FfiMsg',
  [
    V.NewType('GetNextEvent', T.Scalar.Bool),
    V.Unit('CreateWindow'),
    V.Struct('UpdateScene', {
      window: T.RefTo(WindowId),
      msgs: T.Vec(T.RefTo(UpdateSceneMsg))
    })
  ],
  { tagAnnotation: false }
)

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

    V.Unit('KeyDown'),
    V.NewType('KeyPress', T.Scalar.U16),
    V.Unit('KeyUp'),

    V.Unit('Focus'),
    V.Unit('Blur'),

    V.Unit('Resize'),
    V.Unit('Close'),

    // TODO: temp
    V.Unit('Unknown')
  ],
  { tagAnnotation: true }
)

const Event = Union(
  'Event',
  [
    V.Struct('WindowEvent', {
      window: T.RefTo(WindowId),
      event: T.RefTo(WindowEvent)
    })
  ],
  { tagAnnotation: true }
)

const FfiResult = Union(
  'FfiResult',
  [
    V.Unit('Nothing'),
    V.NewType('Event', T.RefTo(Event)),
    V.NewType('WindowId', T.RefTo(WindowId))
  ],
  { tagAnnotation: true }
)

export const exampleEntries: EntryT[] = [
  FfiMsg,
  FfiResult,
  Event,
  WindowEvent,
  UpdateSceneMsg,
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
