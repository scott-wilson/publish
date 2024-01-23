#include <cmocka.h>

#include "cpublish.h"

void _validate_status_ok(const CPublishStatus *status, const char *const file,
                         const int line) {
  assert_non_null(status);

  if (status->status != CPublishStatusTypeOk) {
    fprintf(stderr, "file: %s:%d\n", file, line);
    fprintf(stderr, "status: %d\n", status->status);
    fprintf(stderr, "message: %s\n", status->message);
  }

  assert_int_equal(status->status, CPublishStatusTypeOk);
}

#define validate_status_ok(status)                                             \
  _validate_status_ok(status, __FILE__, __LINE__)
