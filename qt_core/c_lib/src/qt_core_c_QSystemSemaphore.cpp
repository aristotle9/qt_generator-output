#include "qt_core_c_QSystemSemaphore.h"

bool qt_core_c_QSystemSemaphore_acquire(QSystemSemaphore* this_ptr) {
  return this_ptr->acquire();
}

void qt_core_c_QSystemSemaphore_constructor_key(const QString* key, QSystemSemaphore* output) {
  new(output) QSystemSemaphore(*key);
}

void qt_core_c_QSystemSemaphore_constructor_key_initialValue(const QString* key, int initialValue, QSystemSemaphore* output) {
  new(output) QSystemSemaphore(*key, initialValue);
}

void qt_core_c_QSystemSemaphore_constructor_key_initialValue_mode(const QString* key, int initialValue, QSystemSemaphore::AccessMode mode, QSystemSemaphore* output) {
  new(output) QSystemSemaphore(*key, initialValue, mode);
}

void qt_core_c_QSystemSemaphore_destructor(QSystemSemaphore* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QSystemSemaphore::SystemSemaphoreError qt_core_c_QSystemSemaphore_error(const QSystemSemaphore* this_ptr) {
  return this_ptr->error();
}

void qt_core_c_QSystemSemaphore_errorString_to_output(const QSystemSemaphore* this_ptr, QString* output) {
  new(output) QString(this_ptr->errorString());
}

void qt_core_c_QSystemSemaphore_key_to_output(const QSystemSemaphore* this_ptr, QString* output) {
  new(output) QString(this_ptr->key());
}

bool qt_core_c_QSystemSemaphore_release_n(QSystemSemaphore* this_ptr, int n) {
  return this_ptr->release(n);
}

bool qt_core_c_QSystemSemaphore_release_no_args(QSystemSemaphore* this_ptr) {
  return this_ptr->release();
}

void qt_core_c_QSystemSemaphore_setKey_key(QSystemSemaphore* this_ptr, const QString* key) {
  this_ptr->setKey(*key);
}

void qt_core_c_QSystemSemaphore_setKey_key_initialValue(QSystemSemaphore* this_ptr, const QString* key, int initialValue) {
  this_ptr->setKey(*key, initialValue);
}

void qt_core_c_QSystemSemaphore_setKey_key_initialValue_mode(QSystemSemaphore* this_ptr, const QString* key, int initialValue, QSystemSemaphore::AccessMode mode) {
  this_ptr->setKey(*key, initialValue, mode);
}

