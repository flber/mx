#include "raylib.h"

struct CompRenderable {
  Texture2D texture;
} CompRenderable;

struct CompPosition {
  int x;
  int y;
} CompPosition;

typedef struct Component {
  char *t;
  union type {
    struct CompRenderable c_renderable;
    struct CompPosition c_position;
  } type;
} Component;
