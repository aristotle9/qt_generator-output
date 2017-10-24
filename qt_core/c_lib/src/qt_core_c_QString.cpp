#include "qt_core_c_QString.h"

void qt_core_c_QString_G_operator_add_to_output_QByteArray_ba_QString_s(const QByteArray* ba, const QString* s, QString* output) {
  new(output) QString(operator+(*ba, *s));
}

void qt_core_c_QString_G_operator_add_to_output_QChar_s1_QStringRef_s2(const QChar* s1, const QStringRef* s2, QString* output) {
  new(output) QString(operator+(*s1, *s2));
}

void qt_core_c_QString_G_operator_add_to_output_QChar_s1_QString_s2(const QChar* s1, const QString* s2, QString* output) {
  new(output) QString(operator+(*s1, *s2));
}

void qt_core_c_QString_G_operator_add_to_output_QLatin1String_s1_QStringRef_s2(const QLatin1String* s1, const QStringRef* s2, QString* output) {
  new(output) QString(operator+(*s1, *s2));
}

void qt_core_c_QString_G_operator_add_to_output_QStringRef_s1_QChar_s2(const QStringRef* s1, const QChar* s2, QString* output) {
  new(output) QString(operator+(*s1, *s2));
}

void qt_core_c_QString_G_operator_add_to_output_QStringRef_s1_QLatin1String_s2(const QStringRef* s1, const QLatin1String* s2, QString* output) {
  new(output) QString(operator+(*s1, *s2));
}

void qt_core_c_QString_G_operator_add_to_output_QStringRef_s1_QStringRef_s2(const QStringRef* s1, const QStringRef* s2, QString* output) {
  new(output) QString(operator+(*s1, *s2));
}

void qt_core_c_QString_G_operator_add_to_output_QStringRef_s1_QString_s2(const QStringRef* s1, const QString* s2, QString* output) {
  new(output) QString(operator+(*s1, *s2));
}

void qt_core_c_QString_G_operator_add_to_output_QString_s1_QChar_s2(const QString* s1, const QChar* s2, QString* output) {
  new(output) QString(operator+(*s1, *s2));
}

void qt_core_c_QString_G_operator_add_to_output_QString_s1_QStringRef_s2(const QString* s1, const QStringRef* s2, QString* output) {
  new(output) QString(operator+(*s1, *s2));
}

void qt_core_c_QString_G_operator_add_to_output_QString_s1_QString_s2(const QString* s1, const QString* s2, QString* output) {
  new(output) QString(operator+(*s1, *s2));
}

void qt_core_c_QString_G_operator_add_to_output_QString_s1_char_s2(const QString* s1, const char* s2, QString* output) {
  new(output) QString(operator+(*s1, s2));
}

void qt_core_c_QString_G_operator_add_to_output_QString_s_QByteArray_ba(const QString* s, const QByteArray* ba, QString* output) {
  new(output) QString(operator+(*s, *ba));
}

void qt_core_c_QString_G_operator_add_to_output_QString_s_char_c(const QString* s, char c, QString* output) {
  new(output) QString(operator+(*s, c));
}

void qt_core_c_QString_G_operator_add_to_output_char_c_QString_s(char c, const QString* s, QString* output) {
  new(output) QString(operator+(c, *s));
}

void qt_core_c_QString_G_operator_add_to_output_char_s1_QString_s2(const char* s1, const QString* s2, QString* output) {
  new(output) QString(operator+(s1, *s2));
}

