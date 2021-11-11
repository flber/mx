#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <string.h>
#include "raylib.h"
#include "ecs.c"

int main(void)
{
  const int screenWidth = 800;
  const int screenHeight = 450;

  InitWindow(screenWidth, screenHeight, "ecs");

  Manager m = m_new();
  Entity e1 = m_create_entity(&m);
  Entity e2 = m_create_entity(&m);
  Entity e3 = m_create_entity(&m);

  Component comp;
  m_add_component(&m, e2.index, comp);
  Component comps[3];
  m_add_components(&m, e3.index, comps, 3);

  for(int i = 0; i < m.len; i++)
    printf("comps len: %d\n", m.entities[i].comps_len);

  
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

  	char text[20]; 
  	sprintf(text, "entities: %d", m.len);

    DrawText(text, 10, 40, 20, BLACK);
    EndDrawing();
  }

  CloseWindow();
  return 0;
}
