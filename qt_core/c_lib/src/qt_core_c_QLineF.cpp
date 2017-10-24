#include "qt_core_c_QLineF.h"

double qt_core_c_QLineF_angleTo(const QLineF* this_ptr, const QLineF* l) {
  return this_ptr->angleTo(*l);
}

double qt_core_c_QLineF_angle_l(const QLineF* this_ptr, const QLineF* l) {
  return this_ptr->angle(*l);
}

double qt_core_c_QLineF_angle_no_args(const QLineF* this_ptr) {
  return this_ptr->angle();
}

void qt_core_c_QLineF_center_to_output(const QLineF* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->center());
}

void qt_core_c_QLineF_constructor_line(const QLine* line, QLineF* output) {
  new(output) QLineF(*line);
}

void qt_core_c_QLineF_constructor_no_args(QLineF* output) {
  new(output) QLineF();
}

void qt_core_c_QLineF_constructor_pt1_pt2(const QPointF* pt1, const QPointF* pt2, QLineF* output) {
  new(output) QLineF(*pt1, *pt2);
}

void qt_core_c_QLineF_constructor_x1_y1_x2_y2(double x1, double y1, double x2, double y2, QLineF* output) {
  new(output) QLineF(x1, y1, x2, y2);
}

void qt_core_c_QLineF_destructor(QLineF* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

double qt_core_c_QLineF_dx(const QLineF* this_ptr) {
  return this_ptr->dx();
}

double qt_core_c_QLineF_dy(const QLineF* this_ptr) {
  return this_ptr->dy();
}

void qt_core_c_QLineF_fromPolar_to_output(double length, double angle, QLineF* output) {
  new(output) QLineF(QLineF::fromPolar(length, angle));
}

QLineF::IntersectType qt_core_c_QLineF_intersect(const QLineF* this_ptr, const QLineF* l, QPointF* intersectionPoint) {
  return this_ptr->intersect(*l, intersectionPoint);
}

bool qt_core_c_QLineF_isNull(const QLineF* this_ptr) {
  return this_ptr->isNull();
}

double qt_core_c_QLineF_length(const QLineF* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QLineF_normalVector_to_output(const QLineF* this_ptr, QLineF* output) {
  new(output) QLineF(this_ptr->normalVector());
}

bool qt_core_c_QLineF_operator_eq(const QLineF* this_ptr, const QLineF* d) {
  return this_ptr->operator==(*d);
}

bool qt_core_c_QLineF_operator_neq(const QLineF* this_ptr, const QLineF* d) {
  return this_ptr->operator!=(*d);
}

void qt_core_c_QLineF_p1_to_output(const QLineF* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->p1());
}

void qt_core_c_QLineF_p2_to_output(const QLineF* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->p2());
}

void qt_core_c_QLineF_pointAt_to_output(const QLineF* this_ptr, double t, QPointF* output) {
  new(output) QPointF(this_ptr->pointAt(t));
}

void qt_core_c_QLineF_setAngle(QLineF* this_ptr, double angle) {
  this_ptr->setAngle(angle);
}

void qt_core_c_QLineF_setLength(QLineF* this_ptr, double len) {
  this_ptr->setLength(len);
}

void qt_core_c_QLineF_setLine(QLineF* this_ptr, double x1, double y1, double x2, double y2) {
  this_ptr->setLine(x1, y1, x2, y2);
}

void qt_core_c_QLineF_setP1(QLineF* this_ptr, const QPointF* p1) {
  this_ptr->setP1(*p1);
}

void qt_core_c_QLineF_setP2(QLineF* this_ptr, const QPointF* p2) {
  this_ptr->setP2(*p2);
}

void qt_core_c_QLineF_setPoints(QLineF* this_ptr, const QPointF* p1, const QPointF* p2) {
  this_ptr->setPoints(*p1, *p2);
}

void qt_core_c_QLineF_toLine_to_output(const QLineF* this_ptr, QLine* output) {
  new(output) QLine(this_ptr->toLine());
}

void qt_core_c_QLineF_translate_dx_dy(QLineF* this_ptr, double dx, double dy) {
  this_ptr->translate(dx, dy);
}

void qt_core_c_QLineF_translate_p(QLineF* this_ptr, const QPointF* p) {
  this_ptr->translate(*p);
}

void qt_core_c_QLineF_translated_to_output_dx_dy(const QLineF* this_ptr, double dx, double dy, QLineF* output) {
  new(output) QLineF(this_ptr->translated(dx, dy));
}

void qt_core_c_QLineF_translated_to_output_p(const QLineF* this_ptr, const QPointF* p, QLineF* output) {
  new(output) QLineF(this_ptr->translated(*p));
}

void qt_core_c_QLineF_unitVector_to_output(const QLineF* this_ptr, QLineF* output) {
  new(output) QLineF(this_ptr->unitVector());
}

double qt_core_c_QLineF_x1(const QLineF* this_ptr) {
  return this_ptr->x1();
}

double qt_core_c_QLineF_x2(const QLineF* this_ptr) {
  return this_ptr->x2();
}

double qt_core_c_QLineF_y1(const QLineF* this_ptr) {
  return this_ptr->y1();
}

double qt_core_c_QLineF_y2(const QLineF* this_ptr) {
  return this_ptr->y2();
}

