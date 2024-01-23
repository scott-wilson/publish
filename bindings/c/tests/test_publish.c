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

CPublishContext *publish_should_pass(const CPublishBasePublish *publish,
                                     const CPublishContext *context,
                                     CPublishStatus *status) {
  cpublish_status_ok(status);
  assert_non_null(publish);
  assert_non_null(context);
  assert_non_null(status);

  CPublishContext *ctx = cpublish_context_clone(context, status);

  if (status != NULL && status->status == CPublishStatusTypeError) {
    return NULL;
  }

  cpublish_context_set_bool(ctx, "test", true, status);

  if (status != NULL && status->status == CPublishStatusTypeError) {
    return NULL;
  }

  return ctx;
}

CPublishContext *publish_should_fail(const CPublishBasePublish *publish,
                                     const CPublishContext *context,
                                     CPublishStatus *status) {
  cpublish_status_ok(status);
  assert_non_null(publish);
  assert_non_null(context);
  assert_non_null(status);

  cpublish_status_error(status, "Should not be called");

  return NULL;
}

void rollback_should_pass(const CPublishBasePublish *publish,
                          const CPublishContext *context,
                          CPublishStatus *status) {
  cpublish_status_ok(status);
  assert_non_null(publish);
  assert_non_null(context);
  assert_non_null(status);

  cpublish_status_ok(status);
}

void rollback_should_fail(const CPublishBasePublish *publish,
                          const CPublishContext *context,
                          CPublishStatus *status) {
  cpublish_status_ok(status);
  assert_non_null(publish);
  assert_non_null(context);
  assert_non_null(status);

  cpublish_status_error(status, "Should not be called");
}

/*
----------------------------------------------------------------------------
  Checks
*/
static void test_cpublish_publish_new_default_success(void **state) {
  CPublishBasePublish publish = cpublish_publish_new_default();

  assert_ptr_equal(publish.pre_publish_fn, cpublish_publish_default_publish);
  assert_ptr_equal(publish.rollback_pre_publish_fn,
                   cpublish_publish_default_rollback_publish);
  assert_ptr_equal(publish.publish_fn, cpublish_publish_default_error_publish);
  assert_ptr_equal(publish.rollback_publish_fn,
                   cpublish_publish_default_rollback_publish);
  assert_ptr_equal(publish.post_publish_fn, cpublish_publish_default_publish);
  assert_ptr_equal(publish.rollback_post_publish_fn,
                   cpublish_publish_default_rollback_publish);
}

static void test_cpublish_publish_pre_publish_success(void **state) {
  CPublishBasePublish publish = cpublish_publish_new_default();
  publish.pre_publish_fn = publish_should_pass;
  publish.rollback_pre_publish_fn = rollback_should_fail;
  publish.publish_fn = publish_should_fail;
  publish.rollback_publish_fn = rollback_should_fail;
  publish.post_publish_fn = publish_should_fail;
  publish.rollback_post_publish_fn = rollback_should_fail;
  CPublishContext *context = cpublish_context_new();
  CPublishStatus status;
  cpublish_status_ok(&status);

  CPublishContext *ctx =
      cpublish_publish_pre_publish(&publish, context, &status);
  validate_status_ok(&status);
  assert_non_null(ctx);
  const CPublishValue *result = cpublish_context_get(ctx, "test", &status);
  validate_status_ok(&status);
  assert_non_null(result);

  CPublishValueType type = cpublish_value_type(result, &status);
  validate_status_ok(&status);
  assert_int_equal(type, CPublishValueTypeBoolean);

  cpublish_context_destroy(ctx);
  cpublish_context_destroy(context);
}

static void test_cpublish_publish_rollback_pre_publish_success(void **state) {
  CPublishBasePublish publish = cpublish_publish_new_default();
  publish.pre_publish_fn = publish_should_fail;
  publish.rollback_pre_publish_fn = rollback_should_pass;
  publish.publish_fn = publish_should_fail;
  publish.rollback_publish_fn = rollback_should_fail;
  publish.post_publish_fn = publish_should_fail;
  publish.rollback_post_publish_fn = rollback_should_fail;
  CPublishContext *context = cpublish_context_new();
  CPublishStatus status;
  cpublish_status_ok(&status);

  cpublish_publish_rollback_pre_publish(&publish, context, &status);
  validate_status_ok(&status);
  cpublish_context_destroy(context);
}

