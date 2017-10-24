#include "qt_core_c_QEventTransition.h"

QEventTransition* qt_core_c_QEventTransition_G_dynamic_cast_QEventTransition_ptr_QAbstractTransition(QAbstractTransition* ptr) {
  return dynamic_cast<QEventTransition*>(ptr);
}

QEventTransition* qt_core_c_QEventTransition_G_dynamic_cast_QEventTransition_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QEventTransition*>(ptr);
}

QAbstractTransition* qt_core_c_QEventTransition_G_static_cast_QAbstractTransition_ptr(QEventTransition* ptr) {
  return static_cast<QAbstractTransition*>(ptr);
}

QEventTransition* qt_core_c_QEventTransition_G_static_cast_QEventTransition_ptr_QAbstractTransition(QAbstractTransition* ptr) {
  return static_cast<QEventTransition*>(ptr);
}

QEventTransition* qt_core_c_QEventTransition_G_static_cast_QEventTransition_ptr_QObject(QObject* ptr) {
  return static_cast<QEventTransition*>(ptr);
}

QObject* qt_core_c_QEventTransition_G_static_cast_QObject_ptr(QEventTransition* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_core_c_QEventTransition_delete(QEventTransition* this_ptr) {
  delete this_ptr;
}

QObject* qt_core_c_QEventTransition_eventSource(const QEventTransition* this_ptr) {
  return this_ptr->eventSource();
}

const QMetaObject* qt_core_c_QEventTransition_metaObject(const QEventTransition* this_ptr) {
  return this_ptr->metaObject();
}

QEventTransition* qt_core_c_QEventTransition_new_no_args() {
  return new QEventTransition();
}

QEventTransition* qt_core_c_QEventTransition_new_object_type(QObject* object, const QEvent::Type* type) {
  return new QEventTransition(object, *type);
}

QEventTransition* qt_core_c_QEventTransition_new_object_type_sourceState(QObject* object, const QEvent::Type* type, QState* sourceState) {
  return new QEventTransition(object, *type, sourceState);
}

QEventTransition* qt_core_c_QEventTransition_new_sourceState(QState* sourceState) {
  return new QEventTransition(sourceState);
}

void qt_core_c_QEventTransition_setEventSource(QEventTransition* this_ptr, QObject* object) {
  this_ptr->setEventSource(object);
}

void qt_core_c_QEventTransition_setEventType(QEventTransition* this_ptr, const QEvent::Type* type) {
  this_ptr->setEventType(*type);
}

void qt_core_c_QEventTransition_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QEventTransition::trUtf8(s, c, n));
}

void qt_core_c_QEventTransition_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QEventTransition::tr(s, c, n));
}

