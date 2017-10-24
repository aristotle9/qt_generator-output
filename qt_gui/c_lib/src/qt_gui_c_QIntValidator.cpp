#include "qt_gui_c_QIntValidator.h"

QIntValidator* qt_gui_c_QIntValidator_G_dynamic_cast_QIntValidator_ptr(QValidator* ptr) {
  return dynamic_cast<QIntValidator*>(ptr);
}

QIntValidator* qt_gui_c_QIntValidator_G_static_cast_QIntValidator_ptr_QObject(QObject* ptr) {
  return static_cast<QIntValidator*>(ptr);
}

QIntValidator* qt_gui_c_QIntValidator_G_static_cast_QIntValidator_ptr_QValidator(QValidator* ptr) {
  return static_cast<QIntValidator*>(ptr);
}

QObject* qt_gui_c_QIntValidator_G_static_cast_QObject_ptr(QIntValidator* ptr) {
  return static_cast<QObject*>(ptr);
}

QValidator* qt_gui_c_QIntValidator_G_static_cast_QValidator_ptr(QIntValidator* ptr) {
  return static_cast<QValidator*>(ptr);
}

int qt_gui_c_QIntValidator_bottom(const QIntValidator* this_ptr) {
  return this_ptr->bottom();
}

void qt_gui_c_QIntValidator_delete(QIntValidator* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QIntValidator_fixup(const QIntValidator* this_ptr, QString* input) {
  this_ptr->fixup(*input);
}

const QMetaObject* qt_gui_c_QIntValidator_metaObject(const QIntValidator* this_ptr) {
  return this_ptr->metaObject();
}

QIntValidator* qt_gui_c_QIntValidator_new_bottom_top(int bottom, int top) {
  return new QIntValidator(bottom, top);
}

QIntValidator* qt_gui_c_QIntValidator_new_bottom_top_parent(int bottom, int top, QObject* parent) {
  return new QIntValidator(bottom, top, parent);
}

QIntValidator* qt_gui_c_QIntValidator_new_no_args() {
  return new QIntValidator();
}

QIntValidator* qt_gui_c_QIntValidator_new_parent(QObject* parent) {
  return new QIntValidator(parent);
}

int qt_gui_c_QIntValidator_qt_metacall(QIntValidator* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QIntValidator_qt_metacast(QIntValidator* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QIntValidator_setBottom(QIntValidator* this_ptr, int arg1) {
  this_ptr->setBottom(arg1);
}

void qt_gui_c_QIntValidator_setRange(QIntValidator* this_ptr, int bottom, int top) {
  this_ptr->setRange(bottom, top);
}

void qt_gui_c_QIntValidator_setTop(QIntValidator* this_ptr, int arg1) {
  this_ptr->setTop(arg1);
}

int qt_gui_c_QIntValidator_top(const QIntValidator* this_ptr) {
  return this_ptr->top();
}

void qt_gui_c_QIntValidator_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QIntValidator::trUtf8(s, c, n));
}

void qt_gui_c_QIntValidator_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QIntValidator::tr(s, c, n));
}

