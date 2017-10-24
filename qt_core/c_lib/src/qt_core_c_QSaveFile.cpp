#include "qt_core_c_QSaveFile.h"

QSaveFile* qt_core_c_QSaveFile_G_dynamic_cast_QSaveFile_ptr_QFileDevice(QFileDevice* ptr) {
  return dynamic_cast<QSaveFile*>(ptr);
}

QSaveFile* qt_core_c_QSaveFile_G_dynamic_cast_QSaveFile_ptr_QIODevice(QIODevice* ptr) {
  return dynamic_cast<QSaveFile*>(ptr);
}

QSaveFile* qt_core_c_QSaveFile_G_dynamic_cast_QSaveFile_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QSaveFile*>(ptr);
}

QFileDevice* qt_core_c_QSaveFile_G_static_cast_QFileDevice_ptr(QSaveFile* ptr) {
  return static_cast<QFileDevice*>(ptr);
}

QIODevice* qt_core_c_QSaveFile_G_static_cast_QIODevice_ptr(QSaveFile* ptr) {
  return static_cast<QIODevice*>(ptr);
}

QObject* qt_core_c_QSaveFile_G_static_cast_QObject_ptr(QSaveFile* ptr) {
  return static_cast<QObject*>(ptr);
}

QSaveFile* qt_core_c_QSaveFile_G_static_cast_QSaveFile_ptr_QFileDevice(QFileDevice* ptr) {
  return static_cast<QSaveFile*>(ptr);
}

QSaveFile* qt_core_c_QSaveFile_G_static_cast_QSaveFile_ptr_QIODevice(QIODevice* ptr) {
  return static_cast<QSaveFile*>(ptr);
}

QSaveFile* qt_core_c_QSaveFile_G_static_cast_QSaveFile_ptr_QObject(QObject* ptr) {
  return static_cast<QSaveFile*>(ptr);
}

void qt_core_c_QSaveFile_cancelWriting(QSaveFile* this_ptr) {
  this_ptr->cancelWriting();
}

bool qt_core_c_QSaveFile_commit(QSaveFile* this_ptr) {
  return this_ptr->commit();
}

void qt_core_c_QSaveFile_delete(QSaveFile* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QSaveFile_directWriteFallback(const QSaveFile* this_ptr) {
  return this_ptr->directWriteFallback();
}

void qt_core_c_QSaveFile_fileName_to_output(const QSaveFile* this_ptr, QString* output) {
  new(output) QString(this_ptr->fileName());
}

const QMetaObject* qt_core_c_QSaveFile_metaObject(const QSaveFile* this_ptr) {
  return this_ptr->metaObject();
}

QSaveFile* qt_core_c_QSaveFile_new_name(const QString* name) {
  return new QSaveFile(*name);
}

QSaveFile* qt_core_c_QSaveFile_new_name_parent(const QString* name, QObject* parent) {
  return new QSaveFile(*name, parent);
}

QSaveFile* qt_core_c_QSaveFile_new_no_args() {
  return new QSaveFile();
}

QSaveFile* qt_core_c_QSaveFile_new_parent(QObject* parent) {
  return new QSaveFile(parent);
}

bool qt_core_c_QSaveFile_open(QSaveFile* this_ptr, unsigned int flags) {
  return this_ptr->open(QFlags< QIODevice::OpenModeFlag >(flags));
}

void qt_core_c_QSaveFile_setDirectWriteFallback(QSaveFile* this_ptr, bool enabled) {
  this_ptr->setDirectWriteFallback(enabled);
}

void qt_core_c_QSaveFile_setFileName(QSaveFile* this_ptr, const QString* name) {
  this_ptr->setFileName(*name);
}

void qt_core_c_QSaveFile_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSaveFile::trUtf8(s, c, n));
}

void qt_core_c_QSaveFile_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSaveFile::tr(s, c, n));
}

