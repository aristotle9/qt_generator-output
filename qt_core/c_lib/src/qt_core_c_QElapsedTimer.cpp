#include "qt_core_c_QElapsedTimer.h"

QElapsedTimer::ClockType qt_core_c_QElapsedTimer_clockType() {
  return QElapsedTimer::clockType();
}

void qt_core_c_QElapsedTimer_constructor(QElapsedTimer* output) {
  new(output) QElapsedTimer();
}

void qt_core_c_QElapsedTimer_destructor(QElapsedTimer* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

qint64 qt_core_c_QElapsedTimer_elapsed(const QElapsedTimer* this_ptr) {
  return this_ptr->elapsed();
}

bool qt_core_c_QElapsedTimer_hasExpired(const QElapsedTimer* this_ptr, qint64 timeout) {
  return this_ptr->hasExpired(timeout);
}

void qt_core_c_QElapsedTimer_invalidate(QElapsedTimer* this_ptr) {
  this_ptr->invalidate();
}

bool qt_core_c_QElapsedTimer_isMonotonic() {
  return QElapsedTimer::isMonotonic();
}

bool qt_core_c_QElapsedTimer_isValid(const QElapsedTimer* this_ptr) {
  return this_ptr->isValid();
}

qint64 qt_core_c_QElapsedTimer_msecsSinceReference(const QElapsedTimer* this_ptr) {
  return this_ptr->msecsSinceReference();
}

qint64 qt_core_c_QElapsedTimer_msecsTo(const QElapsedTimer* this_ptr, const QElapsedTimer* other) {
  return this_ptr->msecsTo(*other);
}

qint64 qt_core_c_QElapsedTimer_nsecsElapsed(const QElapsedTimer* this_ptr) {
  return this_ptr->nsecsElapsed();
}

bool qt_core_c_QElapsedTimer_operator_eq(const QElapsedTimer* this_ptr, const QElapsedTimer* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QElapsedTimer_operator_neq(const QElapsedTimer* this_ptr, const QElapsedTimer* other) {
  return this_ptr->operator!=(*other);
}

qint64 qt_core_c_QElapsedTimer_restart(QElapsedTimer* this_ptr) {
  return this_ptr->restart();
}

qint64 qt_core_c_QElapsedTimer_secsTo(const QElapsedTimer* this_ptr, const QElapsedTimer* other) {
  return this_ptr->secsTo(*other);
}

void qt_core_c_QElapsedTimer_start(QElapsedTimer* this_ptr) {
  this_ptr->start();
}

