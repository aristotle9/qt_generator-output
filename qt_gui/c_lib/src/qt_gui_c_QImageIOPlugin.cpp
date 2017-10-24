#include "qt_gui_c_QImageIOPlugin.h"

QImageIOPlugin* qt_gui_c_QImageIOPlugin_G_static_cast_QImageIOPlugin_ptr(QObject* ptr) {
  return static_cast<QImageIOPlugin*>(ptr);
}

QObject* qt_gui_c_QImageIOPlugin_G_static_cast_QObject_ptr(QImageIOPlugin* ptr) {
  return static_cast<QObject*>(ptr);
}

unsigned int qt_gui_c_QImageIOPlugin_capabilities(const QImageIOPlugin* this_ptr, QIODevice* device, const QByteArray* format) {
  return uint(this_ptr->capabilities(device, *format));
}

QImageIOHandler* qt_gui_c_QImageIOPlugin_create_device(const QImageIOPlugin* this_ptr, QIODevice* device) {
  return this_ptr->create(device);
}

QImageIOHandler* qt_gui_c_QImageIOPlugin_create_device_format(const QImageIOPlugin* this_ptr, QIODevice* device, const QByteArray* format) {
  return this_ptr->create(device, *format);
}

void qt_gui_c_QImageIOPlugin_delete(QImageIOPlugin* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_gui_c_QImageIOPlugin_metaObject(const QImageIOPlugin* this_ptr) {
  return this_ptr->metaObject();
}

int qt_gui_c_QImageIOPlugin_qt_metacall(QImageIOPlugin* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QImageIOPlugin_qt_metacast(QImageIOPlugin* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QImageIOPlugin_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QImageIOPlugin::trUtf8(s, c, n));
}

void qt_gui_c_QImageIOPlugin_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QImageIOPlugin::tr(s, c, n));
}

