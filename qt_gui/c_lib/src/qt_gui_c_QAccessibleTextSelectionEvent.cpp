#include "qt_gui_c_QAccessibleTextSelectionEvent.h"

QAccessibleTextSelectionEvent* qt_gui_c_QAccessibleTextSelectionEvent_G_dynamic_cast_QAccessibleTextSelectionEvent_ptr_QAccessibleEvent(QAccessibleEvent* ptr) {
  return dynamic_cast<QAccessibleTextSelectionEvent*>(ptr);
}

QAccessibleTextSelectionEvent* qt_gui_c_QAccessibleTextSelectionEvent_G_dynamic_cast_QAccessibleTextSelectionEvent_ptr_QAccessibleTextCursorEvent(QAccessibleTextCursorEvent* ptr) {
  return dynamic_cast<QAccessibleTextSelectionEvent*>(ptr);
}

QAccessibleEvent* qt_gui_c_QAccessibleTextSelectionEvent_G_static_cast_QAccessibleEvent_ptr(QAccessibleTextSelectionEvent* ptr) {
  return static_cast<QAccessibleEvent*>(ptr);
}

QAccessibleTextCursorEvent* qt_gui_c_QAccessibleTextSelectionEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(QAccessibleTextSelectionEvent* ptr) {
  return static_cast<QAccessibleTextCursorEvent*>(ptr);
}

QAccessibleTextSelectionEvent* qt_gui_c_QAccessibleTextSelectionEvent_G_static_cast_QAccessibleTextSelectionEvent_ptr_QAccessibleEvent(QAccessibleEvent* ptr) {
  return static_cast<QAccessibleTextSelectionEvent*>(ptr);
}

QAccessibleTextSelectionEvent* qt_gui_c_QAccessibleTextSelectionEvent_G_static_cast_QAccessibleTextSelectionEvent_ptr_QAccessibleTextCursorEvent(QAccessibleTextCursorEvent* ptr) {
  return static_cast<QAccessibleTextSelectionEvent*>(ptr);
}

void qt_gui_c_QAccessibleTextSelectionEvent_delete(QAccessibleTextSelectionEvent* this_ptr) {
  delete this_ptr;
}

QAccessibleTextSelectionEvent* qt_gui_c_QAccessibleTextSelectionEvent_new_iface_start_end(QAccessibleInterface* iface, int start, int end) {
  return new QAccessibleTextSelectionEvent(iface, start, end);
}

QAccessibleTextSelectionEvent* qt_gui_c_QAccessibleTextSelectionEvent_new_obj_start_end(QObject* obj, int start, int end) {
  return new QAccessibleTextSelectionEvent(obj, start, end);
}

int qt_gui_c_QAccessibleTextSelectionEvent_selectionEnd(const QAccessibleTextSelectionEvent* this_ptr) {
  return this_ptr->selectionEnd();
}

int qt_gui_c_QAccessibleTextSelectionEvent_selectionStart(const QAccessibleTextSelectionEvent* this_ptr) {
  return this_ptr->selectionStart();
}

void qt_gui_c_QAccessibleTextSelectionEvent_setSelection(QAccessibleTextSelectionEvent* this_ptr, int start, int end) {
  this_ptr->setSelection(start, end);
}

