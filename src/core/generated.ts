export type Msg = 
    | { tag: "HandleEvents"}
    | { tag: "Alloc"}
    | { tag: "AppendChild", value: MsgAppendChild}
    | { tag: "InsertBefore", value: MsgInsertBefore}
    | { tag: "RemoveChild", value: MsgRemoveChild}
    | { tag: "SetBorderRadius", value: MsgSetBorderRadius}
    | { tag: "SetSize", value: MsgSetSize}
    | { tag: "SetFlex", value: MsgSetFlex}
    | { tag: "SetFlow", value: MsgSetFlow}
    | { tag: "SetPadding", value: MsgSetPadding}
    | { tag: "SetMargin", value: MsgSetMargin}
    | { tag: "SetBoxShadow", value: MsgSetBoxShadow}
    | { tag: "SetBackgroundColor", value: MsgSetBackgroundColor}
    | { tag: "SetImage", value: MsgSetImage}
    | { tag: "SetText", value: MsgSetText}
    | { tag: "SetBorder", value: MsgSetBorder}
    | { tag: "Render"}
;
export type SurfaceId = number;
export type Dimension = 
    | { tag: "Auto"}
    | { tag: "Point", value: number}
    | { tag: "Percent", value: number}
;

export interface MsgAppendChild {
    parent: SurfaceId;
    child: SurfaceId;
}

export interface MsgInsertBefore {
    parent: SurfaceId;
    child: SurfaceId;
    before: SurfaceId;
}

export interface MsgRemoveChild {
    parent: SurfaceId;
    child: SurfaceId;
}

export interface MsgSetBorderRadius {
    surface: SurfaceId;
    borderRadius: (BorderRadius) | undefined;
}

export interface MsgSetSize {
    surface: SurfaceId;
    size: Size;
}

export interface MsgSetFlex {
    surface: SurfaceId;
    flex: Flex;
}

export interface MsgSetFlow {
    surface: SurfaceId;
    flow: Flow;
}

export interface MsgSetPadding {
    surface: SurfaceId;
    padding: Rect;
}

export interface MsgSetMargin {
    surface: SurfaceId;
    margin: Rect;
}

export interface MsgSetBoxShadow {
    surface: SurfaceId;
    boxShadow: (BoxShadow) | undefined;
}

export interface MsgSetBackgroundColor {
    surface: SurfaceId;
    color: (Color) | undefined;
}

export interface MsgSetImage {
    surface: SurfaceId;
    image: (Image) | undefined;
}

export interface MsgSetText {
    surface: SurfaceId;
    text: (Text) | undefined;
}

export interface MsgSetBorder {
    surface: SurfaceId;
    border: (Border) | undefined;
}

export interface Color {
    0: number;
    1: number;
    2: number;
    3: number;
    length: 4;
}

export interface Flow {
    flexDirection: FlexDirection;
    flexWrap: FlexWrap;
    alignContent: FlexAlign;
    alignItems: FlexAlign;
    justifyContent: JustifyContent;
}

export interface Flex {
    flexGrow: number;
    flexShrink: number;
    flexBasis: Dimension;
}

export interface Size {
    0: Dimension;
    1: Dimension;
    length: 2;
}

export interface Rect {
    0: Dimension;
    1: Dimension;
    2: Dimension;
    3: Dimension;
    length: 4;
}

export interface Vector2f {
    0: number;
    1: number;
    length: 2;
}

export interface BorderRadius {
    0: number;
    1: number;
    2: number;
    3: number;
    length: 4;
}

export interface BoxShadow {
    color: Color;
    offset: Vector2f;
    blur: number;
    spread: number;
}

export interface Image {
    url: string;
}

export interface Text {
    color: Color;
    fontSize: number;
    lineHeight: number;
    text: string;
}

export interface Border {
    top: BorderSide;
    right: BorderSide;
    bottom: BorderSide;
    left: BorderSide;
}

