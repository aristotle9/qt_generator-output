#include "qt_core_c_QSharedMemory.h"

QSharedMemory* qt_core_c_QSharedMemory_G_dynamic_cast_QSharedMemory_ptr(QObject* ptr) {
  return dynamic_cast<QSharedMemory*>(ptr);
}

QObject* qt_core_c_QSharedMemory_G_static_cast_QObject_ptr(QSharedMemory* ptr) {
  return static_cast<QObject*>(ptr);
}

QSharedMemory* qt_core_c_QSharedMemory_G_static_cast_QSharedMemory_ptr(QObject* ptr) {
  return static_cast<QSharedMemory*>(ptr);
}

bool qt_core_c_QSharedMemory_attach_mode(QSharedMemory* this_ptr, QSharedMemory::AccessMode mode) {
  return this_ptr->attach(mode);
}

bool qt_core_c_QSharedMemory_attach_no_args(QSharedMemory* this_ptr) {
  return this_ptr->attach();
}

const void* qt_core_c_QSharedMemory_constData(const QSharedMemory* this_ptr) {
  return this_ptr->constData();
}

bool qt_core_c_QSharedMemory_create_size(QSharedMemory* this_ptr, int size) {
  return this_ptr->create(size);
}

bool qt_core_c_QSharedMemory_create_size_mode(QSharedMemory* this_ptr, int size, QSharedMemory::AccessMode mode) {
  return this_ptr->create(size, mode);
}

void* qt_core_c_QSharedMemory_data(QSharedMemory* this_ptr) {
  return this_ptr->data();
}

const void* qt_core_c_QSharedMemory_data_const(const QSharedMemory* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QSharedMemory_delete(QSharedMemory* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QSharedMemory_detach(QSharedMemory* this_ptr) {
  return this_ptr->detach();
}

QSharedMemory::SharedMemoryError qt_core_c_QSharedMemory_error(const QSharedMemory* this_ptr) {
  return this_ptr->error();
}

void qt_core_c_QSharedMemory_errorString_to_output(const QSharedMemory* this_ptr, QString* output) {
  new(output) QString(this_ptr->errorString());
}

bool qt_core_c_QSharedMemory_isAttached(const QSharedMemory* this_ptr) {
  return this_ptr->isAttached();
}

void qt_core_c_QSharedMemory_key_to_output(const QSharedMemory* this_ptr, QString* output) {
  new(output) QString(this_ptr->key());
}

bool qt_core_c_QSharedMemory_lock(QSharedMemory* this_ptr) {
  return this_ptr->lock();
}

const QMetaObject* qt_core_c_QSharedMemory_metaObject(const QSharedMemory* this_ptr) {
  return this_ptr->metaObject();
}

void qt_core_c_QSharedMemory_nativeKey_to_output(const QSharedMemory* this_ptr, QString* output) {
  new(output) QString(this_ptr->nativeKey());
}

QSharedMemory* qt_core_c_QSharedMemory_new_key(const QString* key) {
  return new QSharedMemory(*key);
}

QSharedMemory* qt_core_c_QSharedMemory_new_key_parent(const QString* key, QObject* parent) {
  return new QSharedMemory(*key, parent);
}

QSharedMemory* qt_core_c_QSharedMemory_new_no_args() {
  return new QSharedMemory();
}

QSharedMemory* qt_core_c_QSharedMemory_new_parent(QObject* parent) {
  return new QSharedMemory(parent);
}

void qt_core_c_QSharedMemory_setKey(QSharedMemory* this_ptr, const QString* key) {
  this_ptr->setKey(*key);
}

void qt_core_c_QSharedMemory_setNativeKey(QSharedMemory* this_ptr, const QString* key) {
  this_ptr->setNativeKey(*key);
}

int qt_core_c_QSharedMemory_size(const QSharedMemory* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QSharedMemory_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSharedMemory::trUtf8(s, c, n));
}

void qt_core_c_QSharedMemory_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSharedMemory::tr(s, c, n));
}

bool qt_core_c_QSharedMemory_unlock(QSharedMemory* this_ptr) {
  return this_ptr->unlock();
}

