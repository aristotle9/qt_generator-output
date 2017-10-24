#include "qt_core_c_QMutexLocker.h"

void qt_core_c_QMutexLocker_constructor(QBasicMutex* m, QMutexLocker* output) {
  new(output) QMutexLocker(m);
}

void qt_core_c_QMutexLocker_destructor(QMutexLocker* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QMutex* qt_core_c_QMutexLocker_mutex(const QMutexLocker* this_ptr) {
  return this_ptr->mutex();
}

void qt_core_c_QMutexLocker_relock(QMutexLocker* this_ptr) {
  this_ptr->relock();
}

void qt_core_c_QMutexLocker_unlock(QMutexLocker* this_ptr) {
  this_ptr->unlock();
}

