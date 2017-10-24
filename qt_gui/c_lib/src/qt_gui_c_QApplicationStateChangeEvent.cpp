#include "qt_gui_c_QApplicationStateChangeEvent.h"

QApplicationStateChangeEvent* qt_gui_c_QApplicationStateChangeEvent_G_static_cast_QApplicationStateChangeEvent_ptr(QEvent* ptr) {
  return static_cast<QApplicationStateChangeEvent*>(ptr);
}

QEvent* qt_gui_c_QApplicationStateChangeEvent_G_static_cast_QEvent_ptr(QApplicationStateChangeEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

void qt_gui_c_QApplicationStateChangeEvent_delete(QApplicationStateChangeEvent* this_ptr) {
  delete this_ptr;
}

QApplicationStateChangeEvent* qt_gui_c_QApplicationStateChangeEvent_new(const Qt::ApplicationState* state) {
  return new QApplicationStateChangeEvent(*state);
}

