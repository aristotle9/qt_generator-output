#include "qt_gui_c_QRegularExpressionValidator.h"

QRegularExpressionValidator* qt_gui_c_QRegularExpressionValidator_G_dynamic_cast_QRegularExpressionValidator_ptr(QValidator* ptr) {
  return dynamic_cast<QRegularExpressionValidator*>(ptr);
}

QObject* qt_gui_c_QRegularExpressionValidator_G_static_cast_QObject_ptr(QRegularExpressionValidator* ptr) {
  return static_cast<QObject*>(ptr);
}

QRegularExpressionValidator* qt_gui_c_QRegularExpressionValidator_G_static_cast_QRegularExpressionValidator_ptr_QObject(QObject* ptr) {
  return static_cast<QRegularExpressionValidator*>(ptr);
}

QRegularExpressionValidator* qt_gui_c_QRegularExpressionValidator_G_static_cast_QRegularExpressionValidator_ptr_QValidator(QValidator* ptr) {
  return static_cast<QRegularExpressionValidator*>(ptr);
}

QValidator* qt_gui_c_QRegularExpressionValidator_G_static_cast_QValidator_ptr(QRegularExpressionValidator* ptr) {
  return static_cast<QValidator*>(ptr);
}

void qt_gui_c_QRegularExpressionValidator_delete(QRegularExpressionValidator* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_gui_c_QRegularExpressionValidator_metaObject(const QRegularExpressionValidator* this_ptr) {
  return this_ptr->metaObject();
}

QRegularExpressionValidator* qt_gui_c_QRegularExpressionValidator_new_no_args() {
  return new QRegularExpressionValidator();
}

QRegularExpressionValidator* qt_gui_c_QRegularExpressionValidator_new_parent(QObject* parent) {
  return new QRegularExpressionValidator(parent);
}

QRegularExpressionValidator* qt_gui_c_QRegularExpressionValidator_new_re(const QRegularExpression* re) {
  return new QRegularExpressionValidator(*re);
}

QRegularExpressionValidator* qt_gui_c_QRegularExpressionValidator_new_re_parent(const QRegularExpression* re, QObject* parent) {
  return new QRegularExpressionValidator(*re, parent);
}

int qt_gui_c_QRegularExpressionValidator_qt_metacall(QRegularExpressionValidator* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QRegularExpressionValidator_qt_metacast(QRegularExpressionValidator* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QRegularExpressionValidator_regularExpression_to_output(const QRegularExpressionValidator* this_ptr, QRegularExpression* output) {
  new(output) QRegularExpression(this_ptr->regularExpression());
}

void qt_gui_c_QRegularExpressionValidator_setRegularExpression(QRegularExpressionValidator* this_ptr, const QRegularExpression* re) {
  this_ptr->setRegularExpression(*re);
}

void qt_gui_c_QRegularExpressionValidator_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QRegularExpressionValidator::trUtf8(s, c, n));
}

void qt_gui_c_QRegularExpressionValidator_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QRegularExpressionValidator::tr(s, c, n));
}

