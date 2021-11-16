#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <string.h>
#include "manager.h"

void
c_new(Entity_ptr ctx)
{
  ctx->in_use = 0;
  Component comps[NCMPS];
  memcpy(ctx->comps, comps, sizeof(comps));
  ctx->comps_len = 0;
}

void
m_new(Manager_ptr ctx)
{
  Entity entities[NENTS];
  memcpy(ctx->entities, entities, sizeof(entities));
  ctx->len = 0;
}

int
m_create_entity(Manager_ptr ctx)
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
m_remove_entity(Manager_ptr ctx, int id)
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
m_add_component(Manager_ptr ctx, int id, Component comp)
{
  ctx->entities[id]->comps[ctx->entities[id]->comps_len] = comp;
  ctx->entities[id]->comps_len++;
}

void
m_add_components(Manager_ptr ctx, int id, Component comps[], int size)
{
  for(int i = 0; i < size; i++){
    ctx->entities[id]->comps[ctx->entities[id]->comps_len] = comps[i];
    ctx->entities[id]->comps_len++;
  }
}

Component*
m_components_of(Manager_ptr ctx, int id)
{
  return ctx->entities[id]->comps;
}

int
m_has_component_of_type(Manager_ptr ctx, int id, char *comp_name)
{
  for(int i = 0; i < ctx->entities[id]->comps_len; i++)
    if(ctx->entities[id]->comps[i].t == comp_name)
      return 1;
  return 0;
}
