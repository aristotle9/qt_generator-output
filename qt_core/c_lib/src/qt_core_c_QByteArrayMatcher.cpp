#include "qt_core_c_QByteArrayMatcher.h"

void qt_core_c_QByteArrayMatcher_constructor_no_args(QByteArrayMatcher* output) {
  new(output) QByteArrayMatcher();
}

void qt_core_c_QByteArrayMatcher_constructor_other(const QByteArrayMatcher* other, QByteArrayMatcher* output) {
  new(output) QByteArrayMatcher(*other);
}

void qt_core_c_QByteArrayMatcher_constructor_pattern(const QByteArray* pattern, QByteArrayMatcher* output) {
  new(output) QByteArrayMatcher(*pattern);
}

void qt_core_c_QByteArrayMatcher_constructor_pattern_length(const char* pattern, int length, QByteArrayMatcher* output) {
  new(output) QByteArrayMatcher(pattern, length);
}

void qt_core_c_QByteArrayMatcher_destructor(QByteArrayMatcher* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

int qt_core_c_QByteArrayMatcher_indexIn_ba(const QByteArrayMatcher* this_ptr, const QByteArray* ba) {
  return this_ptr->indexIn(*ba);
}

int qt_core_c_QByteArrayMatcher_indexIn_ba_from(const QByteArrayMatcher* this_ptr, const QByteArray* ba, int from) {
  return this_ptr->indexIn(*ba, from);
}

int qt_core_c_QByteArrayMatcher_indexIn_str_len(const QByteArrayMatcher* this_ptr, const char* str, int len) {
  return this_ptr->indexIn(str, len);
}

int qt_core_c_QByteArrayMatcher_indexIn_str_len_from(const QByteArrayMatcher* this_ptr, const char* str, int len, int from) {
  return this_ptr->indexIn(str, len, from);
}

QByteArrayMatcher* qt_core_c_QByteArrayMatcher_operator_assign(QByteArrayMatcher* this_ptr, const QByteArrayMatcher* other) {
  return &this_ptr->operator=(*other);
}

void qt_core_c_QByteArrayMatcher_pattern_to_output(const QByteArrayMatcher* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->pattern());
}

void qt_core_c_QByteArrayMatcher_setPattern(QByteArrayMatcher* this_ptr, const QByteArray* pattern) {
  this_ptr->setPattern(*pattern);
}

