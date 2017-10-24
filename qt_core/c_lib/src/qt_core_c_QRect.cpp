#include "qt_core_c_QRect.h"

void qt_core_c_QRect_G_operator_add_to_output_QMarginsF_QRectF(const QMarginsF* lhs, const QRectF* rhs, QRectF* output) {
  new(output) QRectF(operator+(*lhs, *rhs));
}

void qt_core_c_QRect_G_operator_add_to_output_QMargins_QRect(const QMargins* margins, const QRect* rectangle, QRect* output) {
  new(output) QRect(operator+(*margins, *rectangle));
}

void qt_core_c_QRect_G_operator_add_to_output_QRectF_QMarginsF(const QRectF* lhs, const QMarginsF* rhs, QRectF* output) {
  new(output) QRectF(operator+(*lhs, *rhs));
}

void qt_core_c_QRect_G_operator_add_to_output_QRect_QMargins(const QRect* rectangle, const QMargins* margins, QRect* output) {
  new(output) QRect(operator+(*rectangle, *margins));
}

QDataStream* qt_core_c_QRect_G_operator_shl_QDataStream_QRect(QDataStream* arg1, const QRect* arg2) {
  return &operator<<(*arg1, *arg2);
}

QDataStream* qt_core_c_QRect_G_operator_shl_QDataStream_QRectF(QDataStream* arg1, const QRectF* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_core_c_QRect_G_operator_shl_to_output_QDebug_QRect(const QDebug* arg1, const QRect* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

void qt_core_c_QRect_G_operator_shl_to_output_QDebug_QRectF(const QDebug* arg1, const QRectF* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_core_c_QRect_G_operator_shr_QDataStream_QRect(QDataStream* arg1, QRect* arg2) {
  return &operator>>(*arg1, *arg2);
}

QDataStream* qt_core_c_QRect_G_operator_shr_QDataStream_QRectF(QDataStream* arg1, QRectF* arg2) {
  return &operator>>(*arg1, *arg2);
}

void qt_core_c_QRect_G_operator_sub_to_output_QRectF_QMarginsF(const QRectF* lhs, const QMarginsF* rhs, QRectF* output) {
  new(output) QRectF(operator-(*lhs, *rhs));
}

void qt_core_c_QRect_G_operator_sub_to_output_QRect_QMargins(const QRect* lhs, const QMargins* rhs, QRect* output) {
  new(output) QRect(operator-(*lhs, *rhs));
}

void qt_core_c_QRect_adjust(QRect* this_ptr, int x1, int y1, int x2, int y2) {
  this_ptr->adjust(x1, y1, x2, y2);
}

void qt_core_c_QRect_adjusted_to_output(const QRect* this_ptr, int x1, int y1, int x2, int y2, QRect* output) {
  new(output) QRect(this_ptr->adjusted(x1, y1, x2, y2));
}

int qt_core_c_QRect_bottom(const QRect* this_ptr) {
  return this_ptr->bottom();
}

void qt_core_c_QRect_bottomLeft_to_output(const QRect* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->bottomLeft());
}

void qt_core_c_QRect_bottomRight_to_output(const QRect* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->bottomRight());
}

void qt_core_c_QRect_center_to_output(const QRect* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->center());
}

void qt_core_c_QRect_constructor_left_top_width_height(int left, int top, int width, int height, QRect* output) {
  new(output) QRect(left, top, width, height);
}

void qt_core_c_QRect_constructor_no_args(QRect* output) {
  new(output) QRect();
}

void qt_core_c_QRect_constructor_topleft_bottomright(const QPoint* topleft, const QPoint* bottomright, QRect* output) {
  new(output) QRect(*topleft, *bottomright);
}

void qt_core_c_QRect_constructor_topleft_size(const QPoint* topleft, const QSize* size, QRect* output) {
  new(output) QRect(*topleft, *size);
}

bool qt_core_c_QRect_contains_p(const QRect* this_ptr, const QPoint* p) {
  return this_ptr->contains(*p);
}

bool qt_core_c_QRect_contains_p_proper(const QRect* this_ptr, const QPoint* p, bool proper) {
  return this_ptr->contains(*p, proper);
}

bool qt_core_c_QRect_contains_r(const QRect* this_ptr, const QRect* r) {
  return this_ptr->contains(*r);
}

