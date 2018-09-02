#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct {
  int32_t state;
  float dec;
} test_data;

typedef int64_t Handle;

test_data create_data(void);

Handle create_handle(void);

double handle_test(Handle handle);

double simple_test(int32_t val, int64_t fl);

double test(int32_t val, test_data data);
