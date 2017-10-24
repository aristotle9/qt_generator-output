#include "qt_gui_c_QRegExpValidator.h"

QRegExpValidator* qt_gui_c_QRegExpValidator_G_dynamic_cast_QRegExpValidator_ptr(QValidator* ptr) {
  return dynamic_cast<QRegExpValidator*>(ptr);
}

QObject* qt_gui_c_QRegExpValidator_G_static_cast_QObject_ptr(QRegExpValidator* ptr) {
  return static_cast<QObject*>(ptr);
}

QRegExpValidator* qt_gui_c_QRegExpValidator_G_static_cast_QRegExpValidator_ptr_QObject(QObject* ptr) {
  return static_cast<QRegExpValidator*>(ptr);
}

QRegExpValidator* qt_gui_c_QRegExpValidator_G_static_cast_QRegExpValidator_ptr_QValidator(QValidator* ptr) {
  return static_cast<QRegExpValidator*>(ptr);
}

QValidator* qt_gui_c_QRegExpValidator_G_static_cast_QValidator_ptr(QRegExpValidator* ptr) {
  return static_cast<QValidator*>(ptr);
}

void qt_gui_c_QRegExpValidator_delete(QRegExpValidator* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_gui_c_QRegExpValidator_metaObject(const QRegExpValidator* this_ptr) {
  return this_ptr->metaObject();
}

QRegExpValidator* qt_gui_c_QRegExpValidator_new_no_args() {
  return new QRegExpValidator();
}

QRegExpValidator* qt_gui_c_QRegExpValidator_new_parent(QObject* parent) {
  return new QRegExpValidator(parent);
}

QRegExpValidator* qt_gui_c_QRegExpValidator_new_rx(const QRegExp* rx) {
  return new QRegExpValidator(*rx);
}

QRegExpValidator* qt_gui_c_QRegExpValidator_new_rx_parent(const QRegExp* rx, QObject* parent) {
  return new QRegExpValidator(*rx, parent);
}

int qt_gui_c_QRegExpValidator_qt_metacall(QRegExpValidator* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QRegExpValidator_qt_metacast(QRegExpValidator* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

const QRegExp* qt_gui_c_QRegExpValidator_regExp(const QRegExpValidator* this_ptr) {
  return &this_ptr->regExp();
}

void qt_gui_c_QRegExpValidator_setRegExp(QRegExpValidator* this_ptr, const QRegExp* rx) {
  this_ptr->setRegExp(*rx);
}

void qt_gui_c_QRegExpValidator_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QRegExpValidator::trUtf8(s, c, n));
}

void qt_gui_c_QRegExpValidator_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QRegExpValidator::tr(s, c, n));
}

