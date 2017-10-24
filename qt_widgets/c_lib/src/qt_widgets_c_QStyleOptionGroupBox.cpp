#include "qt_widgets_c_QStyleOptionGroupBox.h"

QStyleOptionComplex* qt_widgets_c_QStyleOptionGroupBox_G_static_cast_QStyleOptionComplex_ptr(QStyleOptionGroupBox* ptr) {
  return static_cast<QStyleOptionComplex*>(ptr);
}

QStyleOptionGroupBox* qt_widgets_c_QStyleOptionGroupBox_G_static_cast_QStyleOptionGroupBox_ptr_QStyleOption(QStyleOption* ptr) {
  return static_cast<QStyleOptionGroupBox*>(ptr);
}

QStyleOptionGroupBox* qt_widgets_c_QStyleOptionGroupBox_G_static_cast_QStyleOptionGroupBox_ptr_QStyleOptionComplex(QStyleOptionComplex* ptr) {
  return static_cast<QStyleOptionGroupBox*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionGroupBox_G_static_cast_QStyleOption_ptr(QStyleOptionGroupBox* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

void qt_widgets_c_QStyleOptionGroupBox_delete(QStyleOptionGroupBox* this_ptr) {
  delete this_ptr;
}

int qt_widgets_c_QStyleOptionGroupBox_lineWidth(const QStyleOptionGroupBox* this_ptr) {
  return this_ptr->lineWidth;
}

int qt_widgets_c_QStyleOptionGroupBox_midLineWidth(const QStyleOptionGroupBox* this_ptr) {
  return this_ptr->midLineWidth;
}

QStyleOptionGroupBox* qt_widgets_c_QStyleOptionGroupBox_new_no_args() {
  return new QStyleOptionGroupBox();
}

QStyleOptionGroupBox* qt_widgets_c_QStyleOptionGroupBox_new_other(const QStyleOptionGroupBox* other) {
  return new QStyleOptionGroupBox(*other);
}

void qt_widgets_c_QStyleOptionGroupBox_set_lineWidth(QStyleOptionGroupBox* this_ptr, int value) {
  this_ptr->lineWidth = value;
}

void qt_widgets_c_QStyleOptionGroupBox_set_midLineWidth(QStyleOptionGroupBox* this_ptr, int value) {
  this_ptr->midLineWidth = value;
}

void qt_widgets_c_QStyleOptionGroupBox_set_text(QStyleOptionGroupBox* this_ptr, const QString* value) {
  this_ptr->text = *value;
}

void qt_widgets_c_QStyleOptionGroupBox_set_textColor(QStyleOptionGroupBox* this_ptr, const QColor* value) {
  this_ptr->textColor = *value;
}

const QString* qt_widgets_c_QStyleOptionGroupBox_text(const QStyleOptionGroupBox* this_ptr) {
  return &this_ptr->text;
}

const QColor* qt_widgets_c_QStyleOptionGroupBox_textColor(const QStyleOptionGroupBox* this_ptr) {
  return &this_ptr->textColor;
}

QColor* qt_widgets_c_QStyleOptionGroupBox_textColor_mut(QStyleOptionGroupBox* this_ptr) {
  return &this_ptr->textColor;
}

QString* qt_widgets_c_QStyleOptionGroupBox_text_mut(QStyleOptionGroupBox* this_ptr) {
  return &this_ptr->text;
}

