# this is just for FFI testing, python is not supported ATM
# doesn't work with macos-provided python (relative file-system url)

import cffi

ffi = cffi.FFI()

with open('../include/graffiti.h') as f:
  ffi.cdef(''.join([line for line in f if not line.startswith('#')]))

lib = ffi.dlopen('../target/debug/libgraffiti.dylib')

app = lib.gft_App_init()
win = lib.gft_Window_new(b'Hello', 5, 800, 600)
doc = lib.gft_Document_new()
renderer = lib.gft_Renderer_new(doc, win)

div = lib.gft_Document_create_element(doc, b'div', 3)
hello = lib.gft_Document_create_text_node(doc, b'Hello', 5)
lib.gft_Node_append_child(doc, div)
lib.gft_Node_append_child(div, hello)

print('div id: {}'.format(lib.gft_Node_id(div)))
print('querySelector: {}'.format(lib.gft_Node_id(lib.gft_Node_query_selector(doc, b'div', 3))))

while not lib.gft_Window_should_close(win):
  lib.gft_Renderer_render(renderer)
  lib.gft_App_tick(app)
