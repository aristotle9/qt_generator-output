#include "qt_core_c_QRectF.h"

void qt_core_c_QRectF_adjust(QRectF* this_ptr, double x1, double y1, double x2, double y2) {
  this_ptr->adjust(x1, y1, x2, y2);
}

void qt_core_c_QRectF_adjusted_to_output(const QRectF* this_ptr, double x1, double y1, double x2, double y2, QRectF* output) {
  new(output) QRectF(this_ptr->adjusted(x1, y1, x2, y2));
}

double qt_core_c_QRectF_bottom(const QRectF* this_ptr) {
  return this_ptr->bottom();
}

void qt_core_c_QRectF_bottomLeft_to_output(const QRectF* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->bottomLeft());
}

void qt_core_c_QRectF_bottomRight_to_output(const QRectF* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->bottomRight());
}

void qt_core_c_QRectF_center_to_output(const QRectF* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->center());
}

void qt_core_c_QRectF_constructor_left_top_width_height(double left, double top, double width, double height, QRectF* output) {
  new(output) QRectF(left, top, width, height);
}

void qt_core_c_QRectF_constructor_no_args(QRectF* output) {
  new(output) QRectF();
}

void qt_core_c_QRectF_constructor_rect(const QRect* rect, QRectF* output) {
  new(output) QRectF(*rect);
}

void qt_core_c_QRectF_constructor_topleft_bottomRight(const QPointF* topleft, const QPointF* bottomRight, QRectF* output) {
  new(output) QRectF(*topleft, *bottomRight);
}

void qt_core_c_QRectF_constructor_topleft_size(const QPointF* topleft, const QSizeF* size, QRectF* output) {
  new(output) QRectF(*topleft, *size);
}

bool qt_core_c_QRectF_contains_p(const QRectF* this_ptr, const QPointF* p) {
  return this_ptr->contains(*p);
}

bool qt_core_c_QRectF_contains_r(const QRectF* this_ptr, const QRectF* r) {
  return this_ptr->contains(*r);
}

bool qt_core_c_QRectF_contains_x_y(const QRectF* this_ptr, double x, double y) {
  return this_ptr->contains(x, y);
}

