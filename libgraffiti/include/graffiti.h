#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum NodeType {
  Element = 1,
  Text = 3,
  Comment = 8,
  Document = 9,
};
typedef uint32_t NodeType;

typedef unsigned int Ref_Vec_Rc_Any;

typedef unsigned int Ref_Any;

typedef unsigned int Ref_App;

typedef unsigned int Ref_Window;

typedef unsigned int Ref_WebView;

typedef unsigned int Ref_Document;

typedef unsigned int Ref_Element;

typedef unsigned int Ref_CharacterData;

typedef unsigned int Ref_Node;

typedef unsigned int Ref_CssStyleDeclaration;

typedef unsigned int Ref_Viewport;

extern void *dlopen(const char *filename, int flags);

extern void *dlsym(void *handle, const char *symbol);

extern int dlclose(void *handle);

extern void *LoadLibraryA(const char *filename);

extern void *GetProcAddress(void *module, const char *name);

extern int FreeLibrary(void *handle);

unsigned int gft_Vec_len(Ref_Vec_Rc_Any vec);

Ref_Any gft_Vec_get(Ref_Vec_Rc_Any vec, unsigned int index);

Ref_App gft_App_init(void);

Ref_App gft_App_current(void);

void gft_App_tick(Ref_App app);

void gft_App_wake_up(void);

Ref_Window gft_Window_new(const char *title, int width, int height);

char *gft_Window_title(Ref_Window win);

void gft_Window_set_title(Ref_Window win, const char *title);

int gft_Window_width(Ref_Window win);

int gft_Window_height(Ref_Window win);

void gft_Window_resize(Ref_Window win, int width, int height);

bool gft_Window_should_close(Ref_Window win);

void gft_Window_show(Ref_Window win);

void gft_Window_hide(Ref_Window win);

void gft_Window_focus(Ref_Window win);

void gft_Window_minimize(Ref_Window win);

void gft_Window_maximize(Ref_Window win);

void gft_Window_restore(Ref_Window win);

Ref_WebView gft_WebView_new(void);

void gft_WebView_attach(Ref_WebView webview, Ref_Window win);

void gft_WebView_load_url(Ref_WebView webview, const char *url);

void gft_WebView_eval(Ref_WebView webview, const char *script);

Ref_Document gft_Document_new(void);

Ref_Element gft_Document_create_element(Ref_Document doc, const char *local_name);

Ref_CharacterData gft_Document_create_text_node(Ref_Document doc, const char *data);

Ref_CharacterData gft_Document_create_comment(Ref_Document doc, const char *data);

NodeType gft_Node_node_type(Ref_Node node);

Ref_Node gft_Node_parent_node(Ref_Node node);

Ref_Node gft_Node_first_child(Ref_Node node);

Ref_Node gft_Node_last_child(Ref_Node node);

Ref_Node gft_Node_previous_sibling(Ref_Node node);

Ref_Node gft_Node_next_sibling(Ref_Node node);

void gft_Node_append_child(Ref_Node parent, Ref_Node child);

void gft_Node_insert_before(Ref_Node parent, Ref_Node child, Ref_Node before);

void gft_Node_remove_child(Ref_Node parent, Ref_Node child);

Ref_Element gft_Node_query_selector(Ref_Node node, const char *selector);

Ref_Vec_Rc_Any gft_Node_query_selector_all(Ref_Node node, const char *selector);

char *gft_CharacterData_data(Ref_CharacterData node);

void gft_CharacterData_set_data(Ref_CharacterData node, const char *data);

char *gft_Element_local_name(Ref_Element el);

char *gft_Element_attribute(Ref_Element el, const char *att);

void gft_Element_set_attribute(Ref_Element el, const char *att, const char *val);

void gft_Element_remove_attribute(Ref_Element el, const char *att);

unsigned int gft_CssStyleDeclaration_length(Ref_CssStyleDeclaration style);

void gft_CssStyleDeclaration_set_property(Ref_CssStyleDeclaration style,
                                          const char *prop,
                                          const char *val);

void gft_Viewport_resize(Ref_Viewport viewport, double width, double height);
