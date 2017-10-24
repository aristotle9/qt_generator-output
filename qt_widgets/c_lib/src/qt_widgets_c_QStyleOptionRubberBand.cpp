#include "qt_widgets_c_QStyleOptionRubberBand.h"

QStyleOptionRubberBand* qt_widgets_c_QStyleOptionRubberBand_G_static_cast_QStyleOptionRubberBand_ptr(QStyleOption* ptr) {
  return static_cast<QStyleOptionRubberBand*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionRubberBand_G_static_cast_QStyleOption_ptr(QStyleOptionRubberBand* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

void qt_widgets_c_QStyleOptionRubberBand_delete(QStyleOptionRubberBand* this_ptr) {
  delete this_ptr;
}

QStyleOptionRubberBand* qt_widgets_c_QStyleOptionRubberBand_new_no_args() {
  return new QStyleOptionRubberBand();
}

QStyleOptionRubberBand* qt_widgets_c_QStyleOptionRubberBand_new_other(const QStyleOptionRubberBand* other) {
  return new QStyleOptionRubberBand(*other);
}

bool qt_widgets_c_QStyleOptionRubberBand_opaque(const QStyleOptionRubberBand* this_ptr) {
  return this_ptr->opaque;
}

void qt_widgets_c_QStyleOptionRubberBand_set_opaque(QStyleOptionRubberBand* this_ptr, bool value) {
  this_ptr->opaque = value;
}

void qt_widgets_c_QStyleOptionRubberBand_set_shape(QStyleOptionRubberBand* this_ptr, const QRubberBand::Shape* value) {
  this_ptr->shape = *value;
}

const QRubberBand::Shape* qt_widgets_c_QStyleOptionRubberBand_shape(const QStyleOptionRubberBand* this_ptr) {
  return &this_ptr->shape;
}

QRubberBand::Shape* qt_widgets_c_QStyleOptionRubberBand_shape_mut(QStyleOptionRubberBand* this_ptr) {
  return &this_ptr->shape;
}

