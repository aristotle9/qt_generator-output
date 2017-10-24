#include "qt_core_c_QSemaphore.h"

void qt_core_c_QSemaphore_acquire_n(QSemaphore* this_ptr, int n) {
  this_ptr->acquire(n);
}

void qt_core_c_QSemaphore_acquire_no_args(QSemaphore* this_ptr) {
  this_ptr->acquire();
}

int qt_core_c_QSemaphore_available(const QSemaphore* this_ptr) {
  return this_ptr->available();
}

void qt_core_c_QSemaphore_constructor_n(int n, QSemaphore* output) {
  new(output) QSemaphore(n);
}

void qt_core_c_QSemaphore_constructor_no_args(QSemaphore* output) {
  new(output) QSemaphore();
}

void qt_core_c_QSemaphore_destructor(QSemaphore* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QSemaphore_release_n(QSemaphore* this_ptr, int n) {
  this_ptr->release(n);
}

void qt_core_c_QSemaphore_release_no_args(QSemaphore* this_ptr) {
  this_ptr->release();
}

bool qt_core_c_QSemaphore_tryAcquire_n(QSemaphore* this_ptr, int n) {
  return this_ptr->tryAcquire(n);
}

bool qt_core_c_QSemaphore_tryAcquire_n_timeout(QSemaphore* this_ptr, int n, int timeout) {
  return this_ptr->tryAcquire(n, timeout);
}

bool qt_core_c_QSemaphore_tryAcquire_no_args(QSemaphore* this_ptr) {
  return this_ptr->tryAcquire();
}

