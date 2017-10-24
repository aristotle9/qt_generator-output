#include "qt_core_c_QMutex.h"

QBasicMutex* qt_core_c_QMutex_G_static_cast_QBasicMutex_ptr(QMutex* ptr) {
  return static_cast<QBasicMutex*>(ptr);
}

QMutex* qt_core_c_QMutex_G_static_cast_QMutex_ptr(QBasicMutex* ptr) {
  return static_cast<QMutex*>(ptr);
}

void qt_core_c_QMutex_delete(QMutex* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QMutex_isRecursive(const QMutex* this_ptr) {
  return this_ptr->isRecursive();
}

void qt_core_c_QMutex_lock(QMutex* this_ptr) {
  this_ptr->lock();
}

QMutex* qt_core_c_QMutex_new_mode(QMutex::RecursionMode mode) {
  return new QMutex(mode);
}

QMutex* qt_core_c_QMutex_new_no_args() {
  return new QMutex();
}

bool qt_core_c_QMutex_tryLock_no_args(QMutex* this_ptr) {
  return this_ptr->tryLock();
}

bool qt_core_c_QMutex_tryLock_timeout(QMutex* this_ptr, int timeout) {
  return this_ptr->tryLock(timeout);
}

bool qt_core_c_QMutex_try_lock(QMutex* this_ptr) {
  return this_ptr->try_lock();
}

void qt_core_c_QMutex_unlock(QMutex* this_ptr) {
  this_ptr->unlock();
}

