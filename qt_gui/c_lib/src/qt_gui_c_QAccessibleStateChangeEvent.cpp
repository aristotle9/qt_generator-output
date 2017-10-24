#include "qt_gui_c_QAccessibleStateChangeEvent.h"

QAccessibleStateChangeEvent* qt_gui_c_QAccessibleStateChangeEvent_G_dynamic_cast_QAccessibleStateChangeEvent_ptr(QAccessibleEvent* ptr) {
  return dynamic_cast<QAccessibleStateChangeEvent*>(ptr);
}

QAccessibleEvent* qt_gui_c_QAccessibleStateChangeEvent_G_static_cast_QAccessibleEvent_ptr(QAccessibleStateChangeEvent* ptr) {
  return static_cast<QAccessibleEvent*>(ptr);
}

QAccessibleStateChangeEvent* qt_gui_c_QAccessibleStateChangeEvent_G_static_cast_QAccessibleStateChangeEvent_ptr(QAccessibleEvent* ptr) {
  return static_cast<QAccessibleStateChangeEvent*>(ptr);
}

void qt_gui_c_QAccessibleStateChangeEvent_changedStates_to_output(const QAccessibleStateChangeEvent* this_ptr, QAccessible::State* output) {
  new(output) QAccessible::State(this_ptr->changedStates());
}

void qt_gui_c_QAccessibleStateChangeEvent_delete(QAccessibleStateChangeEvent* this_ptr) {
  delete this_ptr;
}

QAccessibleStateChangeEvent* qt_gui_c_QAccessibleStateChangeEvent_new_iface_state(QAccessibleInterface* iface, const QAccessible::State* state) {
  return new QAccessibleStateChangeEvent(iface, *state);
}

QAccessibleStateChangeEvent* qt_gui_c_QAccessibleStateChangeEvent_new_obj_state(QObject* obj, const QAccessible::State* state) {
  return new QAccessibleStateChangeEvent(obj, *state);
}

