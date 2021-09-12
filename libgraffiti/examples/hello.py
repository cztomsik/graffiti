# this is just for FFI testing, python is not supported ATM
# and this is literally my first python script

import asyncio
from ctypes import cdll, c_int, c_uint, c_char_p
lib = cdll.LoadLibrary('../target/debug/libgraffiti.dylib')

lib.gft_App_init.restype = c_uint

lib.gft_Window_new.argtypes = [c_char_p, c_int, c_int]
lib.gft_Window_new.restype = c_uint
lib.gft_Window_title.argtypes = [c_uint]
lib.gft_Window_title.restype = c_char_p

lib.gft_Document_new.restype = c_uint
lib.gft_Document_create_element.argtypes = [c_uint, c_char_p]
lib.gft_Document_create_element.restype = c_uint
lib.gft_Document_create_text_node.argtypes = [c_uint, c_char_p]
lib.gft_Document_create_text_node.restype = c_uint
lib.gft_Node_append_child.argtypes = [c_uint, c_uint]
lib.gft_Node_query_selector.argtypes = [c_uint, c_char_p]
lib.gft_Node_query_selector.restype = c_uint

lib.gft_App_init()

async def main():
  win = lib.gft_Window_new(b'Hello', 800, 600)

  doc = lib.gft_Document_new()
  div = lib.gft_Document_create_element(doc, b'div')
  hello = lib.gft_Document_create_text_node(doc, b'Hello')
  lib.gft_Node_append_child(doc, div)
  lib.gft_Node_append_child(div, hello)

  print('div: {}'.format(div))
  print('querySelector: {}'.format(lib.gft_Node_query_selector(doc, b'div')))

  while True:
    lib.gft_App_tick()
    await asyncio.sleep(0.1)

loop = asyncio.get_event_loop()
loop.create_task(main())
loop.run_forever()
