#include "qt_gui_c_QTextObject.h"

QObject* qt_gui_c_QTextObject_G_static_cast_QObject_ptr(QTextObject* ptr) {
  return static_cast<QObject*>(ptr);
}

QTextObject* qt_gui_c_QTextObject_G_static_cast_QTextObject_ptr(QObject* ptr) {
  return static_cast<QTextObject*>(ptr);
}

QTextDocument* qt_gui_c_QTextObject_document(const QTextObject* this_ptr) {
  return this_ptr->document();
}

int qt_gui_c_QTextObject_formatIndex(const QTextObject* this_ptr) {
  return this_ptr->formatIndex();
}

void qt_gui_c_QTextObject_format_to_output(const QTextObject* this_ptr, QTextFormat* output) {
  new(output) QTextFormat(this_ptr->format());
}

const QMetaObject* qt_gui_c_QTextObject_metaObject(const QTextObject* this_ptr) {
  return this_ptr->metaObject();
}

int qt_gui_c_QTextObject_objectIndex(const QTextObject* this_ptr) {
  return this_ptr->objectIndex();
}

int qt_gui_c_QTextObject_qt_metacall(QTextObject* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QTextObject_qt_metacast(QTextObject* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QTextObject_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTextObject::trUtf8(s, c, n));
}

void qt_gui_c_QTextObject_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTextObject::tr(s, c, n));
}

