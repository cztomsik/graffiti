export type FfiMsg = 
    | { tag: "HandleEvents"}
    | { tag: "CreateWindow"}
    | { tag: "UpdateScene", value: FfiMsgUpdateScene}
;
export type UpdateSceneMsg = 
    | { tag: "Alloc"}
    | { tag: "AppendChild", value: UpdateSceneMsgAppendChild}
    | { tag: "InsertBefore", value: UpdateSceneMsgInsertBefore}
    | { tag: "RemoveChild", value: UpdateSceneMsgRemoveChild}
    | { tag: "SetBorderRadius", value: UpdateSceneMsgSetBorderRadius}
    | { tag: "SetSize", value: UpdateSceneMsgSetSize}
    | { tag: "SetFlex", value: UpdateSceneMsgSetFlex}
    | { tag: "SetFlow", value: UpdateSceneMsgSetFlow}
    | { tag: "SetPadding", value: UpdateSceneMsgSetPadding}
    | { tag: "SetMargin", value: UpdateSceneMsgSetMargin}
    | { tag: "SetBoxShadow", value: UpdateSceneMsgSetBoxShadow}
    | { tag: "SetBackgroundColor", value: UpdateSceneMsgSetBackgroundColor}
    | { tag: "SetImage", value: UpdateSceneMsgSetImage}
    | { tag: "SetText", value: UpdateSceneMsgSetText}
    | { tag: "SetBorder", value: UpdateSceneMsgSetBorder}
;
export type WindowId = number;
export type SurfaceId = number;
export type Dimension = 
    | { tag: "Auto"}
    | { tag: "Point", value: number}
    | { tag: "Percent", value: number}
;

export interface FfiMsgUpdateScene {
    window: WindowId;
    msgs: Array<UpdateSceneMsg>;
}

export interface UpdateSceneMsgAppendChild {
    parent: SurfaceId;
    child: SurfaceId;
}

export interface UpdateSceneMsgInsertBefore {
    parent: SurfaceId;
    child: SurfaceId;
    before: SurfaceId;
}

export interface UpdateSceneMsgRemoveChild {
    parent: SurfaceId;
    child: SurfaceId;
}

export interface UpdateSceneMsgSetBorderRadius {
    surface: SurfaceId;
    borderRadius: (BorderRadius) | undefined;
}

export interface UpdateSceneMsgSetSize {
    surface: SurfaceId;
    size: Size;
}

export interface UpdateSceneMsgSetFlex {
    surface: SurfaceId;
    flex: Flex;
}

export interface UpdateSceneMsgSetFlow {
    surface: SurfaceId;
    flow: Flow;
}

export interface UpdateSceneMsgSetPadding {
    surface: SurfaceId;
    padding: Rect;
}

export interface UpdateSceneMsgSetMargin {
    surface: SurfaceId;
    margin: Rect;
}

export interface UpdateSceneMsgSetBoxShadow {
    surface: SurfaceId;
    boxShadow: (BoxShadow) | undefined;
}

export interface UpdateSceneMsgSetBackgroundColor {
    surface: SurfaceId;
    color: (Color) | undefined;
}

export interface UpdateSceneMsgSetImage {
    surface: SurfaceId;
    image: (Image) | undefined;
}

export interface UpdateSceneMsgSetText {
    surface: SurfaceId;
    text: (Text) | undefined;
}

export interface UpdateSceneMsgSetBorder {
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
    align: TextAlign;
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

export enum TextAlign {
    Left = "Left",
    Center = "Center",
    Right = "Right"
}

export enum BorderStyle {
    None = "None",
    Solid = "Solid"
}

export function mkFfiMsgHandleEvents(): FfiMsg {
    return { tag: "HandleEvents"};
}

export function mkFfiMsgCreateWindow(): FfiMsg {
    return { tag: "CreateWindow"};
}

export function mkFfiMsgUpdateScene(value: FfiMsgUpdateScene): FfiMsg {
    return { tag: "UpdateScene", value};
}

export function mkUpdateSceneMsgAlloc(): UpdateSceneMsg {
    return { tag: "Alloc"};
}

export function mkUpdateSceneMsgAppendChild(value: UpdateSceneMsgAppendChild): UpdateSceneMsg {
    return { tag: "AppendChild", value};
}

export function mkUpdateSceneMsgInsertBefore(value: UpdateSceneMsgInsertBefore): UpdateSceneMsg {
    return { tag: "InsertBefore", value};
}

export function mkUpdateSceneMsgRemoveChild(value: UpdateSceneMsgRemoveChild): UpdateSceneMsg {
    return { tag: "RemoveChild", value};
}

export function mkUpdateSceneMsgSetBorderRadius(value: UpdateSceneMsgSetBorderRadius): UpdateSceneMsg {
    return { tag: "SetBorderRadius", value};
}

export function mkUpdateSceneMsgSetSize(value: UpdateSceneMsgSetSize): UpdateSceneMsg {
    return { tag: "SetSize", value};
}

export function mkUpdateSceneMsgSetFlex(value: UpdateSceneMsgSetFlex): UpdateSceneMsg {
    return { tag: "SetFlex", value};
}

export function mkUpdateSceneMsgSetFlow(value: UpdateSceneMsgSetFlow): UpdateSceneMsg {
    return { tag: "SetFlow", value};
}

export function mkUpdateSceneMsgSetPadding(value: UpdateSceneMsgSetPadding): UpdateSceneMsg {
    return { tag: "SetPadding", value};
}

export function mkUpdateSceneMsgSetMargin(value: UpdateSceneMsgSetMargin): UpdateSceneMsg {
    return { tag: "SetMargin", value};
}

export function mkUpdateSceneMsgSetBoxShadow(value: UpdateSceneMsgSetBoxShadow): UpdateSceneMsg {
    return { tag: "SetBoxShadow", value};
}

export function mkUpdateSceneMsgSetBackgroundColor(value: UpdateSceneMsgSetBackgroundColor): UpdateSceneMsg {
    return { tag: "SetBackgroundColor", value};
}

export function mkUpdateSceneMsgSetImage(value: UpdateSceneMsgSetImage): UpdateSceneMsg {
    return { tag: "SetImage", value};
}

export function mkUpdateSceneMsgSetText(value: UpdateSceneMsgSetText): UpdateSceneMsg {
    return { tag: "SetText", value};
}

export function mkUpdateSceneMsgSetBorder(value: UpdateSceneMsgSetBorder): UpdateSceneMsg {
    return { tag: "SetBorder", value};
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