bool qt_core_c_QString_G_operator_eq_QByteArray_QStringRef(const QByteArray* lhs, const QStringRef* rhs) {
  return operator==(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_eq_QLatin1String_QChar(const QLatin1String* lhs, const QChar* rhs) {
  return operator==(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_eq_QLatin1String_QLatin1String(const QLatin1String* s1, const QLatin1String* s2) {
  return operator==(*s1, *s2);
}

bool qt_core_c_QString_G_operator_eq_QLatin1String_QStringRef(const QLatin1String* lhs, const QStringRef* rhs) {
  return operator==(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_eq_QStringRef_QByteArray(const QStringRef* lhs, const QByteArray* rhs) {
  return operator==(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_eq_QStringRef_QChar(const QStringRef* lhs, const QChar* rhs) {
  return operator==(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_eq_QStringRef_QLatin1String(const QStringRef* lhs, const QLatin1String* rhs) {
  return operator==(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_eq_QStringRef_QString(const QStringRef* lhs, const QString* rhs) {
  return operator==(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_eq_QStringRef_QStringRef(const QStringRef* s1, const QStringRef* s2) {
  return operator==(*s1, *s2);
}

bool qt_core_c_QString_G_operator_eq_QString_QChar(const QString* lhs, const QChar* rhs) {
  return operator==(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_eq_QString_QStringRef(const QString* lhs, const QStringRef* rhs) {
  return operator==(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_eq_char_QLatin1String(const char* s1, const QLatin1String* s2) {
  return operator==(s1, *s2);
}

bool qt_core_c_QString_G_operator_ge_QByteArray_QStringRef(const QByteArray* lhs, const QStringRef* rhs) {
  return operator>=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_ge_QChar_QLatin1String(const QChar* lhs, const QLatin1String* rhs) {
  return operator>=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_ge_QChar_QString(const QChar* lhs, const QString* rhs) {
  return operator>=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_ge_QChar_QStringRef(const QChar* lhs, const QStringRef* rhs) {
  return operator>=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_ge_QLatin1String_QChar(const QLatin1String* lhs, const QChar* rhs) {
  return operator>=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_ge_QLatin1String_QLatin1String(const QLatin1String* s1, const QLatin1String* s2) {
  return operator>=(*s1, *s2);
}

bool qt_core_c_QString_G_operator_ge_QLatin1String_QStringRef(const QLatin1String* lhs, const QStringRef* rhs) {
  return operator>=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_ge_QStringRef_QByteArray(const QStringRef* lhs, const QByteArray* rhs) {
  return operator>=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_ge_QStringRef_QChar(const QStringRef* lhs, const QChar* rhs) {
  return operator>=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_ge_QStringRef_QLatin1String(const QStringRef* lhs, const QLatin1String* rhs) {
  return operator>=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_ge_QStringRef_QString(const QStringRef* lhs, const QString* rhs) {
  return operator>=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_ge_QStringRef_QStringRef(const QStringRef* s1, const QStringRef* s2) {
  return operator>=(*s1, *s2);
}

bool qt_core_c_QString_G_operator_ge_QString_QChar(const QString* lhs, const QChar* rhs) {
  return operator>=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_ge_QString_QStringRef(const QString* lhs, const QStringRef* rhs) {
  return operator>=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_ge_char_QLatin1String(const char* s1, const QLatin1String* s2) {
  return operator>=(s1, *s2);
}

bool qt_core_c_QString_G_operator_gt_QByteArray_QStringRef(const QByteArray* lhs, const QStringRef* rhs) {
  return operator>(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_gt_QLatin1String_QChar(const QLatin1String* lhs, const QChar* rhs) {
  return operator>(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_gt_QLatin1String_QLatin1String(const QLatin1String* s1, const QLatin1String* s2) {
  return operator>(*s1, *s2);
}

bool qt_core_c_QString_G_operator_gt_QLatin1String_QStringRef(const QLatin1String* lhs, const QStringRef* rhs) {
  return operator>(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_gt_QStringRef_QByteArray(const QStringRef* lhs, const QByteArray* rhs) {
  return operator>(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_gt_QStringRef_QChar(const QStringRef* lhs, const QChar* rhs) {
  return operator>(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_gt_QStringRef_QLatin1String(const QStringRef* lhs, const QLatin1String* rhs) {
  return operator>(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_gt_QStringRef_QString(const QStringRef* lhs, const QString* rhs) {
  return operator>(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_gt_QStringRef_QStringRef(const QStringRef* s1, const QStringRef* s2) {
  return operator>(*s1, *s2);
}

bool qt_core_c_QString_G_operator_gt_QString_QChar(const QString* lhs, const QChar* rhs) {
  return operator>(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_gt_QString_QStringRef(const QString* lhs, const QStringRef* rhs) {
  return operator>(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_gt_char_QLatin1String(const char* s1, const QLatin1String* s2) {
  return operator>(s1, *s2);
}

bool qt_core_c_QString_G_operator_le_QByteArray_QStringRef(const QByteArray* lhs, const QStringRef* rhs) {
  return operator<=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_le_QChar_QLatin1String(const QChar* lhs, const QLatin1String* rhs) {
  return operator<=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_le_QChar_QString(const QChar* lhs, const QString* rhs) {
  return operator<=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_le_QChar_QStringRef(const QChar* lhs, const QStringRef* rhs) {
  return operator<=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_le_QLatin1String_QChar(const QLatin1String* lhs, const QChar* rhs) {
  return operator<=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_le_QLatin1String_QLatin1String(const QLatin1String* s1, const QLatin1String* s2) {
  return operator<=(*s1, *s2);
}

bool qt_core_c_QString_G_operator_le_QLatin1String_QStringRef(const QLatin1String* lhs, const QStringRef* rhs) {
  return operator<=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_le_QStringRef_QByteArray(const QStringRef* lhs, const QByteArray* rhs) {
  return operator<=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_le_QStringRef_QChar(const QStringRef* lhs, const QChar* rhs) {
  return operator<=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_le_QStringRef_QLatin1String(const QStringRef* lhs, const QLatin1String* rhs) {
  return operator<=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_le_QStringRef_QString(const QStringRef* lhs, const QString* rhs) {
  return operator<=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_le_QStringRef_QStringRef(const QStringRef* s1, const QStringRef* s2) {
  return operator<=(*s1, *s2);
}

bool qt_core_c_QString_G_operator_le_QString_QChar(const QString* lhs, const QChar* rhs) {
  return operator<=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_le_QString_QStringRef(const QString* lhs, const QStringRef* rhs) {
  return operator<=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_le_char_QLatin1String(const char* s1, const QLatin1String* s2) {
  return operator<=(s1, *s2);
}

bool qt_core_c_QString_G_operator_lt_QByteArray_QStringRef(const QByteArray* lhs, const QStringRef* rhs) {
  return operator<(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_lt_QLatin1String_QChar(const QLatin1String* lhs, const QChar* rhs) {
  return operator<(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_lt_QLatin1String_QLatin1String(const QLatin1String* s1, const QLatin1String* s2) {
  return operator<(*s1, *s2);
}

bool qt_core_c_QString_G_operator_lt_QLatin1String_QStringRef(const QLatin1String* lhs, const QStringRef* rhs) {
  return operator<(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_lt_QStringRef_QByteArray(const QStringRef* lhs, const QByteArray* rhs) {
  return operator<(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_lt_QStringRef_QChar(const QStringRef* lhs, const QChar* rhs) {
  return operator<(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_lt_QStringRef_QLatin1String(const QStringRef* lhs, const QLatin1String* rhs) {
  return operator<(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_lt_QStringRef_QString(const QStringRef* lhs, const QString* rhs) {
  return operator<(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_lt_QStringRef_QStringRef(const QStringRef* s1, const QStringRef* s2) {
  return operator<(*s1, *s2);
}

bool qt_core_c_QString_G_operator_lt_QString_QChar(const QString* lhs, const QChar* rhs) {
  return operator<(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_lt_QString_QStringRef(const QString* lhs, const QStringRef* rhs) {
  return operator<(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_lt_char_QLatin1String(const char* s1, const QLatin1String* s2) {
  return operator<(s1, *s2);
}

bool qt_core_c_QString_G_operator_neq_QByteArray_QStringRef(const QByteArray* lhs, const QStringRef* rhs) {
  return operator!=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_neq_QChar_QLatin1String(const QChar* lhs, const QLatin1String* rhs) {
  return operator!=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_neq_QChar_QString(const QChar* lhs, const QString* rhs) {
  return operator!=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_neq_QChar_QStringRef(const QChar* lhs, const QStringRef* rhs) {
  return operator!=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_neq_QLatin1String_QChar(const QLatin1String* lhs, const QChar* rhs) {
  return operator!=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_neq_QLatin1String_QLatin1String(const QLatin1String* s1, const QLatin1String* s2) {
  return operator!=(*s1, *s2);
}

bool qt_core_c_QString_G_operator_neq_QLatin1String_QStringRef(const QLatin1String* lhs, const QStringRef* rhs) {
  return operator!=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_neq_QStringRef_QByteArray(const QStringRef* lhs, const QByteArray* rhs) {
  return operator!=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_neq_QStringRef_QChar(const QStringRef* lhs, const QChar* rhs) {
  return operator!=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_neq_QStringRef_QLatin1String(const QStringRef* lhs, const QLatin1String* rhs) {
  return operator!=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_neq_QStringRef_QString(const QStringRef* lhs, const QString* rhs) {
  return operator!=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_neq_QStringRef_QStringRef(const QStringRef* s1, const QStringRef* s2) {
  return operator!=(*s1, *s2);
}

bool qt_core_c_QString_G_operator_neq_QString_QChar(const QString* lhs, const QChar* rhs) {
  return operator!=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_neq_QString_QStringRef(const QString* lhs, const QStringRef* rhs) {
  return operator!=(*lhs, *rhs);
}

bool qt_core_c_QString_G_operator_neq_char_QLatin1String(const char* s1, const QLatin1String* s2) {
  return operator!=(s1, *s2);
}

QDataStream* qt_core_c_QString_G_operator_shl(QDataStream* arg1, const QString* arg2) {
  return &operator<<(*arg1, *arg2);
}

QDataStream* qt_core_c_QString_G_operator_shr(QDataStream* arg1, QString* arg2) {
  return &operator>>(*arg1, *arg2);
}

void qt_core_c_QString_G_swap(QString* value1, QString* value2) {
  swap(*value1, *value2);
}

QString* qt_core_c_QString_append_QByteArray(QString* this_ptr, const QByteArray* s) {
  return &this_ptr->append(*s);
}

QString* qt_core_c_QString_append_QChar(QString* this_ptr, const QChar* c) {
  return &this_ptr->append(*c);
}

QString* qt_core_c_QString_append_QChar_int(QString* this_ptr, const QChar* uc, int len) {
  return &this_ptr->append(uc, len);
}

QString* qt_core_c_QString_append_QLatin1String(QString* this_ptr, const QLatin1String* s) {
  return &this_ptr->append(*s);
}

QString* qt_core_c_QString_append_QString(QString* this_ptr, const QString* s) {
  return &this_ptr->append(*s);
}

QString* qt_core_c_QString_append_QStringRef(QString* this_ptr, const QStringRef* s) {
  return &this_ptr->append(*s);
}

QString* qt_core_c_QString_append_char(QString* this_ptr, const char* s) {
  return &this_ptr->append(s);
}

void qt_core_c_QString_arg_to_output_QChar(const QString* this_ptr, const QChar* a, QString* output) {
  new(output) QString(this_ptr->arg(*a));
}

void qt_core_c_QString_arg_to_output_QChar_int(const QString* this_ptr, const QChar* a, int fieldWidth, QString* output) {
  new(output) QString(this_ptr->arg(*a, fieldWidth));
}

void qt_core_c_QString_arg_to_output_QChar_int_QChar(const QString* this_ptr, const QChar* a, int fieldWidth, const QChar* fillChar, QString* output) {
  new(output) QString(this_ptr->arg(*a, fieldWidth, *fillChar));
}

void qt_core_c_QString_arg_to_output_QString(const QString* this_ptr, const QString* a, QString* output) {
  new(output) QString(this_ptr->arg(*a));
}

void qt_core_c_QString_arg_to_output_QString_QString(const QString* this_ptr, const QString* a1, const QString* a2, QString* output) {
  new(output) QString(this_ptr->arg(*a1, *a2));
}

void qt_core_c_QString_arg_to_output_QString_QString_QString(const QString* this_ptr, const QString* a1, const QString* a2, const QString* a3, QString* output) {
  new(output) QString(this_ptr->arg(*a1, *a2, *a3));
}

void qt_core_c_QString_arg_to_output_QString_QString_QString_QString(const QString* this_ptr, const QString* a1, const QString* a2, const QString* a3, const QString* a4, QString* output) {
  new(output) QString(this_ptr->arg(*a1, *a2, *a3, *a4));
}

void qt_core_c_QString_arg_to_output_QString_QString_QString_QString_QString(const QString* this_ptr, const QString* a1, const QString* a2, const QString* a3, const QString* a4, const QString* a5, QString* output) {
  new(output) QString(this_ptr->arg(*a1, *a2, *a3, *a4, *a5));
}

void qt_core_c_QString_arg_to_output_QString_QString_QString_QString_QString_QString(const QString* this_ptr, const QString* a1, const QString* a2, const QString* a3, const QString* a4, const QString* a5, const QString* a6, QString* output) {
  new(output) QString(this_ptr->arg(*a1, *a2, *a3, *a4, *a5, *a6));
}

void qt_core_c_QString_arg_to_output_QString_QString_QString_QString_QString_QString_QString(const QString* this_ptr, const QString* a1, const QString* a2, const QString* a3, const QString* a4, const QString* a5, const QString* a6, const QString* a7, QString* output) {
  new(output) QString(this_ptr->arg(*a1, *a2, *a3, *a4, *a5, *a6, *a7));
}

void qt_core_c_QString_arg_to_output_QString_QString_QString_QString_QString_QString_QString_QString(const QString* this_ptr, const QString* a1, const QString* a2, const QString* a3, const QString* a4, const QString* a5, const QString* a6, const QString* a7, const QString* a8, QString* output) {
  new(output) QString(this_ptr->arg(*a1, *a2, *a3, *a4, *a5, *a6, *a7, *a8));
}

void qt_core_c_QString_arg_to_output_QString_QString_QString_QString_QString_QString_QString_QString_QString(const QString* this_ptr, const QString* a1, const QString* a2, const QString* a3, const QString* a4, const QString* a5, const QString* a6, const QString* a7, const QString* a8, const QString* a9, QString* output) {
  new(output) QString(this_ptr->arg(*a1, *a2, *a3, *a4, *a5, *a6, *a7, *a8, *a9));
}

void qt_core_c_QString_arg_to_output_QString_int(const QString* this_ptr, const QString* a, int fieldWidth, QString* output) {
  new(output) QString(this_ptr->arg(*a, fieldWidth));
}

void qt_core_c_QString_arg_to_output_QString_int_QChar(const QString* this_ptr, const QString* a, int fieldWidth, const QChar* fillChar, QString* output) {
  new(output) QString(this_ptr->arg(*a, fieldWidth, *fillChar));
}

void qt_core_c_QString_arg_to_output_char(const QString* this_ptr, char a, QString* output) {
  new(output) QString(this_ptr->arg(a));
}

void qt_core_c_QString_arg_to_output_char_int(const QString* this_ptr, char a, int fieldWidth, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth));
}

void qt_core_c_QString_arg_to_output_char_int_QChar(const QString* this_ptr, char a, int fieldWidth, const QChar* fillChar, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth, *fillChar));
}

void qt_core_c_QString_arg_to_output_double(const QString* this_ptr, double a, QString* output) {
  new(output) QString(this_ptr->arg(a));
}

void qt_core_c_QString_arg_to_output_double_int(const QString* this_ptr, double a, int fieldWidth, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth));
}

void qt_core_c_QString_arg_to_output_double_int_char(const QString* this_ptr, double a, int fieldWidth, char fmt, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth, fmt));
}

void qt_core_c_QString_arg_to_output_double_int_char_int(const QString* this_ptr, double a, int fieldWidth, char fmt, int prec, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth, fmt, prec));
}

void qt_core_c_QString_arg_to_output_double_int_char_int_QChar(const QString* this_ptr, double a, int fieldWidth, char fmt, int prec, const QChar* fillChar, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth, fmt, prec, *fillChar));
}

void qt_core_c_QString_arg_to_output_int(const QString* this_ptr, int a, QString* output) {
  new(output) QString(this_ptr->arg(a));
}

void qt_core_c_QString_arg_to_output_int_int(const QString* this_ptr, int a, int fieldWidth, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth));
}

void qt_core_c_QString_arg_to_output_int_int_int(const QString* this_ptr, int a, int fieldWidth, int base, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth, base));
}

void qt_core_c_QString_arg_to_output_int_int_int_QChar(const QString* this_ptr, int a, int fieldWidth, int base, const QChar* fillChar, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth, base, *fillChar));
}

void qt_core_c_QString_arg_to_output_long(const QString* this_ptr, long a, QString* output) {
  new(output) QString(this_ptr->arg(a));
}

void qt_core_c_QString_arg_to_output_long_int(const QString* this_ptr, long a, int fieldwidth, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldwidth));
}

void qt_core_c_QString_arg_to_output_long_int_int(const QString* this_ptr, long a, int fieldwidth, int base, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldwidth, base));
}

void qt_core_c_QString_arg_to_output_long_int_int_QChar(const QString* this_ptr, long a, int fieldwidth, int base, const QChar* fillChar, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldwidth, base, *fillChar));
}

void qt_core_c_QString_arg_to_output_qlonglong(const QString* this_ptr, qlonglong a, QString* output) {
  new(output) QString(this_ptr->arg(a));
}

void qt_core_c_QString_arg_to_output_qlonglong_int(const QString* this_ptr, qlonglong a, int fieldwidth, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldwidth));
}

void qt_core_c_QString_arg_to_output_qlonglong_int_int(const QString* this_ptr, qlonglong a, int fieldwidth, int base, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldwidth, base));
}

void qt_core_c_QString_arg_to_output_qlonglong_int_int_QChar(const QString* this_ptr, qlonglong a, int fieldwidth, int base, const QChar* fillChar, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldwidth, base, *fillChar));
}

void qt_core_c_QString_arg_to_output_qulonglong(const QString* this_ptr, qulonglong a, QString* output) {
  new(output) QString(this_ptr->arg(a));
}

void qt_core_c_QString_arg_to_output_qulonglong_int(const QString* this_ptr, qulonglong a, int fieldwidth, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldwidth));
}

void qt_core_c_QString_arg_to_output_qulonglong_int_int(const QString* this_ptr, qulonglong a, int fieldwidth, int base, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldwidth, base));
}

void qt_core_c_QString_arg_to_output_qulonglong_int_int_QChar(const QString* this_ptr, qulonglong a, int fieldwidth, int base, const QChar* fillChar, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldwidth, base, *fillChar));
}

void qt_core_c_QString_arg_to_output_short(const QString* this_ptr, short a, QString* output) {
  new(output) QString(this_ptr->arg(a));
}

void qt_core_c_QString_arg_to_output_short_int(const QString* this_ptr, short a, int fieldWidth, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth));
}

void qt_core_c_QString_arg_to_output_short_int_int(const QString* this_ptr, short a, int fieldWidth, int base, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth, base));
}

void qt_core_c_QString_arg_to_output_short_int_int_QChar(const QString* this_ptr, short a, int fieldWidth, int base, const QChar* fillChar, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth, base, *fillChar));
}

void qt_core_c_QString_arg_to_output_unsigned_int(const QString* this_ptr, unsigned int a, QString* output) {
  new(output) QString(this_ptr->arg(a));
}

void qt_core_c_QString_arg_to_output_unsigned_int_int(const QString* this_ptr, unsigned int a, int fieldWidth, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth));
}

void qt_core_c_QString_arg_to_output_unsigned_int_int_int(const QString* this_ptr, unsigned int a, int fieldWidth, int base, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth, base));
}

void qt_core_c_QString_arg_to_output_unsigned_int_int_int_QChar(const QString* this_ptr, unsigned int a, int fieldWidth, int base, const QChar* fillChar, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth, base, *fillChar));
}

void qt_core_c_QString_arg_to_output_unsigned_long(const QString* this_ptr, unsigned long a, QString* output) {
  new(output) QString(this_ptr->arg(a));
}

void qt_core_c_QString_arg_to_output_unsigned_long_int(const QString* this_ptr, unsigned long a, int fieldwidth, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldwidth));
}

void qt_core_c_QString_arg_to_output_unsigned_long_int_int(const QString* this_ptr, unsigned long a, int fieldwidth, int base, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldwidth, base));
}

void qt_core_c_QString_arg_to_output_unsigned_long_int_int_QChar(const QString* this_ptr, unsigned long a, int fieldwidth, int base, const QChar* fillChar, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldwidth, base, *fillChar));
}

void qt_core_c_QString_arg_to_output_unsigned_short(const QString* this_ptr, unsigned short a, QString* output) {
  new(output) QString(this_ptr->arg(a));
}

void qt_core_c_QString_arg_to_output_unsigned_short_int(const QString* this_ptr, unsigned short a, int fieldWidth, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth));
}

void qt_core_c_QString_arg_to_output_unsigned_short_int_int(const QString* this_ptr, unsigned short a, int fieldWidth, int base, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth, base));
}

void qt_core_c_QString_arg_to_output_unsigned_short_int_int_QChar(const QString* this_ptr, unsigned short a, int fieldWidth, int base, const QChar* fillChar, QString* output) {
  new(output) QString(this_ptr->arg(a, fieldWidth, base, *fillChar));
}

void qt_core_c_QString_at_to_output(const QString* this_ptr, int i, QChar* output) {
  new(output) QChar(this_ptr->at(i));
}

QChar* qt_core_c_QString_begin(QString* this_ptr) {
  return this_ptr->begin();
}

const QChar* qt_core_c_QString_begin_const(const QString* this_ptr) {
  return this_ptr->begin();
}

int qt_core_c_QString_capacity(const QString* this_ptr) {
  return this_ptr->capacity();
}

const QChar* qt_core_c_QString_cbegin(const QString* this_ptr) {
  return this_ptr->cbegin();
}

const QChar* qt_core_c_QString_cend(const QString* this_ptr) {
  return this_ptr->cend();
}

void qt_core_c_QString_chop(QString* this_ptr, int n) {
  this_ptr->chop(n);
}

void qt_core_c_QString_clear(QString* this_ptr) {
  this_ptr->clear();
}

int qt_core_c_QString_compare_QLatin1String(const QString* this_ptr, const QLatin1String* other) {
  return this_ptr->compare(*other);
}

int qt_core_c_QString_compare_QLatin1String_QString(const QLatin1String* s1, const QString* s2) {
  return QString::compare(*s1, *s2);
}

int qt_core_c_QString_compare_QLatin1String_QString_Qt_CaseSensitivity(const QLatin1String* s1, const QString* s2, const Qt::CaseSensitivity* cs) {
  return QString::compare(*s1, *s2, *cs);
}

int qt_core_c_QString_compare_QLatin1String_Qt_CaseSensitivity(const QString* this_ptr, const QLatin1String* other, const Qt::CaseSensitivity* cs) {
  return this_ptr->compare(*other, *cs);
}

int qt_core_c_QString_compare_QString(const QString* this_ptr, const QString* s) {
  return this_ptr->compare(*s);
}

int qt_core_c_QString_compare_QStringRef(const QString* this_ptr, const QStringRef* s) {
  return this_ptr->compare(*s);
}

int qt_core_c_QString_compare_QStringRef_Qt_CaseSensitivity(const QString* this_ptr, const QStringRef* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->compare(*s, *cs);
}

int qt_core_c_QString_compare_QString_QLatin1String(const QString* s1, const QLatin1String* s2) {
  return QString::compare(*s1, *s2);
}

int qt_core_c_QString_compare_QString_QLatin1String_Qt_CaseSensitivity(const QString* s1, const QLatin1String* s2, const Qt::CaseSensitivity* cs) {
  return QString::compare(*s1, *s2, *cs);
}

int qt_core_c_QString_compare_QString_QString(const QString* s1, const QString* s2) {
  return QString::compare(*s1, *s2);
}

int qt_core_c_QString_compare_QString_QStringRef(const QString* s1, const QStringRef* s2) {
  return QString::compare(*s1, *s2);
}

int qt_core_c_QString_compare_QString_QStringRef_Qt_CaseSensitivity(const QString* s1, const QStringRef* s2, const Qt::CaseSensitivity* arg3) {
  return QString::compare(*s1, *s2, *arg3);
}

int qt_core_c_QString_compare_QString_QString_Qt_CaseSensitivity(const QString* s1, const QString* s2, const Qt::CaseSensitivity* cs) {
  return QString::compare(*s1, *s2, *cs);
}

int qt_core_c_QString_compare_QString_Qt_CaseSensitivity(const QString* this_ptr, const QString* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->compare(*s, *cs);
}

const QChar* qt_core_c_QString_constBegin(const QString* this_ptr) {
  return this_ptr->constBegin();
}

const QChar* qt_core_c_QString_constData(const QString* this_ptr) {
  return this_ptr->constData();
}

const QChar* qt_core_c_QString_constEnd(const QString* this_ptr) {
  return this_ptr->constEnd();
}

void qt_core_c_QString_constructor_a(const QByteArray* a, QString* output) {
  new(output) QString(*a);
}

void qt_core_c_QString_constructor_arg1(const QString* arg1, QString* output) {
  new(output) QString(*arg1);
}

void qt_core_c_QString_constructor_c(const QChar* c, QString* output) {
  new(output) QString(*c);
}

void qt_core_c_QString_constructor_ch(const char* ch, QString* output) {
  new(output) QString(ch);
}

void qt_core_c_QString_constructor_latin1(const QLatin1String* latin1, QString* output) {
  new(output) QString(*latin1);
}

void qt_core_c_QString_constructor_no_args(QString* output) {
  new(output) QString();
}

void qt_core_c_QString_constructor_size_c(int size, const QChar* c, QString* output) {
  new(output) QString(size, *c);
}

void qt_core_c_QString_constructor_unicode(const QChar* unicode, QString* output) {
  new(output) QString(unicode);
}

void qt_core_c_QString_constructor_unicode_size(const QChar* unicode, int size, QString* output) {
  new(output) QString(unicode, size);
}

bool qt_core_c_QString_contains_QChar(const QString* this_ptr, const QChar* c) {
  return this_ptr->contains(*c);
}

bool qt_core_c_QString_contains_QChar_Qt_CaseSensitivity(const QString* this_ptr, const QChar* c, const Qt::CaseSensitivity* cs) {
  return this_ptr->contains(*c, *cs);
}

bool qt_core_c_QString_contains_QLatin1String(const QString* this_ptr, const QLatin1String* s) {
  return this_ptr->contains(*s);
}

bool qt_core_c_QString_contains_QLatin1String_Qt_CaseSensitivity(const QString* this_ptr, const QLatin1String* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->contains(*s, *cs);
}

bool qt_core_c_QString_contains_QRegExp_ref(const QString* this_ptr, QRegExp* rx) {
  return this_ptr->contains(*rx);
}

bool qt_core_c_QString_contains_const_QRegExp_ref(const QString* this_ptr, const QRegExp* rx) {
  return this_ptr->contains(*rx);
}

bool qt_core_c_QString_contains_const_QRegularExpression_ref(const QString* this_ptr, const QRegularExpression* re) {
  return this_ptr->contains(*re);
}

bool qt_core_c_QString_contains_const_QRegularExpression_ref_QRegularExpressionMatch_ptr(const QString* this_ptr, const QRegularExpression* re, QRegularExpressionMatch* match) {
  return this_ptr->contains(*re, match);
}

bool qt_core_c_QString_contains_const_QStringRef_ref(const QString* this_ptr, const QStringRef* s) {
  return this_ptr->contains(*s);
}

bool qt_core_c_QString_contains_const_QStringRef_ref_Qt_CaseSensitivity(const QString* this_ptr, const QStringRef* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->contains(*s, *cs);
}

bool qt_core_c_QString_contains_const_QString_ref(const QString* this_ptr, const QString* s) {
  return this_ptr->contains(*s);
}

bool qt_core_c_QString_contains_const_QString_ref_Qt_CaseSensitivity(const QString* this_ptr, const QString* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->contains(*s, *cs);
}

int qt_core_c_QString_count_QChar(const QString* this_ptr, const QChar* c) {
  return this_ptr->count(*c);
}

int qt_core_c_QString_count_QChar_Qt_CaseSensitivity(const QString* this_ptr, const QChar* c, const Qt::CaseSensitivity* cs) {
  return this_ptr->count(*c, *cs);
}

int qt_core_c_QString_count_QRegExp(const QString* this_ptr, const QRegExp* arg1) {
  return this_ptr->count(*arg1);
}

int qt_core_c_QString_count_QRegularExpression(const QString* this_ptr, const QRegularExpression* re) {
  return this_ptr->count(*re);
}

int qt_core_c_QString_count_QString(const QString* this_ptr, const QString* s) {
  return this_ptr->count(*s);
}

int qt_core_c_QString_count_QStringRef(const QString* this_ptr, const QStringRef* s) {
  return this_ptr->count(*s);
}

int qt_core_c_QString_count_QStringRef_Qt_CaseSensitivity(const QString* this_ptr, const QStringRef* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->count(*s, *cs);
}

int qt_core_c_QString_count_QString_Qt_CaseSensitivity(const QString* this_ptr, const QString* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->count(*s, *cs);
}

int qt_core_c_QString_count_no_args(const QString* this_ptr) {
  return this_ptr->count();
}

QChar* qt_core_c_QString_data(QString* this_ptr) {
  return this_ptr->data();
}

const QChar* qt_core_c_QString_data_const(const QString* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QString_destructor(QString* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QChar* qt_core_c_QString_end(QString* this_ptr) {
  return this_ptr->end();
}

const QChar* qt_core_c_QString_end_const(const QString* this_ptr) {
  return this_ptr->end();
}

bool qt_core_c_QString_endsWith_QChar(const QString* this_ptr, const QChar* c) {
  return this_ptr->endsWith(*c);
}

bool qt_core_c_QString_endsWith_QChar_Qt_CaseSensitivity(const QString* this_ptr, const QChar* c, const Qt::CaseSensitivity* cs) {
  return this_ptr->endsWith(*c, *cs);
}

bool qt_core_c_QString_endsWith_QLatin1String(const QString* this_ptr, const QLatin1String* s) {
  return this_ptr->endsWith(*s);
}

bool qt_core_c_QString_endsWith_QLatin1String_Qt_CaseSensitivity(const QString* this_ptr, const QLatin1String* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->endsWith(*s, *cs);
}

bool qt_core_c_QString_endsWith_QString(const QString* this_ptr, const QString* s) {
  return this_ptr->endsWith(*s);
}

bool qt_core_c_QString_endsWith_QStringRef(const QString* this_ptr, const QStringRef* s) {
  return this_ptr->endsWith(*s);
}

bool qt_core_c_QString_endsWith_QStringRef_Qt_CaseSensitivity(const QString* this_ptr, const QStringRef* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->endsWith(*s, *cs);
}

bool qt_core_c_QString_endsWith_QString_Qt_CaseSensitivity(const QString* this_ptr, const QString* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->endsWith(*s, *cs);
}

QString* qt_core_c_QString_fill_c(QString* this_ptr, const QChar* c) {
  return &this_ptr->fill(*c);
}

QString* qt_core_c_QString_fill_c_size(QString* this_ptr, const QChar* c, int size) {
  return &this_ptr->fill(*c, size);
}

void qt_core_c_QString_fromLatin1_to_output_QByteArray(const QByteArray* str, QString* output) {
  new(output) QString(QString::fromLatin1(*str));
}

void qt_core_c_QString_fromLatin1_to_output_char(const char* str, QString* output) {
  new(output) QString(QString::fromLatin1(str));
}

void qt_core_c_QString_fromLatin1_to_output_char_int(const char* str, int size, QString* output) {
  new(output) QString(QString::fromLatin1(str, size));
}

void qt_core_c_QString_fromLocal8Bit_to_output_QByteArray(const QByteArray* str, QString* output) {
  new(output) QString(QString::fromLocal8Bit(*str));
}

void qt_core_c_QString_fromLocal8Bit_to_output_char(const char* str, QString* output) {
  new(output) QString(QString::fromLocal8Bit(str));
}

void qt_core_c_QString_fromLocal8Bit_to_output_char_int(const char* str, int size, QString* output) {
  new(output) QString(QString::fromLocal8Bit(str, size));
}

void qt_core_c_QString_fromRawData_to_output(const QChar* arg1, int size, QString* output) {
  new(output) QString(QString::fromRawData(arg1, size));
}

void qt_core_c_QString_fromUcs4_to_output_arg1(const unsigned int* arg1, QString* output) {
  new(output) QString(QString::fromUcs4(arg1));
}

void qt_core_c_QString_fromUcs4_to_output_arg1_size(const unsigned int* arg1, int size, QString* output) {
  new(output) QString(QString::fromUcs4(arg1, size));
}

void qt_core_c_QString_fromUcs4_to_output_str(const char32_t* str, QString* output) {
  new(output) QString(QString::fromUcs4(str));
}

void qt_core_c_QString_fromUcs4_to_output_str_size(const char32_t* str, int size, QString* output) {
  new(output) QString(QString::fromUcs4(str, size));
}

void qt_core_c_QString_fromUtf16_to_output_arg1(const unsigned short* arg1, QString* output) {
  new(output) QString(QString::fromUtf16(arg1));
}

void qt_core_c_QString_fromUtf16_to_output_arg1_size(const unsigned short* arg1, int size, QString* output) {
  new(output) QString(QString::fromUtf16(arg1, size));
}

void qt_core_c_QString_fromUtf16_to_output_str(const char16_t* str, QString* output) {
  new(output) QString(QString::fromUtf16(str));
}

void qt_core_c_QString_fromUtf16_to_output_str_size(const char16_t* str, int size, QString* output) {
  new(output) QString(QString::fromUtf16(str, size));
}

void qt_core_c_QString_fromUtf8_to_output_QByteArray(const QByteArray* str, QString* output) {
  new(output) QString(QString::fromUtf8(*str));
}

void qt_core_c_QString_fromUtf8_to_output_char(const char* str, QString* output) {
  new(output) QString(QString::fromUtf8(str));
}

void qt_core_c_QString_fromUtf8_to_output_char_int(const char* str, int size, QString* output) {
  new(output) QString(QString::fromUtf8(str, size));
}

void qt_core_c_QString_fromWCharArray_to_output_string(const wchar_t* string, QString* output) {
  new(output) QString(QString::fromWCharArray(string));
}

void qt_core_c_QString_fromWCharArray_to_output_string_size(const wchar_t* string, int size, QString* output) {
  new(output) QString(QString::fromWCharArray(string, size));
}

int qt_core_c_QString_indexOf_QChar(const QString* this_ptr, const QChar* c) {
  return this_ptr->indexOf(*c);
}

int qt_core_c_QString_indexOf_QChar_int(const QString* this_ptr, const QChar* c, int from) {
  return this_ptr->indexOf(*c, from);
}

int qt_core_c_QString_indexOf_QChar_int_Qt_CaseSensitivity(const QString* this_ptr, const QChar* c, int from, const Qt::CaseSensitivity* cs) {
  return this_ptr->indexOf(*c, from, *cs);
}

int qt_core_c_QString_indexOf_QLatin1String(const QString* this_ptr, const QLatin1String* s) {
  return this_ptr->indexOf(*s);
}

int qt_core_c_QString_indexOf_QLatin1String_int(const QString* this_ptr, const QLatin1String* s, int from) {
  return this_ptr->indexOf(*s, from);
}

int qt_core_c_QString_indexOf_QLatin1String_int_Qt_CaseSensitivity(const QString* this_ptr, const QLatin1String* s, int from, const Qt::CaseSensitivity* cs) {
  return this_ptr->indexOf(*s, from, *cs);
}

int qt_core_c_QString_indexOf_QRegExp_ref(const QString* this_ptr, QRegExp* arg1) {
  return this_ptr->indexOf(*arg1);
}

int qt_core_c_QString_indexOf_QRegExp_ref_int(const QString* this_ptr, QRegExp* arg1, int from) {
  return this_ptr->indexOf(*arg1, from);
}

int qt_core_c_QString_indexOf_const_QRegExp_ref(const QString* this_ptr, const QRegExp* arg1) {
  return this_ptr->indexOf(*arg1);
}

int qt_core_c_QString_indexOf_const_QRegExp_ref_int(const QString* this_ptr, const QRegExp* arg1, int from) {
  return this_ptr->indexOf(*arg1, from);
}

int qt_core_c_QString_indexOf_const_QRegularExpression_ref(const QString* this_ptr, const QRegularExpression* re) {
  return this_ptr->indexOf(*re);
}

int qt_core_c_QString_indexOf_const_QRegularExpression_ref_int(const QString* this_ptr, const QRegularExpression* re, int from) {
  return this_ptr->indexOf(*re, from);
}

int qt_core_c_QString_indexOf_const_QRegularExpression_ref_int_QRegularExpressionMatch_ptr(const QString* this_ptr, const QRegularExpression* re, int from, QRegularExpressionMatch* rmatch) {
  return this_ptr->indexOf(*re, from, rmatch);
}

int qt_core_c_QString_indexOf_const_QStringRef_ref(const QString* this_ptr, const QStringRef* s) {
  return this_ptr->indexOf(*s);
}

int qt_core_c_QString_indexOf_const_QStringRef_ref_int(const QString* this_ptr, const QStringRef* s, int from) {
  return this_ptr->indexOf(*s, from);
}

int qt_core_c_QString_indexOf_const_QStringRef_ref_int_Qt_CaseSensitivity(const QString* this_ptr, const QStringRef* s, int from, const Qt::CaseSensitivity* cs) {
  return this_ptr->indexOf(*s, from, *cs);
}

int qt_core_c_QString_indexOf_const_QString_ref(const QString* this_ptr, const QString* s) {
  return this_ptr->indexOf(*s);
}

int qt_core_c_QString_indexOf_const_QString_ref_int(const QString* this_ptr, const QString* s, int from) {
  return this_ptr->indexOf(*s, from);
}

int qt_core_c_QString_indexOf_const_QString_ref_int_Qt_CaseSensitivity(const QString* this_ptr, const QString* s, int from, const Qt::CaseSensitivity* cs) {
  return this_ptr->indexOf(*s, from, *cs);
}

QString* qt_core_c_QString_insert_int_QByteArray(QString* this_ptr, int i, const QByteArray* s) {
  return &this_ptr->insert(i, *s);
}

QString* qt_core_c_QString_insert_int_QChar(QString* this_ptr, int i, const QChar* c) {
  return &this_ptr->insert(i, *c);
}

QString* qt_core_c_QString_insert_int_QChar_int(QString* this_ptr, int i, const QChar* uc, int len) {
  return &this_ptr->insert(i, uc, len);
}

QString* qt_core_c_QString_insert_int_QLatin1String(QString* this_ptr, int i, const QLatin1String* s) {
  return &this_ptr->insert(i, *s);
}

QString* qt_core_c_QString_insert_int_QString(QString* this_ptr, int i, const QString* s) {
  return &this_ptr->insert(i, *s);
}

QString* qt_core_c_QString_insert_int_QStringRef(QString* this_ptr, int i, const QStringRef* s) {
  return &this_ptr->insert(i, *s);
}

QString* qt_core_c_QString_insert_int_char(QString* this_ptr, int i, const char* s) {
  return &this_ptr->insert(i, s);
}

bool qt_core_c_QString_isEmpty(const QString* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_core_c_QString_isNull(const QString* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QString_isRightToLeft(const QString* this_ptr) {
  return this_ptr->isRightToLeft();
}

int qt_core_c_QString_lastIndexOf_QChar(const QString* this_ptr, const QChar* c) {
  return this_ptr->lastIndexOf(*c);
}

int qt_core_c_QString_lastIndexOf_QChar_int(const QString* this_ptr, const QChar* c, int from) {
  return this_ptr->lastIndexOf(*c, from);
}

int qt_core_c_QString_lastIndexOf_QChar_int_Qt_CaseSensitivity(const QString* this_ptr, const QChar* c, int from, const Qt::CaseSensitivity* cs) {
  return this_ptr->lastIndexOf(*c, from, *cs);
}

int qt_core_c_QString_lastIndexOf_QLatin1String(const QString* this_ptr, const QLatin1String* s) {
  return this_ptr->lastIndexOf(*s);
}

int qt_core_c_QString_lastIndexOf_QLatin1String_int(const QString* this_ptr, const QLatin1String* s, int from) {
  return this_ptr->lastIndexOf(*s, from);
}

int qt_core_c_QString_lastIndexOf_QLatin1String_int_Qt_CaseSensitivity(const QString* this_ptr, const QLatin1String* s, int from, const Qt::CaseSensitivity* cs) {
  return this_ptr->lastIndexOf(*s, from, *cs);
}

int qt_core_c_QString_lastIndexOf_QRegExp_ref(const QString* this_ptr, QRegExp* arg1) {
  return this_ptr->lastIndexOf(*arg1);
}

int qt_core_c_QString_lastIndexOf_QRegExp_ref_int(const QString* this_ptr, QRegExp* arg1, int from) {
  return this_ptr->lastIndexOf(*arg1, from);
}

int qt_core_c_QString_lastIndexOf_const_QRegExp_ref(const QString* this_ptr, const QRegExp* arg1) {
  return this_ptr->lastIndexOf(*arg1);
}

int qt_core_c_QString_lastIndexOf_const_QRegExp_ref_int(const QString* this_ptr, const QRegExp* arg1, int from) {
  return this_ptr->lastIndexOf(*arg1, from);
}

int qt_core_c_QString_lastIndexOf_const_QRegularExpression_ref(const QString* this_ptr, const QRegularExpression* re) {
  return this_ptr->lastIndexOf(*re);
}

int qt_core_c_QString_lastIndexOf_const_QRegularExpression_ref_int(const QString* this_ptr, const QRegularExpression* re, int from) {
  return this_ptr->lastIndexOf(*re, from);
}

int qt_core_c_QString_lastIndexOf_const_QRegularExpression_ref_int_QRegularExpressionMatch_ptr(const QString* this_ptr, const QRegularExpression* re, int from, QRegularExpressionMatch* rmatch) {
  return this_ptr->lastIndexOf(*re, from, rmatch);
}

int qt_core_c_QString_lastIndexOf_const_QStringRef_ref(const QString* this_ptr, const QStringRef* s) {
  return this_ptr->lastIndexOf(*s);
}

int qt_core_c_QString_lastIndexOf_const_QStringRef_ref_int(const QString* this_ptr, const QStringRef* s, int from) {
  return this_ptr->lastIndexOf(*s, from);
}

int qt_core_c_QString_lastIndexOf_const_QStringRef_ref_int_Qt_CaseSensitivity(const QString* this_ptr, const QStringRef* s, int from, const Qt::CaseSensitivity* cs) {
  return this_ptr->lastIndexOf(*s, from, *cs);
}

int qt_core_c_QString_lastIndexOf_const_QString_ref(const QString* this_ptr, const QString* s) {
  return this_ptr->lastIndexOf(*s);
}

int qt_core_c_QString_lastIndexOf_const_QString_ref_int(const QString* this_ptr, const QString* s, int from) {
  return this_ptr->lastIndexOf(*s, from);
}

int qt_core_c_QString_lastIndexOf_const_QString_ref_int_Qt_CaseSensitivity(const QString* this_ptr, const QString* s, int from, const Qt::CaseSensitivity* cs) {
  return this_ptr->lastIndexOf(*s, from, *cs);
}

void qt_core_c_QString_leftJustified_to_output_width(const QString* this_ptr, int width, QString* output) {
  new(output) QString(this_ptr->leftJustified(width));
}

void qt_core_c_QString_leftJustified_to_output_width_fill(const QString* this_ptr, int width, const QChar* fill, QString* output) {
  new(output) QString(this_ptr->leftJustified(width, *fill));
}

void qt_core_c_QString_leftJustified_to_output_width_fill_trunc(const QString* this_ptr, int width, const QChar* fill, bool trunc, QString* output) {
  new(output) QString(this_ptr->leftJustified(width, *fill, trunc));
}

void qt_core_c_QString_leftRef_to_output(const QString* this_ptr, int n, QStringRef* output) {
  new(output) QStringRef(this_ptr->leftRef(n));
}

void qt_core_c_QString_left_to_output(const QString* this_ptr, int n, QString* output) {
  new(output) QString(this_ptr->left(n));
}

int qt_core_c_QString_length(const QString* this_ptr) {
  return this_ptr->length();
}

int qt_core_c_QString_localeAwareCompare_QString(const QString* this_ptr, const QString* s) {
  return this_ptr->localeAwareCompare(*s);
}

int qt_core_c_QString_localeAwareCompare_QStringRef(const QString* this_ptr, const QStringRef* s) {
  return this_ptr->localeAwareCompare(*s);
}

int qt_core_c_QString_localeAwareCompare_QString_QString(const QString* s1, const QString* s2) {
  return QString::localeAwareCompare(*s1, *s2);
}

int qt_core_c_QString_localeAwareCompare_QString_QStringRef(const QString* s1, const QStringRef* s2) {
  return QString::localeAwareCompare(*s1, *s2);
}

void qt_core_c_QString_midRef_to_output_position(const QString* this_ptr, int position, QStringRef* output) {
  new(output) QStringRef(this_ptr->midRef(position));
}

void qt_core_c_QString_midRef_to_output_position_n(const QString* this_ptr, int position, int n, QStringRef* output) {
  new(output) QStringRef(this_ptr->midRef(position, n));
}

void qt_core_c_QString_mid_to_output_position(const QString* this_ptr, int position, QString* output) {
  new(output) QString(this_ptr->mid(position));
}

void qt_core_c_QString_mid_to_output_position_n(const QString* this_ptr, int position, int n, QString* output) {
  new(output) QString(this_ptr->mid(position, n));
}

void qt_core_c_QString_normalized_to_output_mode(const QString* this_ptr, QString::NormalizationForm mode, QString* output) {
  new(output) QString(this_ptr->normalized(mode));
}

void qt_core_c_QString_normalized_to_output_mode_version(const QString* this_ptr, QString::NormalizationForm mode, const QChar::UnicodeVersion* version, QString* output) {
  new(output) QString(this_ptr->normalized(mode, *version));
}

void qt_core_c_QString_number_to_output_double(double arg1, QString* output) {
  new(output) QString(QString::number(arg1));
}

void qt_core_c_QString_number_to_output_double_char(double arg1, char f, QString* output) {
  new(output) QString(QString::number(arg1, f));
}

void qt_core_c_QString_number_to_output_double_char_int(double arg1, char f, int prec, QString* output) {
  new(output) QString(QString::number(arg1, f, prec));
}

void qt_core_c_QString_number_to_output_int(int arg1, QString* output) {
  new(output) QString(QString::number(arg1));
}

void qt_core_c_QString_number_to_output_int_int(int arg1, int base, QString* output) {
  new(output) QString(QString::number(arg1, base));
}

void qt_core_c_QString_number_to_output_long(long arg1, QString* output) {
  new(output) QString(QString::number(arg1));
}

void qt_core_c_QString_number_to_output_long_int(long arg1, int base, QString* output) {
  new(output) QString(QString::number(arg1, base));
}

void qt_core_c_QString_number_to_output_qlonglong(qlonglong arg1, QString* output) {
  new(output) QString(QString::number(arg1));
}

void qt_core_c_QString_number_to_output_qlonglong_int(qlonglong arg1, int base, QString* output) {
  new(output) QString(QString::number(arg1, base));
}

void qt_core_c_QString_number_to_output_qulonglong(qulonglong arg1, QString* output) {
  new(output) QString(QString::number(arg1));
}

void qt_core_c_QString_number_to_output_qulonglong_int(qulonglong arg1, int base, QString* output) {
  new(output) QString(QString::number(arg1, base));
}

void qt_core_c_QString_number_to_output_unsigned_int(unsigned int arg1, QString* output) {
  new(output) QString(QString::number(arg1));
}

void qt_core_c_QString_number_to_output_unsigned_int_int(unsigned int arg1, int base, QString* output) {
  new(output) QString(QString::number(arg1, base));
}

void qt_core_c_QString_number_to_output_unsigned_long(unsigned long arg1, QString* output) {
  new(output) QString(QString::number(arg1));
}

void qt_core_c_QString_number_to_output_unsigned_long_int(unsigned long arg1, int base, QString* output) {
  new(output) QString(QString::number(arg1, base));
}

QString* qt_core_c_QString_operator_add_assign_QByteArray_s(QString* this_ptr, const QByteArray* s) {
  return &this_ptr->operator+=(*s);
}

QString* qt_core_c_QString_operator_add_assign_QChar_SpecialCharacter_c(QString* this_ptr, const QChar::SpecialCharacter* c) {
  return &this_ptr->operator+=(*c);
}

QString* qt_core_c_QString_operator_add_assign_QChar_c(QString* this_ptr, const QChar* c) {
  return &this_ptr->operator+=(*c);
}

QString* qt_core_c_QString_operator_add_assign_QLatin1String_s(QString* this_ptr, const QLatin1String* s) {
  return &this_ptr->operator+=(*s);
}

QString* qt_core_c_QString_operator_add_assign_QStringRef_s(QString* this_ptr, const QStringRef* s) {
  return &this_ptr->operator+=(*s);
}

QString* qt_core_c_QString_operator_add_assign_QString_s(QString* this_ptr, const QString* s) {
  return &this_ptr->operator+=(*s);
}

QString* qt_core_c_QString_operator_add_assign_char_c(QString* this_ptr, char c) {
  return &this_ptr->operator+=(c);
}

QString* qt_core_c_QString_operator_add_assign_char_s(QString* this_ptr, const char* s) {
  return &this_ptr->operator+=(s);
}

QString* qt_core_c_QString_operator_assign_QByteArray_a(QString* this_ptr, const QByteArray* a) {
  return &this_ptr->operator=(*a);
}

QString* qt_core_c_QString_operator_assign_QChar_c(QString* this_ptr, const QChar* c) {
  return &this_ptr->operator=(*c);
}

QString* qt_core_c_QString_operator_assign_QLatin1String_latin1(QString* this_ptr, const QLatin1String* latin1) {
  return &this_ptr->operator=(*latin1);
}

QString* qt_core_c_QString_operator_assign_QString_arg1(QString* this_ptr, const QString* arg1) {
  return &this_ptr->operator=(*arg1);
}

QString* qt_core_c_QString_operator_assign_char_c(QString* this_ptr, char c) {
  return &this_ptr->operator=(c);
}

QString* qt_core_c_QString_operator_assign_char_ch(QString* this_ptr, const char* ch) {
  return &this_ptr->operator=(ch);
}

bool qt_core_c_QString_operator_eq_QByteArray(const QString* this_ptr, const QByteArray* s) {
  return this_ptr->operator==(*s);
}

bool qt_core_c_QString_operator_eq_QLatin1String(const QString* this_ptr, const QLatin1String* s) {
  return this_ptr->operator==(*s);
}

bool qt_core_c_QString_operator_eq_char(const QString* this_ptr, const char* s) {
  return this_ptr->operator==(s);
}

bool qt_core_c_QString_operator_ge_QByteArray(const QString* this_ptr, const QByteArray* s) {
  return this_ptr->operator>=(*s);
}

bool qt_core_c_QString_operator_ge_QLatin1String(const QString* this_ptr, const QLatin1String* s) {
  return this_ptr->operator>=(*s);
}

bool qt_core_c_QString_operator_ge_char(const QString* this_ptr, const char* s) {
  return this_ptr->operator>=(s);
}

bool qt_core_c_QString_operator_gt_QByteArray(const QString* this_ptr, const QByteArray* s) {
  return this_ptr->operator>(*s);
}

bool qt_core_c_QString_operator_gt_QLatin1String(const QString* this_ptr, const QLatin1String* s) {
  return this_ptr->operator>(*s);
}

bool qt_core_c_QString_operator_gt_char(const QString* this_ptr, const char* s) {
  return this_ptr->operator>(s);
}

void qt_core_c_QString_operator_index_to_output_const_int(const QString* this_ptr, int i, QChar* output) {
  new(output) QChar(this_ptr->operator[](i));
}

void qt_core_c_QString_operator_index_to_output_const_unsigned_int(const QString* this_ptr, unsigned int i, QChar* output) {
  new(output) QChar(this_ptr->operator[](i));
}

void qt_core_c_QString_operator_index_to_output_int(QString* this_ptr, int i, QCharRef* output) {
  new(output) QCharRef(this_ptr->operator[](i));
}

void qt_core_c_QString_operator_index_to_output_unsigned_int(QString* this_ptr, unsigned int i, QCharRef* output) {
  new(output) QCharRef(this_ptr->operator[](i));
}

bool qt_core_c_QString_operator_le_QByteArray(const QString* this_ptr, const QByteArray* s) {
  return this_ptr->operator<=(*s);
}

bool qt_core_c_QString_operator_le_QLatin1String(const QString* this_ptr, const QLatin1String* s) {
  return this_ptr->operator<=(*s);
}

bool qt_core_c_QString_operator_le_char(const QString* this_ptr, const char* s) {
  return this_ptr->operator<=(s);
}

bool qt_core_c_QString_operator_lt_QByteArray(const QString* this_ptr, const QByteArray* s) {
  return this_ptr->operator<(*s);
}

bool qt_core_c_QString_operator_lt_QLatin1String(const QString* this_ptr, const QLatin1String* s) {
  return this_ptr->operator<(*s);
}

bool qt_core_c_QString_operator_lt_char(const QString* this_ptr, const char* s) {
  return this_ptr->operator<(s);
}

bool qt_core_c_QString_operator_neq_QByteArray(const QString* this_ptr, const QByteArray* s) {
  return this_ptr->operator!=(*s);
}

bool qt_core_c_QString_operator_neq_QLatin1String(const QString* this_ptr, const QLatin1String* s) {
  return this_ptr->operator!=(*s);
}

bool qt_core_c_QString_operator_neq_char(const QString* this_ptr, const char* s) {
  return this_ptr->operator!=(s);
}

QString* qt_core_c_QString_prepend_QByteArray(QString* this_ptr, const QByteArray* s) {
  return &this_ptr->prepend(*s);
}

QString* qt_core_c_QString_prepend_QChar(QString* this_ptr, const QChar* c) {
  return &this_ptr->prepend(*c);
}

QString* qt_core_c_QString_prepend_QChar_int(QString* this_ptr, const QChar* uc, int len) {
  return &this_ptr->prepend(uc, len);
}

QString* qt_core_c_QString_prepend_QLatin1String(QString* this_ptr, const QLatin1String* s) {
  return &this_ptr->prepend(*s);
}

QString* qt_core_c_QString_prepend_QString(QString* this_ptr, const QString* s) {
  return &this_ptr->prepend(*s);
}

QString* qt_core_c_QString_prepend_QStringRef(QString* this_ptr, const QStringRef* s) {
  return &this_ptr->prepend(*s);
}

QString* qt_core_c_QString_prepend_char(QString* this_ptr, const char* s) {
  return &this_ptr->prepend(s);
}

void qt_core_c_QString_push_back_c(QString* this_ptr, const QChar* c) {
  this_ptr->push_back(*c);
}

void qt_core_c_QString_push_back_s(QString* this_ptr, const QString* s) {
  this_ptr->push_back(*s);
}

void qt_core_c_QString_push_front_c(QString* this_ptr, const QChar* c) {
  this_ptr->push_front(*c);
}

void qt_core_c_QString_push_front_s(QString* this_ptr, const QString* s) {
  this_ptr->push_front(*s);
}

QString* qt_core_c_QString_remove_c(QString* this_ptr, const QChar* c) {
  return &this_ptr->remove(*c);
}

QString* qt_core_c_QString_remove_c_cs(QString* this_ptr, const QChar* c, const Qt::CaseSensitivity* cs) {
  return &this_ptr->remove(*c, *cs);
}

QString* qt_core_c_QString_remove_i_len(QString* this_ptr, int i, int len) {
  return &this_ptr->remove(i, len);
}

QString* qt_core_c_QString_remove_re(QString* this_ptr, const QRegularExpression* re) {
  return &this_ptr->remove(*re);
}

QString* qt_core_c_QString_remove_rx(QString* this_ptr, const QRegExp* rx) {
  return &this_ptr->remove(*rx);
}

QString* qt_core_c_QString_remove_s(QString* this_ptr, const QString* s) {
  return &this_ptr->remove(*s);
}

QString* qt_core_c_QString_remove_s_cs(QString* this_ptr, const QString* s, const Qt::CaseSensitivity* cs) {
  return &this_ptr->remove(*s, *cs);
}

void qt_core_c_QString_repeated_to_output(const QString* this_ptr, int times, QString* output) {
  new(output) QString(this_ptr->repeated(times));
}

QString* qt_core_c_QString_replace_QChar_QChar(QString* this_ptr, const QChar* before, const QChar* after) {
  return &this_ptr->replace(*before, *after);
}

QString* qt_core_c_QString_replace_QChar_QChar_Qt_CaseSensitivity(QString* this_ptr, const QChar* before, const QChar* after, const Qt::CaseSensitivity* cs) {
  return &this_ptr->replace(*before, *after, *cs);
}

QString* qt_core_c_QString_replace_QChar_QLatin1String(QString* this_ptr, const QChar* c, const QLatin1String* after) {
  return &this_ptr->replace(*c, *after);
}

QString* qt_core_c_QString_replace_QChar_QLatin1String_Qt_CaseSensitivity(QString* this_ptr, const QChar* c, const QLatin1String* after, const Qt::CaseSensitivity* cs) {
  return &this_ptr->replace(*c, *after, *cs);
}

QString* qt_core_c_QString_replace_QChar_QString(QString* this_ptr, const QChar* c, const QString* after) {
  return &this_ptr->replace(*c, *after);
}

QString* qt_core_c_QString_replace_QChar_QString_Qt_CaseSensitivity(QString* this_ptr, const QChar* c, const QString* after, const Qt::CaseSensitivity* cs) {
  return &this_ptr->replace(*c, *after, *cs);
}

QString* qt_core_c_QString_replace_QChar_int_QChar_int(QString* this_ptr, const QChar* before, int blen, const QChar* after, int alen) {
  return &this_ptr->replace(before, blen, after, alen);
}

QString* qt_core_c_QString_replace_QChar_int_QChar_int_Qt_CaseSensitivity(QString* this_ptr, const QChar* before, int blen, const QChar* after, int alen, const Qt::CaseSensitivity* cs) {
  return &this_ptr->replace(before, blen, after, alen, *cs);
}

QString* qt_core_c_QString_replace_QLatin1String_QLatin1String(QString* this_ptr, const QLatin1String* before, const QLatin1String* after) {
  return &this_ptr->replace(*before, *after);
}

QString* qt_core_c_QString_replace_QLatin1String_QLatin1String_Qt_CaseSensitivity(QString* this_ptr, const QLatin1String* before, const QLatin1String* after, const Qt::CaseSensitivity* cs) {
  return &this_ptr->replace(*before, *after, *cs);
}

QString* qt_core_c_QString_replace_QLatin1String_QString(QString* this_ptr, const QLatin1String* before, const QString* after) {
  return &this_ptr->replace(*before, *after);
}

QString* qt_core_c_QString_replace_QLatin1String_QString_Qt_CaseSensitivity(QString* this_ptr, const QLatin1String* before, const QString* after, const Qt::CaseSensitivity* cs) {
  return &this_ptr->replace(*before, *after, *cs);
}

QString* qt_core_c_QString_replace_QRegExp_QString(QString* this_ptr, const QRegExp* rx, const QString* after) {
  return &this_ptr->replace(*rx, *after);
}

QString* qt_core_c_QString_replace_QRegularExpression_QString(QString* this_ptr, const QRegularExpression* re, const QString* after) {
  return &this_ptr->replace(*re, *after);
}

QString* qt_core_c_QString_replace_QString_QLatin1String(QString* this_ptr, const QString* before, const QLatin1String* after) {
  return &this_ptr->replace(*before, *after);
}

QString* qt_core_c_QString_replace_QString_QLatin1String_Qt_CaseSensitivity(QString* this_ptr, const QString* before, const QLatin1String* after, const Qt::CaseSensitivity* cs) {
  return &this_ptr->replace(*before, *after, *cs);
}

QString* qt_core_c_QString_replace_QString_QString(QString* this_ptr, const QString* before, const QString* after) {
  return &this_ptr->replace(*before, *after);
}

QString* qt_core_c_QString_replace_QString_QString_Qt_CaseSensitivity(QString* this_ptr, const QString* before, const QString* after, const Qt::CaseSensitivity* cs) {
  return &this_ptr->replace(*before, *after, *cs);
}

QString* qt_core_c_QString_replace_int_int_QChar(QString* this_ptr, int i, int len, const QChar* after) {
  return &this_ptr->replace(i, len, *after);
}

QString* qt_core_c_QString_replace_int_int_QChar_int(QString* this_ptr, int i, int len, const QChar* s, int slen) {
  return &this_ptr->replace(i, len, s, slen);
}

QString* qt_core_c_QString_replace_int_int_QString(QString* this_ptr, int i, int len, const QString* after) {
  return &this_ptr->replace(i, len, *after);
}

void qt_core_c_QString_reserve(QString* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_core_c_QString_resize_size(QString* this_ptr, int size) {
  this_ptr->resize(size);
}

void qt_core_c_QString_resize_size_fillChar(QString* this_ptr, int size, const QChar* fillChar) {
  this_ptr->resize(size, *fillChar);
}

void qt_core_c_QString_rightJustified_to_output_width(const QString* this_ptr, int width, QString* output) {
  new(output) QString(this_ptr->rightJustified(width));
}

void qt_core_c_QString_rightJustified_to_output_width_fill(const QString* this_ptr, int width, const QChar* fill, QString* output) {
  new(output) QString(this_ptr->rightJustified(width, *fill));
}

void qt_core_c_QString_rightJustified_to_output_width_fill_trunc(const QString* this_ptr, int width, const QChar* fill, bool trunc, QString* output) {
  new(output) QString(this_ptr->rightJustified(width, *fill, trunc));
}

void qt_core_c_QString_rightRef_to_output(const QString* this_ptr, int n, QStringRef* output) {
  new(output) QStringRef(this_ptr->rightRef(n));
}

void qt_core_c_QString_right_to_output(const QString* this_ptr, int n, QString* output) {
  new(output) QString(this_ptr->right(n));
}

void qt_core_c_QString_section_to_output_in_sep_start(const QString* this_ptr, const QString* in_sep, int start, QString* output) {
  new(output) QString(this_ptr->section(*in_sep, start));
}

void qt_core_c_QString_section_to_output_in_sep_start_end(const QString* this_ptr, const QString* in_sep, int start, int end, QString* output) {
  new(output) QString(this_ptr->section(*in_sep, start, end));
}

void qt_core_c_QString_section_to_output_in_sep_start_end_flags(const QString* this_ptr, const QString* in_sep, int start, int end, unsigned int flags, QString* output) {
  new(output) QString(this_ptr->section(*in_sep, start, end, QFlags< QString::SectionFlag >(flags)));
}

void qt_core_c_QString_section_to_output_re_start(const QString* this_ptr, const QRegularExpression* re, int start, QString* output) {
  new(output) QString(this_ptr->section(*re, start));
}

void qt_core_c_QString_section_to_output_re_start_end(const QString* this_ptr, const QRegularExpression* re, int start, int end, QString* output) {
  new(output) QString(this_ptr->section(*re, start, end));
}

void qt_core_c_QString_section_to_output_re_start_end_flags(const QString* this_ptr, const QRegularExpression* re, int start, int end, unsigned int flags, QString* output) {
  new(output) QString(this_ptr->section(*re, start, end, QFlags< QString::SectionFlag >(flags)));
}

void qt_core_c_QString_section_to_output_reg_start(const QString* this_ptr, const QRegExp* reg, int start, QString* output) {
  new(output) QString(this_ptr->section(*reg, start));
}

void qt_core_c_QString_section_to_output_reg_start_end(const QString* this_ptr, const QRegExp* reg, int start, int end, QString* output) {
  new(output) QString(this_ptr->section(*reg, start, end));
}

void qt_core_c_QString_section_to_output_reg_start_end_flags(const QString* this_ptr, const QRegExp* reg, int start, int end, unsigned int flags, QString* output) {
  new(output) QString(this_ptr->section(*reg, start, end, QFlags< QString::SectionFlag >(flags)));
}

void qt_core_c_QString_section_to_output_sep_start(const QString* this_ptr, const QChar* sep, int start, QString* output) {
  new(output) QString(this_ptr->section(*sep, start));
}

void qt_core_c_QString_section_to_output_sep_start_end(const QString* this_ptr, const QChar* sep, int start, int end, QString* output) {
  new(output) QString(this_ptr->section(*sep, start, end));
}

void qt_core_c_QString_section_to_output_sep_start_end_flags(const QString* this_ptr, const QChar* sep, int start, int end, unsigned int flags, QString* output) {
  new(output) QString(this_ptr->section(*sep, start, end, QFlags< QString::SectionFlag >(flags)));
}

QString* qt_core_c_QString_setNum_double(QString* this_ptr, double arg1) {
  return &this_ptr->setNum(arg1);
}

QString* qt_core_c_QString_setNum_double_char(QString* this_ptr, double arg1, char f) {
  return &this_ptr->setNum(arg1, f);
}

QString* qt_core_c_QString_setNum_double_char_int(QString* this_ptr, double arg1, char f, int prec) {
  return &this_ptr->setNum(arg1, f, prec);
}

QString* qt_core_c_QString_setNum_float(QString* this_ptr, float arg1) {
  return &this_ptr->setNum(arg1);
}

QString* qt_core_c_QString_setNum_float_char(QString* this_ptr, float arg1, char f) {
  return &this_ptr->setNum(arg1, f);
}

QString* qt_core_c_QString_setNum_float_char_int(QString* this_ptr, float arg1, char f, int prec) {
  return &this_ptr->setNum(arg1, f, prec);
}

QString* qt_core_c_QString_setNum_int(QString* this_ptr, int arg1) {
  return &this_ptr->setNum(arg1);
}

QString* qt_core_c_QString_setNum_int_int(QString* this_ptr, int arg1, int base) {
  return &this_ptr->setNum(arg1, base);
}

QString* qt_core_c_QString_setNum_long(QString* this_ptr, long arg1) {
  return &this_ptr->setNum(arg1);
}

QString* qt_core_c_QString_setNum_long_int(QString* this_ptr, long arg1, int base) {
  return &this_ptr->setNum(arg1, base);
}

QString* qt_core_c_QString_setNum_qlonglong(QString* this_ptr, qlonglong arg1) {
  return &this_ptr->setNum(arg1);
}

QString* qt_core_c_QString_setNum_qlonglong_int(QString* this_ptr, qlonglong arg1, int base) {
  return &this_ptr->setNum(arg1, base);
}

QString* qt_core_c_QString_setNum_qulonglong(QString* this_ptr, qulonglong arg1) {
  return &this_ptr->setNum(arg1);
}

QString* qt_core_c_QString_setNum_qulonglong_int(QString* this_ptr, qulonglong arg1, int base) {
  return &this_ptr->setNum(arg1, base);
}

QString* qt_core_c_QString_setNum_short(QString* this_ptr, short arg1) {
  return &this_ptr->setNum(arg1);
}

QString* qt_core_c_QString_setNum_short_int(QString* this_ptr, short arg1, int base) {
  return &this_ptr->setNum(arg1, base);
}

QString* qt_core_c_QString_setNum_unsigned_int(QString* this_ptr, unsigned int arg1) {
  return &this_ptr->setNum(arg1);
}

QString* qt_core_c_QString_setNum_unsigned_int_int(QString* this_ptr, unsigned int arg1, int base) {
  return &this_ptr->setNum(arg1, base);
}

QString* qt_core_c_QString_setNum_unsigned_long(QString* this_ptr, unsigned long arg1) {
  return &this_ptr->setNum(arg1);
}

QString* qt_core_c_QString_setNum_unsigned_long_int(QString* this_ptr, unsigned long arg1, int base) {
  return &this_ptr->setNum(arg1, base);
}

QString* qt_core_c_QString_setNum_unsigned_short(QString* this_ptr, unsigned short arg1) {
  return &this_ptr->setNum(arg1);
}

QString* qt_core_c_QString_setNum_unsigned_short_int(QString* this_ptr, unsigned short arg1, int base) {
  return &this_ptr->setNum(arg1, base);
}

QString* qt_core_c_QString_setRawData(QString* this_ptr, const QChar* unicode, int size) {
  return &this_ptr->setRawData(unicode, size);
}

QString* qt_core_c_QString_setUnicode(QString* this_ptr, const QChar* unicode, int size) {
  return &this_ptr->setUnicode(unicode, size);
}

QString* qt_core_c_QString_setUtf16(QString* this_ptr, const unsigned short* utf16, int size) {
  return &this_ptr->setUtf16(utf16, size);
}

void qt_core_c_QString_simplified_to_output(QString* this_ptr, QString* output) {
  new(output) QString(this_ptr->simplified());
}

void qt_core_c_QString_simplified_to_output_const(const QString* this_ptr, QString* output) {
  new(output) QString(this_ptr->simplified());
}

int qt_core_c_QString_size(const QString* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QString_splitRef_to_output_QChar(const QString* this_ptr, const QChar* sep, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->splitRef(*sep));
}

void qt_core_c_QString_splitRef_to_output_QChar_QString_SplitBehavior(const QString* this_ptr, const QChar* sep, QString::SplitBehavior behavior, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->splitRef(*sep, behavior));
}

void qt_core_c_QString_splitRef_to_output_QChar_QString_SplitBehavior_Qt_CaseSensitivity(const QString* this_ptr, const QChar* sep, QString::SplitBehavior behavior, const Qt::CaseSensitivity* cs, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->splitRef(*sep, behavior, *cs));
}

void qt_core_c_QString_splitRef_to_output_QRegExp(const QString* this_ptr, const QRegExp* sep, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->splitRef(*sep));
}

void qt_core_c_QString_splitRef_to_output_QRegExp_QString_SplitBehavior(const QString* this_ptr, const QRegExp* sep, QString::SplitBehavior behavior, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->splitRef(*sep, behavior));
}

void qt_core_c_QString_splitRef_to_output_QRegularExpression(const QString* this_ptr, const QRegularExpression* sep, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->splitRef(*sep));
}

void qt_core_c_QString_splitRef_to_output_QRegularExpression_QString_SplitBehavior(const QString* this_ptr, const QRegularExpression* sep, QString::SplitBehavior behavior, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->splitRef(*sep, behavior));
}

void qt_core_c_QString_splitRef_to_output_QString(const QString* this_ptr, const QString* sep, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->splitRef(*sep));
}

void qt_core_c_QString_splitRef_to_output_QString_QString_SplitBehavior(const QString* this_ptr, const QString* sep, QString::SplitBehavior behavior, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->splitRef(*sep, behavior));
}

void qt_core_c_QString_splitRef_to_output_QString_QString_SplitBehavior_Qt_CaseSensitivity(const QString* this_ptr, const QString* sep, QString::SplitBehavior behavior, const Qt::CaseSensitivity* cs, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->splitRef(*sep, behavior, *cs));
}

void qt_core_c_QString_split_to_output_QChar(const QString* this_ptr, const QChar* sep, QStringList* output) {
  new(output) QStringList(this_ptr->split(*sep));
}

void qt_core_c_QString_split_to_output_QChar_QString_SplitBehavior(const QString* this_ptr, const QChar* sep, QString::SplitBehavior behavior, QStringList* output) {
  new(output) QStringList(this_ptr->split(*sep, behavior));
}

void qt_core_c_QString_split_to_output_QChar_QString_SplitBehavior_Qt_CaseSensitivity(const QString* this_ptr, const QChar* sep, QString::SplitBehavior behavior, const Qt::CaseSensitivity* cs, QStringList* output) {
  new(output) QStringList(this_ptr->split(*sep, behavior, *cs));
}

void qt_core_c_QString_split_to_output_QRegExp(const QString* this_ptr, const QRegExp* sep, QStringList* output) {
  new(output) QStringList(this_ptr->split(*sep));
}

void qt_core_c_QString_split_to_output_QRegExp_QString_SplitBehavior(const QString* this_ptr, const QRegExp* sep, QString::SplitBehavior behavior, QStringList* output) {
  new(output) QStringList(this_ptr->split(*sep, behavior));
}

void qt_core_c_QString_split_to_output_QRegularExpression(const QString* this_ptr, const QRegularExpression* sep, QStringList* output) {
  new(output) QStringList(this_ptr->split(*sep));
}

void qt_core_c_QString_split_to_output_QRegularExpression_QString_SplitBehavior(const QString* this_ptr, const QRegularExpression* sep, QString::SplitBehavior behavior, QStringList* output) {
  new(output) QStringList(this_ptr->split(*sep, behavior));
}

void qt_core_c_QString_split_to_output_QString(const QString* this_ptr, const QString* sep, QStringList* output) {
  new(output) QStringList(this_ptr->split(*sep));
}

void qt_core_c_QString_split_to_output_QString_QString_SplitBehavior(const QString* this_ptr, const QString* sep, QString::SplitBehavior behavior, QStringList* output) {
  new(output) QStringList(this_ptr->split(*sep, behavior));
}

void qt_core_c_QString_split_to_output_QString_QString_SplitBehavior_Qt_CaseSensitivity(const QString* this_ptr, const QString* sep, QString::SplitBehavior behavior, const Qt::CaseSensitivity* cs, QStringList* output) {
  new(output) QStringList(this_ptr->split(*sep, behavior, *cs));
}

void qt_core_c_QString_squeeze(QString* this_ptr) {
  this_ptr->squeeze();
}

bool qt_core_c_QString_startsWith_QChar(const QString* this_ptr, const QChar* c) {
  return this_ptr->startsWith(*c);
}

bool qt_core_c_QString_startsWith_QChar_Qt_CaseSensitivity(const QString* this_ptr, const QChar* c, const Qt::CaseSensitivity* cs) {
  return this_ptr->startsWith(*c, *cs);
}

bool qt_core_c_QString_startsWith_QLatin1String(const QString* this_ptr, const QLatin1String* s) {
  return this_ptr->startsWith(*s);
}

bool qt_core_c_QString_startsWith_QLatin1String_Qt_CaseSensitivity(const QString* this_ptr, const QLatin1String* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->startsWith(*s, *cs);
}

bool qt_core_c_QString_startsWith_QString(const QString* this_ptr, const QString* s) {
  return this_ptr->startsWith(*s);
}

bool qt_core_c_QString_startsWith_QStringRef(const QString* this_ptr, const QStringRef* s) {
  return this_ptr->startsWith(*s);
}

bool qt_core_c_QString_startsWith_QStringRef_Qt_CaseSensitivity(const QString* this_ptr, const QStringRef* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->startsWith(*s, *cs);
}

bool qt_core_c_QString_startsWith_QString_Qt_CaseSensitivity(const QString* this_ptr, const QString* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->startsWith(*s, *cs);
}

void qt_core_c_QString_swap(QString* this_ptr, QString* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QString_toCaseFolded_to_output(QString* this_ptr, QString* output) {
  new(output) QString(this_ptr->toCaseFolded());
}

void qt_core_c_QString_toCaseFolded_to_output_const(const QString* this_ptr, QString* output) {
  new(output) QString(this_ptr->toCaseFolded());
}

double qt_core_c_QString_toDouble_no_args(const QString* this_ptr) {
  return this_ptr->toDouble();
}

double qt_core_c_QString_toDouble_ok(const QString* this_ptr, bool* ok) {
  return this_ptr->toDouble(ok);
}

float qt_core_c_QString_toFloat_no_args(const QString* this_ptr) {
  return this_ptr->toFloat();
}

float qt_core_c_QString_toFloat_ok(const QString* this_ptr, bool* ok) {
  return this_ptr->toFloat(ok);
}

void qt_core_c_QString_toHtmlEscaped_to_output(const QString* this_ptr, QString* output) {
  new(output) QString(this_ptr->toHtmlEscaped());
}

int qt_core_c_QString_toInt_no_args(const QString* this_ptr) {
  return this_ptr->toInt();
}

int qt_core_c_QString_toInt_ok(const QString* this_ptr, bool* ok) {
  return this_ptr->toInt(ok);
}

int qt_core_c_QString_toInt_ok_base(const QString* this_ptr, bool* ok, int base) {
  return this_ptr->toInt(ok, base);
}

void qt_core_c_QString_toLatin1_to_output(const QString* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toLatin1());
}

void qt_core_c_QString_toLocal8Bit_to_output(const QString* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toLocal8Bit());
}

qlonglong qt_core_c_QString_toLongLong_no_args(const QString* this_ptr) {
  return this_ptr->toLongLong();
}

qlonglong qt_core_c_QString_toLongLong_ok(const QString* this_ptr, bool* ok) {
  return this_ptr->toLongLong(ok);
}

qlonglong qt_core_c_QString_toLongLong_ok_base(const QString* this_ptr, bool* ok, int base) {
  return this_ptr->toLongLong(ok, base);
}

long qt_core_c_QString_toLong_no_args(const QString* this_ptr) {
  return this_ptr->toLong();
}

long qt_core_c_QString_toLong_ok(const QString* this_ptr, bool* ok) {
  return this_ptr->toLong(ok);
}

long qt_core_c_QString_toLong_ok_base(const QString* this_ptr, bool* ok, int base) {
  return this_ptr->toLong(ok, base);
}

void qt_core_c_QString_toLower_to_output(QString* this_ptr, QString* output) {
  new(output) QString(this_ptr->toLower());
}

void qt_core_c_QString_toLower_to_output_const(const QString* this_ptr, QString* output) {
  new(output) QString(this_ptr->toLower());
}

short qt_core_c_QString_toShort_no_args(const QString* this_ptr) {
  return this_ptr->toShort();
}

short qt_core_c_QString_toShort_ok(const QString* this_ptr, bool* ok) {
  return this_ptr->toShort(ok);
}

short qt_core_c_QString_toShort_ok_base(const QString* this_ptr, bool* ok, int base) {
  return this_ptr->toShort(ok, base);
}

unsigned int qt_core_c_QString_toUInt_no_args(const QString* this_ptr) {
  return this_ptr->toUInt();
}

unsigned int qt_core_c_QString_toUInt_ok(const QString* this_ptr, bool* ok) {
  return this_ptr->toUInt(ok);
}

unsigned int qt_core_c_QString_toUInt_ok_base(const QString* this_ptr, bool* ok, int base) {
  return this_ptr->toUInt(ok, base);
}

qulonglong qt_core_c_QString_toULongLong_no_args(const QString* this_ptr) {
  return this_ptr->toULongLong();
}

qulonglong qt_core_c_QString_toULongLong_ok(const QString* this_ptr, bool* ok) {
  return this_ptr->toULongLong(ok);
}

qulonglong qt_core_c_QString_toULongLong_ok_base(const QString* this_ptr, bool* ok, int base) {
  return this_ptr->toULongLong(ok, base);
}

unsigned long qt_core_c_QString_toULong_no_args(const QString* this_ptr) {
  return this_ptr->toULong();
}

unsigned long qt_core_c_QString_toULong_ok(const QString* this_ptr, bool* ok) {
  return this_ptr->toULong(ok);
}

unsigned long qt_core_c_QString_toULong_ok_base(const QString* this_ptr, bool* ok, int base) {
  return this_ptr->toULong(ok, base);
}

unsigned short qt_core_c_QString_toUShort_no_args(const QString* this_ptr) {
  return this_ptr->toUShort();
}

unsigned short qt_core_c_QString_toUShort_ok(const QString* this_ptr, bool* ok) {
  return this_ptr->toUShort(ok);
}

unsigned short qt_core_c_QString_toUShort_ok_base(const QString* this_ptr, bool* ok, int base) {
  return this_ptr->toUShort(ok, base);
}

void qt_core_c_QString_toUcs4_to_output(const QString* this_ptr, QVector< unsigned int >* output) {
  new(output) QVector< unsigned int >(this_ptr->toUcs4());
}

void qt_core_c_QString_toUpper_to_output(QString* this_ptr, QString* output) {
  new(output) QString(this_ptr->toUpper());
}

void qt_core_c_QString_toUpper_to_output_const(const QString* this_ptr, QString* output) {
  new(output) QString(this_ptr->toUpper());
}

void qt_core_c_QString_toUtf8_to_output(const QString* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toUtf8());
}

int qt_core_c_QString_toWCharArray(const QString* this_ptr, wchar_t* array) {
  return this_ptr->toWCharArray(array);
}

void qt_core_c_QString_trimmed_to_output(QString* this_ptr, QString* output) {
  new(output) QString(this_ptr->trimmed());
}

void qt_core_c_QString_trimmed_to_output_const(const QString* this_ptr, QString* output) {
  new(output) QString(this_ptr->trimmed());
}

void qt_core_c_QString_truncate(QString* this_ptr, int pos) {
  this_ptr->truncate(pos);
}

const QChar* qt_core_c_QString_unicode(const QString* this_ptr) {
  return this_ptr->unicode();
}

const unsigned short* qt_core_c_QString_utf16(const QString* this_ptr) {
  return this_ptr->utf16();
}

