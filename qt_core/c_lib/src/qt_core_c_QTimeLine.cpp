#include "qt_core_c_QTimeLine.h"

QTimeLine* qt_core_c_QTimeLine_G_dynamic_cast_QTimeLine_ptr(QObject* ptr) {
  return dynamic_cast<QTimeLine*>(ptr);
}

QObject* qt_core_c_QTimeLine_G_static_cast_QObject_ptr(QTimeLine* ptr) {
  return static_cast<QObject*>(ptr);
}

QTimeLine* qt_core_c_QTimeLine_G_static_cast_QTimeLine_ptr(QObject* ptr) {
  return static_cast<QTimeLine*>(ptr);
}

int qt_core_c_QTimeLine_currentFrame(const QTimeLine* this_ptr) {
  return this_ptr->currentFrame();
}

int qt_core_c_QTimeLine_currentTime(const QTimeLine* this_ptr) {
  return this_ptr->currentTime();
}

double qt_core_c_QTimeLine_currentValue(const QTimeLine* this_ptr) {
  return this_ptr->currentValue();
}

QTimeLine::CurveShape qt_core_c_QTimeLine_curveShape(const QTimeLine* this_ptr) {
  return this_ptr->curveShape();
}

void qt_core_c_QTimeLine_delete(QTimeLine* this_ptr) {
  delete this_ptr;
}

QTimeLine::Direction qt_core_c_QTimeLine_direction(const QTimeLine* this_ptr) {
  return this_ptr->direction();
}

int qt_core_c_QTimeLine_duration(const QTimeLine* this_ptr) {
  return this_ptr->duration();
}

void qt_core_c_QTimeLine_easingCurve_to_output(const QTimeLine* this_ptr, QEasingCurve* output) {
  new(output) QEasingCurve(this_ptr->easingCurve());
}

int qt_core_c_QTimeLine_endFrame(const QTimeLine* this_ptr) {
  return this_ptr->endFrame();
}

int qt_core_c_QTimeLine_frameForTime(const QTimeLine* this_ptr, int msec) {
  return this_ptr->frameForTime(msec);
}

int qt_core_c_QTimeLine_loopCount(const QTimeLine* this_ptr) {
  return this_ptr->loopCount();
}

const QMetaObject* qt_core_c_QTimeLine_metaObject(const QTimeLine* this_ptr) {
  return this_ptr->metaObject();
}

QTimeLine* qt_core_c_QTimeLine_new_duration(int duration) {
  return new QTimeLine(duration);
}

QTimeLine* qt_core_c_QTimeLine_new_duration_parent(int duration, QObject* parent) {
  return new QTimeLine(duration, parent);
}

QTimeLine* qt_core_c_QTimeLine_new_no_args() {
  return new QTimeLine();
}

void qt_core_c_QTimeLine_resume(QTimeLine* this_ptr) {
  this_ptr->resume();
}

void qt_core_c_QTimeLine_setCurrentTime(QTimeLine* this_ptr, int msec) {
  this_ptr->setCurrentTime(msec);
}

void qt_core_c_QTimeLine_setCurveShape(QTimeLine* this_ptr, QTimeLine::CurveShape shape) {
  this_ptr->setCurveShape(shape);
}

void qt_core_c_QTimeLine_setDirection(QTimeLine* this_ptr, QTimeLine::Direction direction) {
  this_ptr->setDirection(direction);
}

void qt_core_c_QTimeLine_setDuration(QTimeLine* this_ptr, int duration) {
  this_ptr->setDuration(duration);
}

void qt_core_c_QTimeLine_setEasingCurve(QTimeLine* this_ptr, const QEasingCurve* curve) {
  this_ptr->setEasingCurve(*curve);
}

void qt_core_c_QTimeLine_setEndFrame(QTimeLine* this_ptr, int frame) {
  this_ptr->setEndFrame(frame);
}

void qt_core_c_QTimeLine_setFrameRange(QTimeLine* this_ptr, int startFrame, int endFrame) {
  this_ptr->setFrameRange(startFrame, endFrame);
}

void qt_core_c_QTimeLine_setLoopCount(QTimeLine* this_ptr, int count) {
  this_ptr->setLoopCount(count);
}

void qt_core_c_QTimeLine_setPaused(QTimeLine* this_ptr, bool paused) {
  this_ptr->setPaused(paused);
}

void qt_core_c_QTimeLine_setStartFrame(QTimeLine* this_ptr, int frame) {
  this_ptr->setStartFrame(frame);
}

void qt_core_c_QTimeLine_setUpdateInterval(QTimeLine* this_ptr, int interval) {
  this_ptr->setUpdateInterval(interval);
}

void qt_core_c_QTimeLine_start(QTimeLine* this_ptr) {
  this_ptr->start();
}

int qt_core_c_QTimeLine_startFrame(const QTimeLine* this_ptr) {
  return this_ptr->startFrame();
}

QTimeLine::State qt_core_c_QTimeLine_state(const QTimeLine* this_ptr) {
  return this_ptr->state();
}

void qt_core_c_QTimeLine_stop(QTimeLine* this_ptr) {
  this_ptr->stop();
}

void qt_core_c_QTimeLine_toggleDirection(QTimeLine* this_ptr) {
  this_ptr->toggleDirection();
}

void qt_core_c_QTimeLine_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTimeLine::trUtf8(s, c, n));
}

void qt_core_c_QTimeLine_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTimeLine::tr(s, c, n));
}

int qt_core_c_QTimeLine_updateInterval(const QTimeLine* this_ptr) {
  return this_ptr->updateInterval();
}

double qt_core_c_QTimeLine_valueForTime(const QTimeLine* this_ptr, int msec) {
  return this_ptr->valueForTime(msec);
}

