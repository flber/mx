/*
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <string.h>
#include "components.c"

#define NENTS 2048
#define NCMPS 128

typedef struct Entity {
  int in_use;
  Component *comps[NCMPS];
  int comps_len;
} Entity;

typedef struct Manager {
  Entity *entities[NENTS];
  int len;
} Manager;

Manager     m_new                       ();
int         m_create_entity             (Manager *ctx);
void        m_remove_entity             (Manager *ctx, int id);
void        m_add_component             (Manager *ctx, int id, Component comp);
void        m_add_components            (Manager *ctx, int id, Component comps[], int size);
Component*  m_components_of             (Manager *ctx, int id);
int         m_has_component_of_type     (Manager *ctx, int id, char *comp_name);
Component   m_get_component             (Manager *ctx, int id, char *comp_name);
int*        m_entities_with_component   (Manager *ctx, char *comp_name);
int*        m_entities_with_components  (Manager *ctx, char **comp_name);

Manager
m_new()
{
  // Entity e[NENTS] = {};
  Manager m = {
    .entities = {},
    .len = 0,
  };
  return m;
}

int
m_create_entity(Manager *ctx)
{
  // Component c[NCMPS] = {};
  Entity e = {
    .in_use = 1,
    .comps = {},
    .comps_len = 0,
  };
  
  ctx->entities[ctx->len] = &e;
  return ctx->len++;
}

void
m_remove_entity(Manager *ctx, int id)
{
  // Component c[NCMPS] = {};
  Entity n = {
    .in_use = 0,
    .comps = {},
    .comps_len = 0,
  };
  ctx->entities[id] = &n;

  for(int i = id; ctx->entities[i]->in_use; i++)
    ctx->entities[i] = ctx->entities[i+1];
}

void
m_add_component(Manager *ctx, int id, Component comp)
{
  ctx->entities[id]->comps[ctx->entities[id]->comps_len] = &comp;
  ctx->entities[id]->comps_len++;
}

void
m_add_components(Manager *ctx, int id, Component comps[], int size)
{
  for(int i = 0; i < size; i++){
    ctx->entities[id]->comps[ctx->entities[id]->comps_len] = &comps[i];
    ctx->entities[id]->comps_len++;
  }
}

Component*
m_components_of(Manager *ctx, int id)
{
  return *ctx->entities[id]->comps;
}

int
m_has_component_of_type(Manager *ctx, int id, char *comp_name)
{
  for(int i = 0; i < ctx->entities[id]->comps_len; i++)
    if(ctx->entities[id]->comps[i]->t == comp_name)
      return 1;
  return 0;
}
*/
