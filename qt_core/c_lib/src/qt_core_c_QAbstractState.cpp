#include "qt_core_c_QAbstractState.h"

QAbstractState* qt_core_c_QAbstractState_G_dynamic_cast_QAbstractState_ptr(QObject* ptr) {
  return dynamic_cast<QAbstractState*>(ptr);
}

QAbstractState* qt_core_c_QAbstractState_G_static_cast_QAbstractState_ptr(QObject* ptr) {
  return static_cast<QAbstractState*>(ptr);
}

QObject* qt_core_c_QAbstractState_G_static_cast_QObject_ptr(QAbstractState* ptr) {
  return static_cast<QObject*>(ptr);
}

bool qt_core_c_QAbstractState_active(const QAbstractState* this_ptr) {
  return this_ptr->active();
}

void qt_core_c_QAbstractState_delete(QAbstractState* this_ptr) {
  delete this_ptr;
}

QStateMachine* qt_core_c_QAbstractState_machine(const QAbstractState* this_ptr) {
  return this_ptr->machine();
}

const QMetaObject* qt_core_c_QAbstractState_metaObject(const QAbstractState* this_ptr) {
  return this_ptr->metaObject();
}

QState* qt_core_c_QAbstractState_parentState(const QAbstractState* this_ptr) {
  return this_ptr->parentState();
}

void qt_core_c_QAbstractState_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractState::trUtf8(s, c, n));
}

void qt_core_c_QAbstractState_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractState::tr(s, c, n));
}

