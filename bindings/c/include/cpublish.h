#ifndef cpublish_h
#define cpublish_h

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef enum CPublishStatusType {
  CPublishStatusTypeOk,
  CPublishStatusTypeError,
} CPublishStatusType;

typedef enum CPublishValueType {
  CPublishValueTypeNone,
  CPublishValueTypeBoolean,
  CPublishValueTypeInteger,
  CPublishValueTypeFloat,
  CPublishValueTypeString,
  CPublishValueTypeArray,
  CPublishValueTypeObject,
} CPublishValueType;

typedef struct CPublishContext CPublishContext;

typedef struct CPublishContextIter CPublishContextIter;

typedef struct CPublishValue CPublishValue;

typedef struct CPublishValueIterArray CPublishValueIterArray;

typedef struct CPublishValueIterObject CPublishValueIterObject;

typedef struct CPublishStatus {
  enum CPublishStatusType status;
  const char *message;
} CPublishStatus;

/**
 * The CPublishStringView creates a borrowed pointer to a C style string.
 *
 * # Safety
 *
 * The pointer must not outlive the container that owns the string. Also, the
 * pointer should not be null, but that is not a strict requirement.
 */
typedef struct CPublishStringView {
  /**
   * The borrowed pointer to a string.
   *
   * # Safety
   *
   * The string must not outlive the container that owns it.
   */
  const char *string;
} CPublishStringView;

typedef struct CPublishBasePublish {
  struct CPublishContext *(*pre_publish_fn)(const struct CPublishBasePublish *publish,
                                            const struct CPublishContext *context,
                                            struct CPublishStatus *status);
  void (*rollback_pre_publish_fn)(const struct CPublishBasePublish *publish,
                                  const struct CPublishContext *context,
                                  struct CPublishStatus *status);
  struct CPublishContext *(*publish_fn)(const struct CPublishBasePublish *publish,
                                        const struct CPublishContext *context,
                                        struct CPublishStatus *status);
  void (*rollback_publish_fn)(const struct CPublishBasePublish *publish,
                              const struct CPublishContext *context,
                              struct CPublishStatus *status);
  struct CPublishContext *(*post_publish_fn)(const struct CPublishBasePublish *publish,
                                             const struct CPublishContext *context,
                                             struct CPublishStatus *status);
  void (*rollback_post_publish_fn)(const struct CPublishBasePublish *publish,
                                   const struct CPublishContext *context,
                                   struct CPublishStatus *status);
} CPublishBasePublish;

/**
 * The CPublishString contains an owned pointer to a C style string.
 *
 * # Safety
 *
 * The pointer to the string must be destroyed with `cpublish_string_destroy`
 * once it is no longer needed. Also, the pointer must not be modified at all
 * by any functions not exposed by the validation library.
 *
 * Internally, if a CPublishString is created, the system will create a copy of
 * the string being pointed to.
 */
typedef struct CPublishString {
  /**
   * The owned pointer to a string.
   *
   * # Safety
   *
   * This should not be modified at all outside of the validation library.
   * Also, it should only be destroyed with `cpublish_string_destroy`.
   */
  char *string;
  /**
   * Destroy the owned data.
   *
   * # Safety
   *
   * The destroy function should be called once at most.
   *
   * The destroy function should handle if the string pointer is null.
   */
  void (*destroy_fn)(struct CPublishString*);
} CPublishString;

struct CPublishContext *cpublish_context_clone(const struct CPublishContext *context,
                                               struct CPublishStatus *status);

void cpublish_context_destroy(struct CPublishContext *context);

const struct CPublishValue *cpublish_context_get(const struct CPublishContext *context,
                                                 const char *key,
                                                 struct CPublishStatus *status);

bool cpublish_context_is_empty(const struct CPublishContext *context,
                               struct CPublishStatus *status);

struct CPublishContextIter *cpublish_context_iter(const struct CPublishContext *context,
                                                  struct CPublishStatus *status);

void cpublish_context_iter_destroy(struct CPublishContextIter *iter);

bool cpublish_context_iter_is_done(struct CPublishContextIter *iter, struct CPublishStatus *status);

struct CPublishStringView cpublish_context_iter_key(struct CPublishContextIter *iter,
                                                    struct CPublishStatus *status);

void cpublish_context_iter_next(struct CPublishContextIter *iter, struct CPublishStatus *status);

const struct CPublishValue *cpublish_context_iter_value(struct CPublishContextIter *iter,
                                                        struct CPublishStatus *status);

size_t cpublish_context_len(const struct CPublishContext *context, struct CPublishStatus *status);

struct CPublishContext *cpublish_context_new(void);

void cpublish_context_set(struct CPublishContext *context,
                          const char *key,
                          const struct CPublishValue *value,
                          struct CPublishStatus *status);

void cpublish_context_set_bool(struct CPublishContext *context,
                               const char *key,
                               bool value,
                               struct CPublishStatus *status);

void cpublish_context_set_float(struct CPublishContext *context,
                                const char *key,
                                double value,
                                struct CPublishStatus *status);

void cpublish_context_set_int(struct CPublishContext *context,
                              const char *key,
                              int64_t value,
                              struct CPublishStatus *status);

void cpublish_context_set_none(struct CPublishContext *context,
                               const char *key,
                               struct CPublishStatus *status);

void cpublish_context_set_string(struct CPublishContext *context,
                                 const char *key,
                                 const char *value,
                                 struct CPublishStatus *status);

struct CPublishContext *cpublish_publish_default_error_publish(const struct CPublishBasePublish *publish,
                                                               const struct CPublishContext *context,
                                                               struct CPublishStatus *status);

