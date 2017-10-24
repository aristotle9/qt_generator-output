#include "qt_gui_c_QAccessiblePlugin.h"

QAccessiblePlugin* qt_gui_c_QAccessiblePlugin_G_static_cast_QAccessiblePlugin_ptr(QObject* ptr) {
  return static_cast<QAccessiblePlugin*>(ptr);
}

QObject* qt_gui_c_QAccessiblePlugin_G_static_cast_QObject_ptr(QAccessiblePlugin* ptr) {
  return static_cast<QObject*>(ptr);
}

QAccessibleInterface* qt_gui_c_QAccessiblePlugin_create(QAccessiblePlugin* this_ptr, const QString* key, QObject* object) {
  return this_ptr->create(*key, object);
}

void qt_gui_c_QAccessiblePlugin_delete(QAccessiblePlugin* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_gui_c_QAccessiblePlugin_metaObject(const QAccessiblePlugin* this_ptr) {
  return this_ptr->metaObject();
}

int qt_gui_c_QAccessiblePlugin_qt_metacall(QAccessiblePlugin* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QAccessiblePlugin_qt_metacast(QAccessiblePlugin* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QAccessiblePlugin_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAccessiblePlugin::trUtf8(s, c, n));
}

void qt_gui_c_QAccessiblePlugin_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAccessiblePlugin::tr(s, c, n));
}

