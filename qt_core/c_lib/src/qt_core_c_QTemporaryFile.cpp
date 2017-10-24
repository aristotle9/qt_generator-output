#include "qt_core_c_QTemporaryFile.h"

QTemporaryFile* qt_core_c_QTemporaryFile_G_dynamic_cast_QTemporaryFile_ptr_QFile(QFile* ptr) {
  return dynamic_cast<QTemporaryFile*>(ptr);
}

QTemporaryFile* qt_core_c_QTemporaryFile_G_dynamic_cast_QTemporaryFile_ptr_QFileDevice(QFileDevice* ptr) {
  return dynamic_cast<QTemporaryFile*>(ptr);
}

QTemporaryFile* qt_core_c_QTemporaryFile_G_dynamic_cast_QTemporaryFile_ptr_QIODevice(QIODevice* ptr) {
  return dynamic_cast<QTemporaryFile*>(ptr);
}

QTemporaryFile* qt_core_c_QTemporaryFile_G_dynamic_cast_QTemporaryFile_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QTemporaryFile*>(ptr);
}

QFileDevice* qt_core_c_QTemporaryFile_G_static_cast_QFileDevice_ptr(QTemporaryFile* ptr) {
  return static_cast<QFileDevice*>(ptr);
}

QFile* qt_core_c_QTemporaryFile_G_static_cast_QFile_ptr(QTemporaryFile* ptr) {
  return static_cast<QFile*>(ptr);
}

QIODevice* qt_core_c_QTemporaryFile_G_static_cast_QIODevice_ptr(QTemporaryFile* ptr) {
  return static_cast<QIODevice*>(ptr);
}

QObject* qt_core_c_QTemporaryFile_G_static_cast_QObject_ptr(QTemporaryFile* ptr) {
  return static_cast<QObject*>(ptr);
}

QTemporaryFile* qt_core_c_QTemporaryFile_G_static_cast_QTemporaryFile_ptr_QFile(QFile* ptr) {
  return static_cast<QTemporaryFile*>(ptr);
}

QTemporaryFile* qt_core_c_QTemporaryFile_G_static_cast_QTemporaryFile_ptr_QFileDevice(QFileDevice* ptr) {
  return static_cast<QTemporaryFile*>(ptr);
}

QTemporaryFile* qt_core_c_QTemporaryFile_G_static_cast_QTemporaryFile_ptr_QIODevice(QIODevice* ptr) {
  return static_cast<QTemporaryFile*>(ptr);
}

QTemporaryFile* qt_core_c_QTemporaryFile_G_static_cast_QTemporaryFile_ptr_QObject(QObject* ptr) {
  return static_cast<QTemporaryFile*>(ptr);
}

bool qt_core_c_QTemporaryFile_autoRemove(const QTemporaryFile* this_ptr) {
  return this_ptr->autoRemove();
}

QTemporaryFile* qt_core_c_QTemporaryFile_createLocalFile_file(QFile* file) {
  return QTemporaryFile::createLocalFile(*file);
}

QTemporaryFile* qt_core_c_QTemporaryFile_createLocalFile_fileName(const QString* fileName) {
  return QTemporaryFile::createLocalFile(*fileName);
}

QTemporaryFile* qt_core_c_QTemporaryFile_createNativeFile_file(QFile* file) {
  return QTemporaryFile::createNativeFile(*file);
}

QTemporaryFile* qt_core_c_QTemporaryFile_createNativeFile_fileName(const QString* fileName) {
  return QTemporaryFile::createNativeFile(*fileName);
}

void qt_core_c_QTemporaryFile_delete(QTemporaryFile* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QTemporaryFile_fileName_to_output(const QTemporaryFile* this_ptr, QString* output) {
  new(output) QString(this_ptr->fileName());
}

void qt_core_c_QTemporaryFile_fileTemplate_to_output(const QTemporaryFile* this_ptr, QString* output) {
  new(output) QString(this_ptr->fileTemplate());
}

const QMetaObject* qt_core_c_QTemporaryFile_metaObject(const QTemporaryFile* this_ptr) {
  return this_ptr->metaObject();
}

QTemporaryFile* qt_core_c_QTemporaryFile_new_no_args() {
  return new QTemporaryFile();
}

QTemporaryFile* qt_core_c_QTemporaryFile_new_parent(QObject* parent) {
  return new QTemporaryFile(parent);
}

QTemporaryFile* qt_core_c_QTemporaryFile_new_templateName(const QString* templateName) {
  return new QTemporaryFile(*templateName);
}

QTemporaryFile* qt_core_c_QTemporaryFile_new_templateName_parent(const QString* templateName, QObject* parent) {
  return new QTemporaryFile(*templateName, parent);
}

bool qt_core_c_QTemporaryFile_open(QTemporaryFile* this_ptr) {
  return this_ptr->open();
}

void qt_core_c_QTemporaryFile_setAutoRemove(QTemporaryFile* this_ptr, bool b) {
  this_ptr->setAutoRemove(b);
}

void qt_core_c_QTemporaryFile_setFileTemplate(QTemporaryFile* this_ptr, const QString* name) {
  this_ptr->setFileTemplate(*name);
}

void qt_core_c_QTemporaryFile_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTemporaryFile::trUtf8(s, c, n));
}

void qt_core_c_QTemporaryFile_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTemporaryFile::tr(s, c, n));
}

