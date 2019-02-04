# Proposal/Ideas about component and style API

## Make react components be unaware of native api calls

### Why that matters?

I think it makes sense to have as minimal and predictable api surface area as possible when we cross domains (in that case js -> rust).

That includes:

1. Having as small as possible the number of exposed native functions.

Good example of api design is `deno` (it literally two functions: `send` + `recieve`). The idea is that a function represents a generic action which is specialized in the payload. Usually with discriminated unions.

Bad example: Node api. Both new napi and old-school native bindings.

Generally having a small set of functions makes it really easy to add stuff such as analytics (smaller number to instrument) + makes it easier to maintain.

2. Predictable api. That means that making right decisions is easy and making wrong decisions is hard.

One of the technics to achieve this is to reduce implicit state in api design. In that case the order of calls start to matter less.

Example:

```ts
const imageId = native.registerImage(imageData)
native.renderImage(imageId)
```

vs

```ts
native.renderImage(imageData)
```

And, yes, in this case registering an image first totally makes sense. But I wanted to demonstrate how stateless api makes life easier.

Examples from our codebase:

StyleSheet has a cache of flexstyles/brushes. That makes it hard to tell from both sides (js & native) what is the lifecycle of the resource.

### Make <surface> react component use js object instead of native resource handles

Let's start by splitting layout and visual information:

```ts
type Layout = {
  flex?: number
  margin?: number
  padding?: number
  width: number
  height: number
  // marginRight:
  // borderRight etc
  // NOTE: this is visual agnostic
}

type Visual = {
  backgroundColor: string
  borderRadius: string
  borderColorRight: string
  // etc
  // NOTE: this is layout agnostic
}
```

NOTE it is possible that we need an explicit Clip property too

Example:

```ts
<View style={{width:10, height:10, backgroundColor: 'red'}}>

// will be translated into

<surface layout={{width:10, height:10}} visual={{backgroundColor: 'red'}}>
```

Questions:

1.  What about caching styles
2.  How to efficiently reconcile?
3.  But when do we create native objects?

### `StyleSheet.create` api to the rescue

Let's introduce a more efficient form of layout & visual

```ts
type CompiledLayout = {
hash: number // potentially can be a string too
values: number[] // or Float32Array if possible, not sure about 50% values
}

type CompiledVisual = {
hash: number // can be either a hash or just concateneted string
values: string[] // some efficient storage form of colors, radiouses etc
}

type CompiledStyle = {
tag: 'this is compiled style'
layout?: CompiledLayout
visual?: CompiledVisual
}

const StyleSheet = {
create: (style): CompiledStyle => ({...})
}
```

#### To address 1 & 2 maybe make `<surface >` accept only compiled styles?

Pros:

1. Simple. There is no if/else anywhere. Always predictable shape.
2. Better abstraction boundary. Anything below <surface> component api doesn't care about raw style object `{width: '100%'}`
3. Super cheap to compare compiled styles (`layout1.hash === layout2.hash`)

Cons:

1. Might require an extra allocation/computation to compare props in object literals
   Example:

```tsx
// prev props
<View style={{flex:1, backgroundColor: 'red' }}/>

// new props
<View style={{flex:1, backgroundColor: 'blue' }}/>
```

We had compiled props from the previous step, but we don't have compiled props from the new step yet.

If `<surface>` accepts _only_ compiled props we have to compile it first to compare. Not sure if this is a big deal, though.

In that case the code for `<View>` might look something like this

```tsx
const View: SFC<ViewProps> = ({ style }) => {
  if (style.tag !== 'this is compiled style') {
    // NOTE that happens in each render
    const { layout, visual } = StyleSheet.compileStyleObject(style)

    return <surface layout={layout} visual={visual} />
  }
  // here we know that it is already been compiled
  const { layout, visual } = style
  return <surface layout={layout} visual={visual} />
}
```

Note: I will omit style composition for simplicity. Ex:

```tsx
<View style={[someStyle, { backgroundColor: active ? 'red' : blue }]} />
```

But I think it is totally doable to achieve good performance in any variant

### Defer compilation until later

It is possible to do almost no work during render. Let's expand our `CompiledStyle` definition

```ts
type StyleVariant =
  | {
      tag: 'this is compiled style'
      layout?: CompiledLayout
      visual?: CompiledVisual
    }
  | { tag: 'raw style object'; style: ViewStyleProp }
```

```tsx
const View: SFC<ViewProps> = ({ style }) => {
  const styleVariant =
    style.tag !== 'this is compiled style'
      ? style
      : {
          tag: 'raw style object',
          style
        }

  return <surface styleVariant={styleVariant} />
}
```

And then internally inside surface implementation we will compare styles. Note in this case we can compare compiled vs raw style objects.

Example:

```tsx
<View style={{flex:1, backgroundColor: 'red' }}/>

// vs

const styles = StyleSheet.create({viewStyle: {flex:1, backgroundColor: 'red' }})

<View style={styles.viewStyle}/>
```

Personally I would prefer to compile all the time and then measure perf. If compiling is a slow process then move to this option.

## Ok we have a `<surface>` with compiled layout/visual. What next?

How to actually send this data to native?

### Option 1. Have a mapping `id -> native style (brush?)` in js land.

We have a global map that stores native styles.