export interface BorderSide {
    width: number;
    style: BorderStyle;
    color: Color;
}

export enum FlexDirection {
    Column = "Column",
    ColumnReverse = "ColumnReverse",
    Row = "Row",
    RowReverse = "RowReverse"
}

export enum FlexWrap {
    NoWrap = "NoWrap",
    Wrap = "Wrap",
    WrapReverse = "WrapReverse"
}

export enum FlexAlign {
    Auto = "Auto",
    FlexStart = "FlexStart",
    Center = "Center",
    FlexEnd = "FlexEnd",
    Stretch = "Stretch",
    Baseline = "Baseline",
    SpaceBetween = "SpaceBetween",
    SpaceAround = "SpaceAround"
}

export enum JustifyContent {
    FlexStart = "FlexStart",
    Center = "Center",
    FlexEnd = "FlexEnd",
    SpaceBetween = "SpaceBetween",
    SpaceAround = "SpaceAround",
    SpaceEvenly = "SpaceEvenly"
}

export enum BorderStyle {
    None = "None",
    Solid = "Solid"
}

export function mkMsgHandleEvents(): Msg {
    return { tag: "HandleEvents"};
}

export function mkMsgAlloc(): Msg {
    return { tag: "Alloc"};
}

export function mkMsgAppendChild(value: MsgAppendChild): Msg {
    return { tag: "AppendChild", value};
}

export function mkMsgInsertBefore(value: MsgInsertBefore): Msg {
    return { tag: "InsertBefore", value};
}

export function mkMsgRemoveChild(value: MsgRemoveChild): Msg {
    return { tag: "RemoveChild", value};
}

export function mkMsgSetBorderRadius(value: MsgSetBorderRadius): Msg {
    return { tag: "SetBorderRadius", value};
}

export function mkMsgSetSize(value: MsgSetSize): Msg {
    return { tag: "SetSize", value};
}

export function mkMsgSetFlex(value: MsgSetFlex): Msg {
    return { tag: "SetFlex", value};
}

export function mkMsgSetFlow(value: MsgSetFlow): Msg {
    return { tag: "SetFlow", value};
}

export function mkMsgSetPadding(value: MsgSetPadding): Msg {
    return { tag: "SetPadding", value};
}

export function mkMsgSetMargin(value: MsgSetMargin): Msg {
    return { tag: "SetMargin", value};
}

export function mkMsgSetBoxShadow(value: MsgSetBoxShadow): Msg {
    return { tag: "SetBoxShadow", value};
}

export function mkMsgSetBackgroundColor(value: MsgSetBackgroundColor): Msg {
    return { tag: "SetBackgroundColor", value};
}

export function mkMsgSetImage(value: MsgSetImage): Msg {
    return { tag: "SetImage", value};
}

export function mkMsgSetText(value: MsgSetText): Msg {
    return { tag: "SetText", value};
}

export function mkMsgSetBorder(value: MsgSetBorder): Msg {
    return { tag: "SetBorder", value};
}

export function mkMsgRender(): Msg {
    return { tag: "Render"};
}

export function mkColor(p0: number, p1: number, p2: number, p3: number): Color {
    return [p0, p1, p2, p3]
}

export function mkDimensionAuto(): Dimension {
    return { tag: "Auto"};
}

export function mkDimensionPoint(value: number): Dimension {
    return { tag: "Point", value};
}

export function mkDimensionPercent(value: number): Dimension {
    return { tag: "Percent", value};
}

export function mkSize(p0: Dimension, p1: Dimension): Size {
    return [p0, p1]
}

export function mkRect(p0: Dimension, p1: Dimension, p2: Dimension, p3: Dimension): Rect {
    return [p0, p1, p2, p3]
}

export function mkVector2f(p0: number, p1: number): Vector2f {
    return [p0, p1]
}

export function mkBorderRadius(p0: number, p1: number, p2: number, p3: number): BorderRadius {
    return [p0, p1, p2, p3]
}
