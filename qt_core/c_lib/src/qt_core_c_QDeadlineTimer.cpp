#include "qt_core_c_QDeadlineTimer.h"

void qt_core_c_QDeadlineTimer_G_swap(QDeadlineTimer* value1, QDeadlineTimer* value2) {
  swap(*value1, *value2);
}

void qt_core_c_QDeadlineTimer_addNSecs_to_output(const QDeadlineTimer* dt, qint64 nsecs, QDeadlineTimer* output) {
  new(output) QDeadlineTimer(QDeadlineTimer::addNSecs(*dt, nsecs));
}

void qt_core_c_QDeadlineTimer_constructor_arg1(QDeadlineTimer::ForeverConstant arg1, QDeadlineTimer* output) {
  new(output) QDeadlineTimer(arg1);
}

void qt_core_c_QDeadlineTimer_constructor_arg1_type_(QDeadlineTimer::ForeverConstant arg1, const Qt::TimerType* type_, QDeadlineTimer* output) {
  new(output) QDeadlineTimer(arg1, *type_);
}

void qt_core_c_QDeadlineTimer_constructor_msecs(qint64 msecs, QDeadlineTimer* output) {
  new(output) QDeadlineTimer(msecs);
}

void qt_core_c_QDeadlineTimer_constructor_msecs_type(qint64 msecs, const Qt::TimerType* type, QDeadlineTimer* output) {
  new(output) QDeadlineTimer(msecs, *type);
}

void qt_core_c_QDeadlineTimer_constructor_no_args(QDeadlineTimer* output) {
  new(output) QDeadlineTimer();
}

void qt_core_c_QDeadlineTimer_constructor_type_(const Qt::TimerType* type_, QDeadlineTimer* output) {
  new(output) QDeadlineTimer(*type_);
}

void qt_core_c_QDeadlineTimer_current_to_output_no_args(QDeadlineTimer* output) {
  new(output) QDeadlineTimer(QDeadlineTimer::current());
}

void qt_core_c_QDeadlineTimer_current_to_output_timerType(const Qt::TimerType* timerType, QDeadlineTimer* output) {
  new(output) QDeadlineTimer(QDeadlineTimer::current(*timerType));
}

qint64 qt_core_c_QDeadlineTimer_deadline(const QDeadlineTimer* this_ptr) {
  return this_ptr->deadline();
}

qint64 qt_core_c_QDeadlineTimer_deadlineNSecs(const QDeadlineTimer* this_ptr) {
  return this_ptr->deadlineNSecs();
}

void qt_core_c_QDeadlineTimer_destructor(QDeadlineTimer* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QDeadlineTimer_hasExpired(const QDeadlineTimer* this_ptr) {
  return this_ptr->hasExpired();
}

bool qt_core_c_QDeadlineTimer_isForever(const QDeadlineTimer* this_ptr) {
  return this_ptr->isForever();
}

QDeadlineTimer* qt_core_c_QDeadlineTimer_operator_add_assign(QDeadlineTimer* this_ptr, qint64 msecs) {
  return &this_ptr->operator+=(msecs);
}

QDeadlineTimer* qt_core_c_QDeadlineTimer_operator_sub_assign(QDeadlineTimer* this_ptr, qint64 msecs) {
  return &this_ptr->operator-=(msecs);
}

qint64 qt_core_c_QDeadlineTimer_remainingTime(const QDeadlineTimer* this_ptr) {
  return this_ptr->remainingTime();
}

qint64 qt_core_c_QDeadlineTimer_remainingTimeNSecs(const QDeadlineTimer* this_ptr) {
  return this_ptr->remainingTimeNSecs();
}

void qt_core_c_QDeadlineTimer_setDeadline_msecs(QDeadlineTimer* this_ptr, qint64 msecs) {
  this_ptr->setDeadline(msecs);
}

void qt_core_c_QDeadlineTimer_setDeadline_msecs_timerType(QDeadlineTimer* this_ptr, qint64 msecs, const Qt::TimerType* timerType) {
  this_ptr->setDeadline(msecs, *timerType);
}

void qt_core_c_QDeadlineTimer_setPreciseDeadline_secs(QDeadlineTimer* this_ptr, qint64 secs) {
  this_ptr->setPreciseDeadline(secs);
}

void qt_core_c_QDeadlineTimer_setPreciseDeadline_secs_nsecs(QDeadlineTimer* this_ptr, qint64 secs, qint64 nsecs) {
  this_ptr->setPreciseDeadline(secs, nsecs);
}

void qt_core_c_QDeadlineTimer_setPreciseDeadline_secs_nsecs_type(QDeadlineTimer* this_ptr, qint64 secs, qint64 nsecs, const Qt::TimerType* type) {
  this_ptr->setPreciseDeadline(secs, nsecs, *type);
}

void qt_core_c_QDeadlineTimer_setPreciseRemainingTime_secs(QDeadlineTimer* this_ptr, qint64 secs) {
  this_ptr->setPreciseRemainingTime(secs);
}

void qt_core_c_QDeadlineTimer_setPreciseRemainingTime_secs_nsecs(QDeadlineTimer* this_ptr, qint64 secs, qint64 nsecs) {
  this_ptr->setPreciseRemainingTime(secs, nsecs);
}

void qt_core_c_QDeadlineTimer_setPreciseRemainingTime_secs_nsecs_type(QDeadlineTimer* this_ptr, qint64 secs, qint64 nsecs, const Qt::TimerType* type) {
  this_ptr->setPreciseRemainingTime(secs, nsecs, *type);
}

void qt_core_c_QDeadlineTimer_setRemainingTime_msecs(QDeadlineTimer* this_ptr, qint64 msecs) {
  this_ptr->setRemainingTime(msecs);
}

void qt_core_c_QDeadlineTimer_setRemainingTime_msecs_type(QDeadlineTimer* this_ptr, qint64 msecs, const Qt::TimerType* type) {
  this_ptr->setRemainingTime(msecs, *type);
}

void qt_core_c_QDeadlineTimer_setTimerType(QDeadlineTimer* this_ptr, const Qt::TimerType* type) {
  this_ptr->setTimerType(*type);
}

void qt_core_c_QDeadlineTimer_swap(QDeadlineTimer* this_ptr, QDeadlineTimer* other) {
  this_ptr->swap(*other);
}

