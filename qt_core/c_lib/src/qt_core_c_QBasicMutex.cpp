#include "qt_core_c_QBasicMutex.h"

void qt_core_c_QBasicMutex_delete(QBasicMutex* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QBasicMutex_isRecursive(QBasicMutex* this_ptr) {
  return this_ptr->isRecursive();
}

bool qt_core_c_QBasicMutex_isRecursive_const(const QBasicMutex* this_ptr) {
  return this_ptr->isRecursive();
}

void qt_core_c_QBasicMutex_lock(QBasicMutex* this_ptr) {
  this_ptr->lock();
}

bool qt_core_c_QBasicMutex_tryLock(QBasicMutex* this_ptr) {
  return this_ptr->tryLock();
}

bool qt_core_c_QBasicMutex_try_lock(QBasicMutex* this_ptr) {
  return this_ptr->try_lock();
}

void qt_core_c_QBasicMutex_unlock(QBasicMutex* this_ptr) {
  this_ptr->unlock();
}

