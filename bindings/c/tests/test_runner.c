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

CPublishContext *pre_publish_should_pass(const CPublishBasePublish *publish,
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

  cpublish_context_set_string(ctx, "pre_publish_key", "pre_publish_value",
                              status);

  if (status != NULL && status->status == CPublishStatusTypeError) {
    return NULL;
  }

  return ctx;
}

void rollback_pre_publish_should_pass(const CPublishBasePublish *publish,
                                      const CPublishContext *context,
                                      CPublishStatus *status) {
  cpublish_status_ok(status);
  assert_non_null(publish);
  assert_non_null(context);
  assert_non_null(status);

  cpublish_status_ok(status);
}

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

  cpublish_context_set_string(ctx, "publish_key", "publish_value", status);

  if (status != NULL && status->status == CPublishStatusTypeError) {
    return NULL;
  }

  return ctx;
}

void rollback_publish_should_pass(const CPublishBasePublish *publish,
                                  const CPublishContext *context,
                                  CPublishStatus *status) {
  cpublish_status_ok(status);
  assert_non_null(publish);
  assert_non_null(context);
  assert_non_null(status);

  cpublish_status_ok(status);
}

CPublishContext *post_publish_should_pass(const CPublishBasePublish *publish,
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

  cpublish_context_set_string(ctx, "post_publish_key", "post_publish_value",
                              status);

  if (status != NULL && status->status == CPublishStatusTypeError) {
    return NULL;
  }

  return ctx;
}

void rollback_post_publish_should_pass(const CPublishBasePublish *publish,
                                       const CPublishContext *context,
                                       CPublishStatus *status) {
  cpublish_status_ok(status);
  assert_non_null(publish);
  assert_non_null(context);
  assert_non_null(status);

  cpublish_status_ok(status);
}

/*
----------------------------------------------------------------------------
  Checks
*/
static void test_run_success(void **state) {
  CPublishBasePublish publish = cpublish_publish_new_default();
  publish.pre_publish_fn = pre_publish_should_pass;
  publish.rollback_pre_publish_fn = rollback_pre_publish_should_pass;
  publish.publish_fn = publish_should_pass;
  publish.rollback_publish_fn = rollback_publish_should_pass;
  publish.post_publish_fn = post_publish_should_pass;
  publish.rollback_post_publish_fn = rollback_post_publish_should_pass;

  CPublishStatus status;

  CPublishContext *context = cpublish_run(&publish, &status);
  validate_status_ok(&status);
  assert_non_null(context);

  const CPublishValue *pre_publish_value =
      cpublish_context_get(context, "pre_publish_key", &status);
  validate_status_ok(&status);
  assert_non_null(pre_publish_value);
  CPublishString pre_publish_result =
      cpublish_value_string(pre_publish_value, &status);
  validate_status_ok(&status);
  assert_string_equal(pre_publish_result.string, "pre_publish_value");
  cpublish_string_destroy(&pre_publish_result);

  const CPublishValue *publish_value =
      cpublish_context_get(context, "publish_key", &status);
  validate_status_ok(&status);
  assert_non_null(publish_value);
  CPublishString publish_result = cpublish_value_string(publish_value, &status);
  validate_status_ok(&status);
  assert_string_equal(publish_result.string, "publish_value");
  cpublish_string_destroy(&publish_result);

  const CPublishValue *post_publish_value =
      cpublish_context_get(context, "post_publish_key", &status);
  validate_status_ok(&status);
  assert_non_null(post_publish_value);
  CPublishString post_publish_result =
      cpublish_value_string(post_publish_value, &status);
  validate_status_ok(&status);
  assert_string_equal(post_publish_result.string, "post_publish_value");
  cpublish_string_destroy(&post_publish_result);

  cpublish_context_destroy(context);
}

int main(void) {
  const struct CMUnitTest tests[] = {
      cmocka_unit_test(test_run_success),
  };

  return cmocka_run_group_tests(tests, NULL, NULL);
}
