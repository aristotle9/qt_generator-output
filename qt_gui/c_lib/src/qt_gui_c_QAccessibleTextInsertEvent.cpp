#include "qt_gui_c_QAccessibleTextInsertEvent.h"

QAccessibleTextInsertEvent* qt_gui_c_QAccessibleTextInsertEvent_G_dynamic_cast_QAccessibleTextInsertEvent_ptr_QAccessibleEvent(QAccessibleEvent* ptr) {
  return dynamic_cast<QAccessibleTextInsertEvent*>(ptr);
}

QAccessibleTextInsertEvent* qt_gui_c_QAccessibleTextInsertEvent_G_dynamic_cast_QAccessibleTextInsertEvent_ptr_QAccessibleTextCursorEvent(QAccessibleTextCursorEvent* ptr) {
  return dynamic_cast<QAccessibleTextInsertEvent*>(ptr);
}

QAccessibleEvent* qt_gui_c_QAccessibleTextInsertEvent_G_static_cast_QAccessibleEvent_ptr(QAccessibleTextInsertEvent* ptr) {
  return static_cast<QAccessibleEvent*>(ptr);
}

QAccessibleTextCursorEvent* qt_gui_c_QAccessibleTextInsertEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(QAccessibleTextInsertEvent* ptr) {
  return static_cast<QAccessibleTextCursorEvent*>(ptr);
}

QAccessibleTextInsertEvent* qt_gui_c_QAccessibleTextInsertEvent_G_static_cast_QAccessibleTextInsertEvent_ptr_QAccessibleEvent(QAccessibleEvent* ptr) {
  return static_cast<QAccessibleTextInsertEvent*>(ptr);
}

QAccessibleTextInsertEvent* qt_gui_c_QAccessibleTextInsertEvent_G_static_cast_QAccessibleTextInsertEvent_ptr_QAccessibleTextCursorEvent(QAccessibleTextCursorEvent* ptr) {
  return static_cast<QAccessibleTextInsertEvent*>(ptr);
}

int qt_gui_c_QAccessibleTextInsertEvent_changePosition(const QAccessibleTextInsertEvent* this_ptr) {
  return this_ptr->changePosition();
}

void qt_gui_c_QAccessibleTextInsertEvent_delete(QAccessibleTextInsertEvent* this_ptr) {
  delete this_ptr;
}

QAccessibleTextInsertEvent* qt_gui_c_QAccessibleTextInsertEvent_new_iface_position_text(QAccessibleInterface* iface, int position, const QString* text) {
  return new QAccessibleTextInsertEvent(iface, position, *text);
}

QAccessibleTextInsertEvent* qt_gui_c_QAccessibleTextInsertEvent_new_obj_position_text(QObject* obj, int position, const QString* text) {
  return new QAccessibleTextInsertEvent(obj, position, *text);
}

void qt_gui_c_QAccessibleTextInsertEvent_textInserted_to_output(const QAccessibleTextInsertEvent* this_ptr, QString* output) {
  new(output) QString(this_ptr->textInserted());
}

