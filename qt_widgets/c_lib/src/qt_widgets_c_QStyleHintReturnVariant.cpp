#include "qt_widgets_c_QStyleHintReturnVariant.h"

QStyleHintReturnVariant* qt_widgets_c_QStyleHintReturnVariant_G_static_cast_QStyleHintReturnVariant_ptr(QStyleHintReturn* ptr) {
  return static_cast<QStyleHintReturnVariant*>(ptr);
}

QStyleHintReturn* qt_widgets_c_QStyleHintReturnVariant_G_static_cast_QStyleHintReturn_ptr(QStyleHintReturnVariant* ptr) {
  return static_cast<QStyleHintReturn*>(ptr);
}

void qt_widgets_c_QStyleHintReturnVariant_delete(QStyleHintReturnVariant* this_ptr) {
  delete this_ptr;
}

QStyleHintReturnVariant* qt_widgets_c_QStyleHintReturnVariant_new() {
  return new QStyleHintReturnVariant();
}

void qt_widgets_c_QStyleHintReturnVariant_set_variant(QStyleHintReturnVariant* this_ptr, const QVariant* value) {
  this_ptr->variant = *value;
}

const QVariant* qt_widgets_c_QStyleHintReturnVariant_variant(const QStyleHintReturnVariant* this_ptr) {
  return &this_ptr->variant;
}

QVariant* qt_widgets_c_QStyleHintReturnVariant_variant_mut(QStyleHintReturnVariant* this_ptr) {
  return &this_ptr->variant;
}

