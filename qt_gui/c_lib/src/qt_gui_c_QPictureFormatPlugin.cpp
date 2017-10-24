#include "qt_gui_c_QPictureFormatPlugin.h"

QObject* qt_gui_c_QPictureFormatPlugin_G_static_cast_QObject_ptr(QPictureFormatPlugin* ptr) {
  return static_cast<QObject*>(ptr);
}

QPictureFormatPlugin* qt_gui_c_QPictureFormatPlugin_G_static_cast_QPictureFormatPlugin_ptr(QObject* ptr) {
  return static_cast<QPictureFormatPlugin*>(ptr);
}

void qt_gui_c_QPictureFormatPlugin_delete(QPictureFormatPlugin* this_ptr) {
  delete this_ptr;
}

bool qt_gui_c_QPictureFormatPlugin_installIOHandler(QPictureFormatPlugin* this_ptr, const QString* format) {
  return this_ptr->installIOHandler(*format);
}

bool qt_gui_c_QPictureFormatPlugin_loadPicture(QPictureFormatPlugin* this_ptr, const QString* format, const QString* filename, QPicture* pic) {
  return this_ptr->loadPicture(*format, *filename, pic);
}

const QMetaObject* qt_gui_c_QPictureFormatPlugin_metaObject(const QPictureFormatPlugin* this_ptr) {
  return this_ptr->metaObject();
}

int qt_gui_c_QPictureFormatPlugin_qt_metacall(QPictureFormatPlugin* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QPictureFormatPlugin_qt_metacast(QPictureFormatPlugin* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

bool qt_gui_c_QPictureFormatPlugin_savePicture(QPictureFormatPlugin* this_ptr, const QString* format, const QString* filename, const QPicture* pic) {
  return this_ptr->savePicture(*format, *filename, *pic);
}

void qt_gui_c_QPictureFormatPlugin_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPictureFormatPlugin::trUtf8(s, c, n));
}

void qt_gui_c_QPictureFormatPlugin_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPictureFormatPlugin::tr(s, c, n));
}

