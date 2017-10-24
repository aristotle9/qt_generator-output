#include "qt_core_c_QWaitCondition.h"

void qt_core_c_QWaitCondition_constructor(QWaitCondition* output) {
  new(output) QWaitCondition();
}

void qt_core_c_QWaitCondition_destructor(QWaitCondition* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QWaitCondition_notify_all(QWaitCondition* this_ptr) {
  this_ptr->notify_all();
}

void qt_core_c_QWaitCondition_notify_one(QWaitCondition* this_ptr) {
  this_ptr->notify_one();
}

bool qt_core_c_QWaitCondition_wait_lockedMutex(QWaitCondition* this_ptr, QMutex* lockedMutex) {
  return this_ptr->wait(lockedMutex);
}

bool qt_core_c_QWaitCondition_wait_lockedMutex_time(QWaitCondition* this_ptr, QMutex* lockedMutex, unsigned long time) {
  return this_ptr->wait(lockedMutex, time);
}

bool qt_core_c_QWaitCondition_wait_lockedReadWriteLock(QWaitCondition* this_ptr, QReadWriteLock* lockedReadWriteLock) {
  return this_ptr->wait(lockedReadWriteLock);
}

bool qt_core_c_QWaitCondition_wait_lockedReadWriteLock_time(QWaitCondition* this_ptr, QReadWriteLock* lockedReadWriteLock, unsigned long time) {
  return this_ptr->wait(lockedReadWriteLock, time);
}

void qt_core_c_QWaitCondition_wakeAll(QWaitCondition* this_ptr) {
  this_ptr->wakeAll();
}

void qt_core_c_QWaitCondition_wakeOne(QWaitCondition* this_ptr) {
  this_ptr->wakeOne();
}

