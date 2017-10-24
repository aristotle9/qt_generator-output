#include "qt_gui_c_QTextBlockGroup.h"

QTextBlockGroup* qt_gui_c_QTextBlockGroup_G_dynamic_cast_QTextBlockGroup_ptr(QTextObject* ptr) {
  return dynamic_cast<QTextBlockGroup*>(ptr);
}

QObject* qt_gui_c_QTextBlockGroup_G_static_cast_QObject_ptr(QTextBlockGroup* ptr) {
  return static_cast<QObject*>(ptr);
}

QTextBlockGroup* qt_gui_c_QTextBlockGroup_G_static_cast_QTextBlockGroup_ptr_QObject(QObject* ptr) {
  return static_cast<QTextBlockGroup*>(ptr);
}

QTextBlockGroup* qt_gui_c_QTextBlockGroup_G_static_cast_QTextBlockGroup_ptr_QTextObject(QTextObject* ptr) {
  return static_cast<QTextBlockGroup*>(ptr);
}

QTextObject* qt_gui_c_QTextBlockGroup_G_static_cast_QTextObject_ptr(QTextBlockGroup* ptr) {
  return static_cast<QTextObject*>(ptr);
}

const QMetaObject* qt_gui_c_QTextBlockGroup_metaObject(const QTextBlockGroup* this_ptr) {
  return this_ptr->metaObject();
}

int qt_gui_c_QTextBlockGroup_qt_metacall(QTextBlockGroup* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QTextBlockGroup_qt_metacast(QTextBlockGroup* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QTextBlockGroup_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTextBlockGroup::trUtf8(s, c, n));
}

void qt_gui_c_QTextBlockGroup_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTextBlockGroup::tr(s, c, n));
}

