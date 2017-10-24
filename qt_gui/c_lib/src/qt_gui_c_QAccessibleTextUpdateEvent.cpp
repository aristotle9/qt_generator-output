#include "qt_gui_c_QAccessibleTextUpdateEvent.h"

QAccessibleTextUpdateEvent* qt_gui_c_QAccessibleTextUpdateEvent_G_dynamic_cast_QAccessibleTextUpdateEvent_ptr_QAccessibleEvent(QAccessibleEvent* ptr) {
  return dynamic_cast<QAccessibleTextUpdateEvent*>(ptr);
}

QAccessibleTextUpdateEvent* qt_gui_c_QAccessibleTextUpdateEvent_G_dynamic_cast_QAccessibleTextUpdateEvent_ptr_QAccessibleTextCursorEvent(QAccessibleTextCursorEvent* ptr) {
  return dynamic_cast<QAccessibleTextUpdateEvent*>(ptr);
}

QAccessibleEvent* qt_gui_c_QAccessibleTextUpdateEvent_G_static_cast_QAccessibleEvent_ptr(QAccessibleTextUpdateEvent* ptr) {
  return static_cast<QAccessibleEvent*>(ptr);
}

QAccessibleTextCursorEvent* qt_gui_c_QAccessibleTextUpdateEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(QAccessibleTextUpdateEvent* ptr) {
  return static_cast<QAccessibleTextCursorEvent*>(ptr);
}

QAccessibleTextUpdateEvent* qt_gui_c_QAccessibleTextUpdateEvent_G_static_cast_QAccessibleTextUpdateEvent_ptr_QAccessibleEvent(QAccessibleEvent* ptr) {
  return static_cast<QAccessibleTextUpdateEvent*>(ptr);
}

QAccessibleTextUpdateEvent* qt_gui_c_QAccessibleTextUpdateEvent_G_static_cast_QAccessibleTextUpdateEvent_ptr_QAccessibleTextCursorEvent(QAccessibleTextCursorEvent* ptr) {
  return static_cast<QAccessibleTextUpdateEvent*>(ptr);
}

int qt_gui_c_QAccessibleTextUpdateEvent_changePosition(const QAccessibleTextUpdateEvent* this_ptr) {
  return this_ptr->changePosition();
}

void qt_gui_c_QAccessibleTextUpdateEvent_delete(QAccessibleTextUpdateEvent* this_ptr) {
  delete this_ptr;
}

QAccessibleTextUpdateEvent* qt_gui_c_QAccessibleTextUpdateEvent_new_iface_position_oldText_text(QAccessibleInterface* iface, int position, const QString* oldText, const QString* text) {
  return new QAccessibleTextUpdateEvent(iface, position, *oldText, *text);
}

QAccessibleTextUpdateEvent* qt_gui_c_QAccessibleTextUpdateEvent_new_obj_position_oldText_text(QObject* obj, int position, const QString* oldText, const QString* text) {
  return new QAccessibleTextUpdateEvent(obj, position, *oldText, *text);
}

void qt_gui_c_QAccessibleTextUpdateEvent_textInserted_to_output(const QAccessibleTextUpdateEvent* this_ptr, QString* output) {
  new(output) QString(this_ptr->textInserted());
}

void qt_gui_c_QAccessibleTextUpdateEvent_textRemoved_to_output(const QAccessibleTextUpdateEvent* this_ptr, QString* output) {
  new(output) QString(this_ptr->textRemoved());
}

