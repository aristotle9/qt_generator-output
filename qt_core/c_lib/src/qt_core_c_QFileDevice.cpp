#include "qt_core_c_QFileDevice.h"

QFileDevice* qt_core_c_QFileDevice_G_dynamic_cast_QFileDevice_ptr_QIODevice(QIODevice* ptr) {
  return dynamic_cast<QFileDevice*>(ptr);
}

QFileDevice* qt_core_c_QFileDevice_G_dynamic_cast_QFileDevice_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QFileDevice*>(ptr);
}

QFileDevice* qt_core_c_QFileDevice_G_static_cast_QFileDevice_ptr_QIODevice(QIODevice* ptr) {
  return static_cast<QFileDevice*>(ptr);
}

QFileDevice* qt_core_c_QFileDevice_G_static_cast_QFileDevice_ptr_QObject(QObject* ptr) {
  return static_cast<QFileDevice*>(ptr);
}

QIODevice* qt_core_c_QFileDevice_G_static_cast_QIODevice_ptr(QFileDevice* ptr) {
  return static_cast<QIODevice*>(ptr);
}

QObject* qt_core_c_QFileDevice_G_static_cast_QObject_ptr(QFileDevice* ptr) {
  return static_cast<QObject*>(ptr);
}

bool qt_core_c_QFileDevice_atEnd(const QFileDevice* this_ptr) {
  return this_ptr->atEnd();
}

void qt_core_c_QFileDevice_close(QFileDevice* this_ptr) {
  this_ptr->close();
}

void qt_core_c_QFileDevice_delete(QFileDevice* this_ptr) {
  delete this_ptr;
}

QFileDevice::FileError qt_core_c_QFileDevice_error(const QFileDevice* this_ptr) {
  return this_ptr->error();
}

void qt_core_c_QFileDevice_fileName_to_output(const QFileDevice* this_ptr, QString* output) {
  new(output) QString(this_ptr->fileName());
}

bool qt_core_c_QFileDevice_flush(QFileDevice* this_ptr) {
  return this_ptr->flush();
}

int qt_core_c_QFileDevice_handle(const QFileDevice* this_ptr) {
  return this_ptr->handle();
}

bool qt_core_c_QFileDevice_isSequential(const QFileDevice* this_ptr) {
  return this_ptr->isSequential();
}

unsigned char* qt_core_c_QFileDevice_map_offset_size(QFileDevice* this_ptr, qint64 offset, qint64 size) {
  return this_ptr->map(offset, size);
}

unsigned char* qt_core_c_QFileDevice_map_offset_size_flags(QFileDevice* this_ptr, qint64 offset, qint64 size, QFileDevice::MemoryMapFlags flags) {
  return this_ptr->map(offset, size, flags);
}

const QMetaObject* qt_core_c_QFileDevice_metaObject(const QFileDevice* this_ptr) {
  return this_ptr->metaObject();
}

unsigned int qt_core_c_QFileDevice_permissions(const QFileDevice* this_ptr) {
  return uint(this_ptr->permissions());
}

qint64 qt_core_c_QFileDevice_pos(const QFileDevice* this_ptr) {
  return this_ptr->pos();
}

bool qt_core_c_QFileDevice_resize(QFileDevice* this_ptr, qint64 sz) {
  return this_ptr->resize(sz);
}

bool qt_core_c_QFileDevice_seek(QFileDevice* this_ptr, qint64 offset) {
  return this_ptr->seek(offset);
}

bool qt_core_c_QFileDevice_setPermissions(QFileDevice* this_ptr, unsigned int permissionSpec) {
  return this_ptr->setPermissions(QFlags< QFileDevice::Permission >(permissionSpec));
}

qint64 qt_core_c_QFileDevice_size(const QFileDevice* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QFileDevice_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFileDevice::trUtf8(s, c, n));
}

void qt_core_c_QFileDevice_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFileDevice::tr(s, c, n));
}

bool qt_core_c_QFileDevice_unmap(QFileDevice* this_ptr, unsigned char* address) {
  return this_ptr->unmap(address);
}

void qt_core_c_QFileDevice_unsetError(QFileDevice* this_ptr) {
  this_ptr->unsetError();
}

