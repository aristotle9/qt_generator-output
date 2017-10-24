#include "qt_core_c_QAbstractTransition.h"

QAbstractTransition* qt_core_c_QAbstractTransition_G_dynamic_cast_QAbstractTransition_ptr(QObject* ptr) {
  return dynamic_cast<QAbstractTransition*>(ptr);
}

QAbstractTransition* qt_core_c_QAbstractTransition_G_static_cast_QAbstractTransition_ptr(QObject* ptr) {
  return static_cast<QAbstractTransition*>(ptr);
}

QObject* qt_core_c_QAbstractTransition_G_static_cast_QObject_ptr(QAbstractTransition* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_core_c_QAbstractTransition_addAnimation(QAbstractTransition* this_ptr, QAbstractAnimation* animation) {
  this_ptr->addAnimation(animation);
}

void qt_core_c_QAbstractTransition_animations_to_output(const QAbstractTransition* this_ptr, QList< QAbstractAnimation* >* output) {
  new(output) QList< QAbstractAnimation* >(this_ptr->animations());
}

void qt_core_c_QAbstractTransition_delete(QAbstractTransition* this_ptr) {
  delete this_ptr;
}

QStateMachine* qt_core_c_QAbstractTransition_machine(const QAbstractTransition* this_ptr) {
  return this_ptr->machine();
}

const QMetaObject* qt_core_c_QAbstractTransition_metaObject(const QAbstractTransition* this_ptr) {
  return this_ptr->metaObject();
}

void qt_core_c_QAbstractTransition_removeAnimation(QAbstractTransition* this_ptr, QAbstractAnimation* animation) {
  this_ptr->removeAnimation(animation);
}

void qt_core_c_QAbstractTransition_setTargetState(QAbstractTransition* this_ptr, QAbstractState* target) {
  this_ptr->setTargetState(target);
}

void qt_core_c_QAbstractTransition_setTargetStates(QAbstractTransition* this_ptr, const QList< QAbstractState* >* targets) {
  this_ptr->setTargetStates(*targets);
}

void qt_core_c_QAbstractTransition_setTransitionType(QAbstractTransition* this_ptr, QAbstractTransition::TransitionType type) {
  this_ptr->setTransitionType(type);
}

QState* qt_core_c_QAbstractTransition_sourceState(const QAbstractTransition* this_ptr) {
  return this_ptr->sourceState();
}

QAbstractState* qt_core_c_QAbstractTransition_targetState(const QAbstractTransition* this_ptr) {
  return this_ptr->targetState();
}

void qt_core_c_QAbstractTransition_targetStates_to_output(const QAbstractTransition* this_ptr, QList< QAbstractState* >* output) {
  new(output) QList< QAbstractState* >(this_ptr->targetStates());
}

void qt_core_c_QAbstractTransition_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractTransition::trUtf8(s, c, n));
}

void qt_core_c_QAbstractTransition_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractTransition::tr(s, c, n));
}

QAbstractTransition::TransitionType qt_core_c_QAbstractTransition_transitionType(const QAbstractTransition* this_ptr) {
  return this_ptr->transitionType();
}

