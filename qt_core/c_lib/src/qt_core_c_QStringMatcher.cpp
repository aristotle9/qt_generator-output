#include "qt_core_c_QStringMatcher.h"

void qt_core_c_QStringMatcher_constructor_no_args(QStringMatcher* output) {
  new(output) QStringMatcher();
}

void qt_core_c_QStringMatcher_constructor_other(const QStringMatcher* other, QStringMatcher* output) {
  new(output) QStringMatcher(*other);
}

void qt_core_c_QStringMatcher_constructor_pattern(const QString* pattern, QStringMatcher* output) {
  new(output) QStringMatcher(*pattern);
}

void qt_core_c_QStringMatcher_constructor_pattern_cs(const QString* pattern, const Qt::CaseSensitivity* cs, QStringMatcher* output) {
  new(output) QStringMatcher(*pattern, *cs);
}

void qt_core_c_QStringMatcher_constructor_uc_len(const QChar* uc, int len, QStringMatcher* output) {
  new(output) QStringMatcher(uc, len);
}

void qt_core_c_QStringMatcher_constructor_uc_len_cs(const QChar* uc, int len, const Qt::CaseSensitivity* cs, QStringMatcher* output) {
  new(output) QStringMatcher(uc, len, *cs);
}

void qt_core_c_QStringMatcher_destructor(QStringMatcher* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

int qt_core_c_QStringMatcher_indexIn_str(const QStringMatcher* this_ptr, const QString* str) {
  return this_ptr->indexIn(*str);
}

int qt_core_c_QStringMatcher_indexIn_str_from(const QStringMatcher* this_ptr, const QString* str, int from) {
  return this_ptr->indexIn(*str, from);
}

int qt_core_c_QStringMatcher_indexIn_str_length(const QStringMatcher* this_ptr, const QChar* str, int length) {
  return this_ptr->indexIn(str, length);
}

int qt_core_c_QStringMatcher_indexIn_str_length_from(const QStringMatcher* this_ptr, const QChar* str, int length, int from) {
  return this_ptr->indexIn(str, length, from);
}

QStringMatcher* qt_core_c_QStringMatcher_operator_assign(QStringMatcher* this_ptr, const QStringMatcher* other) {
  return &this_ptr->operator=(*other);
}

void qt_core_c_QStringMatcher_pattern_to_output(const QStringMatcher* this_ptr, QString* output) {
  new(output) QString(this_ptr->pattern());
}

void qt_core_c_QStringMatcher_setCaseSensitivity(QStringMatcher* this_ptr, const Qt::CaseSensitivity* cs) {
  this_ptr->setCaseSensitivity(*cs);
}

void qt_core_c_QStringMatcher_setPattern(QStringMatcher* this_ptr, const QString* pattern) {
  this_ptr->setPattern(*pattern);
}

