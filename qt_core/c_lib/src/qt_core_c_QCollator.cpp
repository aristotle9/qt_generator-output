#include "qt_core_c_QCollator.h"

bool qt_core_c_QCollator_G_operator_lt(const QCollatorSortKey* lhs, const QCollatorSortKey* rhs) {
  return operator<(*lhs, *rhs);
}

void qt_core_c_QCollator_G_swap_QCollatorSortKey_QCollatorSortKey(QCollatorSortKey* value1, QCollatorSortKey* value2) {
  swap(*value1, *value2);
}

void qt_core_c_QCollator_G_swap_QCollator_QCollator(QCollator* value1, QCollator* value2) {
  swap(*value1, *value2);
}

int qt_core_c_QCollator_compare_QChar_int_QChar_int(const QCollator* this_ptr, const QChar* s1, int len1, const QChar* s2, int len2) {
  return this_ptr->compare(s1, len1, s2, len2);
}

int qt_core_c_QCollator_compare_QStringRef_QStringRef(const QCollator* this_ptr, const QStringRef* s1, const QStringRef* s2) {
  return this_ptr->compare(*s1, *s2);
}

int qt_core_c_QCollator_compare_QString_QString(const QCollator* this_ptr, const QString* s1, const QString* s2) {
  return this_ptr->compare(*s1, *s2);
}

void qt_core_c_QCollator_constructor_arg1(const QCollator* arg1, QCollator* output) {
  new(output) QCollator(*arg1);
}

void qt_core_c_QCollator_constructor_locale(const QLocale* locale, QCollator* output) {
  new(output) QCollator(*locale);
}

void qt_core_c_QCollator_constructor_no_args(QCollator* output) {
  new(output) QCollator();
}

void qt_core_c_QCollator_destructor(QCollator* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QCollator_ignorePunctuation(const QCollator* this_ptr) {
  return this_ptr->ignorePunctuation();
}

void qt_core_c_QCollator_locale_to_output(const QCollator* this_ptr, QLocale* output) {
  new(output) QLocale(this_ptr->locale());
}

bool qt_core_c_QCollator_numericMode(const QCollator* this_ptr) {
  return this_ptr->numericMode();
}

QCollator* qt_core_c_QCollator_operator_assign(QCollator* this_ptr, const QCollator* arg1) {
  return &this_ptr->operator=(*arg1);
}

bool qt_core_c_QCollator_operator_call(const QCollator* this_ptr, const QString* s1, const QString* s2) {
  return this_ptr->operator()(*s1, *s2);
}

void qt_core_c_QCollator_setCaseSensitivity(QCollator* this_ptr, const Qt::CaseSensitivity* cs) {
  this_ptr->setCaseSensitivity(*cs);
}

void qt_core_c_QCollator_setIgnorePunctuation(QCollator* this_ptr, bool on) {
  this_ptr->setIgnorePunctuation(on);
}

void qt_core_c_QCollator_setLocale(QCollator* this_ptr, const QLocale* locale) {
  this_ptr->setLocale(*locale);
}

void qt_core_c_QCollator_setNumericMode(QCollator* this_ptr, bool on) {
  this_ptr->setNumericMode(on);
}

void qt_core_c_QCollator_sortKey_to_output(const QCollator* this_ptr, const QString* string, QCollatorSortKey* output) {
  new(output) QCollatorSortKey(this_ptr->sortKey(*string));
}

void qt_core_c_QCollator_swap(QCollator* this_ptr, QCollator* other) {
  this_ptr->swap(*other);
}

