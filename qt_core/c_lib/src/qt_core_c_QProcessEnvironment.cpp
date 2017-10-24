#include "qt_core_c_QProcessEnvironment.h"

void qt_core_c_QProcessEnvironment_clear(QProcessEnvironment* this_ptr) {
  this_ptr->clear();
}

void qt_core_c_QProcessEnvironment_constructor_no_args(QProcessEnvironment* output) {
  new(output) QProcessEnvironment();
}

void qt_core_c_QProcessEnvironment_constructor_other(const QProcessEnvironment* other, QProcessEnvironment* output) {
  new(output) QProcessEnvironment(*other);
}

bool qt_core_c_QProcessEnvironment_contains(const QProcessEnvironment* this_ptr, const QString* name) {
  return this_ptr->contains(*name);
}

void qt_core_c_QProcessEnvironment_destructor(QProcessEnvironment* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QProcessEnvironment_insert_e(QProcessEnvironment* this_ptr, const QProcessEnvironment* e) {
  this_ptr->insert(*e);
}

void qt_core_c_QProcessEnvironment_insert_name_value(QProcessEnvironment* this_ptr, const QString* name, const QString* value) {
  this_ptr->insert(*name, *value);
}

bool qt_core_c_QProcessEnvironment_isEmpty(const QProcessEnvironment* this_ptr) {
  return this_ptr->isEmpty();
}

void qt_core_c_QProcessEnvironment_keys_to_output(const QProcessEnvironment* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->keys());
}

QProcessEnvironment* qt_core_c_QProcessEnvironment_operator_assign(QProcessEnvironment* this_ptr, const QProcessEnvironment* other) {
  return &this_ptr->operator=(*other);
}

bool qt_core_c_QProcessEnvironment_operator_eq(const QProcessEnvironment* this_ptr, const QProcessEnvironment* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QProcessEnvironment_operator_neq(const QProcessEnvironment* this_ptr, const QProcessEnvironment* other) {
  return this_ptr->operator!=(*other);
}

void qt_core_c_QProcessEnvironment_remove(QProcessEnvironment* this_ptr, const QString* name) {
  this_ptr->remove(*name);
}

void qt_core_c_QProcessEnvironment_swap(QProcessEnvironment* this_ptr, QProcessEnvironment* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QProcessEnvironment_systemEnvironment_to_output(QProcessEnvironment* output) {
  new(output) QProcessEnvironment(QProcessEnvironment::systemEnvironment());
}

void qt_core_c_QProcessEnvironment_toStringList_to_output(const QProcessEnvironment* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->toStringList());
}

void qt_core_c_QProcessEnvironment_value_to_output_name(const QProcessEnvironment* this_ptr, const QString* name, QString* output) {
  new(output) QString(this_ptr->value(*name));
}

void qt_core_c_QProcessEnvironment_value_to_output_name_defaultValue(const QProcessEnvironment* this_ptr, const QString* name, const QString* defaultValue, QString* output) {
  new(output) QString(this_ptr->value(*name, *defaultValue));
}

