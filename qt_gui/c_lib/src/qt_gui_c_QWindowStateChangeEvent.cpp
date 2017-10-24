#include "qt_gui_c_QWindowStateChangeEvent.h"

QEvent* qt_gui_c_QWindowStateChangeEvent_G_static_cast_QEvent_ptr(QWindowStateChangeEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QWindowStateChangeEvent* qt_gui_c_QWindowStateChangeEvent_G_static_cast_QWindowStateChangeEvent_ptr(QEvent* ptr) {
  return static_cast<QWindowStateChangeEvent*>(ptr);
}

void qt_gui_c_QWindowStateChangeEvent_delete(QWindowStateChangeEvent* this_ptr) {
  delete this_ptr;
}

bool qt_gui_c_QWindowStateChangeEvent_isOverride(const QWindowStateChangeEvent* this_ptr) {
  return this_ptr->isOverride();
}

