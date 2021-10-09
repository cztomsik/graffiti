import { ERR, isDeno, isNodeJS, PLATFORM, TODO } from './util'

// flat object with all of the native functions,
// initialized by `loadNativeApi()`
// TODO: Record<string, Function>
// TODO: consider using top-level await
export let native: any = null

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

const encoder = new TextEncoder()
const decoder = new TextDecoder()

export const encode = (input: string) => {
  const buf = encoder.encode(input)
  return [buf, buf.length]
}

export const decode = (stringRef: number) => {
  if (!stringRef) {
    return null
  }

  const buf = new Uint8Array(native.gft_String_bytes_len(stringRef))
  native.gft_String_copy(stringRef, buf);
  native.gft_Ref_drop(stringRef)
  return decoder.decode(buf)
}

export const loadNativeApi = async () => {
  const libFile = await resolveLibFile()

  if (isNodeJS) {
    return await loadNodejsAddon(libFile)
  }

  if (isDeno) {
    return await loadDenoPlugin(libFile)
  }

  return ERR('unsupported JS engine')
}

// bind wrapper object to be automatically dropped
export const register = <T extends object>(wrapper: T, id) => {
  NATIVE_REGISTRY.register(wrapper, id)
  wrapper[NATIVE_ID] = id

  return wrapper
}

// get id from wrapper so it can be passed to native calls
export const getNativeId = wrapper => wrapper[NATIVE_ID]

const NATIVE_ID = Symbol()
const NATIVE_REGISTRY = new FinalizationRegistry((id: number) => native.Ref_drop(id))

const resolveLibFile = async () => {
  // TODO
  // const PREBUILT_URL = `https://github.com/cztomsik/graffiti/releases/download/${VERSION}`

  const LIB_NAME =
    PLATFORM === 'darwin' ? 'libgraffiti.dylib' : PLATFORM === 'windows' ? 'graffiti.dll' : 'libgraffiti.so'
  const LIB_URL = new URL(`../libgraffiti/target/debug/${LIB_NAME}`, import.meta.url)

  return PLATFORM === 'windows' ? LIB_URL.href.replace('file:///', '') : LIB_URL.pathname
}

// TODO: we couldnt use ffi-napi because of workers
// https://github.com/node-ffi-napi/node-ffi-napi/issues/125
const loadNodejsAddon = async libFile => {
  // tell dylib to register napi extension
  process.env.GFT_NODEJS = '1'

  // require() would make ncc bundle some unnecessary build artifacts
  process['dlopen']({ exports: native = {} }, libFile)
}

const loadDenoPlugin = async (libFile, Deno = globalThis.Deno) => {
  // TODO: fetch using https://deno.land/x/cache (Plug doesn't really add anything here)

  const lib = Deno.dlopen(libFile, {
    // TODO: parse/generate from ffi.rs
    gft_Ref_drop: { parameters: ['u32'], result: 'void' },
    gft_String_bytes_len: { parameters: ['u32'], result: 'u32' },
    gft_String_copy: { parameters: ['u32', 'buffer'], result: 'void' },
    gft_Vec_len: { parameters: ['u32'], result: 'u32' },
    gft_Vec_get: { parameters: ['u32', 'u32'], result: 'u32' },
    gft_App_init: { parameters: [], result: 'u32' },
    gft_App_wake_up: { parameters: [], result: 'void' },
    gft_App_tick: { parameters: ['u32'], result: 'void' },
    gft_Window_new: { parameters: ['buffer', 'u32', 'i32', 'i32'], result: 'u32' },
    gft_Window_width: { parameters: ['u32'], result: 'i32' },
    gft_Window_height: { parameters: ['u32'], result: 'i32' },
    gft_Document_new: { parameters: [], result: 'u32' },
    gft_Document_create_element: { parameters: ['u32', 'buffer', 'u32'], result: 'u32' },
    gft_Document_create_text_node: { parameters: ['u32', 'buffer', 'u32'], result: 'u32' },
    gft_Document_create_comment: { parameters: ['u32', 'buffer', 'u32'], result: 'u32' },
    gft_Node_id: { parameters: ['u32'], result: 'u32' },
    gft_Node_append_child: { parameters: ['u32', 'u32'], result: 'void' },
    gft_Node_insert_before: { parameters: ['u32', 'u32', 'u32'], result: 'void' },
    gft_Node_remove_child: { parameters: ['u32', 'u32'], result: 'void' },
    gft_Node_query_selector: { parameters: ['u32', 'buffer', 'u32'], result: 'u32' },
    gft_Node_query_selector_all: { parameters: ['u32', 'buffer', 'u32'], result: 'u32' },
    gft_Element_attribute_names: { parameters: ['u32'], result: 'u32' },
    gft_Element_attribute: { parameters: ['u32', 'buffer', 'u32'], result: 'u32' },
    gft_Element_set_attribute: { parameters: ['u32', 'buffer', 'u32', 'buffer', 'u32'], result: 'void' },
    gft_CharacterData_data: { parameters: ['u32'], result: 'u32' },
    gft_CharacterData_set_data: { parameters: ['u32', 'buffer', 'u32'], result: 'void' },
    gft_WebView_new: { parameters: [], result: 'u32' },
    gft_WebView_attach: { parameters: ['u32', 'u32'], result: 'void' },
    gft_WebView_load_url: { parameters: ['u32', 'buffer', 'u32'], result: 'void' },
    gft_WebView_eval: { parameters: ['u32', 'buffer', 'u32'], result: 'void' },
  })

  // debug
  native = Object.fromEntries(
    Object.entries<any>(lib.symbols).map(([name, fn]) => {
      return [name, (...args) => {
        const res = (console.log('call', name, ...args), fn(...args))
        console.log('<-', res)
        return res
      }]
    })
  )

  //native = lib.symbols
}
