#include "qt_widgets_c_QStylePlugin.h"

QObject* qt_widgets_c_QStylePlugin_G_static_cast_QObject_ptr(QStylePlugin* ptr) {
  return static_cast<QObject*>(ptr);
}

QStylePlugin* qt_widgets_c_QStylePlugin_G_static_cast_QStylePlugin_ptr(QObject* ptr) {
  return static_cast<QStylePlugin*>(ptr);
}

QStyle* qt_widgets_c_QStylePlugin_create(QStylePlugin* this_ptr, const QString* key) {
  return this_ptr->create(*key);
}

void qt_widgets_c_QStylePlugin_delete(QStylePlugin* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QStylePlugin_metaObject(const QStylePlugin* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QStylePlugin_qt_metacall(QStylePlugin* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QStylePlugin_qt_metacast(QStylePlugin* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QStylePlugin_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStylePlugin::trUtf8(s, c, n));
}

void qt_widgets_c_QStylePlugin_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStylePlugin::tr(s, c, n));
}

