#include "qt_core_c_QStateMachine.h"

QStateMachine::SignalEvent* qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_SignalEvent_ptr(QEvent* ptr) {
  return dynamic_cast<QStateMachine::SignalEvent*>(ptr);
}

QStateMachine::WrappedEvent* qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_WrappedEvent_ptr(QEvent* ptr) {
  return dynamic_cast<QStateMachine::WrappedEvent*>(ptr);
}

QStateMachine* qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_ptr_QAbstractState(QAbstractState* ptr) {
  return dynamic_cast<QStateMachine*>(ptr);
}

QStateMachine* qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QStateMachine*>(ptr);
}

QStateMachine* qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_ptr_QState(QState* ptr) {
  return dynamic_cast<QStateMachine*>(ptr);
}

QAbstractState* qt_core_c_QStateMachine_G_static_cast_QAbstractState_ptr(QStateMachine* ptr) {
  return static_cast<QAbstractState*>(ptr);
}

QEvent* qt_core_c_QStateMachine_G_static_cast_QEvent_ptr_QStateMachine_SignalEvent(QStateMachine::SignalEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QEvent* qt_core_c_QStateMachine_G_static_cast_QEvent_ptr_QStateMachine_WrappedEvent(QStateMachine::WrappedEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QObject* qt_core_c_QStateMachine_G_static_cast_QObject_ptr(QStateMachine* ptr) {
  return static_cast<QObject*>(ptr);
}

QStateMachine::SignalEvent* qt_core_c_QStateMachine_G_static_cast_QStateMachine_SignalEvent_ptr(QEvent* ptr) {
  return static_cast<QStateMachine::SignalEvent*>(ptr);
}

QStateMachine::WrappedEvent* qt_core_c_QStateMachine_G_static_cast_QStateMachine_WrappedEvent_ptr(QEvent* ptr) {
  return static_cast<QStateMachine::WrappedEvent*>(ptr);
}

QStateMachine* qt_core_c_QStateMachine_G_static_cast_QStateMachine_ptr_QAbstractState(QAbstractState* ptr) {
  return static_cast<QStateMachine*>(ptr);
}

QStateMachine* qt_core_c_QStateMachine_G_static_cast_QStateMachine_ptr_QObject(QObject* ptr) {
  return static_cast<QStateMachine*>(ptr);
}

QStateMachine* qt_core_c_QStateMachine_G_static_cast_QStateMachine_ptr_QState(QState* ptr) {
  return static_cast<QStateMachine*>(ptr);
}

QState* qt_core_c_QStateMachine_G_static_cast_QState_ptr(QStateMachine* ptr) {
  return static_cast<QState*>(ptr);
}

void qt_core_c_QStateMachine_SignalEvent_arguments_to_output(const QStateMachine::SignalEvent* this_ptr, QList< QVariant >* output) {
  new(output) QList< QVariant >(this_ptr->arguments());
}

void qt_core_c_QStateMachine_SignalEvent_delete(QStateMachine::SignalEvent* this_ptr) {
  delete this_ptr;
}

QStateMachine::SignalEvent* qt_core_c_QStateMachine_SignalEvent_new(QObject* sender, int signalIndex, const QList< QVariant >* arguments) {
  return new QStateMachine::SignalEvent(sender, signalIndex, *arguments);
}

QObject* qt_core_c_QStateMachine_SignalEvent_sender(const QStateMachine::SignalEvent* this_ptr) {
  return this_ptr->sender();
}

int qt_core_c_QStateMachine_SignalEvent_signalIndex(const QStateMachine::SignalEvent* this_ptr) {
  return this_ptr->signalIndex();
}

void qt_core_c_QStateMachine_WrappedEvent_delete(QStateMachine::WrappedEvent* this_ptr) {
  delete this_ptr;
}

QEvent* qt_core_c_QStateMachine_WrappedEvent_event(const QStateMachine::WrappedEvent* this_ptr) {
  return this_ptr->event();
}

QStateMachine::WrappedEvent* qt_core_c_QStateMachine_WrappedEvent_new(QObject* object, QEvent* event) {
  return new QStateMachine::WrappedEvent(object, event);
}

QObject* qt_core_c_QStateMachine_WrappedEvent_object(const QStateMachine::WrappedEvent* this_ptr) {
  return this_ptr->object();
}

void qt_core_c_QStateMachine_addDefaultAnimation(QStateMachine* this_ptr, QAbstractAnimation* animation) {
  this_ptr->addDefaultAnimation(animation);
}

void qt_core_c_QStateMachine_addState(QStateMachine* this_ptr, QAbstractState* state) {
  this_ptr->addState(state);
}

bool qt_core_c_QStateMachine_cancelDelayedEvent(QStateMachine* this_ptr, int id) {
  return this_ptr->cancelDelayedEvent(id);
}

void qt_core_c_QStateMachine_clearError(QStateMachine* this_ptr) {
  this_ptr->clearError();
}

void qt_core_c_QStateMachine_configuration_to_output(const QStateMachine* this_ptr, QSet< QAbstractState* >* output) {
  new(output) QSet< QAbstractState* >(this_ptr->configuration());
}

void qt_core_c_QStateMachine_defaultAnimations_to_output(const QStateMachine* this_ptr, QList< QAbstractAnimation* >* output) {
  new(output) QList< QAbstractAnimation* >(this_ptr->defaultAnimations());
}

void qt_core_c_QStateMachine_delete(QStateMachine* this_ptr) {
  delete this_ptr;
}

QStateMachine::Error qt_core_c_QStateMachine_error(const QStateMachine* this_ptr) {
  return this_ptr->error();
}

void qt_core_c_QStateMachine_errorString_to_output(const QStateMachine* this_ptr, QString* output) {
  new(output) QString(this_ptr->errorString());
}

bool qt_core_c_QStateMachine_eventFilter(QStateMachine* this_ptr, QObject* watched, QEvent* event) {
  return this_ptr->eventFilter(watched, event);
}

bool qt_core_c_QStateMachine_isAnimated(const QStateMachine* this_ptr) {
  return this_ptr->isAnimated();
}

bool qt_core_c_QStateMachine_isRunning(const QStateMachine* this_ptr) {
  return this_ptr->isRunning();
}

const QMetaObject* qt_core_c_QStateMachine_metaObject(const QStateMachine* this_ptr) {
  return this_ptr->metaObject();
}

QStateMachine* qt_core_c_QStateMachine_new_childMode(const QState::ChildMode* childMode) {
  return new QStateMachine(*childMode);
}

QStateMachine* qt_core_c_QStateMachine_new_childMode_parent(const QState::ChildMode* childMode, QObject* parent) {
  return new QStateMachine(*childMode, parent);
}

QStateMachine* qt_core_c_QStateMachine_new_no_args() {
  return new QStateMachine();
}

QStateMachine* qt_core_c_QStateMachine_new_parent(QObject* parent) {
  return new QStateMachine(parent);
}

int qt_core_c_QStateMachine_postDelayedEvent(QStateMachine* this_ptr, QEvent* event, int delay) {
  return this_ptr->postDelayedEvent(event, delay);
}

void qt_core_c_QStateMachine_postEvent_event(QStateMachine* this_ptr, QEvent* event) {
  this_ptr->postEvent(event);
}

void qt_core_c_QStateMachine_postEvent_event_priority(QStateMachine* this_ptr, QEvent* event, QStateMachine::EventPriority priority) {
  this_ptr->postEvent(event, priority);
}

void qt_core_c_QStateMachine_removeDefaultAnimation(QStateMachine* this_ptr, QAbstractAnimation* animation) {
  this_ptr->removeDefaultAnimation(animation);
}

void qt_core_c_QStateMachine_removeState(QStateMachine* this_ptr, QAbstractState* state) {
  this_ptr->removeState(state);
}

void qt_core_c_QStateMachine_setAnimated(QStateMachine* this_ptr, bool enabled) {
  this_ptr->setAnimated(enabled);
}

void qt_core_c_QStateMachine_setGlobalRestorePolicy(QStateMachine* this_ptr, const QState::RestorePolicy* restorePolicy) {
  this_ptr->setGlobalRestorePolicy(*restorePolicy);
}

void qt_core_c_QStateMachine_setRunning(QStateMachine* this_ptr, bool running) {
  this_ptr->setRunning(running);
}

void qt_core_c_QStateMachine_start(QStateMachine* this_ptr) {
  this_ptr->start();
}

void qt_core_c_QStateMachine_stop(QStateMachine* this_ptr) {
  this_ptr->stop();
}

void qt_core_c_QStateMachine_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStateMachine::trUtf8(s, c, n));
}

void qt_core_c_QStateMachine_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStateMachine::tr(s, c, n));
}

