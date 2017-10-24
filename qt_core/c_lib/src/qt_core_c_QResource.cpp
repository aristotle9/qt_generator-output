#include "qt_core_c_QResource.h"

void qt_core_c_QResource_absoluteFilePath_to_output(const QResource* this_ptr, QString* output) {
  new(output) QString(this_ptr->absoluteFilePath());
}

void qt_core_c_QResource_addSearchPath(const QString* path) {
  QResource::addSearchPath(*path);
}

void qt_core_c_QResource_constructor_file(const QString* file, QResource* output) {
  new(output) QResource(*file);
}

void qt_core_c_QResource_constructor_file_locale(const QString* file, const QLocale* locale, QResource* output) {
  new(output) QResource(*file, *locale);
}

void qt_core_c_QResource_constructor_no_args(QResource* output) {
  new(output) QResource();
}

const unsigned char* qt_core_c_QResource_data(const QResource* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QResource_destructor(QResource* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QResource_fileName_to_output(const QResource* this_ptr, QString* output) {
  new(output) QString(this_ptr->fileName());
}

bool qt_core_c_QResource_isCompressed(const QResource* this_ptr) {
  return this_ptr->isCompressed();
}

bool qt_core_c_QResource_isValid(const QResource* this_ptr) {
  return this_ptr->isValid();
}

void qt_core_c_QResource_lastModified_to_output(const QResource* this_ptr, QDateTime* output) {
  new(output) QDateTime(this_ptr->lastModified());
}

void qt_core_c_QResource_locale_to_output(const QResource* this_ptr, QLocale* output) {
  new(output) QLocale(this_ptr->locale());
}

bool qt_core_c_QResource_registerResource_rccData(const unsigned char* rccData) {
  return QResource::registerResource(rccData);
}

bool qt_core_c_QResource_registerResource_rccData_resourceRoot(const unsigned char* rccData, const QString* resourceRoot) {
  return QResource::registerResource(rccData, *resourceRoot);
}

bool qt_core_c_QResource_registerResource_rccFilename(const QString* rccFilename) {
  return QResource::registerResource(*rccFilename);
}

bool qt_core_c_QResource_registerResource_rccFilename_resourceRoot(const QString* rccFilename, const QString* resourceRoot) {
  return QResource::registerResource(*rccFilename, *resourceRoot);
}

void qt_core_c_QResource_searchPaths_to_output(QStringList* output) {
  new(output) QStringList(QResource::searchPaths());
}

void qt_core_c_QResource_setFileName(QResource* this_ptr, const QString* file) {
  this_ptr->setFileName(*file);
}

void qt_core_c_QResource_setLocale(QResource* this_ptr, const QLocale* locale) {
  this_ptr->setLocale(*locale);
}

qint64 qt_core_c_QResource_size(const QResource* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QResource_unregisterResource_rccData(const unsigned char* rccData) {
  return QResource::unregisterResource(rccData);
}

bool qt_core_c_QResource_unregisterResource_rccData_resourceRoot(const unsigned char* rccData, const QString* resourceRoot) {
  return QResource::unregisterResource(rccData, *resourceRoot);
}

bool qt_core_c_QResource_unregisterResource_rccFilename(const QString* rccFilename) {
  return QResource::unregisterResource(*rccFilename);
}

bool qt_core_c_QResource_unregisterResource_rccFilename_resourceRoot(const QString* rccFilename, const QString* resourceRoot) {
  return QResource::unregisterResource(*rccFilename, *resourceRoot);
}

