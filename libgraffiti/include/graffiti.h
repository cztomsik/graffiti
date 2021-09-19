#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef unsigned int ObjId_Vec_Rc_Any;

typedef unsigned int ObjId_App;

typedef unsigned int ObjId_Window;

typedef unsigned int ObjId_WebView;

typedef unsigned int ObjId_Document;

typedef unsigned int ObjId_Element;

typedef unsigned int ObjId_CharacterData;

typedef unsigned int ObjId_Node;

typedef unsigned int ObjId_CssStyleDeclaration;

typedef unsigned int ObjId_Viewport;

extern void *dlopen(const char *filename, int flags);

extern void *dlsym(void *handle, const char *symbol);

extern int dlclose(void *handle);

extern void *LoadLibraryA(const char *filename);

extern void *GetProcAddress(void *module, const char *name);

extern int FreeLibrary(void *handle);

unsigned int gft_Vec_len(ObjId_Vec_Rc_Any vec);

ObjId_Vec_Rc_Any gft_Vec_get(ObjId_Vec_Rc_Any vec, unsigned int index);

ObjId_App gft_App_init(void);

ObjId_App gft_App_current(void);

void gft_App_tick(ObjId_App app);

void gft_App_wake_up(void);

ObjId_Window gft_Window_new(const char *title, int width, int height);

char *gft_Window_title(ObjId_Window win);

void gft_Window_set_title(ObjId_Window win, const char *title);

int gft_Window_width(ObjId_Window win);

int gft_Window_height(ObjId_Window win);

void gft_Window_resize(ObjId_Window win, int width, int height);

bool gft_Window_should_close(ObjId_Window win);

void gft_Window_show(ObjId_Window win);

void gft_Window_hide(ObjId_Window win);

void gft_Window_focus(ObjId_Window win);

void gft_Window_minimize(ObjId_Window win);

void gft_Window_maximize(ObjId_Window win);

void gft_Window_restore(ObjId_Window win);

ObjId_WebView gft_WebView_new(void);

void gft_WebView_attach(ObjId_WebView webview, ObjId_Window win);

void gft_WebView_load_url(ObjId_WebView webview, const char *url);

void gft_WebView_eval(ObjId_WebView webview, const char *script);

ObjId_Document gft_Document_new(void);

ObjId_Element gft_Document_create_element(ObjId_Document doc, const char *local_name);

ObjId_CharacterData gft_Document_create_text_node(ObjId_Document doc, const char *data);

ObjId_CharacterData gft_Document_create_comment(ObjId_Document doc, const char *data);

uint32_t gft_Node_node_type(ObjId_Node node);

ObjId_Node gft_Node_parent_node(ObjId_Node node);

ObjId_Node gft_Node_first_child(ObjId_Node node);

ObjId_Node gft_Node_last_child(ObjId_Node node);

ObjId_Node gft_Node_previous_sibling(ObjId_Node node);

ObjId_Node gft_Node_next_sibling(ObjId_Node node);

void gft_Node_append_child(ObjId_Node parent, ObjId_Node child);

void gft_Node_insert_before(ObjId_Node parent, ObjId_Node child, ObjId_Node before);

void gft_Node_remove_child(ObjId_Node parent, ObjId_Node child);

ObjId_Element gft_Node_query_selector(ObjId_Node node, const char *selector);

char *gft_CharacterData_data(ObjId_CharacterData node);

void gft_CharacterData_set_data(ObjId_CharacterData node, const char *data);

char *gft_Element_local_name(ObjId_Element el);

char *gft_Element_attribute(ObjId_Element el, const char *att);

void gft_Element_set_attribute(ObjId_Element el, const char *att, const char *val);

void gft_Element_remove_attribute(ObjId_Element el, const char *att);

unsigned int gft_CssStyleDeclaration_length(ObjId_CssStyleDeclaration style);

void gft_CssStyleDeclaration_set_property(ObjId_CssStyleDeclaration style,
                                          const char *prop,
                                          const char *val);

void gft_Viewport_resize(ObjId_Viewport viewport, double width, double height);
