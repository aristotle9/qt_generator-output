#include "qt_core_c_QLine.h"

QDataStream* qt_core_c_QLine_G_operator_shl_QDataStream_QLine(QDataStream* arg1, const QLine* arg2) {
  return &operator<<(*arg1, *arg2);
}

QDataStream* qt_core_c_QLine_G_operator_shl_QDataStream_QLineF(QDataStream* arg1, const QLineF* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_core_c_QLine_G_operator_shl_to_output_QDebug_QLine(const QDebug* d, const QLine* p, QDebug* output) {
  new(output) QDebug(operator<<(*d, *p));
}

void qt_core_c_QLine_G_operator_shl_to_output_QDebug_QLineF(const QDebug* d, const QLineF* p, QDebug* output) {
  new(output) QDebug(operator<<(*d, *p));
}

QDataStream* qt_core_c_QLine_G_operator_shr_QDataStream_QLine(QDataStream* arg1, QLine* arg2) {
  return &operator>>(*arg1, *arg2);
}

QDataStream* qt_core_c_QLine_G_operator_shr_QDataStream_QLineF(QDataStream* arg1, QLineF* arg2) {
  return &operator>>(*arg1, *arg2);
}

void qt_core_c_QLine_center_to_output(const QLine* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->center());
}

void qt_core_c_QLine_constructor_no_args(QLine* output) {
  new(output) QLine();
}

void qt_core_c_QLine_constructor_pt1_pt2(const QPoint* pt1, const QPoint* pt2, QLine* output) {
  new(output) QLine(*pt1, *pt2);
}

void qt_core_c_QLine_constructor_x1_y1_x2_y2(int x1, int y1, int x2, int y2, QLine* output) {
  new(output) QLine(x1, y1, x2, y2);
}

void qt_core_c_QLine_destructor(QLine* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

int qt_core_c_QLine_dx(const QLine* this_ptr) {
  return this_ptr->dx();
}

int qt_core_c_QLine_dy(const QLine* this_ptr) {
  return this_ptr->dy();
}

bool qt_core_c_QLine_isNull(const QLine* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QLine_operator_eq(const QLine* this_ptr, const QLine* d) {
  return this_ptr->operator==(*d);
}

bool qt_core_c_QLine_operator_neq(const QLine* this_ptr, const QLine* d) {
  return this_ptr->operator!=(*d);
}

void qt_core_c_QLine_p1_to_output(const QLine* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->p1());
}

void qt_core_c_QLine_p2_to_output(const QLine* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->p2());
}

void qt_core_c_QLine_setLine(QLine* this_ptr, int x1, int y1, int x2, int y2) {
  this_ptr->setLine(x1, y1, x2, y2);
}

void qt_core_c_QLine_setP1(QLine* this_ptr, const QPoint* p1) {
  this_ptr->setP1(*p1);
}

void qt_core_c_QLine_setP2(QLine* this_ptr, const QPoint* p2) {
  this_ptr->setP2(*p2);
}

void qt_core_c_QLine_setPoints(QLine* this_ptr, const QPoint* p1, const QPoint* p2) {
  this_ptr->setPoints(*p1, *p2);
}

void qt_core_c_QLine_translate_dx_dy(QLine* this_ptr, int dx, int dy) {
  this_ptr->translate(dx, dy);
}

void qt_core_c_QLine_translate_p(QLine* this_ptr, const QPoint* p) {
  this_ptr->translate(*p);
}

void qt_core_c_QLine_translated_to_output_dx_dy(const QLine* this_ptr, int dx, int dy, QLine* output) {
  new(output) QLine(this_ptr->translated(dx, dy));
}

void qt_core_c_QLine_translated_to_output_p(const QLine* this_ptr, const QPoint* p, QLine* output) {
  new(output) QLine(this_ptr->translated(*p));
}

int qt_core_c_QLine_x1(const QLine* this_ptr) {
  return this_ptr->x1();
}

int qt_core_c_QLine_x2(const QLine* this_ptr) {
  return this_ptr->x2();
}

int qt_core_c_QLine_y1(const QLine* this_ptr) {
  return this_ptr->y1();
}

int qt_core_c_QLine_y2(const QLine* this_ptr) {
  return this_ptr->y2();
}

