#include "qt_core_c_QByteRef.h"

char qt_core_c_QByteRef_convert_to_char(const QByteRef* this_ptr) {
  return this_ptr->operator char();
}

void qt_core_c_QByteRef_destructor(QByteRef* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QByteRef* qt_core_c_QByteRef_operator_assign_QByteRef(QByteRef* this_ptr, const QByteRef* c) {
  return &this_ptr->operator=(*c);
}

QByteRef* qt_core_c_QByteRef_operator_assign_char(QByteRef* this_ptr, char c) {
  return &this_ptr->operator=(c);
}

bool qt_core_c_QByteRef_operator_eq(const QByteRef* this_ptr, char c) {
  return this_ptr->operator==(c);
}

bool qt_core_c_QByteRef_operator_ge(const QByteRef* this_ptr, char c) {
  return this_ptr->operator>=(c);
}

bool qt_core_c_QByteRef_operator_gt(const QByteRef* this_ptr, char c) {
  return this_ptr->operator>(c);
}

bool qt_core_c_QByteRef_operator_le(const QByteRef* this_ptr, char c) {
  return this_ptr->operator<=(c);
}

bool qt_core_c_QByteRef_operator_lt(const QByteRef* this_ptr, char c) {
  return this_ptr->operator<(c);
}

bool qt_core_c_QByteRef_operator_neq(const QByteRef* this_ptr, char c) {
  return this_ptr->operator!=(c);
}

