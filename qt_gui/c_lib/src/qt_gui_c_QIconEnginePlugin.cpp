#include "qt_gui_c_QIconEnginePlugin.h"

QIconEnginePlugin* qt_gui_c_QIconEnginePlugin_G_static_cast_QIconEnginePlugin_ptr(QObject* ptr) {
  return static_cast<QIconEnginePlugin*>(ptr);
}

QObject* qt_gui_c_QIconEnginePlugin_G_static_cast_QObject_ptr(QIconEnginePlugin* ptr) {
  return static_cast<QObject*>(ptr);
}

QIconEngine* qt_gui_c_QIconEnginePlugin_create_filename(QIconEnginePlugin* this_ptr, const QString* filename) {
  return this_ptr->create(*filename);
}

QIconEngine* qt_gui_c_QIconEnginePlugin_create_no_args(QIconEnginePlugin* this_ptr) {
  return this_ptr->create();
}

void qt_gui_c_QIconEnginePlugin_delete(QIconEnginePlugin* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_gui_c_QIconEnginePlugin_metaObject(const QIconEnginePlugin* this_ptr) {
  return this_ptr->metaObject();
}

int qt_gui_c_QIconEnginePlugin_qt_metacall(QIconEnginePlugin* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QIconEnginePlugin_qt_metacast(QIconEnginePlugin* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QIconEnginePlugin_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QIconEnginePlugin::trUtf8(s, c, n));
}

void qt_gui_c_QIconEnginePlugin_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QIconEnginePlugin::tr(s, c, n));
}

