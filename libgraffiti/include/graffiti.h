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

typedef uint32_t Ref_Value;

typedef uint32_t Ref_Vec_Value;

typedef uint32_t Ref_String;

typedef uint32_t Ref_App;

typedef Ref_App Option_Ref_App;

typedef uint32_t Ref_Window;

typedef uint32_t WindowId;

typedef Ref_Window Option_Ref_Window;

enum Event_Tag {
  CursorPos,
  MouseDown,
  MouseUp,
  Scroll,
  KeyUp,
  KeyDown,
  KeyPress,
  Resize,
  FramebufferSize,
  Close,
};
typedef uint32_t Event_Tag;

typedef struct CursorPos_Body {
  float _0;
  float _1;
} CursorPos_Body;

typedef struct Scroll_Body {
  float _0;
  float _1;
} Scroll_Body;

typedef struct Resize_Body {
  float _0;
  float _1;
} Resize_Body;

typedef struct FramebufferSize_Body {
  float _0;
  float _1;
} FramebufferSize_Body;

typedef struct Event {
  Event_Tag tag;
  union {
    CursorPos_Body cursor_pos;
    Scroll_Body scroll;
    struct {
      uint32_t key_up;
    };
    struct {
      uint32_t key_down;
    };
    struct {
      uint32_t key_press;
    };
    Resize_Body resize;
    FramebufferSize_Body framebuffer_size;
  };
} Event;

typedef uint32_t Ref_WebView;

typedef uint32_t Ref_DocumentRef;

typedef uint32_t Ref_ElementRef;

typedef uint32_t Ref_CharacterDataRef;

typedef uint32_t NodeId;

typedef uint32_t Ref_NodeRef;

typedef Ref_NodeRef Option_Ref_NodeRef;

typedef Ref_ElementRef Option_Ref_ElementRef;

typedef Ref_String Option_Ref_String;

typedef uint32_t Ref_CssStyleDeclaration;

typedef uint32_t Ref_Renderer;



void gft_Ref_drop(Ref_Value obj);

unsigned int gft_Vec_len(Ref_Vec_Value vec);

Ref_Value gft_Vec_get(Ref_Vec_Value vec, unsigned int index);

unsigned int gft_String_bytes_len(Ref_String string);

void gft_String_copy(Ref_String string, uint8_t *dest_buf);

Ref_App gft_App_init(void);

Option_Ref_App gft_App_current(void);

void gft_App_tick(Ref_App app);

void gft_App_wake_up(Ref_App app);

Ref_Window gft_Window_new(const char *title, uint32_t title_len, int width, int height);

WindowId gft_Window_id(Ref_Window win);

Option_Ref_Window gft_Window_find_by_id(WindowId id);

bool gft_Window_next_event(Ref_Window win, struct Event *event_dest);

Ref_String gft_Window_title(Ref_Window win);

void gft_Window_set_title(Ref_Window win, const char *title, uint32_t title_len);

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

void gft_WebView_load_url(Ref_WebView webview, const char *url, uint32_t url_len);

void gft_WebView_eval(Ref_WebView webview, const char *script, uint32_t script_len);

Ref_DocumentRef gft_Document_new(void);

Ref_ElementRef gft_Document_create_element(Ref_DocumentRef doc,
                                           const char *local_name,
                                           uint32_t local_name_len);

Ref_CharacterDataRef gft_Document_create_text_node(Ref_DocumentRef doc,
                                                   const char *data,
                                                   uint32_t data_len);

Ref_CharacterDataRef gft_Document_create_comment(Ref_DocumentRef doc,
                                                 const char *data,
                                                 uint32_t data_len);

NodeId gft_Node_id(Ref_NodeRef node);

NodeType gft_Node_node_type(Ref_NodeRef node);

Option_Ref_NodeRef gft_Node_parent_node(Ref_NodeRef node);

Option_Ref_NodeRef gft_Node_first_child(Ref_NodeRef node);

Option_Ref_NodeRef gft_Node_last_child(Ref_NodeRef node);

Option_Ref_NodeRef gft_Node_previous_sibling(Ref_NodeRef node);

Option_Ref_NodeRef gft_Node_next_sibling(Ref_NodeRef node);

void gft_Node_append_child(Ref_NodeRef parent, Ref_NodeRef child);

void gft_Node_insert_before(Ref_NodeRef parent, Ref_NodeRef child, Ref_NodeRef before);

void gft_Node_remove_child(Ref_NodeRef parent, Ref_NodeRef child);

Option_Ref_ElementRef gft_Node_query_selector(Ref_NodeRef node,
                                              const char *selector,
                                              uint32_t selector_len);

Ref_Vec_Value gft_Node_query_selector_all(Ref_NodeRef node,
                                          const char *selector,
                                          uint32_t selector_len);

Ref_String gft_CharacterData_data(Ref_CharacterDataRef node);

void gft_CharacterData_set_data(Ref_CharacterDataRef node, const char *data, uint32_t data_len);

Ref_String gft_Element_local_name(Ref_ElementRef el);

Ref_Vec_Value gft_Element_attribute_names(Ref_ElementRef el);

Option_Ref_String gft_Element_attribute(Ref_ElementRef el, const char *att, uint32_t att_len);

void gft_Element_set_attribute(Ref_ElementRef el,
                               const char *att,
                               uint32_t att_len,
                               const char *val,
                               uint32_t val_len);

void gft_Element_remove_attribute(Ref_ElementRef el, const char *att, uint32_t att_len);

bool gft_Element_matches(Ref_ElementRef el, const char *selector, uint32_t selector_len);

Ref_CssStyleDeclaration gft_Element_style(Ref_ElementRef el);

unsigned int gft_CssStyleDeclaration_length(Ref_CssStyleDeclaration style);

Option_Ref_String gft_CssStyleDeclaration_property_value(Ref_CssStyleDeclaration style,
                                                         const char *prop,
                                                         uint32_t prop_len);

void gft_CssStyleDeclaration_set_property(Ref_CssStyleDeclaration style,
                                          const char *prop,
                                          uint32_t prop_len,
                                          const char *val,
                                          uint32_t val_len);

void gft_CssStyleDeclaration_remove_property(Ref_CssStyleDeclaration style,
                                             const char *prop,
                                             uint32_t prop_len);

Ref_String gft_CssStyleDeclaration_css_text(Ref_CssStyleDeclaration style);

void gft_CssStyleDeclaration_set_css_text(Ref_CssStyleDeclaration style,
                                          const char *css_text,
                                          uint32_t css_text_len);

Ref_Renderer gft_Renderer_new(Ref_DocumentRef doc, Ref_Window win);

void gft_Renderer_render(Ref_Renderer renderer);

void gft_Renderer_resize(Ref_Renderer renderer, float width, float height);
