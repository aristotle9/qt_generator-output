#include "qt_gui_c_QDoubleValidator.h"

QDoubleValidator* qt_gui_c_QDoubleValidator_G_dynamic_cast_QDoubleValidator_ptr(QValidator* ptr) {
  return dynamic_cast<QDoubleValidator*>(ptr);
}

QDoubleValidator* qt_gui_c_QDoubleValidator_G_static_cast_QDoubleValidator_ptr_QObject(QObject* ptr) {
  return static_cast<QDoubleValidator*>(ptr);
}

QDoubleValidator* qt_gui_c_QDoubleValidator_G_static_cast_QDoubleValidator_ptr_QValidator(QValidator* ptr) {
  return static_cast<QDoubleValidator*>(ptr);
}

QObject* qt_gui_c_QDoubleValidator_G_static_cast_QObject_ptr(QDoubleValidator* ptr) {
  return static_cast<QObject*>(ptr);
}

QValidator* qt_gui_c_QDoubleValidator_G_static_cast_QValidator_ptr(QDoubleValidator* ptr) {
  return static_cast<QValidator*>(ptr);
}

double qt_gui_c_QDoubleValidator_bottom(const QDoubleValidator* this_ptr) {
  return this_ptr->bottom();
}

int qt_gui_c_QDoubleValidator_decimals(const QDoubleValidator* this_ptr) {
  return this_ptr->decimals();
}

void qt_gui_c_QDoubleValidator_delete(QDoubleValidator* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_gui_c_QDoubleValidator_metaObject(const QDoubleValidator* this_ptr) {
  return this_ptr->metaObject();
}

QDoubleValidator* qt_gui_c_QDoubleValidator_new_bottom_top_decimals(double bottom, double top, int decimals) {
  return new QDoubleValidator(bottom, top, decimals);
}

QDoubleValidator* qt_gui_c_QDoubleValidator_new_bottom_top_decimals_parent(double bottom, double top, int decimals, QObject* parent) {
  return new QDoubleValidator(bottom, top, decimals, parent);
}

QDoubleValidator* qt_gui_c_QDoubleValidator_new_no_args() {
  return new QDoubleValidator();
}

QDoubleValidator* qt_gui_c_QDoubleValidator_new_parent(QObject* parent) {
  return new QDoubleValidator(parent);
}

QDoubleValidator::Notation qt_gui_c_QDoubleValidator_notation(const QDoubleValidator* this_ptr) {
  return this_ptr->notation();
}

int qt_gui_c_QDoubleValidator_qt_metacall(QDoubleValidator* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QDoubleValidator_qt_metacast(QDoubleValidator* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QDoubleValidator_setBottom(QDoubleValidator* this_ptr, double arg1) {
  this_ptr->setBottom(arg1);
}

void qt_gui_c_QDoubleValidator_setDecimals(QDoubleValidator* this_ptr, int arg1) {
  this_ptr->setDecimals(arg1);
}

void qt_gui_c_QDoubleValidator_setNotation(QDoubleValidator* this_ptr, QDoubleValidator::Notation arg1) {
  this_ptr->setNotation(arg1);
}

void qt_gui_c_QDoubleValidator_setRange_bottom_top(QDoubleValidator* this_ptr, double bottom, double top) {
  this_ptr->setRange(bottom, top);
}

void qt_gui_c_QDoubleValidator_setRange_bottom_top_decimals(QDoubleValidator* this_ptr, double bottom, double top, int decimals) {
  this_ptr->setRange(bottom, top, decimals);
}

void qt_gui_c_QDoubleValidator_setTop(QDoubleValidator* this_ptr, double arg1) {
  this_ptr->setTop(arg1);
}

double qt_gui_c_QDoubleValidator_top(const QDoubleValidator* this_ptr) {
  return this_ptr->top();
}

void qt_gui_c_QDoubleValidator_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDoubleValidator::trUtf8(s, c, n));
}

void qt_gui_c_QDoubleValidator_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDoubleValidator::tr(s, c, n));
}

