#include "qt_core_c_QTimer.h"

QTimer* qt_core_c_QTimer_G_dynamic_cast_QTimer_ptr(QObject* ptr) {
  return dynamic_cast<QTimer*>(ptr);
}

QObject* qt_core_c_QTimer_G_static_cast_QObject_ptr(QTimer* ptr) {
  return static_cast<QObject*>(ptr);
}

QTimer* qt_core_c_QTimer_G_static_cast_QTimer_ptr(QObject* ptr) {
  return static_cast<QTimer*>(ptr);
}

void qt_core_c_QTimer_delete(QTimer* this_ptr) {
  delete this_ptr;
}

int qt_core_c_QTimer_interval(const QTimer* this_ptr) {
  return this_ptr->interval();
}

bool qt_core_c_QTimer_isActive(const QTimer* this_ptr) {
  return this_ptr->isActive();
}

bool qt_core_c_QTimer_isSingleShot(const QTimer* this_ptr) {
  return this_ptr->isSingleShot();
}

const QMetaObject* qt_core_c_QTimer_metaObject(const QTimer* this_ptr) {
  return this_ptr->metaObject();
}

QTimer* qt_core_c_QTimer_new_no_args() {
  return new QTimer();
}

QTimer* qt_core_c_QTimer_new_parent(QObject* parent) {
  return new QTimer(parent);
}

int qt_core_c_QTimer_remainingTime(const QTimer* this_ptr) {
  return this_ptr->remainingTime();
}

void qt_core_c_QTimer_setInterval(QTimer* this_ptr, int msec) {
  this_ptr->setInterval(msec);
}

void qt_core_c_QTimer_setSingleShot(QTimer* this_ptr, bool singleShot) {
  this_ptr->setSingleShot(singleShot);
}

void qt_core_c_QTimer_setTimerType(QTimer* this_ptr, const Qt::TimerType* atype) {
  this_ptr->setTimerType(*atype);
}

void qt_core_c_QTimer_singleShot_msec_receiver_member(int msec, const QObject* receiver, const char* member) {
  QTimer::singleShot(msec, receiver, member);
}

void qt_core_c_QTimer_singleShot_msec_timerType_receiver_member(int msec, const Qt::TimerType* timerType, const QObject* receiver, const char* member) {
  QTimer::singleShot(msec, *timerType, receiver, member);
}

void qt_core_c_QTimer_start_msec(QTimer* this_ptr, int msec) {
  this_ptr->start(msec);
}

void qt_core_c_QTimer_start_no_args(QTimer* this_ptr) {
  this_ptr->start();
}

void qt_core_c_QTimer_stop(QTimer* this_ptr) {
  this_ptr->stop();
}

int qt_core_c_QTimer_timerId(const QTimer* this_ptr) {
  return this_ptr->timerId();
}

void qt_core_c_QTimer_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTimer::trUtf8(s, c, n));
}

void qt_core_c_QTimer_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTimer::tr(s, c, n));
}

