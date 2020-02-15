#include "lib.h"
#include <stdio.h>

void cool_function(int i, char c, CoolStruct* cs) {

  printf("Here is call from rust data: %d %c cool struct: x: %d, %d", i, c, cs->x, cs->y);

};