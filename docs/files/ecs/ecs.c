#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <string.h>
#include "components.c"

#define NENTS 2048
#define NCMPS 128

typedef struct Entity {
  int index;
  Component *comps;
  int comps_len;
} Entity;

typedef struct Manager {
  Entity* entities;
  int len;
} Manager;

Manager   m_new                       ();
Entity    m_create_entity             (Manager *ctx);
void      m_remove_entity             (Manager *ctx, int id);
void      m_add_component             (Manager *ctx, int id, Component comp);
void      m_add_componenets           (Manager *ctx, int id, Component comps[], int size);
Entity    m_components_of             (Manager *ctx, int id);
int       m_has_component_of_type     (Manager *ctx, int id, char *comp_name);
Component m_get_component             (Manager *ctx, int id, char *comp_name);
int*      m_entities_with_component   (Manager *ctx, char *comp_name);
int*      m_entities_with_components  (Manager *ctx, char **comp_name);

Manager
m_new()
{
  Entity e[NENTS];
  int len = 0;

  Manager m = {
    e,
    len,
  };
  return m;
}

Entity
m_create_entity(Manager *ctx)
{
  Component c[NCMPS];
  Entity e = {
    ctx->len,
    c,
    0,
  };
  ctx->entities[ctx->len] = e;
  ctx->len++;
  return ctx->entities[ctx->len-1];
}

void
m_remove_entity(Manager *ctx, int id)
{
  fprintf(stderr, "removing entity");
  Component c[NCMPS];
  Entity n = {
    -1,
    c,
    0,
  };
  ctx->entities[id] = n;

  for(int i = id; i < NENTS-1; i++){
    ctx->entities[i] = ctx->entities[i+1];
  }
  ctx->entities[NENTS] = n;
  ctx->len--;
}

void
m_add_component(Manager *ctx, int id, Component comp)
{
  ctx->entities[id].comps[ctx->entities[id].comps_len] = comp;
  ctx->entities[id].comps_len++;
}

void
m_add_components(Manager *ctx, int id, Component comps[], int size)
{
  for(int i = 0; i < size; i++){
    ctx->entities[id].comps[ctx->entities[id].comps_len] = comps[i];
    ctx->entities[id].comps_len++;
  }
}
