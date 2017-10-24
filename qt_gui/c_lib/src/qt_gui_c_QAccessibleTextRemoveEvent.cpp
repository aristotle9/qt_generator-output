#include "qt_gui_c_QAccessibleTextRemoveEvent.h"

QAccessibleTextRemoveEvent* qt_gui_c_QAccessibleTextRemoveEvent_G_dynamic_cast_QAccessibleTextRemoveEvent_ptr_QAccessibleEvent(QAccessibleEvent* ptr) {
  return dynamic_cast<QAccessibleTextRemoveEvent*>(ptr);
}

QAccessibleTextRemoveEvent* qt_gui_c_QAccessibleTextRemoveEvent_G_dynamic_cast_QAccessibleTextRemoveEvent_ptr_QAccessibleTextCursorEvent(QAccessibleTextCursorEvent* ptr) {
  return dynamic_cast<QAccessibleTextRemoveEvent*>(ptr);
}

QAccessibleEvent* qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleEvent_ptr(QAccessibleTextRemoveEvent* ptr) {
  return static_cast<QAccessibleEvent*>(ptr);
}

QAccessibleTextCursorEvent* qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(QAccessibleTextRemoveEvent* ptr) {
  return static_cast<QAccessibleTextCursorEvent*>(ptr);
}

QAccessibleTextRemoveEvent* qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleTextRemoveEvent_ptr_QAccessibleEvent(QAccessibleEvent* ptr) {
  return static_cast<QAccessibleTextRemoveEvent*>(ptr);
}

QAccessibleTextRemoveEvent* qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleTextRemoveEvent_ptr_QAccessibleTextCursorEvent(QAccessibleTextCursorEvent* ptr) {
  return static_cast<QAccessibleTextRemoveEvent*>(ptr);
}

int qt_gui_c_QAccessibleTextRemoveEvent_changePosition(const QAccessibleTextRemoveEvent* this_ptr) {
  return this_ptr->changePosition();
}

void qt_gui_c_QAccessibleTextRemoveEvent_delete(QAccessibleTextRemoveEvent* this_ptr) {
  delete this_ptr;
}

QAccessibleTextRemoveEvent* qt_gui_c_QAccessibleTextRemoveEvent_new_iface_position_text(QAccessibleInterface* iface, int position, const QString* text) {
  return new QAccessibleTextRemoveEvent(iface, position, *text);
}

QAccessibleTextRemoveEvent* qt_gui_c_QAccessibleTextRemoveEvent_new_obj_position_text(QObject* obj, int position, const QString* text) {
  return new QAccessibleTextRemoveEvent(obj, position, *text);
}

void qt_gui_c_QAccessibleTextRemoveEvent_textRemoved_to_output(const QAccessibleTextRemoveEvent* this_ptr, QString* output) {
  new(output) QString(this_ptr->textRemoved());
}

