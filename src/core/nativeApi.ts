import * as os from 'os';

// require() would make ncc bundle some unnecessary build artifacts
process['dlopen'](module, `${__dirname}/../../libgraffiti/target/libgraffiti.node`)

export const send = (msg) => {
  console.log('send', require('util').inspect(msg, { depth: 4 }))

  // send (sync)
  exports['nativeSend'](msg)

  // TODO: res
  // nodejs extension can throw too so maybe everything could be done there
  return { events: [] }

  /*
  const res = JSON.parse(resBuf.toString('utf-8'))

  if (res.error) {
    throw new Error(res.error)
  }

  return res
  */
}

export const ApiMsg = {
  CreateWindow: (width, height) => [0, width, height],
  GetEvents: (poll) => [1, poll],
  UpdateScene: (window, changes) => [2, window, changes]
}

export const Dimension = {
  UNDEFINED: [0],
  AUTO: [1],
  Points: (points) => [2, points],
  Percent: (percent) => [3, percent],
}

export enum FlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}

export enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

export enum AlignProp {
    AlignContent,
    AlignItems,
    AlignSelf,
    JustifyContent,
}

export enum Align {
    Auto,
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

export enum TextAlign {
    Left,
    Center,
    Right,
}

export enum DimensionProp {
    Width,
    Height,
    MinWidth,
    MinHeight,
    MaxWidth,
    MaxHeight,

    PaddingLeft,
    PaddingRight,
    PaddingTop,
    PaddingBottom,

    MarginLeft,
    MarginRight,
    MarginTop,
    MarginBottom,

    FlexGrow,
    FlexShrink,
    FlexBasis,
}
