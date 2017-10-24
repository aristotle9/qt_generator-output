#include "qt_core_c_QFinalState.h"

QFinalState* qt_core_c_QFinalState_G_dynamic_cast_QFinalState_ptr_QAbstractState(QAbstractState* ptr) {
  return dynamic_cast<QFinalState*>(ptr);
}

QFinalState* qt_core_c_QFinalState_G_dynamic_cast_QFinalState_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QFinalState*>(ptr);
}

QAbstractState* qt_core_c_QFinalState_G_static_cast_QAbstractState_ptr(QFinalState* ptr) {
  return static_cast<QAbstractState*>(ptr);
}

QFinalState* qt_core_c_QFinalState_G_static_cast_QFinalState_ptr_QAbstractState(QAbstractState* ptr) {
  return static_cast<QFinalState*>(ptr);
}

QFinalState* qt_core_c_QFinalState_G_static_cast_QFinalState_ptr_QObject(QObject* ptr) {
  return static_cast<QFinalState*>(ptr);
}

QObject* qt_core_c_QFinalState_G_static_cast_QObject_ptr(QFinalState* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_core_c_QFinalState_delete(QFinalState* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_core_c_QFinalState_metaObject(const QFinalState* this_ptr) {
  return this_ptr->metaObject();
}

QFinalState* qt_core_c_QFinalState_new_no_args() {
  return new QFinalState();
}

QFinalState* qt_core_c_QFinalState_new_parent(QState* parent) {
  return new QFinalState(parent);
}

void qt_core_c_QFinalState_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFinalState::trUtf8(s, c, n));
}

void qt_core_c_QFinalState_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFinalState::tr(s, c, n));
}

