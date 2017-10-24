#include "qt_core_c_QThread.h"

QThread* qt_core_c_QThread_G_dynamic_cast_QThread_ptr(QObject* ptr) {
  return dynamic_cast<QThread*>(ptr);
}

QObject* qt_core_c_QThread_G_static_cast_QObject_ptr(QThread* ptr) {
  return static_cast<QObject*>(ptr);
}

QThread* qt_core_c_QThread_G_static_cast_QThread_ptr(QObject* ptr) {
  return static_cast<QThread*>(ptr);
}

QThread* qt_core_c_QThread_currentThread() {
  return QThread::currentThread();
}

void qt_core_c_QThread_delete(QThread* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QThread_event(QThread* this_ptr, QEvent* event) {
  return this_ptr->event(event);
}

QAbstractEventDispatcher* qt_core_c_QThread_eventDispatcher(const QThread* this_ptr) {
  return this_ptr->eventDispatcher();
}

void qt_core_c_QThread_exit_no_args(QThread* this_ptr) {
  this_ptr->exit();
}

void qt_core_c_QThread_exit_retcode(QThread* this_ptr, int retcode) {
  this_ptr->exit(retcode);
}

int qt_core_c_QThread_idealThreadCount() {
  return QThread::idealThreadCount();
}

bool qt_core_c_QThread_isFinished(const QThread* this_ptr) {
  return this_ptr->isFinished();
}

bool qt_core_c_QThread_isInterruptionRequested(const QThread* this_ptr) {
  return this_ptr->isInterruptionRequested();
}

bool qt_core_c_QThread_isRunning(const QThread* this_ptr) {
  return this_ptr->isRunning();
}

int qt_core_c_QThread_loopLevel(const QThread* this_ptr) {
  return this_ptr->loopLevel();
}

const QMetaObject* qt_core_c_QThread_metaObject(const QThread* this_ptr) {
  return this_ptr->metaObject();
}

void qt_core_c_QThread_msleep(unsigned long arg1) {
  QThread::msleep(arg1);
}

QThread* qt_core_c_QThread_new_no_args() {
  return new QThread();
}

QThread* qt_core_c_QThread_new_parent(QObject* parent) {
  return new QThread(parent);
}

QThread::Priority qt_core_c_QThread_priority(const QThread* this_ptr) {
  return this_ptr->priority();
}

void qt_core_c_QThread_quit(QThread* this_ptr) {
  this_ptr->quit();
}

void qt_core_c_QThread_requestInterruption(QThread* this_ptr) {
  this_ptr->requestInterruption();
}

void qt_core_c_QThread_setEventDispatcher(QThread* this_ptr, QAbstractEventDispatcher* eventDispatcher) {
  this_ptr->setEventDispatcher(eventDispatcher);
}

void qt_core_c_QThread_setPriority(QThread* this_ptr, QThread::Priority priority) {
  this_ptr->setPriority(priority);
}

void qt_core_c_QThread_setStackSize(QThread* this_ptr, unsigned int stackSize) {
  this_ptr->setStackSize(stackSize);
}

void qt_core_c_QThread_sleep(unsigned long arg1) {
  QThread::sleep(arg1);
}

unsigned int qt_core_c_QThread_stackSize(const QThread* this_ptr) {
  return this_ptr->stackSize();
}

void qt_core_c_QThread_start_arg1(QThread* this_ptr, QThread::Priority arg1) {
  this_ptr->start(arg1);
}

void qt_core_c_QThread_start_no_args(QThread* this_ptr) {
  this_ptr->start();
}

void qt_core_c_QThread_terminate(QThread* this_ptr) {
  this_ptr->terminate();
}

void qt_core_c_QThread_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QThread::trUtf8(s, c, n));
}

void qt_core_c_QThread_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QThread::tr(s, c, n));
}

void qt_core_c_QThread_usleep(unsigned long arg1) {
  QThread::usleep(arg1);
}

bool qt_core_c_QThread_wait_no_args(QThread* this_ptr) {
  return this_ptr->wait();
}

bool qt_core_c_QThread_wait_time(QThread* this_ptr, unsigned long time) {
  return this_ptr->wait(time);
}

void qt_core_c_QThread_yieldCurrentThread() {
  QThread::yieldCurrentThread();
}

