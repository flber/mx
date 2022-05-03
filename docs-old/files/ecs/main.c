#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <string.h>
#include "raylib.h"
#include "manager.c"

int main(void)
{
  const int screenWidth = 800;
  const int screenHeight = 450;

  InitWindow(screenWidth, screenHeight, "ecs");

  Entity entities[NENTS];
  Manager m_init;
  memcpy(m_init.entities, entities, sizeof(entities));
  m_init.len = 0;
  Manager_ptr m = &m_init;
  m_new(m);
  // printf("m.len: %d", m->len);

  /*
  int e1 = m_create_entity(m);

  Image image = LoadImage("selfie.png");
  Component comp_rend = {
    .t = "CompRenderable",
    .type.c_renderable = {
      LoadTextureFromImage(image),
    }
  };
  UnloadImage(image);
  Component comp_pos = {
    .t = "CompPosition",
    .type.c_position = {
      .x = 100,
      .y = 100,
    }
  };
  Component e1_comps[] = {comp_rend, comp_pos};
  m_add_components(m, e1, e1_comps, 2);

  puts("-----");
  printf("e1.comps[0]: %s\n", m_components_of(m, e1)[0].t);
  printf("e1.comps[1]: %s\n", m_components_of(m, e1)[1].t);
  puts("adding e2");
  int e2 = m_create_entity(m);
  Component e2_comp = {
    .t = "CompPosition",
    .type.c_position = {
      .x = 12,
      .y = 12,
    }
  };
  m_add_component(m, e2, e2_comp);
  printf("e1.comps[0]: %s\n", m_components_of(m, e1)[0].t);
  printf("e1.comps[1]: %s\n", m_components_of(m, e1)[1].t);
  printf("e1.comps[1].x: %d\n", m_components_of(m, e1)[1].type.c_position.x);
  printf("e2.comps[0].x: %d\n", m_components_of(m, e2)[0].type.c_position.x);
  puts("-----");
 
  SetTargetFPS(60);
  while (!WindowShouldClose())
  {
    // Update
    //----------------------------------------------------------------------------------

    // Draw
    //----------------------------------------------------------------------------------
    BeginDrawing();
    ClearBackground(RAYWHITE);
    DrawText("window", 10, 10, 20, BLACK);

  	char e[100];
  	for(int i = 0; i < m->len; i++){
  	  sprintf(e, "e[%i]: { %s, %i }", i, (m->entities[i]->in_use ? "true":"false"), m->entities[i]->comps_len);
      DrawText(e, 10, 40+(i*25), 20, BLACK);
  	}

    EndDrawing();
  }

  CloseWindow();
  */
  return 0;
}