struct CPublishContext *cpublish_publish_default_publish(const struct CPublishBasePublish *publish,
                                                         const struct CPublishContext *context,
                                                         struct CPublishStatus *status);

void cpublish_publish_default_rollback_publish(const struct CPublishBasePublish *publish,
                                               const struct CPublishContext *context,
                                               struct CPublishStatus *status);

struct CPublishBasePublish cpublish_publish_new_default(void);

struct CPublishContext *cpublish_publish_post_publish(const struct CPublishBasePublish *publish,
                                                      const struct CPublishContext *context,
                                                      struct CPublishStatus *status);

struct CPublishContext *cpublish_publish_pre_publish(const struct CPublishBasePublish *publish,
                                                     const struct CPublishContext *context,
                                                     struct CPublishStatus *status);

struct CPublishContext *cpublish_publish_publish(const struct CPublishBasePublish *publish,
                                                 const struct CPublishContext *context,
                                                 struct CPublishStatus *status);

void cpublish_publish_rollback_post_publish(const struct CPublishBasePublish *publish,
                                            const struct CPublishContext *context,
                                            struct CPublishStatus *status);

void cpublish_publish_rollback_pre_publish(const struct CPublishBasePublish *publish,
                                           const struct CPublishContext *context,
                                           struct CPublishStatus *status);

void cpublish_publish_rollback_publish(const struct CPublishBasePublish *publish,
                                       const struct CPublishContext *context,
                                       struct CPublishStatus *status);

struct CPublishContext *cpublish_run(const struct CPublishBasePublish *publish,
                                     struct CPublishStatus *status);

void cpublish_status_destroy(struct CPublishStatus *status);

void cpublish_status_error(struct CPublishStatus *status, const char *message);

void cpublish_status_ok(struct CPublishStatus *status);

/**
 * Destroy a string pointer.
 *
 * # Safety
 *
 * The pointer must not be null, and must not already have been destroyed (AKA:
 * double free). Once the destroy function is called, all pointers to the
 * string are invalid.
 */
void cpublish_string_destroy(struct CPublishString *string);

const struct CPublishValue *cpublish_value_array_get(const struct CPublishValue *value,
                                                     size_t index,
                                                     struct CPublishStatus *status);

struct CPublishValueIterArray *cpublish_value_array_iter(const struct CPublishValue *value,
                                                         struct CPublishStatus *status);

size_t cpublish_value_array_len(const struct CPublishValue *value, struct CPublishStatus *status);

void cpublish_value_array_push(struct CPublishValue *value,
                               const struct CPublishValue *item,
                               struct CPublishStatus *status);

bool cpublish_value_bool(const struct CPublishValue *value, struct CPublishStatus *status);

void cpublish_value_destroy(struct CPublishValue *value);

double cpublish_value_float(const struct CPublishValue *value, struct CPublishStatus *status);

int64_t cpublish_value_int(const struct CPublishValue *value, struct CPublishStatus *status);

void cpublish_value_iter_array_destroy(struct CPublishValueIterArray *iter);

bool cpublish_value_iter_array_is_done(struct CPublishValueIterArray *iter,
                                       struct CPublishStatus *status);

void cpublish_value_iter_array_next(struct CPublishValueIterArray *iter,
                                    struct CPublishStatus *status);

const struct CPublishValue *cpublish_value_iter_array_value(struct CPublishValueIterArray *iter,
                                                            struct CPublishStatus *status);

void cpublish_value_iter_object_destroy(struct CPublishValueIterObject *iter);

bool cpublish_value_iter_object_is_done(struct CPublishValueIterObject *iter,
                                        struct CPublishStatus *status);

struct CPublishStringView cpublish_value_iter_object_key(struct CPublishValueIterObject *iter,
                                                         struct CPublishStatus *status);

void cpublish_value_iter_object_next(struct CPublishValueIterObject *iter,
                                     struct CPublishStatus *status);

const struct CPublishValue *cpublish_value_iter_object_value(struct CPublishValueIterObject *iter,
                                                             struct CPublishStatus *status);

struct CPublishValue *cpublish_value_new_array(void);

struct CPublishValue *cpublish_value_new_array_with_capacity(size_t capacity);

struct CPublishValue *cpublish_value_new_bool(bool value);

struct CPublishValue *cpublish_value_new_float(double value);

struct CPublishValue *cpublish_value_new_int(int64_t value);

struct CPublishValue *cpublish_value_new_none(void);

struct CPublishValue *cpublish_value_new_object(void);

struct CPublishValue *cpublish_value_new_object_with_capacity(size_t capacity);

struct CPublishValue *cpublish_value_new_string(const char *value);

const struct CPublishValue *cpublish_value_object_get(const struct CPublishValue *value,
                                                      const char *key,
                                                      struct CPublishStatus *status);

void cpublish_value_object_insert(struct CPublishValue *value,
                                  const char *key,
                                  const struct CPublishValue *item,
                                  struct CPublishStatus *status);

struct CPublishValueIterObject *cpublish_value_object_iter(const struct CPublishValue *value,
                                                           struct CPublishStatus *status);

size_t cpublish_value_object_len(const struct CPublishValue *value, struct CPublishStatus *status);

struct CPublishString cpublish_value_string(const struct CPublishValue *value,
                                            struct CPublishStatus *status);

enum CPublishValueType cpublish_value_type(const struct CPublishValue *value,
                                           struct CPublishStatus *status);

#endif /* cpublish_h */
