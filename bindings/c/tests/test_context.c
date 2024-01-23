#include <float.h>
#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include <cmocka.h>

#include "cpublish.h"
#include "test_utils.h"

/*
----------------------------------------------------------------------------
  Checks
*/

static void test_cpublish_context_set_none_success(void **state) {
  CPublishStatus status;

  CPublishContext *context = cpublish_context_new();
  assert_non_null(context);

  size_t len;
  len = cpublish_context_len(context, &status);
  validate_status_ok(&status);
  assert_int_equal(len, 0);
  assert_int_equal(cpublish_context_is_empty(context, &status), true);

  cpublish_context_set_none(context, "test", &status);
  validate_status_ok(&status);

  len = cpublish_context_len(context, &status);
  assert_int_equal(len, 1);
  assert_int_equal(cpublish_context_is_empty(context, &status), false);

  const CPublishValue *value = cpublish_context_get(context, "test", &status);
  assert_non_null(value);
  validate_status_ok(&status);

  assert_int_equal(cpublish_value_type(value, &status), CPublishValueTypeNone);
  validate_status_ok(&status);

  cpublish_context_destroy(context);
}

static void test_cpublish_context_set_bool_success(void **state) {
  CPublishStatus status;

  CPublishContext *context = cpublish_context_new();
  assert_non_null(context);

  size_t len;
  len = cpublish_context_len(context, &status);
  validate_status_ok(&status);
  assert_int_equal(len, 0);
  assert_int_equal(cpublish_context_is_empty(context, &status), true);

  cpublish_context_set_bool(context, "test", true, &status);
  validate_status_ok(&status);

  len = cpublish_context_len(context, &status);
  assert_int_equal(len, 1);
  assert_int_equal(cpublish_context_is_empty(context, &status), false);

  const CPublishValue *value = cpublish_context_get(context, "test", &status);
  assert_non_null(value);
  validate_status_ok(&status);

  assert_int_equal(cpublish_value_type(value, &status),
                   CPublishValueTypeBoolean);
  validate_status_ok(&status);

  assert_true(cpublish_value_bool(value, &status));
  validate_status_ok(&status);

  cpublish_context_destroy(context);
}

static void test_cpublish_context_set_int_success(void **state) {
  CPublishStatus status;

  CPublishContext *context = cpublish_context_new();
  assert_non_null(context);

  size_t len;
  len = cpublish_context_len(context, &status);
  validate_status_ok(&status);
  assert_int_equal(len, 0);
  assert_int_equal(cpublish_context_is_empty(context, &status), true);

  cpublish_context_set_int(context, "test", 1, &status);
  validate_status_ok(&status);

  len = cpublish_context_len(context, &status);
  assert_int_equal(len, 1);
  assert_int_equal(cpublish_context_is_empty(context, &status), false);

  const CPublishValue *value = cpublish_context_get(context, "test", &status);
  assert_non_null(value);
  validate_status_ok(&status);

  assert_int_equal(cpublish_value_type(value, &status),
                   CPublishValueTypeInteger);
  validate_status_ok(&status);

  assert_int_equal(cpublish_value_int(value, &status), 1);
  validate_status_ok(&status);

  cpublish_context_destroy(context);
}

static void test_cpublish_context_set_float_success(void **state) {
  CPublishStatus status;

  CPublishContext *context = cpublish_context_new();
  assert_non_null(context);

  size_t len;
  len = cpublish_context_len(context, &status);
  validate_status_ok(&status);
  assert_int_equal(len, 0);
  assert_int_equal(cpublish_context_is_empty(context, &status), true);

  cpublish_context_set_float(context, "test", 1.0, &status);
  validate_status_ok(&status);

  len = cpublish_context_len(context, &status);
  assert_int_equal(len, 1);
  assert_int_equal(cpublish_context_is_empty(context, &status), false);

  const CPublishValue *value = cpublish_context_get(context, "test", &status);
  assert_non_null(value);
  validate_status_ok(&status);

  assert_int_equal(cpublish_value_type(value, &status), CPublishValueTypeFloat);
  validate_status_ok(&status);

  assert_float_equal(cpublish_value_float(value, &status), 1.0, DBL_EPSILON);
  validate_status_ok(&status);

  cpublish_context_destroy(context);
}

