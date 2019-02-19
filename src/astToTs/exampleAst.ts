import { EntryT, EntryType, Type, Scalar, T, Variant as V } from './schema/ast'

const { Tuple, Enum, Struct, Union, Newtype } = EntryType

const Msg = Union({
  name: 'Msg',
  variants: [
    V.Unit('Hello'),
    V.NewType({ name: 'World', type: T.Scalar(Scalar.Str) }),
    V.Tuple({
      name: 'You',
      fields: [T.Scalar(Scalar.U32), T.Scalar(Scalar.Bool)]
    }),
    V.Struct({
      name: 'All',
      members: [{ name: 'people', type: T.Scalar(Scalar.Str) }]
    })
  ]
})

const SurfaceId = Newtype({ name: 'SurfaceId', type: T.Scalar(Scalar.U32) })

const X = Union({
  name: 'X',
  variants: [
    V.Struct({
      name: 'AppendChild',
      members: [
        { name: 'parent', type: T.RefTo(SurfaceId.value.name) },
        { name: 'child', type: T.RefTo(SurfaceId.value.name) }
      ]
    }),
    V.Struct({
      name: 'RemoveChild',
      members: [
        { name: 'parent', type: T.RefTo(SurfaceId.value.name) },
        { name: 'child', type: T.RefTo(SurfaceId.value.name) }
      ]
    })
  ]
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
  members: [
    { name: 'color', type: T.RefTo(Color.value.name) },
    { name: 'offset', type: T.RefTo(Vector2f.value.name) },
    { name: 'blur', type: T.Scalar(Scalar.F32) },
    { name: 'spread', type: T.Scalar(Scalar.F32) }
  ]
})

const BorderStyle = Enum({ name: 'BorderStyle', variants: ['None', 'Solid'] })

const BorderSide = Struct({
  name: 'BorderSide',
  members: [
    { name: 'width', type: T.Scalar(Scalar.F32) },
    { name: 'color', type: T.RefTo(Color.value.name) },
    { name: 'style', type: T.RefTo(BorderStyle.value.name) }
  ]
})

const Border = Struct({
  name: 'Border',
  members: ['top', 'right', 'bottom', 'left'].map(edge => ({
    name: edge,
    type: T.RefTo(BorderSide.value.name)
  }))
})

const Image = Struct({
  name: 'Image',
  members: [{ name: 'url', type: T.Scalar(Scalar.Str) }]
})

const Text = Struct({
  name: 'Text',
  members: [{ name: 'text', type: T.Scalar(Scalar.Str) }]
})

const SurfaceCanHave = Struct({
  name: 'SurfaceCanHave',
  members: [
    { name: 'borderRadius', type: T.Scalar(Scalar.F32) },
    { name: 'boxShadow', type: T.Option(T.RefTo(BoxShadow.value.name)) },
    { name: 'backgroundColor', type: T.Option(T.RefTo(Color.value.name)) },
    { name: 'backgroundImage', type: T.Option(T.RefTo(Image.value.name)) },
    { name: 'text', type: T.Option(T.RefTo(Text.value.name)) },
    { name: 'border', type: T.Option(T.RefTo(Border.value.name)) }
  ]
})

export const exampleEntries: EntryT[] = [
  Msg,
  X,
  SurfaceId,
  SurfaceCanHave,
  Color,
  Vector2f,
  BoxShadow,
  Image,
  Text,
  Border,
  BorderSide,
  BorderStyle
]
