#include "qt_gui_c_QCloseEvent.h"

QCloseEvent* qt_gui_c_QCloseEvent_G_static_cast_QCloseEvent_ptr(QEvent* ptr) {
  return static_cast<QCloseEvent*>(ptr);
}

QEvent* qt_gui_c_QCloseEvent_G_static_cast_QEvent_ptr(QCloseEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

void qt_gui_c_QCloseEvent_delete(QCloseEvent* this_ptr) {
  delete this_ptr;
}

QCloseEvent* qt_gui_c_QCloseEvent_new() {
  return new QCloseEvent();
}

