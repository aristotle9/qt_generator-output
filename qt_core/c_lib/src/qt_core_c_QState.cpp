#include "qt_core_c_QState.h"

QState* qt_core_c_QState_G_dynamic_cast_QState_ptr_QAbstractState(QAbstractState* ptr) {
  return dynamic_cast<QState*>(ptr);
}

QState* qt_core_c_QState_G_dynamic_cast_QState_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QState*>(ptr);
}

QAbstractState* qt_core_c_QState_G_static_cast_QAbstractState_ptr(QState* ptr) {
  return static_cast<QAbstractState*>(ptr);
}

QObject* qt_core_c_QState_G_static_cast_QObject_ptr(QState* ptr) {
  return static_cast<QObject*>(ptr);
}

QState* qt_core_c_QState_G_static_cast_QState_ptr_QAbstractState(QAbstractState* ptr) {
  return static_cast<QState*>(ptr);
}

QState* qt_core_c_QState_G_static_cast_QState_ptr_QObject(QObject* ptr) {
  return static_cast<QState*>(ptr);
}

QSignalTransition* qt_core_c_QState_addTransition_sender_signal_target(QState* this_ptr, const QObject* sender, const char* signal, QAbstractState* target) {
  return this_ptr->addTransition(sender, signal, target);
}

QAbstractTransition* qt_core_c_QState_addTransition_target(QState* this_ptr, QAbstractState* target) {
  return this_ptr->addTransition(target);
}

void qt_core_c_QState_addTransition_transition(QState* this_ptr, QAbstractTransition* transition) {
  this_ptr->addTransition(transition);
}

void qt_core_c_QState_assignProperty(QState* this_ptr, QObject* object, const char* name, const QVariant* value) {
  this_ptr->assignProperty(object, name, *value);
}

QState::ChildMode qt_core_c_QState_childMode(const QState* this_ptr) {
  return this_ptr->childMode();
}

void qt_core_c_QState_delete(QState* this_ptr) {
  delete this_ptr;
}

QAbstractState* qt_core_c_QState_errorState(const QState* this_ptr) {
  return this_ptr->errorState();
}

QAbstractState* qt_core_c_QState_initialState(const QState* this_ptr) {
  return this_ptr->initialState();
}

const QMetaObject* qt_core_c_QState_metaObject(const QState* this_ptr) {
  return this_ptr->metaObject();
}

QState* qt_core_c_QState_new_childMode(QState::ChildMode childMode) {
  return new QState(childMode);
}

QState* qt_core_c_QState_new_childMode_parent(QState::ChildMode childMode, QState* parent) {
  return new QState(childMode, parent);
}

QState* qt_core_c_QState_new_no_args() {
  return new QState();
}

QState* qt_core_c_QState_new_parent(QState* parent) {
  return new QState(parent);
}

void qt_core_c_QState_removeTransition(QState* this_ptr, QAbstractTransition* transition) {
  this_ptr->removeTransition(transition);
}

void qt_core_c_QState_setChildMode(QState* this_ptr, QState::ChildMode mode) {
  this_ptr->setChildMode(mode);
}

void qt_core_c_QState_setErrorState(QState* this_ptr, QAbstractState* state) {
  this_ptr->setErrorState(state);
}

void qt_core_c_QState_setInitialState(QState* this_ptr, QAbstractState* state) {
  this_ptr->setInitialState(state);
}

void qt_core_c_QState_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QState::trUtf8(s, c, n));
}

void qt_core_c_QState_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QState::tr(s, c, n));
}

void qt_core_c_QState_transitions_to_output(const QState* this_ptr, QList< QAbstractTransition* >* output) {
  new(output) QList< QAbstractTransition* >(this_ptr->transitions());
}

