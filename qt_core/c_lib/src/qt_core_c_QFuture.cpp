#include "qt_core_c_QFuture.h"

void qt_core_c_QFuture_void_cancel(QFuture< void >* this_ptr) {
  this_ptr->cancel();
}

void qt_core_c_QFuture_void_constructor(QFuture< void >* output) {
  new(output) QFuture< void >();
}

void qt_core_c_QFuture_void_destructor(QFuture< void >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QFuture_void_isCanceled(const QFuture< void >* this_ptr) {
  return this_ptr->isCanceled();
}

bool qt_core_c_QFuture_void_isFinished(const QFuture< void >* this_ptr) {
  return this_ptr->isFinished();
}

bool qt_core_c_QFuture_void_isPaused(const QFuture< void >* this_ptr) {
  return this_ptr->isPaused();
}

bool qt_core_c_QFuture_void_isRunning(const QFuture< void >* this_ptr) {
  return this_ptr->isRunning();
}

bool qt_core_c_QFuture_void_isStarted(const QFuture< void >* this_ptr) {
  return this_ptr->isStarted();
}

bool qt_core_c_QFuture_void_operator_eq(const QFuture< void >* this_ptr, const QFuture< void >* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QFuture_void_operator_neq(const QFuture< void >* this_ptr, const QFuture< void >* other) {
  return this_ptr->operator!=(*other);
}

void qt_core_c_QFuture_void_pause(QFuture< void >* this_ptr) {
  this_ptr->pause();
}

int qt_core_c_QFuture_void_progressMaximum(const QFuture< void >* this_ptr) {
  return this_ptr->progressMaximum();
}

int qt_core_c_QFuture_void_progressMinimum(const QFuture< void >* this_ptr) {
  return this_ptr->progressMinimum();
}

void qt_core_c_QFuture_void_progressText_to_output(const QFuture< void >* this_ptr, QString* output) {
  new(output) QString(this_ptr->progressText());
}

int qt_core_c_QFuture_void_progressValue(const QFuture< void >* this_ptr) {
  return this_ptr->progressValue();
}

int qt_core_c_QFuture_void_resultCount(const QFuture< void >* this_ptr) {
  return this_ptr->resultCount();
}

void qt_core_c_QFuture_void_resume(QFuture< void >* this_ptr) {
  this_ptr->resume();
}

void qt_core_c_QFuture_void_setPaused(QFuture< void >* this_ptr, bool paused) {
  this_ptr->setPaused(paused);
}

void qt_core_c_QFuture_void_togglePaused(QFuture< void >* this_ptr) {
  this_ptr->togglePaused();
}

void qt_core_c_QFuture_void_waitForFinished(QFuture< void >* this_ptr) {
  this_ptr->waitForFinished();
}

