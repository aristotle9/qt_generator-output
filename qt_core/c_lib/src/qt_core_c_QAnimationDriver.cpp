#include "qt_core_c_QAnimationDriver.h"

QAnimationDriver* qt_core_c_QAnimationDriver_G_dynamic_cast_QAnimationDriver_ptr(QObject* ptr) {
  return dynamic_cast<QAnimationDriver*>(ptr);
}

QAnimationDriver* qt_core_c_QAnimationDriver_G_static_cast_QAnimationDriver_ptr(QObject* ptr) {
  return static_cast<QAnimationDriver*>(ptr);
}

QObject* qt_core_c_QAnimationDriver_G_static_cast_QObject_ptr(QAnimationDriver* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_core_c_QAnimationDriver_advance(QAnimationDriver* this_ptr) {
  this_ptr->advance();
}

void qt_core_c_QAnimationDriver_delete(QAnimationDriver* this_ptr) {
  delete this_ptr;
}

qint64 qt_core_c_QAnimationDriver_elapsed(const QAnimationDriver* this_ptr) {
  return this_ptr->elapsed();
}

void qt_core_c_QAnimationDriver_install(QAnimationDriver* this_ptr) {
  this_ptr->install();
}

bool qt_core_c_QAnimationDriver_isRunning(const QAnimationDriver* this_ptr) {
  return this_ptr->isRunning();
}

const QMetaObject* qt_core_c_QAnimationDriver_metaObject(const QAnimationDriver* this_ptr) {
  return this_ptr->metaObject();
}

QAnimationDriver* qt_core_c_QAnimationDriver_new_no_args() {
  return new QAnimationDriver();
}

QAnimationDriver* qt_core_c_QAnimationDriver_new_parent(QObject* parent) {
  return new QAnimationDriver(parent);
}

void qt_core_c_QAnimationDriver_setStartTime(QAnimationDriver* this_ptr, qint64 startTime) {
  this_ptr->setStartTime(startTime);
}

qint64 qt_core_c_QAnimationDriver_startTime(const QAnimationDriver* this_ptr) {
  return this_ptr->startTime();
}

void qt_core_c_QAnimationDriver_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAnimationDriver::trUtf8(s, c, n));
}

void qt_core_c_QAnimationDriver_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAnimationDriver::tr(s, c, n));
}

void qt_core_c_QAnimationDriver_uninstall(QAnimationDriver* this_ptr) {
  this_ptr->uninstall();
}

