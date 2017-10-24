#include "qt_widgets_c_QStyleOptionFocusRect.h"

QStyleOptionFocusRect* qt_widgets_c_QStyleOptionFocusRect_G_static_cast_QStyleOptionFocusRect_ptr(QStyleOption* ptr) {
  return static_cast<QStyleOptionFocusRect*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionFocusRect_G_static_cast_QStyleOption_ptr(QStyleOptionFocusRect* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

const QColor* qt_widgets_c_QStyleOptionFocusRect_backgroundColor(const QStyleOptionFocusRect* this_ptr) {
  return &this_ptr->backgroundColor;
}

QColor* qt_widgets_c_QStyleOptionFocusRect_backgroundColor_mut(QStyleOptionFocusRect* this_ptr) {
  return &this_ptr->backgroundColor;
}

void qt_widgets_c_QStyleOptionFocusRect_constructor_no_args(QStyleOptionFocusRect* output) {
  new(output) QStyleOptionFocusRect();
}

void qt_widgets_c_QStyleOptionFocusRect_constructor_other(const QStyleOptionFocusRect* other, QStyleOptionFocusRect* output) {
  new(output) QStyleOptionFocusRect(*other);
}

void qt_widgets_c_QStyleOptionFocusRect_destructor(QStyleOptionFocusRect* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

void qt_widgets_c_QStyleOptionFocusRect_set_backgroundColor(QStyleOptionFocusRect* this_ptr, const QColor* value) {
  this_ptr->backgroundColor = *value;
}

