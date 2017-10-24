#include "qt_core_c_QSignalBlocker.h"

void qt_core_c_QSignalBlocker_constructor_QObject_ptr(QObject* o, QSignalBlocker* output) {
  new(output) QSignalBlocker(o);
}

void qt_core_c_QSignalBlocker_constructor_QObject_ref(QObject* o, QSignalBlocker* output) {
  new(output) QSignalBlocker(*o);
}

void qt_core_c_QSignalBlocker_destructor(QSignalBlocker* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QSignalBlocker_reblock(QSignalBlocker* this_ptr) {
  this_ptr->reblock();
}

void qt_core_c_QSignalBlocker_unblock(QSignalBlocker* this_ptr) {
  this_ptr->unblock();
}

