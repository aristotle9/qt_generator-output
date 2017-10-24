#include "qt_core_c_QSize.h"

QDataStream* qt_core_c_QSize_G_operator_shl_QDataStream_QSize(QDataStream* arg1, const QSize* arg2) {
  return &operator<<(*arg1, *arg2);
}

QDataStream* qt_core_c_QSize_G_operator_shl_QDataStream_QSizeF(QDataStream* arg1, const QSizeF* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_core_c_QSize_G_operator_shl_to_output_QDebug_QSize(const QDebug* arg1, const QSize* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

void qt_core_c_QSize_G_operator_shl_to_output_QDebug_QSizeF(const QDebug* arg1, const QSizeF* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_core_c_QSize_G_operator_shr_QDataStream_QSize(QDataStream* arg1, QSize* arg2) {
  return &operator>>(*arg1, *arg2);
}

QDataStream* qt_core_c_QSize_G_operator_shr_QDataStream_QSizeF(QDataStream* arg1, QSizeF* arg2) {
  return &operator>>(*arg1, *arg2);
}

void qt_core_c_QSize_boundedTo_to_output(const QSize* this_ptr, const QSize* arg1, QSize* output) {
  new(output) QSize(this_ptr->boundedTo(*arg1));
}

void qt_core_c_QSize_constructor_no_args(QSize* output) {
  new(output) QSize();
}

void qt_core_c_QSize_constructor_w_h(int w, int h, QSize* output) {
  new(output) QSize(w, h);
}

void qt_core_c_QSize_destructor(QSize* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QSize_expandedTo_to_output(const QSize* this_ptr, const QSize* arg1, QSize* output) {
  new(output) QSize(this_ptr->expandedTo(*arg1));
}

int qt_core_c_QSize_height(const QSize* this_ptr) {
  return this_ptr->height();
}

bool qt_core_c_QSize_isEmpty(const QSize* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_core_c_QSize_isNull(const QSize* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QSize_isValid(const QSize* this_ptr) {
  return this_ptr->isValid();
}

QSize* qt_core_c_QSize_operator_add_assign(QSize* this_ptr, const QSize* arg1) {
  return &this_ptr->operator+=(*arg1);
}

QSize* qt_core_c_QSize_operator_div_assign(QSize* this_ptr, double c) {
  return &this_ptr->operator/=(c);
}

QSize* qt_core_c_QSize_operator_mul_assign(QSize* this_ptr, double c) {
  return &this_ptr->operator*=(c);
}

QSize* qt_core_c_QSize_operator_sub_assign(QSize* this_ptr, const QSize* arg1) {
  return &this_ptr->operator-=(*arg1);
}

int* qt_core_c_QSize_rheight(QSize* this_ptr) {
  return &this_ptr->rheight();
}

int* qt_core_c_QSize_rwidth(QSize* this_ptr) {
  return &this_ptr->rwidth();
}

void qt_core_c_QSize_scale_s_mode(QSize* this_ptr, const QSize* s, const Qt::AspectRatioMode* mode) {
  this_ptr->scale(*s, *mode);
}

void qt_core_c_QSize_scale_w_h_mode(QSize* this_ptr, int w, int h, const Qt::AspectRatioMode* mode) {
  this_ptr->scale(w, h, *mode);
}

void qt_core_c_QSize_scaled_to_output_s_mode(const QSize* this_ptr, const QSize* s, const Qt::AspectRatioMode* mode, QSize* output) {
  new(output) QSize(this_ptr->scaled(*s, *mode));
}

void qt_core_c_QSize_scaled_to_output_w_h_mode(const QSize* this_ptr, int w, int h, const Qt::AspectRatioMode* mode, QSize* output) {
  new(output) QSize(this_ptr->scaled(w, h, *mode));
}

void qt_core_c_QSize_setHeight(QSize* this_ptr, int h) {
  this_ptr->setHeight(h);
}

void qt_core_c_QSize_setWidth(QSize* this_ptr, int w) {
  this_ptr->setWidth(w);
}

void qt_core_c_QSize_transpose(QSize* this_ptr) {
  this_ptr->transpose();
}

void qt_core_c_QSize_transposed_to_output(const QSize* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->transposed());
}

int qt_core_c_QSize_width(const QSize* this_ptr) {
  return this_ptr->width();
}

