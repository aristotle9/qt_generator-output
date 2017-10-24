#include "qt_gui_c_QValidator.h"

QObject* qt_gui_c_QValidator_G_static_cast_QObject_ptr(QValidator* ptr) {
  return static_cast<QObject*>(ptr);
}

QValidator* qt_gui_c_QValidator_G_static_cast_QValidator_ptr(QObject* ptr) {
  return static_cast<QValidator*>(ptr);
}

void qt_gui_c_QValidator_delete(QValidator* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QValidator_fixup(const QValidator* this_ptr, QString* arg1) {
  this_ptr->fixup(*arg1);
}

void qt_gui_c_QValidator_locale_to_output(const QValidator* this_ptr, QLocale* output) {
  new(output) QLocale(this_ptr->locale());
}

const QMetaObject* qt_gui_c_QValidator_metaObject(const QValidator* this_ptr) {
  return this_ptr->metaObject();
}

int qt_gui_c_QValidator_qt_metacall(QValidator* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QValidator_qt_metacast(QValidator* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QValidator_setLocale(QValidator* this_ptr, const QLocale* locale) {
  this_ptr->setLocale(*locale);
}

void qt_gui_c_QValidator_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QValidator::trUtf8(s, c, n));
}

void qt_gui_c_QValidator_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QValidator::tr(s, c, n));
}

QValidator::State qt_gui_c_QValidator_validate(const QValidator* this_ptr, QString* arg1, int* arg2) {
  return this_ptr->validate(*arg1, *arg2);
}

