#include "qt_gui_c_QRadialGradient.h"

QGradient* qt_gui_c_QRadialGradient_G_static_cast_QGradient_ptr(QRadialGradient* ptr) {
  return static_cast<QGradient*>(ptr);
}

QRadialGradient* qt_gui_c_QRadialGradient_G_static_cast_QRadialGradient_ptr(QGradient* ptr) {
  return static_cast<QRadialGradient*>(ptr);
}

double qt_gui_c_QRadialGradient_centerRadius(const QRadialGradient* this_ptr) {
  return this_ptr->centerRadius();
}

void qt_gui_c_QRadialGradient_center_to_output(const QRadialGradient* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->center());
}

void qt_gui_c_QRadialGradient_delete(QRadialGradient* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QRadialGradient_focalPoint_to_output(const QRadialGradient* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->focalPoint());
}

double qt_gui_c_QRadialGradient_focalRadius(const QRadialGradient* this_ptr) {
  return this_ptr->focalRadius();
}

QRadialGradient* qt_gui_c_QRadialGradient_new_center_centerRadius_focalPoint_focalRadius(const QPointF* center, double centerRadius, const QPointF* focalPoint, double focalRadius) {
  return new QRadialGradient(*center, centerRadius, *focalPoint, focalRadius);
}

QRadialGradient* qt_gui_c_QRadialGradient_new_center_radius(const QPointF* center, double radius) {
  return new QRadialGradient(*center, radius);
}

QRadialGradient* qt_gui_c_QRadialGradient_new_center_radius_focalPoint(const QPointF* center, double radius, const QPointF* focalPoint) {
  return new QRadialGradient(*center, radius, *focalPoint);
}

QRadialGradient* qt_gui_c_QRadialGradient_new_cx_cy_centerRadius_fx_fy_focalRadius(double cx, double cy, double centerRadius, double fx, double fy, double focalRadius) {
  return new QRadialGradient(cx, cy, centerRadius, fx, fy, focalRadius);
}

QRadialGradient* qt_gui_c_QRadialGradient_new_cx_cy_radius(double cx, double cy, double radius) {
  return new QRadialGradient(cx, cy, radius);
}

QRadialGradient* qt_gui_c_QRadialGradient_new_cx_cy_radius_fx_fy(double cx, double cy, double radius, double fx, double fy) {
  return new QRadialGradient(cx, cy, radius, fx, fy);
}

QRadialGradient* qt_gui_c_QRadialGradient_new_no_args() {
  return new QRadialGradient();
}

double qt_gui_c_QRadialGradient_radius(const QRadialGradient* this_ptr) {
  return this_ptr->radius();
}

void qt_gui_c_QRadialGradient_setCenterRadius(QRadialGradient* this_ptr, double radius) {
  this_ptr->setCenterRadius(radius);
}

void qt_gui_c_QRadialGradient_setCenter_center(QRadialGradient* this_ptr, const QPointF* center) {
  this_ptr->setCenter(*center);
}

void qt_gui_c_QRadialGradient_setCenter_x_y(QRadialGradient* this_ptr, double x, double y) {
  this_ptr->setCenter(x, y);
}

void qt_gui_c_QRadialGradient_setFocalPoint_focalPoint(QRadialGradient* this_ptr, const QPointF* focalPoint) {
  this_ptr->setFocalPoint(*focalPoint);
}

void qt_gui_c_QRadialGradient_setFocalPoint_x_y(QRadialGradient* this_ptr, double x, double y) {
  this_ptr->setFocalPoint(x, y);
}

void qt_gui_c_QRadialGradient_setFocalRadius(QRadialGradient* this_ptr, double radius) {
  this_ptr->setFocalRadius(radius);
}

void qt_gui_c_QRadialGradient_setRadius(QRadialGradient* this_ptr, double radius) {
  this_ptr->setRadius(radius);
}

