#include "qt_core_c_QMargins.h"

void qt_core_c_QMargins_G_operator_add_to_output_QMarginsF_QMarginsF(const QMarginsF* lhs, const QMarginsF* rhs, QMarginsF* output) {
  new(output) QMarginsF(operator+(*lhs, *rhs));
}

void qt_core_c_QMargins_G_operator_add_to_output_QMarginsF_double(const QMarginsF* lhs, double rhs, QMarginsF* output) {
  new(output) QMarginsF(operator+(*lhs, rhs));
}

void qt_core_c_QMargins_G_operator_add_to_output_QMargins_QMargins(const QMargins* m1, const QMargins* m2, QMargins* output) {
  new(output) QMargins(operator+(*m1, *m2));
}

void qt_core_c_QMargins_G_operator_add_to_output_QMargins_int(const QMargins* lhs, int rhs, QMargins* output) {
  new(output) QMargins(operator+(*lhs, rhs));
}

void qt_core_c_QMargins_G_operator_add_to_output_double_QMarginsF(double lhs, const QMarginsF* rhs, QMarginsF* output) {
  new(output) QMarginsF(operator+(lhs, *rhs));
}

void qt_core_c_QMargins_G_operator_add_to_output_int_QMargins(int lhs, const QMargins* rhs, QMargins* output) {
  new(output) QMargins(operator+(lhs, *rhs));
}

void qt_core_c_QMargins_G_operator_div_to_output_QMarginsF_double(const QMarginsF* lhs, double divisor, QMarginsF* output) {
  new(output) QMarginsF(operator/(*lhs, divisor));
}

void qt_core_c_QMargins_G_operator_div_to_output_QMargins_double(const QMargins* margins, double divisor, QMargins* output) {
  new(output) QMargins(operator/(*margins, divisor));
}

void qt_core_c_QMargins_G_operator_div_to_output_QMargins_int(const QMargins* margins, int divisor, QMargins* output) {
  new(output) QMargins(operator/(*margins, divisor));
}

bool qt_core_c_QMargins_G_operator_eq(const QMarginsF* lhs, const QMarginsF* rhs) {
  return operator==(*lhs, *rhs);
}

void qt_core_c_QMargins_G_operator_mul_to_output_QMarginsF_double(const QMarginsF* lhs, double rhs, QMarginsF* output) {
  new(output) QMarginsF(operator*(*lhs, rhs));
}

void qt_core_c_QMargins_G_operator_mul_to_output_QMargins_double(const QMargins* margins, double factor, QMargins* output) {
  new(output) QMargins(operator*(*margins, factor));
}

void qt_core_c_QMargins_G_operator_mul_to_output_QMargins_int(const QMargins* margins, int factor, QMargins* output) {
  new(output) QMargins(operator*(*margins, factor));
}

void qt_core_c_QMargins_G_operator_mul_to_output_double_QMargins(double factor, const QMargins* margins, QMargins* output) {
  new(output) QMargins(operator*(factor, *margins));
}

void qt_core_c_QMargins_G_operator_mul_to_output_double_QMarginsF(double lhs, const QMarginsF* rhs, QMarginsF* output) {
  new(output) QMarginsF(operator*(lhs, *rhs));
}

void qt_core_c_QMargins_G_operator_mul_to_output_int_QMargins(int factor, const QMargins* margins, QMargins* output) {
  new(output) QMargins(operator*(factor, *margins));
}

void qt_core_c_QMargins_G_operator_neg_to_output_QMargins(const QMargins* margins, QMargins* output) {
  new(output) QMargins(operator-(*margins));
}

void qt_core_c_QMargins_G_operator_neg_to_output_QMarginsF(const QMarginsF* margins, QMarginsF* output) {
  new(output) QMarginsF(operator-(*margins));
}

bool qt_core_c_QMargins_G_operator_neq(const QMarginsF* lhs, const QMarginsF* rhs) {
  return operator!=(*lhs, *rhs);
}

QDataStream* qt_core_c_QMargins_G_operator_shl_QDataStream_QMargins(QDataStream* arg1, const QMargins* arg2) {
  return &operator<<(*arg1, *arg2);
}

