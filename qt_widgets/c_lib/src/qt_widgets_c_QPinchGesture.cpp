#include "qt_widgets_c_QPinchGesture.h"

QPinchGesture* qt_widgets_c_QPinchGesture_G_dynamic_cast_QPinchGesture_ptr(QGesture* ptr) {
  return dynamic_cast<QPinchGesture*>(ptr);
}

QGesture* qt_widgets_c_QPinchGesture_G_static_cast_QGesture_ptr(QPinchGesture* ptr) {
  return static_cast<QGesture*>(ptr);
}

QObject* qt_widgets_c_QPinchGesture_G_static_cast_QObject_ptr(QPinchGesture* ptr) {
  return static_cast<QObject*>(ptr);
}

QPinchGesture* qt_widgets_c_QPinchGesture_G_static_cast_QPinchGesture_ptr_QGesture(QGesture* ptr) {
  return static_cast<QPinchGesture*>(ptr);
}

QPinchGesture* qt_widgets_c_QPinchGesture_G_static_cast_QPinchGesture_ptr_QObject(QObject* ptr) {
  return static_cast<QPinchGesture*>(ptr);
}

void qt_widgets_c_QPinchGesture_centerPoint_to_output(const QPinchGesture* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->centerPoint());
}

unsigned int qt_widgets_c_QPinchGesture_changeFlags(const QPinchGesture* this_ptr) {
  return uint(this_ptr->changeFlags());
}

void qt_widgets_c_QPinchGesture_delete(QPinchGesture* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QPinchGesture_lastCenterPoint_to_output(const QPinchGesture* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->lastCenterPoint());
}

double qt_widgets_c_QPinchGesture_lastRotationAngle(const QPinchGesture* this_ptr) {
  return this_ptr->lastRotationAngle();
}

double qt_widgets_c_QPinchGesture_lastScaleFactor(const QPinchGesture* this_ptr) {
  return this_ptr->lastScaleFactor();
}

const QMetaObject* qt_widgets_c_QPinchGesture_metaObject(const QPinchGesture* this_ptr) {
  return this_ptr->metaObject();
}

QPinchGesture* qt_widgets_c_QPinchGesture_new_no_args() {
  return new QPinchGesture();
}

QPinchGesture* qt_widgets_c_QPinchGesture_new_parent(QObject* parent) {
  return new QPinchGesture(parent);
}

int qt_widgets_c_QPinchGesture_qt_metacall(QPinchGesture* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QPinchGesture_qt_metacast(QPinchGesture* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

double qt_widgets_c_QPinchGesture_rotationAngle(const QPinchGesture* this_ptr) {
  return this_ptr->rotationAngle();
}

double qt_widgets_c_QPinchGesture_scaleFactor(const QPinchGesture* this_ptr) {
  return this_ptr->scaleFactor();
}

void qt_widgets_c_QPinchGesture_setCenterPoint(QPinchGesture* this_ptr, const QPointF* value) {
  this_ptr->setCenterPoint(*value);
}

void qt_widgets_c_QPinchGesture_setChangeFlags(QPinchGesture* this_ptr, unsigned int value) {
  this_ptr->setChangeFlags(QFlags< QPinchGesture::ChangeFlag >(value));
}

void qt_widgets_c_QPinchGesture_setLastCenterPoint(QPinchGesture* this_ptr, const QPointF* value) {
  this_ptr->setLastCenterPoint(*value);
}

void qt_widgets_c_QPinchGesture_setLastRotationAngle(QPinchGesture* this_ptr, double value) {
  this_ptr->setLastRotationAngle(value);
}

void qt_widgets_c_QPinchGesture_setLastScaleFactor(QPinchGesture* this_ptr, double value) {
  this_ptr->setLastScaleFactor(value);
}

void qt_widgets_c_QPinchGesture_setRotationAngle(QPinchGesture* this_ptr, double value) {
  this_ptr->setRotationAngle(value);
}

void qt_widgets_c_QPinchGesture_setScaleFactor(QPinchGesture* this_ptr, double value) {
  this_ptr->setScaleFactor(value);
}

void qt_widgets_c_QPinchGesture_setStartCenterPoint(QPinchGesture* this_ptr, const QPointF* value) {
  this_ptr->setStartCenterPoint(*value);
}

void qt_widgets_c_QPinchGesture_setTotalChangeFlags(QPinchGesture* this_ptr, unsigned int value) {
  this_ptr->setTotalChangeFlags(QFlags< QPinchGesture::ChangeFlag >(value));
}

void qt_widgets_c_QPinchGesture_setTotalRotationAngle(QPinchGesture* this_ptr, double value) {
  this_ptr->setTotalRotationAngle(value);
}

void qt_widgets_c_QPinchGesture_setTotalScaleFactor(QPinchGesture* this_ptr, double value) {
  this_ptr->setTotalScaleFactor(value);
}

void qt_widgets_c_QPinchGesture_startCenterPoint_to_output(const QPinchGesture* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->startCenterPoint());
}

unsigned int qt_widgets_c_QPinchGesture_totalChangeFlags(const QPinchGesture* this_ptr) {
  return uint(this_ptr->totalChangeFlags());
}

double qt_widgets_c_QPinchGesture_totalRotationAngle(const QPinchGesture* this_ptr) {
  return this_ptr->totalRotationAngle();
}

double qt_widgets_c_QPinchGesture_totalScaleFactor(const QPinchGesture* this_ptr) {
  return this_ptr->totalScaleFactor();
}

void qt_widgets_c_QPinchGesture_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPinchGesture::trUtf8(s, c, n));
}

void qt_widgets_c_QPinchGesture_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPinchGesture::tr(s, c, n));
}

