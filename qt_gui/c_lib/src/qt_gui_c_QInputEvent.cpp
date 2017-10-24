#include "qt_gui_c_QInputEvent.h"

QEvent* qt_gui_c_QInputEvent_G_static_cast_QEvent_ptr(QInputEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QInputEvent* qt_gui_c_QInputEvent_G_static_cast_QInputEvent_ptr(QEvent* ptr) {
  return static_cast<QInputEvent*>(ptr);
}

void qt_gui_c_QInputEvent_delete(QInputEvent* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QInputEvent_setTimestamp(QInputEvent* this_ptr, unsigned long atimestamp) {
  this_ptr->setTimestamp(atimestamp);
}

unsigned long qt_gui_c_QInputEvent_timestamp(const QInputEvent* this_ptr) {
  return this_ptr->timestamp();
}

