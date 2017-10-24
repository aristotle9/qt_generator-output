#include "qt_core_c_QEasingCurve.h"

void qt_core_c_QEasingCurve_G_swap(QEasingCurve* value1, QEasingCurve* value2) {
  swap(*value1, *value2);
}

void qt_core_c_QEasingCurve_addCubicBezierSegment(QEasingCurve* this_ptr, const QPointF* c1, const QPointF* c2, const QPointF* endPoint) {
  this_ptr->addCubicBezierSegment(*c1, *c2, *endPoint);
}

void qt_core_c_QEasingCurve_addTCBSegment(QEasingCurve* this_ptr, const QPointF* nextPoint, double t, double c, double b) {
  this_ptr->addTCBSegment(*nextPoint, t, c, b);
}

double qt_core_c_QEasingCurve_amplitude(const QEasingCurve* this_ptr) {
  return this_ptr->amplitude();
}

void qt_core_c_QEasingCurve_constructor_no_args(QEasingCurve* output) {
  new(output) QEasingCurve();
}

void qt_core_c_QEasingCurve_constructor_other(const QEasingCurve* other, QEasingCurve* output) {
  new(output) QEasingCurve(*other);
}

void qt_core_c_QEasingCurve_constructor_type(QEasingCurve::Type type, QEasingCurve* output) {
  new(output) QEasingCurve(type);
}

double (*qt_core_c_QEasingCurve_customType(const QEasingCurve* this_ptr))(double) {
  return this_ptr->customType();
}

void qt_core_c_QEasingCurve_destructor(QEasingCurve* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QEasingCurve* qt_core_c_QEasingCurve_operator_assign(QEasingCurve* this_ptr, const QEasingCurve* other) {
  return &this_ptr->operator=(*other);
}

bool qt_core_c_QEasingCurve_operator_eq(const QEasingCurve* this_ptr, const QEasingCurve* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QEasingCurve_operator_neq(const QEasingCurve* this_ptr, const QEasingCurve* other) {
  return this_ptr->operator!=(*other);
}

double qt_core_c_QEasingCurve_overshoot(const QEasingCurve* this_ptr) {
  return this_ptr->overshoot();
}

double qt_core_c_QEasingCurve_period(const QEasingCurve* this_ptr) {
  return this_ptr->period();
}

void qt_core_c_QEasingCurve_setAmplitude(QEasingCurve* this_ptr, double amplitude) {
  this_ptr->setAmplitude(amplitude);
}

void qt_core_c_QEasingCurve_setCustomType(QEasingCurve* this_ptr, double (*func)(double)) {
  this_ptr->setCustomType(func);
}

void qt_core_c_QEasingCurve_setOvershoot(QEasingCurve* this_ptr, double overshoot) {
  this_ptr->setOvershoot(overshoot);
}

void qt_core_c_QEasingCurve_setPeriod(QEasingCurve* this_ptr, double period) {
  this_ptr->setPeriod(period);
}

void qt_core_c_QEasingCurve_setType(QEasingCurve* this_ptr, QEasingCurve::Type type) {
  this_ptr->setType(type);
}

void qt_core_c_QEasingCurve_swap(QEasingCurve* this_ptr, QEasingCurve* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QEasingCurve_toCubicSpline_to_output(const QEasingCurve* this_ptr, QVector< QPointF >* output) {
  new(output) QVector< QPointF >(this_ptr->toCubicSpline());
}

QEasingCurve::Type qt_core_c_QEasingCurve_type(const QEasingCurve* this_ptr) {
  return this_ptr->type();
}

double qt_core_c_QEasingCurve_valueForProgress(const QEasingCurve* this_ptr, double progress) {
  return this_ptr->valueForProgress(progress);
}

