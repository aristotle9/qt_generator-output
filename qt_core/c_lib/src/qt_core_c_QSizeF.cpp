#include "qt_core_c_QSizeF.h"

void qt_core_c_QSizeF_boundedTo_to_output(const QSizeF* this_ptr, const QSizeF* arg1, QSizeF* output) {
  new(output) QSizeF(this_ptr->boundedTo(*arg1));
}

void qt_core_c_QSizeF_constructor_no_args(QSizeF* output) {
  new(output) QSizeF();
}

void qt_core_c_QSizeF_constructor_sz(const QSize* sz, QSizeF* output) {
  new(output) QSizeF(*sz);
}

void qt_core_c_QSizeF_constructor_w_h(double w, double h, QSizeF* output) {
  new(output) QSizeF(w, h);
}

void qt_core_c_QSizeF_destructor(QSizeF* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QSizeF_expandedTo_to_output(const QSizeF* this_ptr, const QSizeF* arg1, QSizeF* output) {
  new(output) QSizeF(this_ptr->expandedTo(*arg1));
}

double qt_core_c_QSizeF_height(const QSizeF* this_ptr) {
  return this_ptr->height();
}

bool qt_core_c_QSizeF_isEmpty(const QSizeF* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_core_c_QSizeF_isNull(const QSizeF* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QSizeF_isValid(const QSizeF* this_ptr) {
  return this_ptr->isValid();
}

QSizeF* qt_core_c_QSizeF_operator_add_assign(QSizeF* this_ptr, const QSizeF* arg1) {
  return &this_ptr->operator+=(*arg1);
}

QSizeF* qt_core_c_QSizeF_operator_div_assign(QSizeF* this_ptr, double c) {
  return &this_ptr->operator/=(c);
}

QSizeF* qt_core_c_QSizeF_operator_mul_assign(QSizeF* this_ptr, double c) {
  return &this_ptr->operator*=(c);
}

QSizeF* qt_core_c_QSizeF_operator_sub_assign(QSizeF* this_ptr, const QSizeF* arg1) {
  return &this_ptr->operator-=(*arg1);
}

double* qt_core_c_QSizeF_rheight(QSizeF* this_ptr) {
  return &this_ptr->rheight();
}

double* qt_core_c_QSizeF_rwidth(QSizeF* this_ptr) {
  return &this_ptr->rwidth();
}

void qt_core_c_QSizeF_scale_s_mode(QSizeF* this_ptr, const QSizeF* s, const Qt::AspectRatioMode* mode) {
  this_ptr->scale(*s, *mode);
}

void qt_core_c_QSizeF_scale_w_h_mode(QSizeF* this_ptr, double w, double h, const Qt::AspectRatioMode* mode) {
  this_ptr->scale(w, h, *mode);
}

void qt_core_c_QSizeF_scaled_to_output_s_mode(const QSizeF* this_ptr, const QSizeF* s, const Qt::AspectRatioMode* mode, QSizeF* output) {
  new(output) QSizeF(this_ptr->scaled(*s, *mode));
}

void qt_core_c_QSizeF_scaled_to_output_w_h_mode(const QSizeF* this_ptr, double w, double h, const Qt::AspectRatioMode* mode, QSizeF* output) {
  new(output) QSizeF(this_ptr->scaled(w, h, *mode));
}

void qt_core_c_QSizeF_setHeight(QSizeF* this_ptr, double h) {
  this_ptr->setHeight(h);
}

void qt_core_c_QSizeF_setWidth(QSizeF* this_ptr, double w) {
  this_ptr->setWidth(w);
}

void qt_core_c_QSizeF_toSize_to_output(const QSizeF* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->toSize());
}

void qt_core_c_QSizeF_transpose(QSizeF* this_ptr) {
  this_ptr->transpose();
}

void qt_core_c_QSizeF_transposed_to_output(const QSizeF* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->transposed());
}

double qt_core_c_QSizeF_width(const QSizeF* this_ptr) {
  return this_ptr->width();
}

