#include "qt_gui_c_QConicalGradient.h"

QConicalGradient* qt_gui_c_QConicalGradient_G_static_cast_QConicalGradient_ptr(QGradient* ptr) {
  return static_cast<QConicalGradient*>(ptr);
}

QGradient* qt_gui_c_QConicalGradient_G_static_cast_QGradient_ptr(QConicalGradient* ptr) {
  return static_cast<QGradient*>(ptr);
}

double qt_gui_c_QConicalGradient_angle(const QConicalGradient* this_ptr) {
  return this_ptr->angle();
}

void qt_gui_c_QConicalGradient_center_to_output(const QConicalGradient* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->center());
}

void qt_gui_c_QConicalGradient_delete(QConicalGradient* this_ptr) {
  delete this_ptr;
}

QConicalGradient* qt_gui_c_QConicalGradient_new_center_startAngle(const QPointF* center, double startAngle) {
  return new QConicalGradient(*center, startAngle);
}

QConicalGradient* qt_gui_c_QConicalGradient_new_cx_cy_startAngle(double cx, double cy, double startAngle) {
  return new QConicalGradient(cx, cy, startAngle);
}

QConicalGradient* qt_gui_c_QConicalGradient_new_no_args() {
  return new QConicalGradient();
}

void qt_gui_c_QConicalGradient_setAngle(QConicalGradient* this_ptr, double angle) {
  this_ptr->setAngle(angle);
}

void qt_gui_c_QConicalGradient_setCenter_center(QConicalGradient* this_ptr, const QPointF* center) {
  this_ptr->setCenter(*center);
}

void qt_gui_c_QConicalGradient_setCenter_x_y(QConicalGradient* this_ptr, double x, double y) {
  this_ptr->setCenter(x, y);
}

