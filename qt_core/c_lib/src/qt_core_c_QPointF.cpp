#include "qt_core_c_QPointF.h"

void qt_core_c_QPointF_constructor_no_args(QPointF* output) {
  new(output) QPointF();
}

void qt_core_c_QPointF_constructor_p(const QPoint* p, QPointF* output) {
  new(output) QPointF(*p);
}

void qt_core_c_QPointF_constructor_xpos_ypos(double xpos, double ypos, QPointF* output) {
  new(output) QPointF(xpos, ypos);
}

void qt_core_c_QPointF_destructor(QPointF* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

double qt_core_c_QPointF_dotProduct(const QPointF* p1, const QPointF* p2) {
  return QPointF::dotProduct(*p1, *p2);
}

bool qt_core_c_QPointF_isNull(const QPointF* this_ptr) {
  return this_ptr->isNull();
}

double qt_core_c_QPointF_manhattanLength(const QPointF* this_ptr) {
  return this_ptr->manhattanLength();
}

QPointF* qt_core_c_QPointF_operator_add_assign(QPointF* this_ptr, const QPointF* p) {
  return &this_ptr->operator+=(*p);
}

QPointF* qt_core_c_QPointF_operator_div_assign(QPointF* this_ptr, double c) {
  return &this_ptr->operator/=(c);
}

QPointF* qt_core_c_QPointF_operator_mul_assign(QPointF* this_ptr, double c) {
  return &this_ptr->operator*=(c);
}

QPointF* qt_core_c_QPointF_operator_sub_assign(QPointF* this_ptr, const QPointF* p) {
  return &this_ptr->operator-=(*p);
}

double* qt_core_c_QPointF_rx(QPointF* this_ptr) {
  return &this_ptr->rx();
}

double* qt_core_c_QPointF_ry(QPointF* this_ptr) {
  return &this_ptr->ry();
}

void qt_core_c_QPointF_setX(QPointF* this_ptr, double x) {
  this_ptr->setX(x);
}

void qt_core_c_QPointF_setY(QPointF* this_ptr, double y) {
  this_ptr->setY(y);
}

void qt_core_c_QPointF_toPoint_to_output(const QPointF* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->toPoint());
}

double qt_core_c_QPointF_x(const QPointF* this_ptr) {
  return this_ptr->x();
}

double qt_core_c_QPointF_y(const QPointF* this_ptr) {
  return this_ptr->y();
}

