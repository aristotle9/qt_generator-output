#include "qt_core_c_QLibrary.h"

QLibrary* qt_core_c_QLibrary_G_dynamic_cast_QLibrary_ptr(QObject* ptr) {
  return dynamic_cast<QLibrary*>(ptr);
}

QLibrary* qt_core_c_QLibrary_G_static_cast_QLibrary_ptr(QObject* ptr) {
  return static_cast<QLibrary*>(ptr);
}

QObject* qt_core_c_QLibrary_G_static_cast_QObject_ptr(QLibrary* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_core_c_QLibrary_delete(QLibrary* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QLibrary_errorString_to_output(const QLibrary* this_ptr, QString* output) {
  new(output) QString(this_ptr->errorString());
}

void qt_core_c_QLibrary_fileName_to_output(const QLibrary* this_ptr, QString* output) {
  new(output) QString(this_ptr->fileName());
}

bool qt_core_c_QLibrary_isLibrary(const QString* fileName) {
  return QLibrary::isLibrary(*fileName);
}

bool qt_core_c_QLibrary_isLoaded(const QLibrary* this_ptr) {
  return this_ptr->isLoaded();
}

bool qt_core_c_QLibrary_load(QLibrary* this_ptr) {
  return this_ptr->load();
}

unsigned int qt_core_c_QLibrary_loadHints(const QLibrary* this_ptr) {
  return uint(this_ptr->loadHints());
}

const QMetaObject* qt_core_c_QLibrary_metaObject(const QLibrary* this_ptr) {
  return this_ptr->metaObject();
}

QLibrary* qt_core_c_QLibrary_new_fileName(const QString* fileName) {
  return new QLibrary(*fileName);
}

QLibrary* qt_core_c_QLibrary_new_fileName_parent(const QString* fileName, QObject* parent) {
  return new QLibrary(*fileName, parent);
}

QLibrary* qt_core_c_QLibrary_new_fileName_verNum(const QString* fileName, int verNum) {
  return new QLibrary(*fileName, verNum);
}

QLibrary* qt_core_c_QLibrary_new_fileName_verNum_parent(const QString* fileName, int verNum, QObject* parent) {
  return new QLibrary(*fileName, verNum, parent);
}

QLibrary* qt_core_c_QLibrary_new_fileName_version(const QString* fileName, const QString* version) {
  return new QLibrary(*fileName, *version);
}

QLibrary* qt_core_c_QLibrary_new_fileName_version_parent(const QString* fileName, const QString* version, QObject* parent) {
  return new QLibrary(*fileName, *version, parent);
}

QLibrary* qt_core_c_QLibrary_new_no_args() {
  return new QLibrary();
}

QLibrary* qt_core_c_QLibrary_new_parent(QObject* parent) {
  return new QLibrary(parent);
}

void (*qt_core_c_QLibrary_resolve_fileName_symbol(const QString* fileName, const char* symbol))() {
  return QLibrary::resolve(*fileName, symbol);
}

void (*qt_core_c_QLibrary_resolve_fileName_verNum_symbol(const QString* fileName, int verNum, const char* symbol))() {
  return QLibrary::resolve(*fileName, verNum, symbol);
}

void (*qt_core_c_QLibrary_resolve_fileName_version_symbol(const QString* fileName, const QString* version, const char* symbol))() {
  return QLibrary::resolve(*fileName, *version, symbol);
}

void (*qt_core_c_QLibrary_resolve_symbol(QLibrary* this_ptr, const char* symbol))() {
  return this_ptr->resolve(symbol);
}

void qt_core_c_QLibrary_setFileName(QLibrary* this_ptr, const QString* fileName) {
  this_ptr->setFileName(*fileName);
}

void qt_core_c_QLibrary_setFileNameAndVersion_fileName_verNum(QLibrary* this_ptr, const QString* fileName, int verNum) {
  this_ptr->setFileNameAndVersion(*fileName, verNum);
}

void qt_core_c_QLibrary_setFileNameAndVersion_fileName_version(QLibrary* this_ptr, const QString* fileName, const QString* version) {
  this_ptr->setFileNameAndVersion(*fileName, *version);
}

void qt_core_c_QLibrary_setLoadHints(QLibrary* this_ptr, unsigned int hints) {
  this_ptr->setLoadHints(QFlags< QLibrary::LoadHint >(hints));
}

void qt_core_c_QLibrary_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QLibrary::trUtf8(s, c, n));
}

void qt_core_c_QLibrary_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QLibrary::tr(s, c, n));
}

bool qt_core_c_QLibrary_unload(QLibrary* this_ptr) {
  return this_ptr->unload();
}