```ts
// global mapping
const nativeStyles = new Map<Id, N.ResourceHandle>()
// and one for flex

class Surface implements Container<Surface> {
  ref: N.Surface
  nativeLayot: N.FlexStyle
  nativeBrush: N.ResourceHandle

  constructor(private layout: CompiledLayout, private visual: CompiledVisual) {
    this.ref = native.surface_create()
    update(layout, visual)
  }

  update(layout: CompiledLayout, visual: CompiledVisual) {
    let cached = nativeStyles.get(visual.hash)
    if (cached) {
      this.nativeBrush = cached
    } else {
      this.nativeBrush = cached = native.op_resource_create(
        JSON.stringify(toOpRes(visual))
      )
      nativeStyles.set(visual.hash, cached)
    }
    native.surface_update_visual(this.ref, cached)

    // similar logic for layout
  }

  compare(layout: CompiledLayout, visual: CompiledVisual): boolean {
    // compare this.layout.hash with layout.hash
  }
  // rest of the impl
}
```

Pros:

1. Supposed to be efficient. We can reuse native styles for multiple surfaces
2. In this example `cached` can be something really simple (number?)

Cons:

1. We never "free" resources. It is possible to introduce something like refcount but can be cumbersome and error prone.
2. Complex. Essentially we are "leaking" internals of the native implementation.

### Option 2. Pass compiled styles to rust side.

The main goal is eliminate caching in js land altogether.

First, we don't need hash anymore.

```ts
type CompiledLayout = {
  // I think we can encode that as just floats
  // example: {width: 10, height: '10%'} -> [0,10,1,0.1]
  // width 10 -> [0 - means float value, 10 - value itself ]
  // height 10 -> [1 - means percentage value, 0.1 - value itself ]
  values: Float32Array
}

type CompiledVisual = {
  values: Float32Array // same here
}

// result of StyleSheet.create is exactly the same
type CompiledStyle = {
  tag: 'this is compiled style'
  layout?: CompiledLayout
  visual?: CompiledVisual
}
```

The reason for why we can do this is that we don't keep these styles in a map anymore.

The diffing part is pretty straightforward:

```ts
// either it is the same reference (came from StyleSheet.create)
// or all values have to match exactly
const compareVisual = (a: CompiledVisual, b: CompiledVisual): boolean =>
  a === b || a.reduce((eq, el, i) => eq && a[i] === b[i], true)
```

Now let's take a look at how the `Surface` might look like after that:

```ts
class Surface implements Container<Surface> {
  ref: N.Surface

  // in reality visual can be optional
  constructor(private layout: CompiledLayout, private visual: CompiledVisual) {
    // here it is raw bytearrays that are being passed in
    this.ref = native.surface_create(layout.buffer, visual.buffer)
  }

  update(layout: CompiledLayout, visual: CompiledVisual) {
    this.layout = layout
    this.visual = visual
    native.surface_update(this.ref, layout.buffer, visual.buffer)
  }

  compare(layout: CompiledLayout, visual: CompiledVisual): boolean {
    return (
      compareVisual(this.visual, visual) && compareLayout(this.layout, layout)
    )
  }
  // rest of the impl
}
```

I think it looks pretty clean :)

Pros:

1. It simplifies resource management on js side (by a lot!)
2. It reduces coupling with native api. There is no need for `op_resource_create` anymore.
3. Plays nicely with `StyleSheet.create` api.
4. Still allows style caching but only on native side.
5. It is easy to compose different properties on react components. Ex: `<image layout={layout} imageId={155} />`.

Cons:

1. We pass buffers instead of numbers if layout/style changes. I think it is worth it though. Note that there is no `JSON.stringify` call anymore (better perf).
2. The api itself might feel more rigid. Instead of `OpResouce` operations we pass explicit values for visuals, layouts etc.
3. We need to keep in sync encoding/decoding of `CompiledLayout` between js and rust.

Overall, I think that this option has the best tradeoffs. It is simple, efficient (diffing & sending to native) and reduces api surface significantly.

## Summary

I do want to remove coupling with native from `StyleSheet` and react components like `<surface>`.

In this document I explored two ways how we can do that:

1. Split caching of native resources into two parts:

- A consistent hashing of layout/visuals into a string or number. `StyleSheet` will _only_ know about that part.
- A map `hash -> native resource`. Surface will operate on this level. Note: that this would require more granularity of native resources (we loose ability to batch a lot of resources together under a single `N.ResourceHandle`)

2. Remove caching from js side. That would require:

- An efficient (speed + memory) representation of layout/visual. I think `Float32Array` is a good fit.
- We send the entire style buffer when it changes (and only when it changes thanks to reconciler). I think it is not a big deal. Size estimation for layout: 26 values x 4 bytes x 2 (each encodes type + value) ~ 200 bytes. Which is pretty much nothing :)
- keep in sync encoding/decoding these buffers in js/rust. Note we do have to maintain it even now but with the json protocol instead of binary one.

As I mentioned earlier I'm heavily biased towards the 2nd.

Why?

> Having as small as possible the number of exposed native functions.

It reduces api surface area by a lot. We don't need to expose things such as `RenderOperation` type & `RenderOp.PopStackingContext()` helpers.

> Predictable api. That means that making right decisions is easy and making wrong decisions is hard.

We eliminate a lot of state from the api (pass buffers instead of handles), but also eliminate caching of resources from js as well and don't have to worry about their lifecycle!

@Simon.
