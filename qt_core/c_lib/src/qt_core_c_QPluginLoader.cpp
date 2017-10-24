#include "qt_core_c_QPluginLoader.h"

QPluginLoader* qt_core_c_QPluginLoader_G_dynamic_cast_QPluginLoader_ptr(QObject* ptr) {
  return dynamic_cast<QPluginLoader*>(ptr);
}

QObject* qt_core_c_QPluginLoader_G_static_cast_QObject_ptr(QPluginLoader* ptr) {
  return static_cast<QObject*>(ptr);
}

QPluginLoader* qt_core_c_QPluginLoader_G_static_cast_QPluginLoader_ptr(QObject* ptr) {
  return static_cast<QPluginLoader*>(ptr);
}

void qt_core_c_QPluginLoader_delete(QPluginLoader* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QPluginLoader_errorString_to_output(const QPluginLoader* this_ptr, QString* output) {
  new(output) QString(this_ptr->errorString());
}

void qt_core_c_QPluginLoader_fileName_to_output(const QPluginLoader* this_ptr, QString* output) {
  new(output) QString(this_ptr->fileName());
}

QObject* qt_core_c_QPluginLoader_instance(QPluginLoader* this_ptr) {
  return this_ptr->instance();
}

bool qt_core_c_QPluginLoader_isLoaded(const QPluginLoader* this_ptr) {
  return this_ptr->isLoaded();
}

bool qt_core_c_QPluginLoader_load(QPluginLoader* this_ptr) {
  return this_ptr->load();
}

void qt_core_c_QPluginLoader_metaData_to_output(const QPluginLoader* this_ptr, QJsonObject* output) {
  new(output) QJsonObject(this_ptr->metaData());
}

const QMetaObject* qt_core_c_QPluginLoader_metaObject(const QPluginLoader* this_ptr) {
  return this_ptr->metaObject();
}

QPluginLoader* qt_core_c_QPluginLoader_new_fileName(const QString* fileName) {
  return new QPluginLoader(*fileName);
}

QPluginLoader* qt_core_c_QPluginLoader_new_fileName_parent(const QString* fileName, QObject* parent) {
  return new QPluginLoader(*fileName, parent);
}

QPluginLoader* qt_core_c_QPluginLoader_new_no_args() {
  return new QPluginLoader();
}

QPluginLoader* qt_core_c_QPluginLoader_new_parent(QObject* parent) {
  return new QPluginLoader(parent);
}

void qt_core_c_QPluginLoader_setFileName(QPluginLoader* this_ptr, const QString* fileName) {
  this_ptr->setFileName(*fileName);
}

void qt_core_c_QPluginLoader_staticInstances_to_output(QList< QObject* >* output) {
  new(output) QList< QObject* >(QPluginLoader::staticInstances());
}

void qt_core_c_QPluginLoader_staticPlugins_to_output(QVector< QStaticPlugin >* output) {
  new(output) QVector< QStaticPlugin >(QPluginLoader::staticPlugins());
}

void qt_core_c_QPluginLoader_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPluginLoader::trUtf8(s, c, n));
}

void qt_core_c_QPluginLoader_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPluginLoader::tr(s, c, n));
}

bool qt_core_c_QPluginLoader_unload(QPluginLoader* this_ptr) {
  return this_ptr->unload();
}

