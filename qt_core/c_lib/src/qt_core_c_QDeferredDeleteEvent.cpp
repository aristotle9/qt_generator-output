#include "qt_core_c_QDeferredDeleteEvent.h"

QDeferredDeleteEvent* qt_core_c_QDeferredDeleteEvent_G_dynamic_cast_QDeferredDeleteEvent_ptr(QEvent* ptr) {
  return dynamic_cast<QDeferredDeleteEvent*>(ptr);
}

QDeferredDeleteEvent* qt_core_c_QDeferredDeleteEvent_G_static_cast_QDeferredDeleteEvent_ptr(QEvent* ptr) {
  return static_cast<QDeferredDeleteEvent*>(ptr);
}

QEvent* qt_core_c_QDeferredDeleteEvent_G_static_cast_QEvent_ptr(QDeferredDeleteEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

void qt_core_c_QDeferredDeleteEvent_delete(QDeferredDeleteEvent* this_ptr) {
  delete this_ptr;
}

int qt_core_c_QDeferredDeleteEvent_loopLevel(const QDeferredDeleteEvent* this_ptr) {
  return this_ptr->loopLevel();
}

QDeferredDeleteEvent* qt_core_c_QDeferredDeleteEvent_new() {
  return new QDeferredDeleteEvent();
}