QDataStream* qt_core_c_QMargins_G_operator_shl_QDataStream_QMarginsF(QDataStream* arg1, const QMarginsF* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_core_c_QMargins_G_operator_shl_to_output_QDebug_QMargins(const QDebug* arg1, const QMargins* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

void qt_core_c_QMargins_G_operator_shl_to_output_QDebug_QMarginsF(const QDebug* arg1, const QMarginsF* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_core_c_QMargins_G_operator_shr_QDataStream_QMargins(QDataStream* arg1, QMargins* arg2) {
  return &operator>>(*arg1, *arg2);
}

QDataStream* qt_core_c_QMargins_G_operator_shr_QDataStream_QMarginsF(QDataStream* arg1, QMarginsF* arg2) {
  return &operator>>(*arg1, *arg2);
}

void qt_core_c_QMargins_G_operator_sub_to_output_QMarginsF_QMarginsF(const QMarginsF* lhs, const QMarginsF* rhs, QMarginsF* output) {
  new(output) QMarginsF(operator-(*lhs, *rhs));
}

void qt_core_c_QMargins_G_operator_sub_to_output_QMarginsF_double(const QMarginsF* lhs, double rhs, QMarginsF* output) {
  new(output) QMarginsF(operator-(*lhs, rhs));
}

void qt_core_c_QMargins_G_operator_sub_to_output_QMargins_QMargins(const QMargins* m1, const QMargins* m2, QMargins* output) {
  new(output) QMargins(operator-(*m1, *m2));
}

void qt_core_c_QMargins_G_operator_sub_to_output_QMargins_int(const QMargins* lhs, int rhs, QMargins* output) {
  new(output) QMargins(operator-(*lhs, rhs));
}

void qt_core_c_QMargins_G_operator_unary_plus_to_output_QMargins(const QMargins* margins, QMargins* output) {
  new(output) QMargins(operator+(*margins));
}

void qt_core_c_QMargins_G_operator_unary_plus_to_output_QMarginsF(const QMarginsF* margins, QMarginsF* output) {
  new(output) QMarginsF(operator+(*margins));
}

int qt_core_c_QMargins_bottom(const QMargins* this_ptr) {
  return this_ptr->bottom();
}

void qt_core_c_QMargins_constructor_left_top_right_bottom(int left, int top, int right, int bottom, QMargins* output) {
  new(output) QMargins(left, top, right, bottom);
}

void qt_core_c_QMargins_constructor_no_args(QMargins* output) {
  new(output) QMargins();
}

void qt_core_c_QMargins_destructor(QMargins* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QMargins_isNull(const QMargins* this_ptr) {
  return this_ptr->isNull();
}

int qt_core_c_QMargins_left(const QMargins* this_ptr) {
  return this_ptr->left();
}

QMargins* qt_core_c_QMargins_operator_add_assign_arg1(QMargins* this_ptr, int arg1) {
  return &this_ptr->operator+=(arg1);
}

QMargins* qt_core_c_QMargins_operator_add_assign_margins(QMargins* this_ptr, const QMargins* margins) {
  return &this_ptr->operator+=(*margins);
}

QMargins* qt_core_c_QMargins_operator_div_assign_double(QMargins* this_ptr, double arg1) {
  return &this_ptr->operator/=(arg1);
}

QMargins* qt_core_c_QMargins_operator_div_assign_int(QMargins* this_ptr, int arg1) {
  return &this_ptr->operator/=(arg1);
}

QMargins* qt_core_c_QMargins_operator_mul_assign_double(QMargins* this_ptr, double arg1) {
  return &this_ptr->operator*=(arg1);
}

QMargins* qt_core_c_QMargins_operator_mul_assign_int(QMargins* this_ptr, int arg1) {
  return &this_ptr->operator*=(arg1);
}

QMargins* qt_core_c_QMargins_operator_sub_assign_arg1(QMargins* this_ptr, int arg1) {
  return &this_ptr->operator-=(arg1);
}

QMargins* qt_core_c_QMargins_operator_sub_assign_margins(QMargins* this_ptr, const QMargins* margins) {
  return &this_ptr->operator-=(*margins);
}

int qt_core_c_QMargins_right(const QMargins* this_ptr) {
  return this_ptr->right();
}

void qt_core_c_QMargins_setBottom(QMargins* this_ptr, int bottom) {
  this_ptr->setBottom(bottom);
}

void qt_core_c_QMargins_setLeft(QMargins* this_ptr, int left) {
  this_ptr->setLeft(left);
}

void qt_core_c_QMargins_setRight(QMargins* this_ptr, int right) {
  this_ptr->setRight(right);
}

void qt_core_c_QMargins_setTop(QMargins* this_ptr, int top) {
  this_ptr->setTop(top);
}

int qt_core_c_QMargins_top(const QMargins* this_ptr) {
  return this_ptr->top();
}

