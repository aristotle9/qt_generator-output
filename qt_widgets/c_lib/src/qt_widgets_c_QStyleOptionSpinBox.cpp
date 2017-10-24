#include "qt_widgets_c_QStyleOptionSpinBox.h"

QStyleOptionComplex* qt_widgets_c_QStyleOptionSpinBox_G_static_cast_QStyleOptionComplex_ptr(QStyleOptionSpinBox* ptr) {
  return static_cast<QStyleOptionComplex*>(ptr);
}

QStyleOptionSpinBox* qt_widgets_c_QStyleOptionSpinBox_G_static_cast_QStyleOptionSpinBox_ptr_QStyleOption(QStyleOption* ptr) {
  return static_cast<QStyleOptionSpinBox*>(ptr);
}

QStyleOptionSpinBox* qt_widgets_c_QStyleOptionSpinBox_G_static_cast_QStyleOptionSpinBox_ptr_QStyleOptionComplex(QStyleOptionComplex* ptr) {
  return static_cast<QStyleOptionSpinBox*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionSpinBox_G_static_cast_QStyleOption_ptr(QStyleOptionSpinBox* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

const QAbstractSpinBox::ButtonSymbols* qt_widgets_c_QStyleOptionSpinBox_buttonSymbols(const QStyleOptionSpinBox* this_ptr) {
  return &this_ptr->buttonSymbols;
}

QAbstractSpinBox::ButtonSymbols* qt_widgets_c_QStyleOptionSpinBox_buttonSymbols_mut(QStyleOptionSpinBox* this_ptr) {
  return &this_ptr->buttonSymbols;
}

void qt_widgets_c_QStyleOptionSpinBox_delete(QStyleOptionSpinBox* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QStyleOptionSpinBox_frame(const QStyleOptionSpinBox* this_ptr) {
  return this_ptr->frame;
}

QStyleOptionSpinBox* qt_widgets_c_QStyleOptionSpinBox_new_no_args() {
  return new QStyleOptionSpinBox();
}

QStyleOptionSpinBox* qt_widgets_c_QStyleOptionSpinBox_new_other(const QStyleOptionSpinBox* other) {
  return new QStyleOptionSpinBox(*other);
}

void qt_widgets_c_QStyleOptionSpinBox_set_buttonSymbols(QStyleOptionSpinBox* this_ptr, const QAbstractSpinBox::ButtonSymbols* value) {
  this_ptr->buttonSymbols = *value;
}

void qt_widgets_c_QStyleOptionSpinBox_set_frame(QStyleOptionSpinBox* this_ptr, bool value) {
  this_ptr->frame = value;
}

