#include "qt_core_c_QRunnable.h"

bool qt_core_c_QRunnable_autoDelete(const QRunnable* this_ptr) {
  return this_ptr->autoDelete();
}

void qt_core_c_QRunnable_delete(QRunnable* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QRunnable_run(QRunnable* this_ptr) {
  this_ptr->run();
}

void qt_core_c_QRunnable_setAutoDelete(QRunnable* this_ptr, bool _autoDelete) {
  this_ptr->setAutoDelete(_autoDelete);
}

