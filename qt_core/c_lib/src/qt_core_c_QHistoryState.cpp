#include "qt_core_c_QHistoryState.h"

QHistoryState* qt_core_c_QHistoryState_G_dynamic_cast_QHistoryState_ptr_QAbstractState(QAbstractState* ptr) {
  return dynamic_cast<QHistoryState*>(ptr);
}

QHistoryState* qt_core_c_QHistoryState_G_dynamic_cast_QHistoryState_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QHistoryState*>(ptr);
}

QAbstractState* qt_core_c_QHistoryState_G_static_cast_QAbstractState_ptr(QHistoryState* ptr) {
  return static_cast<QAbstractState*>(ptr);
}

QHistoryState* qt_core_c_QHistoryState_G_static_cast_QHistoryState_ptr_QAbstractState(QAbstractState* ptr) {
  return static_cast<QHistoryState*>(ptr);
}

QHistoryState* qt_core_c_QHistoryState_G_static_cast_QHistoryState_ptr_QObject(QObject* ptr) {
  return static_cast<QHistoryState*>(ptr);
}

QObject* qt_core_c_QHistoryState_G_static_cast_QObject_ptr(QHistoryState* ptr) {
  return static_cast<QObject*>(ptr);
}

QAbstractState* qt_core_c_QHistoryState_defaultState(const QHistoryState* this_ptr) {
  return this_ptr->defaultState();
}

QAbstractTransition* qt_core_c_QHistoryState_defaultTransition(const QHistoryState* this_ptr) {
  return this_ptr->defaultTransition();
}

void qt_core_c_QHistoryState_delete(QHistoryState* this_ptr) {
  delete this_ptr;
}

QHistoryState::HistoryType qt_core_c_QHistoryState_historyType(const QHistoryState* this_ptr) {
  return this_ptr->historyType();
}

const QMetaObject* qt_core_c_QHistoryState_metaObject(const QHistoryState* this_ptr) {
  return this_ptr->metaObject();
}

QHistoryState* qt_core_c_QHistoryState_new_no_args() {
  return new QHistoryState();
}

QHistoryState* qt_core_c_QHistoryState_new_parent(QState* parent) {
  return new QHistoryState(parent);
}

QHistoryState* qt_core_c_QHistoryState_new_type(QHistoryState::HistoryType type) {
  return new QHistoryState(type);
}

QHistoryState* qt_core_c_QHistoryState_new_type_parent(QHistoryState::HistoryType type, QState* parent) {
  return new QHistoryState(type, parent);
}

void qt_core_c_QHistoryState_setDefaultState(QHistoryState* this_ptr, QAbstractState* state) {
  this_ptr->setDefaultState(state);
}

void qt_core_c_QHistoryState_setDefaultTransition(QHistoryState* this_ptr, QAbstractTransition* transition) {
  this_ptr->setDefaultTransition(transition);
}

void qt_core_c_QHistoryState_setHistoryType(QHistoryState* this_ptr, QHistoryState::HistoryType type) {
  this_ptr->setHistoryType(type);
}

void qt_core_c_QHistoryState_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QHistoryState::trUtf8(s, c, n));
}

void qt_core_c_QHistoryState_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QHistoryState::tr(s, c, n));
}

