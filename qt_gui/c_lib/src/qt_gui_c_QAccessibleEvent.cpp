#include "qt_gui_c_QAccessibleEvent.h"

QAccessibleInterface* qt_gui_c_QAccessibleEvent_accessibleInterface(const QAccessibleEvent* this_ptr) {
  return this_ptr->accessibleInterface();
}

int qt_gui_c_QAccessibleEvent_child(const QAccessibleEvent* this_ptr) {
  return this_ptr->child();
}

void qt_gui_c_QAccessibleEvent_delete(QAccessibleEvent* this_ptr) {
  delete this_ptr;
}

QAccessibleEvent* qt_gui_c_QAccessibleEvent_new_iface_typ(QAccessibleInterface* iface, const QAccessible::Event* typ) {
  return new QAccessibleEvent(iface, *typ);
}

QAccessibleEvent* qt_gui_c_QAccessibleEvent_new_obj_typ(QObject* obj, const QAccessible::Event* typ) {
  return new QAccessibleEvent(obj, *typ);
}

QObject* qt_gui_c_QAccessibleEvent_object(const QAccessibleEvent* this_ptr) {
  return this_ptr->object();
}

void qt_gui_c_QAccessibleEvent_setChild(QAccessibleEvent* this_ptr, int chld) {
  this_ptr->setChild(chld);
}

