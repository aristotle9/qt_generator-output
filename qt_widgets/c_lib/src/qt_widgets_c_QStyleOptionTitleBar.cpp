#include "qt_widgets_c_QStyleOptionTitleBar.h"

QStyleOptionComplex* qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOptionComplex_ptr(QStyleOptionTitleBar* ptr) {
  return static_cast<QStyleOptionComplex*>(ptr);
}

QStyleOptionTitleBar* qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOptionTitleBar_ptr_QStyleOption(QStyleOption* ptr) {
  return static_cast<QStyleOptionTitleBar*>(ptr);
}

QStyleOptionTitleBar* qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOptionTitleBar_ptr_QStyleOptionComplex(QStyleOptionComplex* ptr) {
  return static_cast<QStyleOptionTitleBar*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOption_ptr(QStyleOptionTitleBar* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

void qt_widgets_c_QStyleOptionTitleBar_constructor_no_args(QStyleOptionTitleBar* output) {
  new(output) QStyleOptionTitleBar();
}

void qt_widgets_c_QStyleOptionTitleBar_constructor_other(const QStyleOptionTitleBar* other, QStyleOptionTitleBar* output) {
  new(output) QStyleOptionTitleBar(*other);
}

void qt_widgets_c_QStyleOptionTitleBar_destructor(QStyleOptionTitleBar* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

const QIcon* qt_widgets_c_QStyleOptionTitleBar_icon(const QStyleOptionTitleBar* this_ptr) {
  return &this_ptr->icon;
}

QIcon* qt_widgets_c_QStyleOptionTitleBar_icon_mut(QStyleOptionTitleBar* this_ptr) {
  return &this_ptr->icon;
}

void qt_widgets_c_QStyleOptionTitleBar_set_icon(QStyleOptionTitleBar* this_ptr, const QIcon* value) {
  this_ptr->icon = *value;
}

void qt_widgets_c_QStyleOptionTitleBar_set_text(QStyleOptionTitleBar* this_ptr, const QString* value) {
  this_ptr->text = *value;
}

void qt_widgets_c_QStyleOptionTitleBar_set_titleBarState(QStyleOptionTitleBar* this_ptr, int value) {
  this_ptr->titleBarState = value;
}

const QString* qt_widgets_c_QStyleOptionTitleBar_text(const QStyleOptionTitleBar* this_ptr) {
  return &this_ptr->text;
}

QString* qt_widgets_c_QStyleOptionTitleBar_text_mut(QStyleOptionTitleBar* this_ptr) {
  return &this_ptr->text;
}

int qt_widgets_c_QStyleOptionTitleBar_titleBarState(const QStyleOptionTitleBar* this_ptr) {
  return this_ptr->titleBarState;
}

