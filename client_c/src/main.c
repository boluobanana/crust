#include <stdio.h>
#include "../../lib_rust/lib_rust.h"

int main()
{

  printf("Hello, C! \n");
  int rust_add_result = rust_add(1, 2);
  const char* s = rust_add_prefix("cccc string");
  int arr [] = { 2, 3, 66, 1 };
  int sum = rust_sum(arr, 4);

  printf("rust add result: %d \n", rust_add_result);
  printf("rust add prefix result: %s \n", s);
  printf("rust sum result: %d \n", sum);

  return 0;
}