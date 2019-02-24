export type Msg = 
    | { tag: "CreateSurface"}
    | { tag: "SurfaceMsg", value: MsgSurfaceMsg}
;
export type SurfaceMsg = 
    | { tag: "AppendChild", value: SurfaceMsgAppendChild}
    | { tag: "InsertBefore", value: SurfaceMsgInsertBefore}
    | { tag: "RemoveChild", value: SurfaceMsgRemoveChild}
    | { tag: "SetSize", value: Size}
    | { tag: "SetFlex", value: Flex}
    | { tag: "SetPadding", value: Rect}
    | { tag: "SetMargin", value: Rect}
    | { tag: "SetBoxShadow", value: (BoxShadow) | undefined}
    | { tag: "SetBackgroundColor", value: (BackgroundColor) | undefined}
    | { tag: "SetImage", value: (Image) | undefined}
    | { tag: "SetText", value: (Text) | undefined}
    | { tag: "SetBorder", value: (Border) | undefined}
;
export type SurfaceId = number & { type: 'SurfaceId'};
export type Dimension = 
    | { tag: "Auto"}
    | { tag: "Point", value: number}
    | { tag: "Percent", value: number}
;
export type BackgroundColor = Color & { type: 'BackgroundColor'};

export interface MsgSurfaceMsg {
    surface: SurfaceId;
    msg: SurfaceMsg;
}

export interface SurfaceMsgAppendChild {
    parent: SurfaceId;
    child: SurfaceId;
}

export interface SurfaceMsgInsertBefore {
    parent: SurfaceId;
    child: SurfaceId;
    before: SurfaceId;
}

export interface SurfaceMsgRemoveChild {
    parent: SurfaceId;
    child: SurfaceId;
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
    text: string;
}

export interface Border {
    top: BorderSide;
    right: BorderSide;
    bottom: BorderSide;
    left: BorderSide;
}

export interface BorderSide {
    color: Color;
    style: BorderStyle;
}

export enum BorderStyle {
    None,
    Solid
}

export function mkMsgCreateSurface(): Msg {
    return { tag: "CreateSurface"};
}

export function mkMsgSurfaceMsg(value: MsgSurfaceMsg): Msg {
    return { tag: "SurfaceMsg", value};
}

export function mkSurfaceMsgAppendChild(value: SurfaceMsgAppendChild): SurfaceMsg {
    return { tag: "AppendChild", value};
}

export function mkSurfaceMsgInsertBefore(value: SurfaceMsgInsertBefore): SurfaceMsg {
    return { tag: "InsertBefore", value};
}

export function mkSurfaceMsgRemoveChild(value: SurfaceMsgRemoveChild): SurfaceMsg {
    return { tag: "RemoveChild", value};
}

export function mkSurfaceMsgSetSize(value: Size): SurfaceMsg {
    return { tag: "SetSize", value};
}

export function mkSurfaceMsgSetFlex(value: Flex): SurfaceMsg {
    return { tag: "SetFlex", value};
}

export function mkSurfaceMsgSetPadding(value: Rect): SurfaceMsg {
    return { tag: "SetPadding", value};
}

export function mkSurfaceMsgSetMargin(value: Rect): SurfaceMsg {
    return { tag: "SetMargin", value};
}

export function mkSurfaceMsgSetBoxShadow(value: (BoxShadow) | undefined): SurfaceMsg {
    return { tag: "SetBoxShadow", value};
}

export function mkSurfaceMsgSetBackgroundColor(value: (BackgroundColor) | undefined): SurfaceMsg {
    return { tag: "SetBackgroundColor", value};
}

export function mkSurfaceMsgSetImage(value: (Image) | undefined): SurfaceMsg {
    return { tag: "SetImage", value};
}

export function mkSurfaceMsgSetText(value: (Text) | undefined): SurfaceMsg {
    return { tag: "SetText", value};
}

export function mkSurfaceMsgSetBorder(value: (Border) | undefined): SurfaceMsg {
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

export function mkBackgroundColor(val: Color): Color & { type: 'BackgroundColor'} {
    return val as any
}
