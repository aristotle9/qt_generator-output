#include "qt_core_c_QReadWriteLock.h"

void qt_core_c_QReadWriteLock_delete(QReadWriteLock* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QReadWriteLock_lockForRead(QReadWriteLock* this_ptr) {
  this_ptr->lockForRead();
}

void qt_core_c_QReadWriteLock_lockForWrite(QReadWriteLock* this_ptr) {
  this_ptr->lockForWrite();
}

QReadWriteLock* qt_core_c_QReadWriteLock_new_no_args() {
  return new QReadWriteLock();
}

QReadWriteLock* qt_core_c_QReadWriteLock_new_recursionMode(QReadWriteLock::RecursionMode recursionMode) {
  return new QReadWriteLock(recursionMode);
}

bool qt_core_c_QReadWriteLock_tryLockForRead_no_args(QReadWriteLock* this_ptr) {
  return this_ptr->tryLockForRead();
}

bool qt_core_c_QReadWriteLock_tryLockForRead_timeout(QReadWriteLock* this_ptr, int timeout) {
  return this_ptr->tryLockForRead(timeout);
}

bool qt_core_c_QReadWriteLock_tryLockForWrite_no_args(QReadWriteLock* this_ptr) {
  return this_ptr->tryLockForWrite();
}

bool qt_core_c_QReadWriteLock_tryLockForWrite_timeout(QReadWriteLock* this_ptr, int timeout) {
  return this_ptr->tryLockForWrite(timeout);
}

void qt_core_c_QReadWriteLock_unlock(QReadWriteLock* this_ptr) {
  this_ptr->unlock();
}

