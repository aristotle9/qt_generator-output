#include "qt_core_c_QSocketNotifier.h"

QSocketNotifier* qt_core_c_QSocketNotifier_G_dynamic_cast_QSocketNotifier_ptr(QObject* ptr) {
  return dynamic_cast<QSocketNotifier*>(ptr);
}

QObject* qt_core_c_QSocketNotifier_G_static_cast_QObject_ptr(QSocketNotifier* ptr) {
  return static_cast<QObject*>(ptr);
}

QSocketNotifier* qt_core_c_QSocketNotifier_G_static_cast_QSocketNotifier_ptr(QObject* ptr) {
  return static_cast<QSocketNotifier*>(ptr);
}

void qt_core_c_QSocketNotifier_delete(QSocketNotifier* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QSocketNotifier_isEnabled(const QSocketNotifier* this_ptr) {
  return this_ptr->isEnabled();
}

const QMetaObject* qt_core_c_QSocketNotifier_metaObject(const QSocketNotifier* this_ptr) {
  return this_ptr->metaObject();
}

QSocketNotifier* qt_core_c_QSocketNotifier_new_socket_arg2(qintptr socket, QSocketNotifier::Type arg2) {
  return new QSocketNotifier(socket, arg2);
}

QSocketNotifier* qt_core_c_QSocketNotifier_new_socket_arg2_parent(qintptr socket, QSocketNotifier::Type arg2, QObject* parent) {
  return new QSocketNotifier(socket, arg2, parent);
}

void qt_core_c_QSocketNotifier_setEnabled(QSocketNotifier* this_ptr, bool arg1) {
  this_ptr->setEnabled(arg1);
}

qintptr qt_core_c_QSocketNotifier_socket(const QSocketNotifier* this_ptr) {
  return this_ptr->socket();
}

void qt_core_c_QSocketNotifier_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSocketNotifier::trUtf8(s, c, n));
}

void qt_core_c_QSocketNotifier_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSocketNotifier::tr(s, c, n));
}

QSocketNotifier::Type qt_core_c_QSocketNotifier_type(const QSocketNotifier* this_ptr) {
  return this_ptr->type();
}

