#include "qt_core_c_QMarginsF.h"

double qt_core_c_QMarginsF_bottom(const QMarginsF* this_ptr) {
  return this_ptr->bottom();
}

void qt_core_c_QMarginsF_constructor_left_top_right_bottom(double left, double top, double right, double bottom, QMarginsF* output) {
  new(output) QMarginsF(left, top, right, bottom);
}

void qt_core_c_QMarginsF_constructor_margins(const QMargins* margins, QMarginsF* output) {
  new(output) QMarginsF(*margins);
}

void qt_core_c_QMarginsF_constructor_no_args(QMarginsF* output) {
  new(output) QMarginsF();
}

void qt_core_c_QMarginsF_destructor(QMarginsF* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QMarginsF_isNull(const QMarginsF* this_ptr) {
  return this_ptr->isNull();
}

double qt_core_c_QMarginsF_left(const QMarginsF* this_ptr) {
  return this_ptr->left();
}

QMarginsF* qt_core_c_QMarginsF_operator_add_assign_addend(QMarginsF* this_ptr, double addend) {
  return &this_ptr->operator+=(addend);
}

QMarginsF* qt_core_c_QMarginsF_operator_add_assign_margins(QMarginsF* this_ptr, const QMarginsF* margins) {
  return &this_ptr->operator+=(*margins);
}

QMarginsF* qt_core_c_QMarginsF_operator_div_assign(QMarginsF* this_ptr, double divisor) {
  return &this_ptr->operator/=(divisor);
}

QMarginsF* qt_core_c_QMarginsF_operator_mul_assign(QMarginsF* this_ptr, double factor) {
  return &this_ptr->operator*=(factor);
}

QMarginsF* qt_core_c_QMarginsF_operator_sub_assign_margins(QMarginsF* this_ptr, const QMarginsF* margins) {
  return &this_ptr->operator-=(*margins);
}

QMarginsF* qt_core_c_QMarginsF_operator_sub_assign_subtrahend(QMarginsF* this_ptr, double subtrahend) {
  return &this_ptr->operator-=(subtrahend);
}

double qt_core_c_QMarginsF_right(const QMarginsF* this_ptr) {
  return this_ptr->right();
}

void qt_core_c_QMarginsF_setBottom(QMarginsF* this_ptr, double bottom) {
  this_ptr->setBottom(bottom);
}

void qt_core_c_QMarginsF_setLeft(QMarginsF* this_ptr, double left) {
  this_ptr->setLeft(left);
}

void qt_core_c_QMarginsF_setRight(QMarginsF* this_ptr, double right) {
  this_ptr->setRight(right);
}

void qt_core_c_QMarginsF_setTop(QMarginsF* this_ptr, double top) {
  this_ptr->setTop(top);
}

void qt_core_c_QMarginsF_toMargins_to_output(const QMarginsF* this_ptr, QMargins* output) {
  new(output) QMargins(this_ptr->toMargins());
}

double qt_core_c_QMarginsF_top(const QMarginsF* this_ptr) {
  return this_ptr->top();
}

