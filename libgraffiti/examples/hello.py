# this is just for FFI testing, python is not supported ATM
# and this is literally my first python script

import asyncio
from ctypes import cdll, c_int, c_uint, c_char_p
lib = cdll.LoadLibrary('../target/debug/libgraffiti.dylib')

lib.gft_App_init.restype = c_uint
lib.gft_App_tick.argtypes = [c_uint]

lib.gft_Window_new.argtypes = [c_char_p, c_uint, c_int, c_int]
lib.gft_Window_new.restype = c_uint

lib.gft_Document_new.restype = c_uint
lib.gft_Document_create_element.argtypes = [c_uint, c_char_p, c_uint]
lib.gft_Document_create_element.restype = c_uint
lib.gft_Document_create_text_node.argtypes = [c_uint, c_char_p, c_uint]
lib.gft_Document_create_text_node.restype = c_uint
lib.gft_Node_append_child.argtypes = [c_uint, c_uint]
lib.gft_Node_query_selector.argtypes = [c_uint, c_char_p, c_uint]
lib.gft_Node_query_selector.restype = c_uint
lib.gft_Node_id.argtypes = [c_uint]
lib.gft_Node_id.restype = c_uint

app = lib.gft_App_init()

async def main():
  win = lib.gft_Window_new(b'Hello', 5, 800, 600)

  doc = lib.gft_Document_new()
  div = lib.gft_Document_create_element(doc, b'div', 3)
  hello = lib.gft_Document_create_text_node(doc, b'Hello', 5)
  lib.gft_Node_append_child(doc, div)
  lib.gft_Node_append_child(div, hello)

  print('div id: {}'.format(lib.gft_Node_id(div)))
  print('querySelector: {}'.format(lib.gft_Node_id(lib.gft_Node_query_selector(doc, b'div', 3))))

  while True:
    lib.gft_App_tick(app)
    await asyncio.sleep(0.1)

loop = asyncio.get_event_loop()
loop.create_task(main())
loop.run_forever()
