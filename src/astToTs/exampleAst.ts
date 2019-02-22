import { EntryT, EntryType, Scalar, T, Variant as V } from './schema/ast'

const { Tuple, Enum, Struct, Union, Newtype } = EntryType

const SurfaceId = Newtype({ name: 'SurfaceId', type: T.Scalar(Scalar.U32) })

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

const Flex = Struct({
  name: 'Flex',
  members: {
    grow: T.Scalar(Scalar.F32),
    shrink: T.Scalar(Scalar.F32),
    basis: T.RefTo(Dimension.value.name)
  }
})

const Color = Tuple({
  name: 'Color',
  fields: new Array(4).fill(T.Scalar(Scalar.F32))
})

const Vector2f = Tuple({
  name: 'Vector2f',
  fields: [T.Scalar(Scalar.F32), T.Scalar(Scalar.F32)]
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
    color: T.RefTo(Color.value.name),
    style: T.RefTo(BorderStyle.value.name)
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

const Text = Struct({
  name: 'Text',
  members: { text: T.Scalar(Scalar.Str) }
})

const SurfaceMsg = Union({
  name: 'SurfaceMsg',
  variants: [
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
    V.NewType({
      name: 'SetSize',
      type: T.RefTo(Size.value.name)
    }),
    V.NewType({
      name: 'SetFlex',
      type: T.RefTo(Flex.value.name)
    }),
    V.NewType({
      name: 'SetPadding',
      type: T.RefTo(Rect.value.name)
    }),
    V.NewType({
      name: 'SetMargin',
      type: T.RefTo(Rect.value.name)
    }),
    V.NewType({
      name: 'SetBoxShadow',
      type: T.Option(T.RefTo(BoxShadow.value.name))
    }),
    V.NewType({
      name: 'SetBackgroundColor',
      type: T.Option(T.RefTo(Color.value.name))
    }),
    V.NewType({
      name: 'SetImage',
      type: T.Option(T.RefTo(Image.value.name))
    }),
    V.NewType({
      name: 'SetText',
      type: T.Option(T.RefTo(Text.value.name))
    }),
    V.NewType({
      name: 'SetBorder',
      type: T.Option(T.RefTo(Border.value.name))
    })
  ]
})

const Msg = Union({
  name: 'Msg',
  variants: [
    V.Unit('CreateSurface'),
    V.Struct({
      name: 'SurfaceMsg',
      members: {
        surface: T.RefTo(SurfaceId.value.name),
        msg: T.RefTo(SurfaceMsg.value.name)
      }
    })
  ]
})

export const exampleEntries: EntryT[] = [
  Msg,
  SurfaceMsg,
  SurfaceId,
  Color,
  Flex,
  Dimension,
  Size,
  Rect,
  Vector2f,
  BoxShadow,
  Image,
  Text,
  Border,
  BorderSide,
  BorderStyle
]
