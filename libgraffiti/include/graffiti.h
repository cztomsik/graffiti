#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum NodeKind {
  Element = 1,
  Text = 3,
  Comment = 8,
  Document = 9,
};
typedef uint32_t NodeKind;

typedef struct AABB AABB;

typedef struct Option_NodeId Option_NodeId;

typedef uint32_t Id_String;

typedef uint32_t Id_App;

typedef uint32_t Id_Window;

typedef uint32_t Id_Document;

typedef uint32_t Id_DomNode;

typedef Id_DomNode NodeId;





uintptr_t gft_String_bytes_len(Id_String string);

void gft_String_copy(Id_String string, uint8_t *dest_buf);

void gft_String_drop(Id_String string);

void gft_App_init(void);

void gft_App_tick(Id_App app);

void gft_App_wake_up(Id_App app);

void gft_App_drop(Id_App app);

Id_Window gft_Window_new(const uint8_t *title, uintptr_t title_len, int32_t width, int32_t height);

Id_String gft_Window_title(Id_Window win);

void gft_Window_set_title(Id_Window win, const uint8_t *title, uintptr_t title_len);

int32_t gft_Window_width(Id_Window win);

int32_t gft_Window_height(Id_Window win);

void gft_Window_resize(Id_Window win, int32_t width, int32_t height);

bool gft_Window_should_close(Id_Window win);

void gft_Window_show(Id_Window win);

void gft_Window_hide(Id_Window win);

void gft_Window_focus(Id_Window win);

void gft_Window_minimize(Id_Window win);

void gft_Window_maximize(Id_Window win);

void gft_Window_restore(Id_Window win);

void gft_Window_drop(Id_Window window);

Id_Document gft_Document_new(void);

NodeId gft_Document_root(Id_Document doc);

NodeId gft_Document_create_element(Id_Document doc,
                                   const uint8_t *local_name,
                                   uintptr_t local_name_len);

NodeId gft_Document_create_text_node(Id_Document doc, const uint8_t *data, uintptr_t data_len);

NodeKind gft_Document_node_kind(Id_Document doc, NodeId node);

struct Option_NodeId gft_Document_node_parent_node(Id_Document doc, NodeId node);

struct Option_NodeId gft_Document_node_first_child(Id_Document doc, NodeId node);

struct Option_NodeId gft_Document_node_last_child(Id_Document doc, NodeId node);

struct Option_NodeId gft_Document_node_previous_sibling(Id_Document doc, NodeId node);

struct Option_NodeId gft_Document_node_next_sibling(Id_Document doc, NodeId node);

void gft_Document_append_child(Id_Document doc, NodeId parent, NodeId child);

void gft_Document_insert_before(Id_Document doc, NodeId parent, NodeId child, NodeId before);

void gft_Document_remove_child(Id_Document doc, NodeId parent, NodeId child);

struct Option_NodeId gft_Document_query_selector(Id_Document doc,
                                                 NodeId node,
                                                 const uint8_t *selector,
                                                 uintptr_t selector_len);

void gft_Document_drop(Id_Document doc);
