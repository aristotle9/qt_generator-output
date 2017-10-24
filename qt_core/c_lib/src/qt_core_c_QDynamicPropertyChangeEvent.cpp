#include "qt_core_c_QDynamicPropertyChangeEvent.h"

QDynamicPropertyChangeEvent* qt_core_c_QDynamicPropertyChangeEvent_G_dynamic_cast_QDynamicPropertyChangeEvent_ptr(QEvent* ptr) {
  return dynamic_cast<QDynamicPropertyChangeEvent*>(ptr);
}

QDynamicPropertyChangeEvent* qt_core_c_QDynamicPropertyChangeEvent_G_static_cast_QDynamicPropertyChangeEvent_ptr(QEvent* ptr) {
  return static_cast<QDynamicPropertyChangeEvent*>(ptr);
}

QEvent* qt_core_c_QDynamicPropertyChangeEvent_G_static_cast_QEvent_ptr(QDynamicPropertyChangeEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

void qt_core_c_QDynamicPropertyChangeEvent_delete(QDynamicPropertyChangeEvent* this_ptr) {
  delete this_ptr;
}

QDynamicPropertyChangeEvent* qt_core_c_QDynamicPropertyChangeEvent_new(const QByteArray* name) {
  return new QDynamicPropertyChangeEvent(*name);
}

void qt_core_c_QDynamicPropertyChangeEvent_propertyName_to_output(const QDynamicPropertyChangeEvent* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->propertyName());
}

