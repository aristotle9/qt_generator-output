#include "qt_core_c_QTimerEvent.h"

QTimerEvent* qt_core_c_QTimerEvent_G_dynamic_cast_QTimerEvent_ptr(QEvent* ptr) {
  return dynamic_cast<QTimerEvent*>(ptr);
}

QEvent* qt_core_c_QTimerEvent_G_static_cast_QEvent_ptr(QTimerEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QTimerEvent* qt_core_c_QTimerEvent_G_static_cast_QTimerEvent_ptr(QEvent* ptr) {
  return static_cast<QTimerEvent*>(ptr);
}

void qt_core_c_QTimerEvent_delete(QTimerEvent* this_ptr) {
  delete this_ptr;
}

QTimerEvent* qt_core_c_QTimerEvent_new(int timerId) {
  return new QTimerEvent(timerId);
}

int qt_core_c_QTimerEvent_timerId(const QTimerEvent* this_ptr) {
  return this_ptr->timerId();
}

