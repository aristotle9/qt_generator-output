#include "qt_widgets_c_QStyleHintReturnMask.h"

QStyleHintReturnMask* qt_widgets_c_QStyleHintReturnMask_G_static_cast_QStyleHintReturnMask_ptr(QStyleHintReturn* ptr) {
  return static_cast<QStyleHintReturnMask*>(ptr);
}

QStyleHintReturn* qt_widgets_c_QStyleHintReturnMask_G_static_cast_QStyleHintReturn_ptr(QStyleHintReturnMask* ptr) {
  return static_cast<QStyleHintReturn*>(ptr);
}

void qt_widgets_c_QStyleHintReturnMask_delete(QStyleHintReturnMask* this_ptr) {
  delete this_ptr;
}

QStyleHintReturnMask* qt_widgets_c_QStyleHintReturnMask_new() {
  return new QStyleHintReturnMask();
}

const QRegion* qt_widgets_c_QStyleHintReturnMask_region(const QStyleHintReturnMask* this_ptr) {
  return &this_ptr->region;
}

QRegion* qt_widgets_c_QStyleHintReturnMask_region_mut(QStyleHintReturnMask* this_ptr) {
  return &this_ptr->region;
}

void qt_widgets_c_QStyleHintReturnMask_set_region(QStyleHintReturnMask* this_ptr, const QRegion* value) {
  this_ptr->region = *value;
}

