#include "qt_widgets_c_QStyleOptionSizeGrip.h"

QStyleOptionComplex* qt_widgets_c_QStyleOptionSizeGrip_G_static_cast_QStyleOptionComplex_ptr(QStyleOptionSizeGrip* ptr) {
  return static_cast<QStyleOptionComplex*>(ptr);
}

QStyleOptionSizeGrip* qt_widgets_c_QStyleOptionSizeGrip_G_static_cast_QStyleOptionSizeGrip_ptr_QStyleOption(QStyleOption* ptr) {
  return static_cast<QStyleOptionSizeGrip*>(ptr);
}

QStyleOptionSizeGrip* qt_widgets_c_QStyleOptionSizeGrip_G_static_cast_QStyleOptionSizeGrip_ptr_QStyleOptionComplex(QStyleOptionComplex* ptr) {
  return static_cast<QStyleOptionSizeGrip*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionSizeGrip_G_static_cast_QStyleOption_ptr(QStyleOptionSizeGrip* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

void qt_widgets_c_QStyleOptionSizeGrip_constructor_no_args(QStyleOptionSizeGrip* output) {
  new(output) QStyleOptionSizeGrip();
}

void qt_widgets_c_QStyleOptionSizeGrip_constructor_other(const QStyleOptionSizeGrip* other, QStyleOptionSizeGrip* output) {
  new(output) QStyleOptionSizeGrip(*other);
}

const Qt::Corner* qt_widgets_c_QStyleOptionSizeGrip_corner(const QStyleOptionSizeGrip* this_ptr) {
  return &this_ptr->corner;
}

Qt::Corner* qt_widgets_c_QStyleOptionSizeGrip_corner_mut(QStyleOptionSizeGrip* this_ptr) {
  return &this_ptr->corner;
}

void qt_widgets_c_QStyleOptionSizeGrip_destructor(QStyleOptionSizeGrip* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

void qt_widgets_c_QStyleOptionSizeGrip_set_corner(QStyleOptionSizeGrip* this_ptr, const Qt::Corner* value) {
  this_ptr->corner = *value;
}

