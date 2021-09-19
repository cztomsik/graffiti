// clang -I ../include hello.c -L ../target/debug -lgraffiti -o hello && ./hello

#include <stdio.h>
#include <graffiti.h>

int main() {
  ObjId_App app = gft_App_init();

  ObjId_Window win = gft_Window_new("Hello", 400, 300);

  ObjId_Document doc = gft_Document_new();
  ObjId_Element div = gft_Document_create_element(doc, "div");
  ObjId_CharacterData hello = gft_Document_create_text_node(doc, "Hello");
  gft_Node_append_child(doc, div);
  gft_Node_append_child(div, hello);

  printf("%s\n", gft_CharacterData_data(hello));

  while (!gft_Window_should_close(win)) {
    gft_App_tick();
  }

  return 0;
}
