#include "qt_core_c_QChildEvent.h"

QChildEvent* qt_core_c_QChildEvent_G_dynamic_cast_QChildEvent_ptr(QEvent* ptr) {
  return dynamic_cast<QChildEvent*>(ptr);
}

QChildEvent* qt_core_c_QChildEvent_G_static_cast_QChildEvent_ptr(QEvent* ptr) {
  return static_cast<QChildEvent*>(ptr);
}

QEvent* qt_core_c_QChildEvent_G_static_cast_QEvent_ptr(QChildEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

bool qt_core_c_QChildEvent_added(const QChildEvent* this_ptr) {
  return this_ptr->added();
}

QObject* qt_core_c_QChildEvent_child(const QChildEvent* this_ptr) {
  return this_ptr->child();
}

void qt_core_c_QChildEvent_delete(QChildEvent* this_ptr) {
  delete this_ptr;
}

QChildEvent* qt_core_c_QChildEvent_new(QEvent::Type type, QObject* child) {
  return new QChildEvent(type, child);
}

bool qt_core_c_QChildEvent_polished(const QChildEvent* this_ptr) {
  return this_ptr->polished();
}

bool qt_core_c_QChildEvent_removed(const QChildEvent* this_ptr) {
  return this_ptr->removed();
}

