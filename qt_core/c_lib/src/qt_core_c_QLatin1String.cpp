#include "qt_core_c_QLatin1String.h"

void qt_core_c_QLatin1String_at_to_output(const QLatin1String* this_ptr, int i, QLatin1Char* output) {
  new(output) QLatin1Char(this_ptr->at(i));
}

void qt_core_c_QLatin1String_constructor_QByteArray(const QByteArray* s, QLatin1String* output) {
  new(output) QLatin1String(*s);
}

void qt_core_c_QLatin1String_constructor_char(const char* s, QLatin1String* output) {
  new(output) QLatin1String(s);
}

void qt_core_c_QLatin1String_constructor_char_int(const char* s, int sz, QLatin1String* output) {
  new(output) QLatin1String(s, sz);
}

void qt_core_c_QLatin1String_constructor_no_args(QLatin1String* output) {
  new(output) QLatin1String();
}

const char* qt_core_c_QLatin1String_data(const QLatin1String* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QLatin1String_destructor(QLatin1String* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

const char* qt_core_c_QLatin1String_latin1(const QLatin1String* this_ptr) {
  return this_ptr->latin1();
}

void qt_core_c_QLatin1String_left_to_output(const QLatin1String* this_ptr, int n, QLatin1String* output) {
  new(output) QLatin1String(this_ptr->left(n));
}

void qt_core_c_QLatin1String_mid_to_output_pos(const QLatin1String* this_ptr, int pos, QLatin1String* output) {
  new(output) QLatin1String(this_ptr->mid(pos));
}

void qt_core_c_QLatin1String_mid_to_output_pos_n(const QLatin1String* this_ptr, int pos, int n, QLatin1String* output) {
  new(output) QLatin1String(this_ptr->mid(pos, n));
}

bool qt_core_c_QLatin1String_operator_eq_QByteArray(const QLatin1String* this_ptr, const QByteArray* s) {
  return this_ptr->operator==(*s);
}

bool qt_core_c_QLatin1String_operator_eq_QString(const QLatin1String* this_ptr, const QString* s) {
  return this_ptr->operator==(*s);
}

bool qt_core_c_QLatin1String_operator_eq_char(const QLatin1String* this_ptr, const char* s) {
  return this_ptr->operator==(s);
}

bool qt_core_c_QLatin1String_operator_ge_QByteArray(const QLatin1String* this_ptr, const QByteArray* s) {
  return this_ptr->operator>=(*s);
}

bool qt_core_c_QLatin1String_operator_ge_QString(const QLatin1String* this_ptr, const QString* s) {
  return this_ptr->operator>=(*s);
}

bool qt_core_c_QLatin1String_operator_ge_char(const QLatin1String* this_ptr, const char* s) {
  return this_ptr->operator>=(s);
}

bool qt_core_c_QLatin1String_operator_gt_QByteArray(const QLatin1String* this_ptr, const QByteArray* s) {
  return this_ptr->operator>(*s);
}

bool qt_core_c_QLatin1String_operator_gt_QString(const QLatin1String* this_ptr, const QString* s) {
  return this_ptr->operator>(*s);
}

bool qt_core_c_QLatin1String_operator_gt_char(const QLatin1String* this_ptr, const char* s) {
  return this_ptr->operator>(s);
}

void qt_core_c_QLatin1String_operator_index_to_output(const QLatin1String* this_ptr, int i, QLatin1Char* output) {
  new(output) QLatin1Char(this_ptr->operator[](i));
}

bool qt_core_c_QLatin1String_operator_le_QByteArray(const QLatin1String* this_ptr, const QByteArray* s) {
  return this_ptr->operator<=(*s);
}

bool qt_core_c_QLatin1String_operator_le_QString(const QLatin1String* this_ptr, const QString* s) {
  return this_ptr->operator<=(*s);
}

bool qt_core_c_QLatin1String_operator_le_char(const QLatin1String* this_ptr, const char* s) {
  return this_ptr->operator<=(s);
}

bool qt_core_c_QLatin1String_operator_lt_QByteArray(const QLatin1String* this_ptr, const QByteArray* s) {
  return this_ptr->operator<(*s);
}

bool qt_core_c_QLatin1String_operator_lt_QString(const QLatin1String* this_ptr, const QString* s) {
  return this_ptr->operator<(*s);
}

bool qt_core_c_QLatin1String_operator_lt_char(const QLatin1String* this_ptr, const char* s) {
  return this_ptr->operator<(s);
}

bool qt_core_c_QLatin1String_operator_neq_QByteArray(const QLatin1String* this_ptr, const QByteArray* s) {
  return this_ptr->operator!=(*s);
}

bool qt_core_c_QLatin1String_operator_neq_QString(const QLatin1String* this_ptr, const QString* s) {
  return this_ptr->operator!=(*s);
}

bool qt_core_c_QLatin1String_operator_neq_char(const QLatin1String* this_ptr, const char* s) {
  return this_ptr->operator!=(s);
}

void qt_core_c_QLatin1String_right_to_output(const QLatin1String* this_ptr, int n, QLatin1String* output) {
  new(output) QLatin1String(this_ptr->right(n));
}

int qt_core_c_QLatin1String_size(const QLatin1String* this_ptr) {
  return this_ptr->size();
}

