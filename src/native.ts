import { ERR, isDeno, isNodeJS, PLATFORM, TODO } from './util'

// flat object with all of the native functions,
export const native: Record<string, Function> = await loadNativeApi()

/*
// unpack Vec<Ref<Value>> to array of refs
// note that native.gft_Ref_drop(ref) still needs to be called for each ref
export const getRefs = vec => {
  const refs: number[] = []

  for (let i = 0, len = native.gft_Vec_len(vec); i < len; i++) {
    refs.push(native.gft_Vec_get(vec, i))
  }
  native.gft_Ref_drop(vec)

  return refs
}
*/

const encoder = new TextEncoder()

export const encode = (input: string) => encoder.encode(`${input}\0`)

export const decode = ptr => new globalThis.Deno.UnsafePointerView(ptr).getCString()

export const atom = (atom: string) => native.gft_Atom_from(encode(atom))

async function loadNativeApi() {
  const libFile = await resolveLibFile()

  if (isNodeJS) {
    return await loadNodejsAddon(libFile)
  }

  if (isDeno) {
    return await loadDenoPlugin(libFile)
  }

  return ERR('unsupported JS engine')
}

/*
// bind wrapper object to be automatically dropped
export const register = <T extends object>(wrapper: T, id) => {
  NATIVE_REGISTRY.register(wrapper, id)
  wrapper[NATIVE_ID] = id

  return wrapper
}

// get id from wrapper so it can be passed to native calls
export const getNativeId = wrapper => wrapper[NATIVE_ID]

const NATIVE_ID = Symbol()
const NATIVE_REGISTRY = new FinalizationRegistry((id: number) => native.gft_Ref_drop(id))
*/

async function resolveLibFile() {
  // TODO
  // const PREBUILT_URL = `https://github.com/cztomsik/graffiti/releases/download/${VERSION}`

  const LIB_NAME =
    PLATFORM === 'darwin' ? 'libgraffiti.dylib' : PLATFORM === 'windows' ? 'graffiti.dll' : 'libgraffiti.so'
  const LIB_URL = new URL(`../libgraffiti/target/debug/${LIB_NAME}`, import.meta.url)

  return PLATFORM === 'windows' ? LIB_URL.href.replace('file:///', '') : LIB_URL.pathname
}

async function loadNodejsAddon(libFile) {
  // TODO: we cant use ffi-napi because of workers
  // https://github.com/node-ffi-napi/node-ffi-napi/issues/125
  return TODO()
}

async function loadDenoPlugin(libFile, Deno = globalThis.Deno) {
  // TODO: fetch using https://deno.land/x/cache (Plug doesn't really add anything here)

  const lib = Deno.dlopen(libFile, {
    // TODO: parse/generate from ffi.rs
    gft_Atom_from: { parameters: ['pointer'], result: 'u32' },
    gft_App_init: { parameters: [], result: 'void' },
    gft_App_tick: { parameters: [], result: 'void' },
    gft_App_wake_up: { parameters: [], result: 'void' },
    gft_Window_new: { parameters: ['pointer', 'i32', 'i32'], result: 'u32' },
    // gft_Window_next_event: { parameters: ['u32', 'pointer'], result: 'u8' },
    gft_Window_width: { parameters: ['u32'], result: 'i32' },
    gft_Window_height: { parameters: ['u32'], result: 'i32' },
    gft_Document_new: { parameters: [], result: 'u32' },
    gft_Document_create_element: { parameters: ['u32', 'u32'], result: 'u32' },
    gft_Document_create_text_node: { parameters: ['u32', 'pointer'], result: 'u32' },
    gft_Document_append_child: { parameters: ['u32', 'u32', 'u32'], result: 'void' },
    gft_Document_insert_before: { parameters: ['u32', 'u32', 'u32', 'u32'], result: 'void' },
    gft_Document_remove_child: { parameters: ['u32', 'u32', 'u32'], result: 'void' },
    gft_Document_query_selector: { parameters: ['u32', 'pointer', 'u32'], result: 'u32' },
    // gft_Document_query_selector_all: { parameters: ['u32', 'pointer', 'u32'], result: 'u32' },
    gft_Document_attribute: { parameters: ['u32', 'u32', 'u32'], result: 'pointer' },
    gft_Document_set_attribute: { parameters: ['u32', 'u32', 'u32', 'pointer'], result: 'void' },
    gft_Document_remove_attribute: { parameters: ['u32', 'u32', 'u32'], result: 'void' },
    // gft_Document_attribute_names: { parameters: ['u32'], result: 'u32' },
    // gft_Document_style: { parameters: ['u32', 'u32'], result: 'u32' },
    // gft_Document_set_style: { parameters: ['u32', 'u32', 'pointer'], result: 'void' },
    gft_Document_text: { parameters: ['u32', 'u32'], result: 'pointer' },
    gft_Document_set_text: { parameters: ['u32', 'u32', 'pointer'], result: 'void' },
    // gft_Renderer_new: { parameters: ['u32', 'u32'], result: 'u32' },
    // gft_Renderer_render: { parameters: ['u32'], result: 'void' },
    // gft_Renderer_resize: { parameters: ['u32', 'f32', 'f32'], result: 'void' },
  })

  // debug
  // return Object.fromEntries(
  //   Object.entries<any>(lib.symbols).map(([name, fn]) => {
  //     return [name, (...args) => {
  //       const res = (console.log('call', name, ...args), fn(...args))
  //       console.log('<-', res)
  //       return res
  //     }]
  //   })
  // )

  return lib.symbols
}
