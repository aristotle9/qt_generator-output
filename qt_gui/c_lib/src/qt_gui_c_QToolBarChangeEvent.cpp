#include "qt_gui_c_QToolBarChangeEvent.h"

QEvent* qt_gui_c_QToolBarChangeEvent_G_static_cast_QEvent_ptr(QToolBarChangeEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QToolBarChangeEvent* qt_gui_c_QToolBarChangeEvent_G_static_cast_QToolBarChangeEvent_ptr(QEvent* ptr) {
  return static_cast<QToolBarChangeEvent*>(ptr);
}

void qt_gui_c_QToolBarChangeEvent_delete(QToolBarChangeEvent* this_ptr) {
  delete this_ptr;
}

QToolBarChangeEvent* qt_gui_c_QToolBarChangeEvent_new(bool t) {
  return new QToolBarChangeEvent(t);
}

bool qt_gui_c_QToolBarChangeEvent_toggle(const QToolBarChangeEvent* this_ptr) {
  return this_ptr->toggle();
}

