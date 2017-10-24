#include "qt_core_c_QWriteLocker.h"

void qt_core_c_QWriteLocker_constructor(QReadWriteLock* readWriteLock, QWriteLocker* output) {
  new(output) QWriteLocker(readWriteLock);
}

void qt_core_c_QWriteLocker_destructor(QWriteLocker* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QReadWriteLock* qt_core_c_QWriteLocker_readWriteLock(const QWriteLocker* this_ptr) {
  return this_ptr->readWriteLock();
}

void qt_core_c_QWriteLocker_relock(QWriteLocker* this_ptr) {
  this_ptr->relock();
}

void qt_core_c_QWriteLocker_unlock(QWriteLocker* this_ptr) {
  this_ptr->unlock();
}

