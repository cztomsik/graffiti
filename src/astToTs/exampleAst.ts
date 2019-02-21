import { EntryT, EntryType, Scalar, T, Variant as V } from './schema/ast'

const { Tuple, Enum, Struct, Union, Newtype } = EntryType

const Msg = Union({
  name: 'Msg',
  variants: [
    V.Unit('Hello'),
    V.NewType({ name: 'World', type: T.Scalar(Scalar.Str) }),
    /*
    V.Tuple({
      name: 'You',
      fields: [T.Scalar(Scalar.U32), T.Scalar(Scalar.Bool)]
    }),
    */
    V.Struct({
      name: 'All',
      members: { people: T.Scalar(Scalar.Str) }
    })
  ]
})

const SurfaceId = Newtype({ name: 'SurfaceId', type: T.Scalar(Scalar.U32) })

const X = Union({
  name: 'X',
  variants: [
    V.Struct({
      name: 'AppendChild',
      members: {
        parent: T.RefTo(SurfaceId.value.name),
        child: T.RefTo(SurfaceId.value.name)
      }
    }),
    V.Struct({
      name: 'RemoveChild',
      members: {
        parent: T.RefTo(SurfaceId.value.name),
        child: T.RefTo(SurfaceId.value.name)
      }
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

const SurfaceCanHave = Struct({
  name: 'SurfaceCanHave',
  members: {
    borderRadius: T.Scalar(Scalar.F32),
    boxShadow: T.Option(T.RefTo(BoxShadow.value.name)),
    backgroundColor: T.Option(T.RefTo(Color.value.name)),
    backgroundImage: T.Option(T.RefTo(Image.value.name)),
    text: T.Option(T.RefTo(Text.value.name)),
    border: T.Option(T.RefTo(Border.value.name))
  }
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