void qt_core_c_QRectF_destructor(QRectF* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QRectF_getCoords(const QRectF* this_ptr, double* x1, double* y1, double* x2, double* y2) {
  this_ptr->getCoords(x1, y1, x2, y2);
}

void qt_core_c_QRectF_getRect(const QRectF* this_ptr, double* x, double* y, double* w, double* h) {
  this_ptr->getRect(x, y, w, h);
}

double qt_core_c_QRectF_height(const QRectF* this_ptr) {
  return this_ptr->height();
}

void qt_core_c_QRectF_intersected_to_output(const QRectF* this_ptr, const QRectF* other, QRectF* output) {
  new(output) QRectF(this_ptr->intersected(*other));
}

bool qt_core_c_QRectF_intersects(const QRectF* this_ptr, const QRectF* r) {
  return this_ptr->intersects(*r);
}

bool qt_core_c_QRectF_isEmpty(const QRectF* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_core_c_QRectF_isNull(const QRectF* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QRectF_isValid(const QRectF* this_ptr) {
  return this_ptr->isValid();
}

double qt_core_c_QRectF_left(const QRectF* this_ptr) {
  return this_ptr->left();
}

void qt_core_c_QRectF_marginsAdded_to_output(const QRectF* this_ptr, const QMarginsF* margins, QRectF* output) {
  new(output) QRectF(this_ptr->marginsAdded(*margins));
}

void qt_core_c_QRectF_marginsRemoved_to_output(const QRectF* this_ptr, const QMarginsF* margins, QRectF* output) {
  new(output) QRectF(this_ptr->marginsRemoved(*margins));
}

void qt_core_c_QRectF_moveBottom(QRectF* this_ptr, double pos) {
  this_ptr->moveBottom(pos);
}

void qt_core_c_QRectF_moveBottomLeft(QRectF* this_ptr, const QPointF* p) {
  this_ptr->moveBottomLeft(*p);
}

void qt_core_c_QRectF_moveBottomRight(QRectF* this_ptr, const QPointF* p) {
  this_ptr->moveBottomRight(*p);
}

void qt_core_c_QRectF_moveCenter(QRectF* this_ptr, const QPointF* p) {
  this_ptr->moveCenter(*p);
}

void qt_core_c_QRectF_moveLeft(QRectF* this_ptr, double pos) {
  this_ptr->moveLeft(pos);
}

void qt_core_c_QRectF_moveRight(QRectF* this_ptr, double pos) {
  this_ptr->moveRight(pos);
}

void qt_core_c_QRectF_moveTo_p(QRectF* this_ptr, const QPointF* p) {
  this_ptr->moveTo(*p);
}

void qt_core_c_QRectF_moveTo_x_y(QRectF* this_ptr, double x, double y) {
  this_ptr->moveTo(x, y);
}

void qt_core_c_QRectF_moveTop(QRectF* this_ptr, double pos) {
  this_ptr->moveTop(pos);
}

void qt_core_c_QRectF_moveTopLeft(QRectF* this_ptr, const QPointF* p) {
  this_ptr->moveTopLeft(*p);
}

void qt_core_c_QRectF_moveTopRight(QRectF* this_ptr, const QPointF* p) {
  this_ptr->moveTopRight(*p);
}

void qt_core_c_QRectF_normalized_to_output(const QRectF* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->normalized());
}

QRectF* qt_core_c_QRectF_operator_add_assign(QRectF* this_ptr, const QMarginsF* margins) {
  return &this_ptr->operator+=(*margins);
}

QRectF* qt_core_c_QRectF_operator_bit_and_assign(QRectF* this_ptr, const QRectF* r) {
  return &this_ptr->operator&=(*r);
}

void qt_core_c_QRectF_operator_bit_and_to_output(const QRectF* this_ptr, const QRectF* r, QRectF* output) {
  new(output) QRectF(this_ptr->operator&(*r));
}

QRectF* qt_core_c_QRectF_operator_bit_or_assign(QRectF* this_ptr, const QRectF* r) {
  return &this_ptr->operator|=(*r);
}

void qt_core_c_QRectF_operator_bit_or_to_output(const QRectF* this_ptr, const QRectF* r, QRectF* output) {
  new(output) QRectF(this_ptr->operator|(*r));
}

QRectF* qt_core_c_QRectF_operator_sub_assign(QRectF* this_ptr, const QMarginsF* margins) {
  return &this_ptr->operator-=(*margins);
}

double qt_core_c_QRectF_right(const QRectF* this_ptr) {
  return this_ptr->right();
}

void qt_core_c_QRectF_setBottom(QRectF* this_ptr, double pos) {
  this_ptr->setBottom(pos);
}

void qt_core_c_QRectF_setBottomLeft(QRectF* this_ptr, const QPointF* p) {
  this_ptr->setBottomLeft(*p);
}

void qt_core_c_QRectF_setBottomRight(QRectF* this_ptr, const QPointF* p) {
  this_ptr->setBottomRight(*p);
}

void qt_core_c_QRectF_setCoords(QRectF* this_ptr, double x1, double y1, double x2, double y2) {
  this_ptr->setCoords(x1, y1, x2, y2);
}

void qt_core_c_QRectF_setHeight(QRectF* this_ptr, double h) {
  this_ptr->setHeight(h);
}

void qt_core_c_QRectF_setLeft(QRectF* this_ptr, double pos) {
  this_ptr->setLeft(pos);
}

void qt_core_c_QRectF_setRect(QRectF* this_ptr, double x, double y, double w, double h) {
  this_ptr->setRect(x, y, w, h);
}

void qt_core_c_QRectF_setRight(QRectF* this_ptr, double pos) {
  this_ptr->setRight(pos);
}

void qt_core_c_QRectF_setSize(QRectF* this_ptr, const QSizeF* s) {
  this_ptr->setSize(*s);
}

void qt_core_c_QRectF_setTop(QRectF* this_ptr, double pos) {
  this_ptr->setTop(pos);
}

void qt_core_c_QRectF_setTopLeft(QRectF* this_ptr, const QPointF* p) {
  this_ptr->setTopLeft(*p);
}

void qt_core_c_QRectF_setTopRight(QRectF* this_ptr, const QPointF* p) {
  this_ptr->setTopRight(*p);
}

void qt_core_c_QRectF_setWidth(QRectF* this_ptr, double w) {
  this_ptr->setWidth(w);
}

void qt_core_c_QRectF_setX(QRectF* this_ptr, double pos) {
  this_ptr->setX(pos);
}

void qt_core_c_QRectF_setY(QRectF* this_ptr, double pos) {
  this_ptr->setY(pos);
}

void qt_core_c_QRectF_size_to_output(const QRectF* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->size());
}

void qt_core_c_QRectF_toAlignedRect_to_output(const QRectF* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->toAlignedRect());
}

void qt_core_c_QRectF_toRect_to_output(const QRectF* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->toRect());
}

double qt_core_c_QRectF_top(const QRectF* this_ptr) {
  return this_ptr->top();
}

void qt_core_c_QRectF_topLeft_to_output(const QRectF* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->topLeft());
}

void qt_core_c_QRectF_topRight_to_output(const QRectF* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->topRight());
}

void qt_core_c_QRectF_translate_dx_dy(QRectF* this_ptr, double dx, double dy) {
  this_ptr->translate(dx, dy);
}

void qt_core_c_QRectF_translate_p(QRectF* this_ptr, const QPointF* p) {
  this_ptr->translate(*p);
}

void qt_core_c_QRectF_translated_to_output_dx_dy(const QRectF* this_ptr, double dx, double dy, QRectF* output) {
  new(output) QRectF(this_ptr->translated(dx, dy));
}

void qt_core_c_QRectF_translated_to_output_p(const QRectF* this_ptr, const QPointF* p, QRectF* output) {
  new(output) QRectF(this_ptr->translated(*p));
}

void qt_core_c_QRectF_transposed_to_output(const QRectF* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->transposed());
}

void qt_core_c_QRectF_united_to_output(const QRectF* this_ptr, const QRectF* other, QRectF* output) {
  new(output) QRectF(this_ptr->united(*other));
}

double qt_core_c_QRectF_width(const QRectF* this_ptr) {
  return this_ptr->width();
}

double qt_core_c_QRectF_x(const QRectF* this_ptr) {
  return this_ptr->x();
}

double qt_core_c_QRectF_y(const QRectF* this_ptr) {
  return this_ptr->y();
}

