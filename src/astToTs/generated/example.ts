export type Msg = 
    | { tag: "HandleEvents"}
    | { tag: "AppendChild", value: MsgAppendChild}
    | { tag: "InsertBefore", value: MsgInsertBefore}
    | { tag: "RemoveChild", value: MsgRemoveChild}
    | { tag: "SetSize", value: MsgSetSize}
    | { tag: "SetFlex", value: MsgSetFlex}
    | { tag: "SetPadding", value: MsgSetPadding}
    | { tag: "SetMargin", value: MsgSetMargin}
    | { tag: "SetBoxShadow", value: MsgSetBoxShadow}
    | { tag: "SetBackgroundColor", value: MsgSetBackgroundColor}
    | { tag: "SetImage", value: MsgSetImage}
    | { tag: "SetText", value: MsgSetText}
    | { tag: "SetBorder", value: MsgSetBorder}
;
export type SurfaceId = number & { type: 'SurfaceId'};
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

export interface MsgSetSize {
    surface: SurfaceId;
    size: Size;
}

export interface MsgSetFlex {
    surface: SurfaceId;
    flex: Flex;
}

export interface MsgSetPadding {
    surface: SurfaceId;
    rect: Rect;
}

export interface MsgSetMargin {
    surface: SurfaceId;
    rect: Rect;
}

export interface MsgSetBoxShadow {
    surface: SurfaceId;
    rect: (BoxShadow) | undefined;
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

export interface Flex {
    grow: number;
    shrink: number;
    basis: Dimension;
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

export enum BorderStyle {
    None,
    Solid
}

export function mkMsgHandleEvents(): Msg {
    return { tag: "HandleEvents"};
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

export function mkMsgSetSize(value: MsgSetSize): Msg {
    return { tag: "SetSize", value};
}

export function mkMsgSetFlex(value: MsgSetFlex): Msg {
    return { tag: "SetFlex", value};
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

export function mkSurfaceId(val: number): number & { type: 'SurfaceId'} {
    return val as any
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