bool qt_core_c_QRect_contains_r_proper(const QRect* this_ptr, const QRect* r, bool proper) {
  return this_ptr->contains(*r, proper);
}

bool qt_core_c_QRect_contains_x_y(const QRect* this_ptr, int x, int y) {
  return this_ptr->contains(x, y);
}

bool qt_core_c_QRect_contains_x_y_proper(const QRect* this_ptr, int x, int y, bool proper) {
  return this_ptr->contains(x, y, proper);
}

void qt_core_c_QRect_destructor(QRect* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QRect_getCoords(const QRect* this_ptr, int* x1, int* y1, int* x2, int* y2) {
  this_ptr->getCoords(x1, y1, x2, y2);
}

void qt_core_c_QRect_getRect(const QRect* this_ptr, int* x, int* y, int* w, int* h) {
  this_ptr->getRect(x, y, w, h);
}

int qt_core_c_QRect_height(const QRect* this_ptr) {
  return this_ptr->height();
}

void qt_core_c_QRect_intersected_to_output(const QRect* this_ptr, const QRect* other, QRect* output) {
  new(output) QRect(this_ptr->intersected(*other));
}

bool qt_core_c_QRect_intersects(const QRect* this_ptr, const QRect* r) {
  return this_ptr->intersects(*r);
}

bool qt_core_c_QRect_isEmpty(const QRect* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_core_c_QRect_isNull(const QRect* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QRect_isValid(const QRect* this_ptr) {
  return this_ptr->isValid();
}

int qt_core_c_QRect_left(const QRect* this_ptr) {
  return this_ptr->left();
}

void qt_core_c_QRect_marginsAdded_to_output(const QRect* this_ptr, const QMargins* margins, QRect* output) {
  new(output) QRect(this_ptr->marginsAdded(*margins));
}

void qt_core_c_QRect_marginsRemoved_to_output(const QRect* this_ptr, const QMargins* margins, QRect* output) {
  new(output) QRect(this_ptr->marginsRemoved(*margins));
}

void qt_core_c_QRect_moveBottom(QRect* this_ptr, int pos) {
  this_ptr->moveBottom(pos);
}

void qt_core_c_QRect_moveBottomLeft(QRect* this_ptr, const QPoint* p) {
  this_ptr->moveBottomLeft(*p);
}

void qt_core_c_QRect_moveBottomRight(QRect* this_ptr, const QPoint* p) {
  this_ptr->moveBottomRight(*p);
}

void qt_core_c_QRect_moveCenter(QRect* this_ptr, const QPoint* p) {
  this_ptr->moveCenter(*p);
}

void qt_core_c_QRect_moveLeft(QRect* this_ptr, int pos) {
  this_ptr->moveLeft(pos);
}

void qt_core_c_QRect_moveRight(QRect* this_ptr, int pos) {
  this_ptr->moveRight(pos);
}

void qt_core_c_QRect_moveTo_p(QRect* this_ptr, const QPoint* p) {
  this_ptr->moveTo(*p);
}

void qt_core_c_QRect_moveTo_x_t(QRect* this_ptr, int x, int t) {
  this_ptr->moveTo(x, t);
}

void qt_core_c_QRect_moveTop(QRect* this_ptr, int pos) {
  this_ptr->moveTop(pos);
}

void qt_core_c_QRect_moveTopLeft(QRect* this_ptr, const QPoint* p) {
  this_ptr->moveTopLeft(*p);
}

void qt_core_c_QRect_moveTopRight(QRect* this_ptr, const QPoint* p) {
  this_ptr->moveTopRight(*p);
}

void qt_core_c_QRect_normalized_to_output(const QRect* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->normalized());
}

QRect* qt_core_c_QRect_operator_add_assign(QRect* this_ptr, const QMargins* margins) {
  return &this_ptr->operator+=(*margins);
}

QRect* qt_core_c_QRect_operator_bit_and_assign(QRect* this_ptr, const QRect* r) {
  return &this_ptr->operator&=(*r);
}

void qt_core_c_QRect_operator_bit_and_to_output(const QRect* this_ptr, const QRect* r, QRect* output) {
  new(output) QRect(this_ptr->operator&(*r));
}

QRect* qt_core_c_QRect_operator_bit_or_assign(QRect* this_ptr, const QRect* r) {
  return &this_ptr->operator|=(*r);
}

void qt_core_c_QRect_operator_bit_or_to_output(const QRect* this_ptr, const QRect* r, QRect* output) {
  new(output) QRect(this_ptr->operator|(*r));
}

QRect* qt_core_c_QRect_operator_sub_assign(QRect* this_ptr, const QMargins* margins) {
  return &this_ptr->operator-=(*margins);
}

int qt_core_c_QRect_right(const QRect* this_ptr) {
  return this_ptr->right();
}

void qt_core_c_QRect_setBottom(QRect* this_ptr, int pos) {
  this_ptr->setBottom(pos);
}

void qt_core_c_QRect_setBottomLeft(QRect* this_ptr, const QPoint* p) {
  this_ptr->setBottomLeft(*p);
}

void qt_core_c_QRect_setBottomRight(QRect* this_ptr, const QPoint* p) {
  this_ptr->setBottomRight(*p);
}

void qt_core_c_QRect_setCoords(QRect* this_ptr, int x1, int y1, int x2, int y2) {
  this_ptr->setCoords(x1, y1, x2, y2);
}

void qt_core_c_QRect_setHeight(QRect* this_ptr, int h) {
  this_ptr->setHeight(h);
}

void qt_core_c_QRect_setLeft(QRect* this_ptr, int pos) {
  this_ptr->setLeft(pos);
}

void qt_core_c_QRect_setRect(QRect* this_ptr, int x, int y, int w, int h) {
  this_ptr->setRect(x, y, w, h);
}

void qt_core_c_QRect_setRight(QRect* this_ptr, int pos) {
  this_ptr->setRight(pos);
}

void qt_core_c_QRect_setSize(QRect* this_ptr, const QSize* s) {
  this_ptr->setSize(*s);
}

void qt_core_c_QRect_setTop(QRect* this_ptr, int pos) {
  this_ptr->setTop(pos);
}

void qt_core_c_QRect_setTopLeft(QRect* this_ptr, const QPoint* p) {
  this_ptr->setTopLeft(*p);
}

void qt_core_c_QRect_setTopRight(QRect* this_ptr, const QPoint* p) {
  this_ptr->setTopRight(*p);
}

void qt_core_c_QRect_setWidth(QRect* this_ptr, int w) {
  this_ptr->setWidth(w);
}

void qt_core_c_QRect_setX(QRect* this_ptr, int x) {
  this_ptr->setX(x);
}

void qt_core_c_QRect_setY(QRect* this_ptr, int y) {
  this_ptr->setY(y);
}

void qt_core_c_QRect_size_to_output(const QRect* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->size());
}

int qt_core_c_QRect_top(const QRect* this_ptr) {
  return this_ptr->top();
}

void qt_core_c_QRect_topLeft_to_output(const QRect* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->topLeft());
}

