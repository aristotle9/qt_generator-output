#include "qt_core_c_QEventLoop.h"

QEventLoop* qt_core_c_QEventLoop_G_dynamic_cast_QEventLoop_ptr(QObject* ptr) {
  return dynamic_cast<QEventLoop*>(ptr);
}

QEventLoop* qt_core_c_QEventLoop_G_static_cast_QEventLoop_ptr(QObject* ptr) {
  return static_cast<QEventLoop*>(ptr);
}

QObject* qt_core_c_QEventLoop_G_static_cast_QObject_ptr(QEventLoop* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_core_c_QEventLoop_delete(QEventLoop* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QEventLoop_event(QEventLoop* this_ptr, QEvent* event) {
  return this_ptr->event(event);
}

int qt_core_c_QEventLoop_exec_flags(QEventLoop* this_ptr, unsigned int flags) {
  return this_ptr->exec(QFlags< QEventLoop::ProcessEventsFlag >(flags));
}

int qt_core_c_QEventLoop_exec_no_args(QEventLoop* this_ptr) {
  return this_ptr->exec();
}

void qt_core_c_QEventLoop_exit_no_args(QEventLoop* this_ptr) {
  this_ptr->exit();
}

void qt_core_c_QEventLoop_exit_returnCode(QEventLoop* this_ptr, int returnCode) {
  this_ptr->exit(returnCode);
}

bool qt_core_c_QEventLoop_isRunning(const QEventLoop* this_ptr) {
  return this_ptr->isRunning();
}

const QMetaObject* qt_core_c_QEventLoop_metaObject(const QEventLoop* this_ptr) {
  return this_ptr->metaObject();
}

QEventLoop* qt_core_c_QEventLoop_new_no_args() {
  return new QEventLoop();
}

QEventLoop* qt_core_c_QEventLoop_new_parent(QObject* parent) {
  return new QEventLoop(parent);
}

bool qt_core_c_QEventLoop_processEvents_flags(QEventLoop* this_ptr, unsigned int flags) {
  return this_ptr->processEvents(QFlags< QEventLoop::ProcessEventsFlag >(flags));
}

void qt_core_c_QEventLoop_processEvents_flags_maximumTime(QEventLoop* this_ptr, unsigned int flags, int maximumTime) {
  this_ptr->processEvents(QFlags< QEventLoop::ProcessEventsFlag >(flags), maximumTime);
}

bool qt_core_c_QEventLoop_processEvents_no_args(QEventLoop* this_ptr) {
  return this_ptr->processEvents();
}

void qt_core_c_QEventLoop_quit(QEventLoop* this_ptr) {
  this_ptr->quit();
}

void qt_core_c_QEventLoop_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QEventLoop::trUtf8(s, c, n));
}

void qt_core_c_QEventLoop_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QEventLoop::tr(s, c, n));
}

void qt_core_c_QEventLoop_wakeUp(QEventLoop* this_ptr) {
  this_ptr->wakeUp();
}

