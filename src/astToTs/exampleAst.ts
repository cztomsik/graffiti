import { EntryT, EntryType, Scalar, T, Variant as V } from './schema/ast'

const { Tuple, Enum, Struct, Union, Newtype, Alias } = EntryType

const SurfaceId = Alias({ name: 'SurfaceId', type: T.Scalar(Scalar.U16) })

const Dimension = Union({
  name: 'Dimension',
  variants: [
    V.Unit('Auto'),
    V.NewType({ name: 'Point', type: T.Scalar(Scalar.F32) }),
    V.NewType({ name: 'Percent', type: T.Scalar(Scalar.F32) })
  ]
})

const Size = Tuple({
  name: 'Size',
  fields: [T.RefTo(Dimension.value.name), T.RefTo(Dimension.value.name)]
})

const Rect = Tuple({
  name: 'Rect',
  fields: [
    T.RefTo(Dimension.value.name),
    T.RefTo(Dimension.value.name),
    T.RefTo(Dimension.value.name),
    T.RefTo(Dimension.value.name)
  ]
})

const FlexAlign = Enum({ name: 'FlexAlign', variants: ['Auto', 'FlexStart', 'Center', 'FlexEnd', 'Stretch', 'Baseline', 'SpaceBetween', 'SpaceAround'] })

const JustifyContent = Enum({ name: 'JustifyContent', variants: ['FlexStart', 'Center', 'FlexEnd', 'SpaceBetween', 'SpaceAround', 'SpaceEvenly'] })

const FlexDirection = Enum({ name: 'FlexDirection', variants: ['Column', 'ColumnReverse', 'Row', 'RowReverse'] })

const FlexWrap = Enum({ name: 'FlexWrap', variants: ['NoWrap', 'Wrap', 'WrapReverse'] })

const Flow = Struct({
  name: 'Flow',
  members: {
    flexDirection: T.RefTo(FlexDirection.value.name),
    flexWrap: T.RefTo(FlexWrap.value.name),
    alignContent: T.RefTo(FlexAlign.value.name),
    alignItems: T.RefTo(FlexAlign.value.name),
    justifyContent: T.RefTo(JustifyContent.value.name),
  }
})

const Flex = Struct({
  name: 'Flex',
  members: {
    flexGrow: T.Scalar(Scalar.F32),
    flexShrink: T.Scalar(Scalar.F32),
    flexBasis: T.RefTo(Dimension.value.name)
  }
})

const Color = Tuple({
  name: 'Color',
  fields: new Array(4).fill(T.Scalar(Scalar.U8))
})

const Vector2f = Tuple({
  name: 'Vector2f',
  fields: [T.Scalar(Scalar.F32), T.Scalar(Scalar.F32)]
})

const BorderRadius = Tuple({
  name: 'BorderRadius',
  fields: new Array(4).fill(T.Scalar(Scalar.F32))
})

const BoxShadow = Struct({
  name: 'BoxShadow',
  members: {
    color: T.RefTo(Color.value.name),
    offset: T.RefTo(Vector2f.value.name),
    blur: T.Scalar(Scalar.F32),
    spread: T.Scalar(Scalar.F32)
  }
})

const BorderStyle = Enum({ name: 'BorderStyle', variants: ['None', 'Solid'] })

const BorderSide = Struct({
  name: 'BorderSide',
  members: {
    width: T.Scalar(Scalar.F32),
    style: T.RefTo(BorderStyle.value.name),
    color: T.RefTo(Color.value.name)
  }
})

const Border = Struct({
  name: 'Border',
  members: {
    top: T.RefTo(BorderSide.value.name),
    right: T.RefTo(BorderSide.value.name),
    bottom: T.RefTo(BorderSide.value.name),
    left: T.RefTo(BorderSide.value.name)
  }
})

const Image = Struct({
  name: 'Image',
  members: { url: T.Scalar(Scalar.Str) }
})

// TODO: font family/query, weight, size
const Text = Struct({
  name: 'Text',
  members: {
    color: T.RefTo(Color.value.name),
    fontSize: T.Scalar(Scalar.F32),
    lineHeight: T.Scalar(Scalar.F32),
    text: T.Scalar(Scalar.Str)
  }
})

const Msg = Union({
  name: 'Msg',
  variants: [
    V.Unit('HandleEvents'),
    V.Unit('Alloc'),
    V.Struct({
      name: 'AppendChild',
      members: {
        parent: T.RefTo(SurfaceId.value.name),
        child: T.RefTo(SurfaceId.value.name)
      }
    }),
    V.Struct({
      name: 'InsertBefore',
      members: {
        parent: T.RefTo(SurfaceId.value.name),
        child: T.RefTo(SurfaceId.value.name),
        before: T.RefTo(SurfaceId.value.name)
      }
    }),
    V.Struct({
      name: 'RemoveChild',
      members: {
        parent: T.RefTo(SurfaceId.value.name),
        child: T.RefTo(SurfaceId.value.name)
      }
    }),
    V.Struct({
      name: 'SetBorderRadius',
      members: {
        surface: T.RefTo(SurfaceId.value.name),
        borderRadius: T.Option(T.RefTo(BorderRadius.value.name))
      }
    }),
    V.Struct({
      name: 'SetSize',
      members: {
        surface: T.RefTo(SurfaceId.value.name),
        size: T.RefTo(Size.value.name)
      }
    }),
    V.Struct({
      name: 'SetFlex',
      members: {
        surface: T.RefTo(SurfaceId.value.name),
        flex: T.RefTo(Flex.value.name)
      }
    }),
    V.Struct({
      name: 'SetFlow',
      members: {
        surface: T.RefTo(SurfaceId.value.name),
        flow: T.RefTo(Flow.value.name)
      }
    }),
    V.Struct({
      name: 'SetPadding',
      members: {
        surface: T.RefTo(SurfaceId.value.name),
        padding: T.RefTo(Rect.value.name)
      }
    }),
    V.Struct({
      name: 'SetMargin',
      members: {
        surface: T.RefTo(SurfaceId.value.name),
        margin: T.RefTo(Rect.value.name)
      }
    }),
    V.Struct({
      name: 'SetBoxShadow',
      members: {
        surface: T.RefTo(SurfaceId.value.name),
        boxShadow: T.Option(T.RefTo(BoxShadow.value.name))
      }
    }),
    V.Struct({
      name: 'SetBackgroundColor',
      members: {
        surface: T.RefTo(SurfaceId.value.name),
        color: T.Option(T.RefTo(Color.value.name))
      }
    }),
    V.Struct({
      name: 'SetImage',
      members: {
        surface: T.RefTo(SurfaceId.value.name),
        image: T.Option(T.RefTo(Image.value.name))
      }
    }),
    V.Struct({
      name: 'SetText',
      members: {
        surface: T.RefTo(SurfaceId.value.name),
        text: T.Option(T.RefTo(Text.value.name))
      }
    }),
    V.Struct({
      name: 'SetBorder',
      members: {
        surface: T.RefTo(SurfaceId.value.name),
        border: T.Option(T.RefTo(Border.value.name))
      }
    }),
    V.Struct({
      name: 'Render',
      members: {
        surface: T.RefTo(SurfaceId.value.name),
      }
    })
  ]
})

export const exampleEntries: EntryT[] = [
  Msg,
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
  Vector2f,
  BorderRadius,
  BoxShadow,
  Image,
  Text,
  Border,
  BorderSide,
  BorderStyle
]
