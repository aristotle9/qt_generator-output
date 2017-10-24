#include "qt_core_c_QBasicTimer.h"

void qt_core_c_QBasicTimer_delete(QBasicTimer* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QBasicTimer_isActive(const QBasicTimer* this_ptr) {
  return this_ptr->isActive();
}

QBasicTimer* qt_core_c_QBasicTimer_new() {
  return new QBasicTimer();
}

void qt_core_c_QBasicTimer_start_msec_obj(QBasicTimer* this_ptr, int msec, QObject* obj) {
  this_ptr->start(msec, obj);
}

void qt_core_c_QBasicTimer_start_msec_timerType_obj(QBasicTimer* this_ptr, int msec, const Qt::TimerType* timerType, QObject* obj) {
  this_ptr->start(msec, *timerType, obj);
}

void qt_core_c_QBasicTimer_stop(QBasicTimer* this_ptr) {
  this_ptr->stop();
}

int qt_core_c_QBasicTimer_timerId(const QBasicTimer* this_ptr) {
  return this_ptr->timerId();
}

