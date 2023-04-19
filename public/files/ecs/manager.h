/*#include "components.c"

#define NENTS 2048
#define NCMPS 128

typedef struct Entity {
  int in_use;
  Component comps[NCMPS];
  int comps_len;
} Entity;

typedef Entity * Entity_ptr;
void e_new (Entity_ptr ctx);

typedef struct Manager {
  Entity_ptr entities[NENTS];
  int len;
} Manager;

typedef Manager * Manager_ptr;

void        m_new                       (Manager_ptr ctx);
int         m_create_entity             (Manager_ptr ctx);
void        m_remove_entity             (Manager_ptr ctx, int id);
void        m_add_component             (Manager_ptr ctx, int id, Component comp);
void        m_add_components            (Manager_ptr ctx, int id, Component comps[], int size);
Component*  m_components_of             (Manager_ptr ctx, int id);
int         m_has_component_of_type     (Manager_ptr ctx, int id, char *comp_name);
Component   m_get_component             (Manager_ptr ctx, int id, char *comp_name);
int*        m_entities_with_component   (Manager_ptr ctx, char *comp_name);
int*        m_entities_with_components  (Manager_ptr ctx, char **comp_name);
*/