static void test_cpublish_context_set_string_success(void **state) {
  CPublishStatus status;

  CPublishContext *context = cpublish_context_new();
  assert_non_null(context);

  size_t len;
  len = cpublish_context_len(context, &status);
  validate_status_ok(&status);
  assert_int_equal(len, 0);
  assert_int_equal(cpublish_context_is_empty(context, &status), true);

  cpublish_context_set_string(context, "test", "test", &status);
  validate_status_ok(&status);

  len = cpublish_context_len(context, &status);
  assert_int_equal(len, 1);
  assert_int_equal(cpublish_context_is_empty(context, &status), false);

  const CPublishValue *value = cpublish_context_get(context, "test", &status);
  assert_non_null(value);
  validate_status_ok(&status);

  assert_int_equal(cpublish_value_type(value, &status),
                   CPublishValueTypeString);
  validate_status_ok(&status);

  CPublishString result = cpublish_value_string(value, &status);
  validate_status_ok(&status);
  assert_string_equal(result.string, "test");
  validate_status_ok(&status);

  cpublish_string_destroy(&result);

  cpublish_context_destroy(context);
}

static void test_cpublish_context_set_success(void **state) {
  CPublishStatus status;

  CPublishContext *context = cpublish_context_new();
  assert_non_null(context);

  size_t len;
  len = cpublish_context_len(context, &status);
  validate_status_ok(&status);
  assert_int_equal(len, 0);
  assert_int_equal(cpublish_context_is_empty(context, &status), true);

  CPublishValue *value = cpublish_value_new_none();
  assert_non_null(value);

  cpublish_context_set(context, "test", value, &status);
  validate_status_ok(&status);
  cpublish_value_destroy(value);

  len = cpublish_context_len(context, &status);
  assert_int_equal(len, 1);
  assert_int_equal(cpublish_context_is_empty(context, &status), false);

  const CPublishValue *value_out =
      cpublish_context_get(context, "test", &status);
  assert_non_null(value_out);
  validate_status_ok(&status);

  assert_int_equal(cpublish_value_type(value_out, &status),
                   CPublishValueTypeNone);
  validate_status_ok(&status);

  cpublish_context_destroy(context);
}

static void test_cpublish_context_clone_success(void **state) {
  CPublishStatus status;

  CPublishContext *context = cpublish_context_new();
  assert_non_null(context);

  size_t len;
  len = cpublish_context_len(context, &status);
  validate_status_ok(&status);
  assert_int_equal(len, 0);
  assert_int_equal(cpublish_context_is_empty(context, &status), true);

  cpublish_context_set_none(context, "test", &status);
  validate_status_ok(&status);

  len = cpublish_context_len(context, &status);
  assert_int_equal(len, 1);
  assert_int_equal(cpublish_context_is_empty(context, &status), false);

  CPublishContext *cloned_context = cpublish_context_clone(context, &status);

  const CPublishValue *value =
      cpublish_context_get(cloned_context, "test", &status);
  assert_non_null(value);
  validate_status_ok(&status);

  assert_int_equal(cpublish_value_type(value, &status), CPublishValueTypeNone);
  validate_status_ok(&status);

  cpublish_context_destroy(context);
  cpublish_context_destroy(cloned_context);
}

int main(void) {
  const struct CMUnitTest tests[] = {
      cmocka_unit_test(test_cpublish_context_set_none_success),
      cmocka_unit_test(test_cpublish_context_set_bool_success),
      cmocka_unit_test(test_cpublish_context_set_int_success),
      cmocka_unit_test(test_cpublish_context_set_float_success),
      cmocka_unit_test(test_cpublish_context_set_string_success),
      cmocka_unit_test(test_cpublish_context_set_success),
      cmocka_unit_test(test_cpublish_context_clone_success),
  };

  return cmocka_run_group_tests(tests, NULL, NULL);
}
