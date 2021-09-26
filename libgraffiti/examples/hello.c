// clang -I ../include hello.c -L ../target/debug -lgraffiti -o hello && ./hello

#include <stdio.h>
#include <graffiti.h>

int main() {
  Ref_App app = gft_App_init();
  Ref_Window win = gft_Window_new("Hello", 400, 300);

  Ref_Document doc = gft_Document_new();
  Ref_Element div = gft_Document_create_element(doc, "div");
  Ref_CharacterData hello = gft_Document_create_text_node(doc, "Hello");
  gft_Node_append_child(doc, div);
  gft_Node_append_child(div, hello);

  printf("%s\n", gft_CharacterData_data(hello));

  while (!gft_Window_should_close(win)) {
    gft_App_tick(app);
  }

  // cleanup
  gft_Ref_drop(hello);
  gft_Ref_drop(div);
  gft_Ref_drop(doc);
  gft_Ref_drop(win);
  gft_Ref_drop(app);

  return 0;
}
