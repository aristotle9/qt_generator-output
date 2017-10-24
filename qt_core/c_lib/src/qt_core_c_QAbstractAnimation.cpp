#include "qt_core_c_QAbstractAnimation.h"

QAbstractAnimation* qt_core_c_QAbstractAnimation_G_dynamic_cast_QAbstractAnimation_ptr(QObject* ptr) {
  return dynamic_cast<QAbstractAnimation*>(ptr);
}

QAbstractAnimation* qt_core_c_QAbstractAnimation_G_static_cast_QAbstractAnimation_ptr(QObject* ptr) {
  return static_cast<QAbstractAnimation*>(ptr);
}

QObject* qt_core_c_QAbstractAnimation_G_static_cast_QObject_ptr(QAbstractAnimation* ptr) {
  return static_cast<QObject*>(ptr);
}

int qt_core_c_QAbstractAnimation_currentLoop(const QAbstractAnimation* this_ptr) {
  return this_ptr->currentLoop();
}

int qt_core_c_QAbstractAnimation_currentLoopTime(const QAbstractAnimation* this_ptr) {
  return this_ptr->currentLoopTime();
}

int qt_core_c_QAbstractAnimation_currentTime(const QAbstractAnimation* this_ptr) {
  return this_ptr->currentTime();
}

void qt_core_c_QAbstractAnimation_delete(QAbstractAnimation* this_ptr) {
  delete this_ptr;
}

QAbstractAnimation::Direction qt_core_c_QAbstractAnimation_direction(const QAbstractAnimation* this_ptr) {
  return this_ptr->direction();
}

int qt_core_c_QAbstractAnimation_duration(const QAbstractAnimation* this_ptr) {
  return this_ptr->duration();
}

QAnimationGroup* qt_core_c_QAbstractAnimation_group(const QAbstractAnimation* this_ptr) {
  return this_ptr->group();
}

int qt_core_c_QAbstractAnimation_loopCount(const QAbstractAnimation* this_ptr) {
  return this_ptr->loopCount();
}

const QMetaObject* qt_core_c_QAbstractAnimation_metaObject(const QAbstractAnimation* this_ptr) {
  return this_ptr->metaObject();
}

void qt_core_c_QAbstractAnimation_pause(QAbstractAnimation* this_ptr) {
  this_ptr->pause();
}

void qt_core_c_QAbstractAnimation_resume(QAbstractAnimation* this_ptr) {
  this_ptr->resume();
}

void qt_core_c_QAbstractAnimation_setCurrentTime(QAbstractAnimation* this_ptr, int msecs) {
  this_ptr->setCurrentTime(msecs);
}

void qt_core_c_QAbstractAnimation_setDirection(QAbstractAnimation* this_ptr, QAbstractAnimation::Direction direction) {
  this_ptr->setDirection(direction);
}

void qt_core_c_QAbstractAnimation_setLoopCount(QAbstractAnimation* this_ptr, int loopCount) {
  this_ptr->setLoopCount(loopCount);
}

void qt_core_c_QAbstractAnimation_setPaused(QAbstractAnimation* this_ptr, bool arg1) {
  this_ptr->setPaused(arg1);
}

void qt_core_c_QAbstractAnimation_start_no_args(QAbstractAnimation* this_ptr) {
  this_ptr->start();
}

void qt_core_c_QAbstractAnimation_start_policy(QAbstractAnimation* this_ptr, const QAbstractAnimation::DeletionPolicy* policy) {
  this_ptr->start(*policy);
}

QAbstractAnimation::State qt_core_c_QAbstractAnimation_state(const QAbstractAnimation* this_ptr) {
  return this_ptr->state();
}

void qt_core_c_QAbstractAnimation_stop(QAbstractAnimation* this_ptr) {
  this_ptr->stop();
}

int qt_core_c_QAbstractAnimation_totalDuration(const QAbstractAnimation* this_ptr) {
  return this_ptr->totalDuration();
}

void qt_core_c_QAbstractAnimation_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractAnimation::trUtf8(s, c, n));
}

void qt_core_c_QAbstractAnimation_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractAnimation::tr(s, c, n));
}

