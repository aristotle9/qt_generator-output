#include "qt_gui_c_QAccessibleTextCursorEvent.h"

QAccessibleTextCursorEvent* qt_gui_c_QAccessibleTextCursorEvent_G_dynamic_cast_QAccessibleTextCursorEvent_ptr(QAccessibleEvent* ptr) {
  return dynamic_cast<QAccessibleTextCursorEvent*>(ptr);
}

QAccessibleEvent* qt_gui_c_QAccessibleTextCursorEvent_G_static_cast_QAccessibleEvent_ptr(QAccessibleTextCursorEvent* ptr) {
  return static_cast<QAccessibleEvent*>(ptr);
}

QAccessibleTextCursorEvent* qt_gui_c_QAccessibleTextCursorEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(QAccessibleEvent* ptr) {
  return static_cast<QAccessibleTextCursorEvent*>(ptr);
}

int qt_gui_c_QAccessibleTextCursorEvent_cursorPosition(const QAccessibleTextCursorEvent* this_ptr) {
  return this_ptr->cursorPosition();
}

void qt_gui_c_QAccessibleTextCursorEvent_delete(QAccessibleTextCursorEvent* this_ptr) {
  delete this_ptr;
}

QAccessibleTextCursorEvent* qt_gui_c_QAccessibleTextCursorEvent_new_iface_cursorPos(QAccessibleInterface* iface, int cursorPos) {
  return new QAccessibleTextCursorEvent(iface, cursorPos);
}

QAccessibleTextCursorEvent* qt_gui_c_QAccessibleTextCursorEvent_new_obj_cursorPos(QObject* obj, int cursorPos) {
  return new QAccessibleTextCursorEvent(obj, cursorPos);
}

void qt_gui_c_QAccessibleTextCursorEvent_setCursorPosition(QAccessibleTextCursorEvent* this_ptr, int position) {
  this_ptr->setCursorPosition(position);
}

