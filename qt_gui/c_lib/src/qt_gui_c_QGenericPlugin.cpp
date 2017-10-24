#include "qt_gui_c_QGenericPlugin.h"

QGenericPlugin* qt_gui_c_QGenericPlugin_G_static_cast_QGenericPlugin_ptr(QObject* ptr) {
  return static_cast<QGenericPlugin*>(ptr);
}

QObject* qt_gui_c_QGenericPlugin_G_static_cast_QObject_ptr(QGenericPlugin* ptr) {
  return static_cast<QObject*>(ptr);
}

QObject* qt_gui_c_QGenericPlugin_create(QGenericPlugin* this_ptr, const QString* name, const QString* spec) {
  return this_ptr->create(*name, *spec);
}

void qt_gui_c_QGenericPlugin_delete(QGenericPlugin* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_gui_c_QGenericPlugin_metaObject(const QGenericPlugin* this_ptr) {
  return this_ptr->metaObject();
}

int qt_gui_c_QGenericPlugin_qt_metacall(QGenericPlugin* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QGenericPlugin_qt_metacast(QGenericPlugin* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QGenericPlugin_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGenericPlugin::trUtf8(s, c, n));
}

void qt_gui_c_QGenericPlugin_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGenericPlugin::tr(s, c, n));
}

