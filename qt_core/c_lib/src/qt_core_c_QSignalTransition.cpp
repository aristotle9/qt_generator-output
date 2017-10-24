#include "qt_core_c_QSignalTransition.h"

QSignalTransition* qt_core_c_QSignalTransition_G_dynamic_cast_QSignalTransition_ptr_QAbstractTransition(QAbstractTransition* ptr) {
  return dynamic_cast<QSignalTransition*>(ptr);
}

QSignalTransition* qt_core_c_QSignalTransition_G_dynamic_cast_QSignalTransition_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QSignalTransition*>(ptr);
}

QAbstractTransition* qt_core_c_QSignalTransition_G_static_cast_QAbstractTransition_ptr(QSignalTransition* ptr) {
  return static_cast<QAbstractTransition*>(ptr);
}

QObject* qt_core_c_QSignalTransition_G_static_cast_QObject_ptr(QSignalTransition* ptr) {
  return static_cast<QObject*>(ptr);
}

QSignalTransition* qt_core_c_QSignalTransition_G_static_cast_QSignalTransition_ptr_QAbstractTransition(QAbstractTransition* ptr) {
  return static_cast<QSignalTransition*>(ptr);
}

QSignalTransition* qt_core_c_QSignalTransition_G_static_cast_QSignalTransition_ptr_QObject(QObject* ptr) {
  return static_cast<QSignalTransition*>(ptr);
}

void qt_core_c_QSignalTransition_delete(QSignalTransition* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_core_c_QSignalTransition_metaObject(const QSignalTransition* this_ptr) {
  return this_ptr->metaObject();
}

QSignalTransition* qt_core_c_QSignalTransition_new_no_args() {
  return new QSignalTransition();
}

QSignalTransition* qt_core_c_QSignalTransition_new_sender_signal(const QObject* sender, const char* signal) {
  return new QSignalTransition(sender, signal);
}

QSignalTransition* qt_core_c_QSignalTransition_new_sender_signal_sourceState(const QObject* sender, const char* signal, QState* sourceState) {
  return new QSignalTransition(sender, signal, sourceState);
}

QSignalTransition* qt_core_c_QSignalTransition_new_sourceState(QState* sourceState) {
  return new QSignalTransition(sourceState);
}

QObject* qt_core_c_QSignalTransition_senderObject(const QSignalTransition* this_ptr) {
  return this_ptr->senderObject();
}

void qt_core_c_QSignalTransition_setSenderObject(QSignalTransition* this_ptr, const QObject* sender) {
  this_ptr->setSenderObject(sender);
}

void qt_core_c_QSignalTransition_setSignal(QSignalTransition* this_ptr, const QByteArray* signal) {
  this_ptr->setSignal(*signal);
}

void qt_core_c_QSignalTransition_signal_to_output(const QSignalTransition* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->signal());
}

void qt_core_c_QSignalTransition_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSignalTransition::trUtf8(s, c, n));
}

void qt_core_c_QSignalTransition_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSignalTransition::tr(s, c, n));
}

