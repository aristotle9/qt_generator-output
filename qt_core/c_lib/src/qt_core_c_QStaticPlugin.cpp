#include "qt_core_c_QStaticPlugin.h"

void qt_core_c_QStaticPlugin_G_qRegisterStaticPluginFunction(const QStaticPlugin* staticPlugin) {
  qRegisterStaticPluginFunction(*staticPlugin);
}

void qt_core_c_QStaticPlugin_destructor(QStaticPlugin* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QObject* (*qt_core_c_QStaticPlugin_instance(const QStaticPlugin* this_ptr))() {
  return this_ptr->instance;
}

void qt_core_c_QStaticPlugin_metaData_to_output(const QStaticPlugin* this_ptr, QJsonObject* output) {
  new(output) QJsonObject(this_ptr->metaData());
}

const char* (*qt_core_c_QStaticPlugin_rawMetaData(const QStaticPlugin* this_ptr))() {
  return this_ptr->rawMetaData;
}

void qt_core_c_QStaticPlugin_set_instance(QStaticPlugin* this_ptr, QObject* (*value)()) {
  this_ptr->instance = value;
}

void qt_core_c_QStaticPlugin_set_rawMetaData(QStaticPlugin* this_ptr, const char* (*value)()) {
  this_ptr->rawMetaData = value;
}