void qt_core_c_QRect_topRight_to_output(const QRect* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->topRight());
}

void qt_core_c_QRect_translate_dx_dy(QRect* this_ptr, int dx, int dy) {
  this_ptr->translate(dx, dy);
}

void qt_core_c_QRect_translate_p(QRect* this_ptr, const QPoint* p) {
  this_ptr->translate(*p);
}

void qt_core_c_QRect_translated_to_output_dx_dy(const QRect* this_ptr, int dx, int dy, QRect* output) {
  new(output) QRect(this_ptr->translated(dx, dy));
}

void qt_core_c_QRect_translated_to_output_p(const QRect* this_ptr, const QPoint* p, QRect* output) {
  new(output) QRect(this_ptr->translated(*p));
}

void qt_core_c_QRect_transposed_to_output(const QRect* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->transposed());
}

void qt_core_c_QRect_united_to_output(const QRect* this_ptr, const QRect* other, QRect* output) {
  new(output) QRect(this_ptr->united(*other));
}

int qt_core_c_QRect_width(const QRect* this_ptr) {
  return this_ptr->width();
}

int qt_core_c_QRect_x(const QRect* this_ptr) {
  return this_ptr->x();
}

int qt_core_c_QRect_y(const QRect* this_ptr) {
  return this_ptr->y();
}

