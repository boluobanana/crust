#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

int32_t rust_add(int32_t a, int32_t b);

const char *rust_add_prefix(const char *text);

int32_t rust_sum(const int32_t *ptr, uintptr_t length);
