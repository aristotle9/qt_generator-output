#include "qt_core_c_QStorageInfo.h"

bool qt_core_c_QStorageInfo_G_operator_neq(const QStorageInfo* first, const QStorageInfo* second) {
  return operator!=(*first, *second);
}

void qt_core_c_QStorageInfo_G_swap(QStorageInfo* value1, QStorageInfo* value2) {
  swap(*value1, *value2);
}

int qt_core_c_QStorageInfo_blockSize(const QStorageInfo* this_ptr) {
  return this_ptr->blockSize();
}

qint64 qt_core_c_QStorageInfo_bytesAvailable(const QStorageInfo* this_ptr) {
  return this_ptr->bytesAvailable();
}

qint64 qt_core_c_QStorageInfo_bytesFree(const QStorageInfo* this_ptr) {
  return this_ptr->bytesFree();
}

qint64 qt_core_c_QStorageInfo_bytesTotal(const QStorageInfo* this_ptr) {
  return this_ptr->bytesTotal();
}

void qt_core_c_QStorageInfo_constructor_dir(const QDir* dir, QStorageInfo* output) {
  new(output) QStorageInfo(*dir);
}

void qt_core_c_QStorageInfo_constructor_no_args(QStorageInfo* output) {
  new(output) QStorageInfo();
}

void qt_core_c_QStorageInfo_constructor_other(const QStorageInfo* other, QStorageInfo* output) {
  new(output) QStorageInfo(*other);
}

void qt_core_c_QStorageInfo_constructor_path(const QString* path, QStorageInfo* output) {
  new(output) QStorageInfo(*path);
}

void qt_core_c_QStorageInfo_destructor(QStorageInfo* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QStorageInfo_device_to_output(const QStorageInfo* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->device());
}

void qt_core_c_QStorageInfo_displayName_to_output(const QStorageInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->displayName());
}

void qt_core_c_QStorageInfo_fileSystemType_to_output(const QStorageInfo* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->fileSystemType());
}

bool qt_core_c_QStorageInfo_isReadOnly(const QStorageInfo* this_ptr) {
  return this_ptr->isReadOnly();
}

bool qt_core_c_QStorageInfo_isReady(const QStorageInfo* this_ptr) {
  return this_ptr->isReady();
}

bool qt_core_c_QStorageInfo_isRoot(const QStorageInfo* this_ptr) {
  return this_ptr->isRoot();
}

bool qt_core_c_QStorageInfo_isValid(const QStorageInfo* this_ptr) {
  return this_ptr->isValid();
}

void qt_core_c_QStorageInfo_mountedVolumes_to_output(QList< QStorageInfo >* output) {
  new(output) QList< QStorageInfo >(QStorageInfo::mountedVolumes());
}

void qt_core_c_QStorageInfo_name_to_output(const QStorageInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->name());
}

QStorageInfo* qt_core_c_QStorageInfo_operator_assign(QStorageInfo* this_ptr, const QStorageInfo* other) {
  return &this_ptr->operator=(*other);
}

void qt_core_c_QStorageInfo_refresh(QStorageInfo* this_ptr) {
  this_ptr->refresh();
}

void qt_core_c_QStorageInfo_rootPath_to_output(const QStorageInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->rootPath());
}

void qt_core_c_QStorageInfo_root_to_output(QStorageInfo* output) {
  new(output) QStorageInfo(QStorageInfo::root());
}

void qt_core_c_QStorageInfo_setPath(QStorageInfo* this_ptr, const QString* path) {
  this_ptr->setPath(*path);
}

void qt_core_c_QStorageInfo_subvolume_to_output(const QStorageInfo* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->subvolume());
}

void qt_core_c_QStorageInfo_swap(QStorageInfo* this_ptr, QStorageInfo* other) {
  this_ptr->swap(*other);
}

