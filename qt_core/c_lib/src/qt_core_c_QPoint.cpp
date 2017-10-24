#include "qt_core_c_QPoint.h"

QDataStream* qt_core_c_QPoint_G_operator_shl_QDataStream_QPoint(QDataStream* arg1, const QPoint* arg2) {
  return &operator<<(*arg1, *arg2);
}

QDataStream* qt_core_c_QPoint_G_operator_shl_QDataStream_QPointF(QDataStream* arg1, const QPointF* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_core_c_QPoint_G_operator_shl_to_output_arg1_arg2(const QDebug* arg1, const QPoint* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

void qt_core_c_QPoint_G_operator_shl_to_output_d_p(const QDebug* d, const QPointF* p, QDebug* output) {
  new(output) QDebug(operator<<(*d, *p));
}

QDataStream* qt_core_c_QPoint_G_operator_shr_QDataStream_QPoint(QDataStream* arg1, QPoint* arg2) {
  return &operator>>(*arg1, *arg2);
}

QDataStream* qt_core_c_QPoint_G_operator_shr_QDataStream_QPointF(QDataStream* arg1, QPointF* arg2) {
  return &operator>>(*arg1, *arg2);
}

void qt_core_c_QPoint_constructor_no_args(QPoint* output) {
  new(output) QPoint();
}

void qt_core_c_QPoint_constructor_xpos_ypos(int xpos, int ypos, QPoint* output) {
  new(output) QPoint(xpos, ypos);
}

void qt_core_c_QPoint_destructor(QPoint* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

int qt_core_c_QPoint_dotProduct(const QPoint* p1, const QPoint* p2) {
  return QPoint::dotProduct(*p1, *p2);
}

bool qt_core_c_QPoint_isNull(const QPoint* this_ptr) {
  return this_ptr->isNull();
}

int qt_core_c_QPoint_manhattanLength(const QPoint* this_ptr) {
  return this_ptr->manhattanLength();
}

QPoint* qt_core_c_QPoint_operator_add_assign(QPoint* this_ptr, const QPoint* p) {
  return &this_ptr->operator+=(*p);
}

QPoint* qt_core_c_QPoint_operator_div_assign(QPoint* this_ptr, double divisor) {
  return &this_ptr->operator/=(divisor);
}

QPoint* qt_core_c_QPoint_operator_mul_assign_double(QPoint* this_ptr, double factor) {
  return &this_ptr->operator*=(factor);
}

QPoint* qt_core_c_QPoint_operator_mul_assign_float(QPoint* this_ptr, float factor) {
  return &this_ptr->operator*=(factor);
}

QPoint* qt_core_c_QPoint_operator_mul_assign_int(QPoint* this_ptr, int factor) {
  return &this_ptr->operator*=(factor);
}

QPoint* qt_core_c_QPoint_operator_sub_assign(QPoint* this_ptr, const QPoint* p) {
  return &this_ptr->operator-=(*p);
}

int* qt_core_c_QPoint_rx(QPoint* this_ptr) {
  return &this_ptr->rx();
}

int* qt_core_c_QPoint_ry(QPoint* this_ptr) {
  return &this_ptr->ry();
}

void qt_core_c_QPoint_setX(QPoint* this_ptr, int x) {
  this_ptr->setX(x);
}

void qt_core_c_QPoint_setY(QPoint* this_ptr, int y) {
  this_ptr->setY(y);
}

int qt_core_c_QPoint_x(const QPoint* this_ptr) {
  return this_ptr->x();
}

int qt_core_c_QPoint_y(const QPoint* this_ptr) {
  return this_ptr->y();
}

