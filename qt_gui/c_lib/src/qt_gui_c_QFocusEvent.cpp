#include "qt_gui_c_QFocusEvent.h"

QEvent* qt_gui_c_QFocusEvent_G_static_cast_QEvent_ptr(QFocusEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QFocusEvent* qt_gui_c_QFocusEvent_G_static_cast_QFocusEvent_ptr(QEvent* ptr) {
  return static_cast<QFocusEvent*>(ptr);
}

void qt_gui_c_QFocusEvent_delete(QFocusEvent* this_ptr) {
  delete this_ptr;
}

bool qt_gui_c_QFocusEvent_gotFocus(const QFocusEvent* this_ptr) {
  return this_ptr->gotFocus();
}

bool qt_gui_c_QFocusEvent_lostFocus(const QFocusEvent* this_ptr) {
  return this_ptr->lostFocus();
}

QFocusEvent* qt_gui_c_QFocusEvent_new_type(QEvent::Type type) {
  return new QFocusEvent(type);
}

QFocusEvent* qt_gui_c_QFocusEvent_new_type_reason(QEvent::Type type, const Qt::FocusReason* reason) {
  return new QFocusEvent(type, *reason);
}

