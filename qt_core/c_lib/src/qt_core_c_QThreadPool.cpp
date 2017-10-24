#include "qt_core_c_QThreadPool.h"

QThreadPool* qt_core_c_QThreadPool_G_dynamic_cast_QThreadPool_ptr(QObject* ptr) {
  return dynamic_cast<QThreadPool*>(ptr);
}

QObject* qt_core_c_QThreadPool_G_static_cast_QObject_ptr(QThreadPool* ptr) {
  return static_cast<QObject*>(ptr);
}

QThreadPool* qt_core_c_QThreadPool_G_static_cast_QThreadPool_ptr(QObject* ptr) {
  return static_cast<QThreadPool*>(ptr);
}

int qt_core_c_QThreadPool_activeThreadCount(const QThreadPool* this_ptr) {
  return this_ptr->activeThreadCount();
}

void qt_core_c_QThreadPool_cancel(QThreadPool* this_ptr, QRunnable* runnable) {
  this_ptr->cancel(runnable);
}

void qt_core_c_QThreadPool_clear(QThreadPool* this_ptr) {
  this_ptr->clear();
}

void qt_core_c_QThreadPool_delete(QThreadPool* this_ptr) {
  delete this_ptr;
}

int qt_core_c_QThreadPool_expiryTimeout(const QThreadPool* this_ptr) {
  return this_ptr->expiryTimeout();
}

QThreadPool* qt_core_c_QThreadPool_globalInstance() {
  return QThreadPool::globalInstance();
}

int qt_core_c_QThreadPool_maxThreadCount(const QThreadPool* this_ptr) {
  return this_ptr->maxThreadCount();
}

const QMetaObject* qt_core_c_QThreadPool_metaObject(const QThreadPool* this_ptr) {
  return this_ptr->metaObject();
}

QThreadPool* qt_core_c_QThreadPool_new_no_args() {
  return new QThreadPool();
}

QThreadPool* qt_core_c_QThreadPool_new_parent(QObject* parent) {
  return new QThreadPool(parent);
}

void qt_core_c_QThreadPool_releaseThread(QThreadPool* this_ptr) {
  this_ptr->releaseThread();
}

void qt_core_c_QThreadPool_reserveThread(QThreadPool* this_ptr) {
  this_ptr->reserveThread();
}

void qt_core_c_QThreadPool_setExpiryTimeout(QThreadPool* this_ptr, int expiryTimeout) {
  this_ptr->setExpiryTimeout(expiryTimeout);
}

void qt_core_c_QThreadPool_setMaxThreadCount(QThreadPool* this_ptr, int maxThreadCount) {
  this_ptr->setMaxThreadCount(maxThreadCount);
}

void qt_core_c_QThreadPool_start_runnable(QThreadPool* this_ptr, QRunnable* runnable) {
  this_ptr->start(runnable);
}

void qt_core_c_QThreadPool_start_runnable_priority(QThreadPool* this_ptr, QRunnable* runnable, int priority) {
  this_ptr->start(runnable, priority);
}

void qt_core_c_QThreadPool_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QThreadPool::trUtf8(s, c, n));
}

void qt_core_c_QThreadPool_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QThreadPool::tr(s, c, n));
}

bool qt_core_c_QThreadPool_tryStart(QThreadPool* this_ptr, QRunnable* runnable) {
  return this_ptr->tryStart(runnable);
}

bool qt_core_c_QThreadPool_tryTake(QThreadPool* this_ptr, QRunnable* runnable) {
  return this_ptr->tryTake(runnable);
}

bool qt_core_c_QThreadPool_waitForDone_msecs(QThreadPool* this_ptr, int msecs) {
  return this_ptr->waitForDone(msecs);
}

bool qt_core_c_QThreadPool_waitForDone_no_args(QThreadPool* this_ptr) {
  return this_ptr->waitForDone();
}