static void test_cpublish_publish_publish_success(void **state) {
  CPublishBasePublish publish = cpublish_publish_new_default();
  publish.pre_publish_fn = publish_should_fail;
  publish.rollback_pre_publish_fn = rollback_should_fail;
  publish.publish_fn = publish_should_pass;
  publish.rollback_publish_fn = rollback_should_fail;
  publish.post_publish_fn = publish_should_fail;
  publish.rollback_post_publish_fn = rollback_should_fail;
  CPublishContext *context = cpublish_context_new();
  CPublishStatus status;
  cpublish_status_ok(&status);

  CPublishContext *ctx = cpublish_publish_publish(&publish, context, &status);
  validate_status_ok(&status);
  assert_non_null(ctx);
  const CPublishValue *result = cpublish_context_get(ctx, "test", &status);
  validate_status_ok(&status);
  assert_non_null(result);

  CPublishValueType type = cpublish_value_type(result, &status);
  validate_status_ok(&status);
  assert_int_equal(type, CPublishValueTypeBoolean);

  cpublish_context_destroy(ctx);
  cpublish_context_destroy(context);
}

static void test_cpublish_publish_rollback_publish_success(void **state) {
  CPublishBasePublish publish = cpublish_publish_new_default();
  publish.pre_publish_fn = publish_should_fail;
  publish.rollback_pre_publish_fn = rollback_should_fail;
  publish.publish_fn = publish_should_fail;
  publish.rollback_publish_fn = rollback_should_pass;
  publish.post_publish_fn = publish_should_fail;
  publish.rollback_post_publish_fn = rollback_should_fail;
  CPublishContext *context = cpublish_context_new();
  CPublishStatus status;
  cpublish_status_ok(&status);

  cpublish_publish_rollback_publish(&publish, context, &status);
  validate_status_ok(&status);

  cpublish_context_destroy(context);
}

static void test_cpublish_publish_post_publish_success(void **state) {
  CPublishBasePublish publish = cpublish_publish_new_default();
  publish.pre_publish_fn = publish_should_fail;
  publish.rollback_pre_publish_fn = rollback_should_fail;
  publish.publish_fn = publish_should_fail;
  publish.rollback_publish_fn = rollback_should_fail;
  publish.post_publish_fn = publish_should_pass;
  publish.rollback_post_publish_fn = rollback_should_fail;
  CPublishContext *context = cpublish_context_new();
  CPublishStatus status;
  cpublish_status_ok(&status);

  CPublishContext *ctx =
      cpublish_publish_post_publish(&publish, context, &status);
  validate_status_ok(&status);
  assert_non_null(ctx);
  const CPublishValue *result = cpublish_context_get(ctx, "test", &status);
  validate_status_ok(&status);
  assert_non_null(result);

  CPublishValueType type = cpublish_value_type(result, &status);
  validate_status_ok(&status);
  assert_int_equal(type, CPublishValueTypeBoolean);

  cpublish_context_destroy(ctx);
  cpublish_context_destroy(context);
}

static void test_cpublish_publish_rollback_post_publish_success(void **state) {
  CPublishBasePublish publish = cpublish_publish_new_default();
  publish.pre_publish_fn = publish_should_fail;
  publish.rollback_pre_publish_fn = rollback_should_fail;
  publish.publish_fn = publish_should_fail;
  publish.rollback_publish_fn = rollback_should_fail;
  publish.post_publish_fn = publish_should_fail;
  publish.rollback_post_publish_fn = rollback_should_pass;
  CPublishContext *context = cpublish_context_new();
  CPublishStatus status;
  cpublish_status_ok(&status);

  cpublish_publish_rollback_post_publish(&publish, context, &status);
  validate_status_ok(&status);

  cpublish_context_destroy(context);
}

int main(void) {
  const struct CMUnitTest tests[] = {
      cmocka_unit_test(test_cpublish_publish_new_default_success),
      cmocka_unit_test(test_cpublish_publish_pre_publish_success),
      cmocka_unit_test(test_cpublish_publish_rollback_pre_publish_success),
      cmocka_unit_test(test_cpublish_publish_publish_success),
      cmocka_unit_test(test_cpublish_publish_rollback_publish_success),
      cmocka_unit_test(test_cpublish_publish_post_publish_success),
      cmocka_unit_test(test_cpublish_publish_rollback_post_publish_success),
  };

  return cmocka_run_group_tests(tests, NULL, NULL);
}
