#include "qt_widgets_c_QSizePolicy.h"

void qt_widgets_c_QSizePolicy_G_operator_shl_to_output(const QDebug* dbg, const QSizePolicy* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*dbg, *arg2));
}

unsigned int qt_widgets_c_QSizePolicy_G_qHash_key(const QSizePolicy* key) {
  return qHash(*key);
}

unsigned int qt_widgets_c_QSizePolicy_G_qHash_key_seed(const QSizePolicy* key, unsigned int seed) {
  return qHash(*key, seed);
}

void qt_widgets_c_QSizePolicy_constructor_horizontal_vertical(QSizePolicy::Policy horizontal, QSizePolicy::Policy vertical, QSizePolicy* output) {
  new(output) QSizePolicy(horizontal, vertical);
}

void qt_widgets_c_QSizePolicy_constructor_horizontal_vertical_type(QSizePolicy::Policy horizontal, QSizePolicy::Policy vertical, QSizePolicy::ControlType type, QSizePolicy* output) {
  new(output) QSizePolicy(horizontal, vertical, type);
}

void qt_widgets_c_QSizePolicy_constructor_no_args(QSizePolicy* output) {
  new(output) QSizePolicy();
}

QSizePolicy::ControlType qt_widgets_c_QSizePolicy_controlType(const QSizePolicy* this_ptr) {
  return this_ptr->controlType();
}

void qt_widgets_c_QSizePolicy_convert_to_QVariant_to_output(const QSizePolicy* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

void qt_widgets_c_QSizePolicy_destructor(QSizePolicy* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QSizePolicy_hasHeightForWidth(const QSizePolicy* this_ptr) {
  return this_ptr->hasHeightForWidth();
}

bool qt_widgets_c_QSizePolicy_hasWidthForHeight(const QSizePolicy* this_ptr) {
  return this_ptr->hasWidthForHeight();
}

QSizePolicy::Policy qt_widgets_c_QSizePolicy_horizontalPolicy(const QSizePolicy* this_ptr) {
  return this_ptr->horizontalPolicy();
}

int qt_widgets_c_QSizePolicy_horizontalStretch(const QSizePolicy* this_ptr) {
  return this_ptr->horizontalStretch();
}

bool qt_widgets_c_QSizePolicy_operator_eq(const QSizePolicy* this_ptr, const QSizePolicy* s) {
  return this_ptr->operator==(*s);
}

bool qt_widgets_c_QSizePolicy_operator_neq(const QSizePolicy* this_ptr, const QSizePolicy* s) {
  return this_ptr->operator!=(*s);
}

bool qt_widgets_c_QSizePolicy_retainSizeWhenHidden(const QSizePolicy* this_ptr) {
  return this_ptr->retainSizeWhenHidden();
}

void qt_widgets_c_QSizePolicy_setControlType(QSizePolicy* this_ptr, QSizePolicy::ControlType type) {
  this_ptr->setControlType(type);
}

void qt_widgets_c_QSizePolicy_setHeightForWidth(QSizePolicy* this_ptr, bool b) {
  this_ptr->setHeightForWidth(b);
}

void qt_widgets_c_QSizePolicy_setHorizontalPolicy(QSizePolicy* this_ptr, QSizePolicy::Policy d) {
  this_ptr->setHorizontalPolicy(d);
}

void qt_widgets_c_QSizePolicy_setHorizontalStretch(QSizePolicy* this_ptr, int stretchFactor) {
  this_ptr->setHorizontalStretch(stretchFactor);
}

void qt_widgets_c_QSizePolicy_setRetainSizeWhenHidden(QSizePolicy* this_ptr, bool retainSize) {
  this_ptr->setRetainSizeWhenHidden(retainSize);
}

void qt_widgets_c_QSizePolicy_setVerticalPolicy(QSizePolicy* this_ptr, QSizePolicy::Policy d) {
  this_ptr->setVerticalPolicy(d);
}

void qt_widgets_c_QSizePolicy_setVerticalStretch(QSizePolicy* this_ptr, int stretchFactor) {
  this_ptr->setVerticalStretch(stretchFactor);
}

void qt_widgets_c_QSizePolicy_setWidthForHeight(QSizePolicy* this_ptr, bool b) {
  this_ptr->setWidthForHeight(b);
}

void qt_widgets_c_QSizePolicy_transpose(QSizePolicy* this_ptr) {
  this_ptr->transpose();
}

void qt_widgets_c_QSizePolicy_transposed_to_output(const QSizePolicy* this_ptr, QSizePolicy* output) {
  new(output) QSizePolicy(this_ptr->transposed());
}

QSizePolicy::Policy qt_widgets_c_QSizePolicy_verticalPolicy(const QSizePolicy* this_ptr) {
  return this_ptr->verticalPolicy();
}

int qt_widgets_c_QSizePolicy_verticalStretch(const QSizePolicy* this_ptr) {
  return this_ptr->verticalStretch();
}

