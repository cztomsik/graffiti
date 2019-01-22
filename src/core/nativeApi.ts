export module N {
  export interface Window {
    opaque: 'opaque win reference'
  }

  export interface Surface {
    opaque: 'opaque surface reference'
  }

  export interface FlexStyle {
    opaque: 'opaque compiled flex style object reference'
  }

  export enum MeasureMode {
    Undefined = 0,
    Exactly = 1,
    AtMost = 2
  }

  export interface MeasureCallback {
    (
      availableWidth: number,
      widthMode: MeasureMode,
      availableHeight: number,
      heightMode: MeasureMode
    ): { width: number; height: number }
  }

  export interface ResourceHandle {
    opaque: 'opaque handle of brush or clip'
  }
}

export interface NativeApi {
  window_create: (
    title: string,
    width: number,
    hieght: number,
    socketPath: string
  ) => N.Window

  window_handle_events: (win: N.Window) => void

  window_render_surface: (
    win: N.Window,
    surface: N.Surface,
    availableWidth: number,
    availableHeight: number
  ) => void

  surface_create: () => N.Surface

  surface_append_child: (to: N.Surface, child: N.Surface) => void

  surface_insert_before: (
    to: N.Surface,
    child: N.Surface,
    before: N.Surface
  ) => void

  surface_remove_child: (from: N.Surface, child: N.Surface) => void

  surface_mark_dirty: (surface: N.Surface) => void

  surface_calculate_layout: (
    surface: N.Surface,
    availableWidth: number,
    availableHeight: number
  ) => void

  surface_update: (
    surface: N.Surface,
    brush: N.ResourceHandle | undefined,
    clip: N.ResourceHandle | undefined,
    layout: N.FlexStyle
  ) => void

  surface_set_measure_func: (
    surface: N.Surface,
    callback: N.MeasureCallback
  ) => void

  flex_style_create: (serializedStyle: string) => N.FlexStyle

  op_resource_create: (serializedOperations: string) => N.ResourceHandle

  window_get_glyph_indices_and_advances: (
    win: N.Window,
    fontSize: number,
    str: string
  ) => [
    // indices
    // -> Uint32Array
    number[],
    // advances
    // -> Float32Array
    number[]
  ]
}
