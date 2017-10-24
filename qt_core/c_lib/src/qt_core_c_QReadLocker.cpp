#include "qt_core_c_QReadLocker.h"

void qt_core_c_QReadLocker_constructor(QReadWriteLock* readWriteLock, QReadLocker* output) {
  new(output) QReadLocker(readWriteLock);
}

void qt_core_c_QReadLocker_destructor(QReadLocker* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QReadWriteLock* qt_core_c_QReadLocker_readWriteLock(const QReadLocker* this_ptr) {
  return this_ptr->readWriteLock();
}

void qt_core_c_QReadLocker_relock(QReadLocker* this_ptr) {
  this_ptr->relock();
}

void qt_core_c_QReadLocker_unlock(QReadLocker* this_ptr) {
  this_ptr->unlock();
}

