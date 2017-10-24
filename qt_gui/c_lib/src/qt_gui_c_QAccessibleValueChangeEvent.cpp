#include "qt_gui_c_QAccessibleValueChangeEvent.h"

QAccessibleValueChangeEvent* qt_gui_c_QAccessibleValueChangeEvent_G_dynamic_cast_QAccessibleValueChangeEvent_ptr(QAccessibleEvent* ptr) {
  return dynamic_cast<QAccessibleValueChangeEvent*>(ptr);
}

QAccessibleEvent* qt_gui_c_QAccessibleValueChangeEvent_G_static_cast_QAccessibleEvent_ptr(QAccessibleValueChangeEvent* ptr) {
  return static_cast<QAccessibleEvent*>(ptr);
}

QAccessibleValueChangeEvent* qt_gui_c_QAccessibleValueChangeEvent_G_static_cast_QAccessibleValueChangeEvent_ptr(QAccessibleEvent* ptr) {
  return static_cast<QAccessibleValueChangeEvent*>(ptr);
}

void qt_gui_c_QAccessibleValueChangeEvent_delete(QAccessibleValueChangeEvent* this_ptr) {
  delete this_ptr;
}

QAccessibleValueChangeEvent* qt_gui_c_QAccessibleValueChangeEvent_new_iface_val(QAccessibleInterface* iface, const QVariant* val) {
  return new QAccessibleValueChangeEvent(iface, *val);
}

QAccessibleValueChangeEvent* qt_gui_c_QAccessibleValueChangeEvent_new_obj_val(QObject* obj, const QVariant* val) {
  return new QAccessibleValueChangeEvent(obj, *val);
}

void qt_gui_c_QAccessibleValueChangeEvent_setValue(QAccessibleValueChangeEvent* this_ptr, const QVariant* val) {
  this_ptr->setValue(*val);
}

void qt_gui_c_QAccessibleValueChangeEvent_value_to_output(const QAccessibleValueChangeEvent* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->value());
}

