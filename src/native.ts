import { ERR, isDeno, isNodeJS, PLATFORM, TODO } from './util'

// flat object with all of the native functions,
// initialized by `loadNativeApi()`
export let native: any = null

export const TRUE = 1
export const NULL = 0

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

// bind wrapper object to be cached and/or collected when necessary,
export const register = <T extends object>(wrapper: T, id) => {
  // TODO: remove checks later

  if (id > WRAPPER_REFS.length) {
    console.log('unexpected hole in refs', id, wrapper)
  }

  if (WRAPPER_REFS[id] !== undefined) {
    console.log('already registered', id, wrapper)
  }

  NATIVE_REGISTRY.register(wrapper, id)
  WRAPPER_REFS[id] = new WeakRef(wrapper)
  wrapper[NATIVE_ID] = id

  return wrapper
}

// get wrapper for a given given native id
export const lookup = id => (id === NULL ? null : WRAPPER_REFS[id]?.deref())

// get id from wrapper so it can be passed to native calls
export const getNativeId = wrapper => wrapper[NATIVE_ID]


const NATIVE_ID = Symbol()
const WRAPPER_REFS: WeakRef<any>[] = [
  // [0] should never be accessed so this is both fine and it also works as a check
  null as any,
]
const NATIVE_REGISTRY = new FinalizationRegistry((id: number) => {
  native.Rc_drop(id)
  WRAPPER_REFS[id] = undefined as any
})

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

  // TODO: wait for https://github.com/denoland/deno/pull/11648

  const lib = Deno.dlopen(libFile, {
    // TODO: parse/generate from ffi.rs
    gft_App_init: { parameters: [], result: 'u32' },
    gft_App_wake_up: { parameters: [], result: 'void' },
    gft_App_tick: { parameters: ['u32'], result: 'u32' },
    gft_Window_new: { parameters: ['buffer', 'i32', 'i32'], result: 'u32' },
    gft_Window_width: { parameters: ['u32'], result: 'i32' },
    gft_Window_height: { parameters: ['u32'], result: 'i32' },
    gft_Document_new: { parameters: [], result: 'u32' },
    gft_Document_create_element: { parameters: ['u32', 'buffer'], result: 'u32' },
    gft_Document_create_text_node: { parameters: ['u32', 'buffer'], result: 'u32' },
    gft_Document_create_comment: { parameters: ['u32', 'buffer'], result: 'u32' },
    gft_Node_append_child: { parameters: ['u32', 'u32'], result: 'u32' },
    gft_Node_insert_before: { parameters: ['u32', 'u32', 'u32'], result: 'u32' },
    gft_WebView_new: { parameters: [], result: 'u32' },
    gft_WebView_attach: { parameters: ['u32', 'u32'], result: 'void' },
    gft_WebView_load_url: { parameters: ['u32', 'buffer'], result: 'void' },
    gft_WebView_eval: { parameters: ['u32', 'buffer'], result: 'void' },
  })

  // debug
  native = Object.fromEntries(
    Object.entries<any>(lib.symbols).map(([name, fn]) => {
      return [name, (...args) => (console.log('call', name, ...args), fn(...args))]
    })
  )

  //native = lib.symbols
}
