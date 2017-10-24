#include "qt_gui_c_QRegion.h"

void qt_gui_c_QRegion_G_operator_shl_to_output(const QDebug* arg1, const QRegion* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

void qt_gui_c_QRegion_G_swap(QRegion* value1, QRegion* value2) {
  swap(*value1, *value2);
}

const QRect* qt_gui_c_QRegion_begin(const QRegion* this_ptr) {
  return this_ptr->begin();
}

void qt_gui_c_QRegion_boundingRect_to_output(const QRegion* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->boundingRect());
}

const QRect* qt_gui_c_QRegion_cbegin(const QRegion* this_ptr) {
  return this_ptr->cbegin();
}

const QRect* qt_gui_c_QRegion_cend(const QRegion* this_ptr) {
  return this_ptr->cend();
}

bool qt_gui_c_QRegion_contains_p(const QRegion* this_ptr, const QPoint* p) {
  return this_ptr->contains(*p);
}

bool qt_gui_c_QRegion_contains_r(const QRegion* this_ptr, const QRect* r) {
  return this_ptr->contains(*r);
}

void qt_gui_c_QRegion_convert_to_QVariant_to_output(const QRegion* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

void qt_gui_c_QRegion_delete(QRegion* this_ptr) {
  delete this_ptr;
}

const QRect* qt_gui_c_QRegion_end(const QRegion* this_ptr) {
  return this_ptr->end();
}

QRegion* qt_gui_c_QRegion_intersected_as_ptr_QRect(const QRegion* this_ptr, const QRect* r) {
  return new QRegion(this_ptr->intersected(*r));
}

QRegion* qt_gui_c_QRegion_intersected_as_ptr_QRegion(const QRegion* this_ptr, const QRegion* r) {
  return new QRegion(this_ptr->intersected(*r));
}

bool qt_gui_c_QRegion_intersects_QRect(const QRegion* this_ptr, const QRect* r) {
  return this_ptr->intersects(*r);
}

bool qt_gui_c_QRegion_intersects_QRegion(const QRegion* this_ptr, const QRegion* r) {
  return this_ptr->intersects(*r);
}

bool qt_gui_c_QRegion_isEmpty(const QRegion* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_gui_c_QRegion_isNull(const QRegion* this_ptr) {
  return this_ptr->isNull();
}

QRegion* qt_gui_c_QRegion_new_bitmap(const QBitmap* bitmap) {
  return new QRegion(*bitmap);
}

QRegion* qt_gui_c_QRegion_new_no_args() {
  return new QRegion();
}

QRegion* qt_gui_c_QRegion_new_pa(const QPolygon* pa) {
  return new QRegion(*pa);
}

QRegion* qt_gui_c_QRegion_new_pa_fillRule(const QPolygon* pa, const Qt::FillRule* fillRule) {
  return new QRegion(*pa, *fillRule);
}

QRegion* qt_gui_c_QRegion_new_r(const QRect* r) {
  return new QRegion(*r);
}

QRegion* qt_gui_c_QRegion_new_r_t(const QRect* r, QRegion::RegionType t) {
  return new QRegion(*r, t);
}

QRegion* qt_gui_c_QRegion_new_region(const QRegion* region) {
  return new QRegion(*region);
}

QRegion* qt_gui_c_QRegion_new_x_y_w_h(int x, int y, int w, int h) {
  return new QRegion(x, y, w, h);
}

QRegion* qt_gui_c_QRegion_new_x_y_w_h_t(int x, int y, int w, int h, QRegion::RegionType t) {
  return new QRegion(x, y, w, h, t);
}

QRegion* qt_gui_c_QRegion_operator_add_as_ptr_QRect(const QRegion* this_ptr, const QRect* r) {
  return new QRegion(this_ptr->operator+(*r));
}

QRegion* qt_gui_c_QRegion_operator_add_as_ptr_QRegion(const QRegion* this_ptr, const QRegion* r) {
  return new QRegion(this_ptr->operator+(*r));
}

QRegion* qt_gui_c_QRegion_operator_add_assign_QRect(QRegion* this_ptr, const QRect* r) {
  return &this_ptr->operator+=(*r);
}

QRegion* qt_gui_c_QRegion_operator_add_assign_QRegion(QRegion* this_ptr, const QRegion* r) {
  return &this_ptr->operator+=(*r);
}

QRegion* qt_gui_c_QRegion_operator_assign(QRegion* this_ptr, const QRegion* arg1) {
  return &this_ptr->operator=(*arg1);
}

QRegion* qt_gui_c_QRegion_operator_bit_and_as_ptr_QRect(const QRegion* this_ptr, const QRect* r) {
  return new QRegion(this_ptr->operator&(*r));
}

QRegion* qt_gui_c_QRegion_operator_bit_and_as_ptr_QRegion(const QRegion* this_ptr, const QRegion* r) {
  return new QRegion(this_ptr->operator&(*r));
}

QRegion* qt_gui_c_QRegion_operator_bit_and_assign_QRect(QRegion* this_ptr, const QRect* r) {
  return &this_ptr->operator&=(*r);
}

QRegion* qt_gui_c_QRegion_operator_bit_and_assign_QRegion(QRegion* this_ptr, const QRegion* r) {
  return &this_ptr->operator&=(*r);
}

QRegion* qt_gui_c_QRegion_operator_bit_or_as_ptr(const QRegion* this_ptr, const QRegion* r) {
  return new QRegion(this_ptr->operator|(*r));
}

QRegion* qt_gui_c_QRegion_operator_bit_or_assign(QRegion* this_ptr, const QRegion* r) {
  return &this_ptr->operator|=(*r);
}

QRegion* qt_gui_c_QRegion_operator_bit_xor_as_ptr(const QRegion* this_ptr, const QRegion* r) {
  return new QRegion(this_ptr->operator^(*r));
}

QRegion* qt_gui_c_QRegion_operator_bit_xor_assign(QRegion* this_ptr, const QRegion* r) {
  return &this_ptr->operator^=(*r);
}

bool qt_gui_c_QRegion_operator_eq(const QRegion* this_ptr, const QRegion* r) {
  return this_ptr->operator==(*r);
}

bool qt_gui_c_QRegion_operator_neq(const QRegion* this_ptr, const QRegion* r) {
  return this_ptr->operator!=(*r);
}

QRegion* qt_gui_c_QRegion_operator_sub_as_ptr(const QRegion* this_ptr, const QRegion* r) {
  return new QRegion(this_ptr->operator-(*r));
}

QRegion* qt_gui_c_QRegion_operator_sub_assign(QRegion* this_ptr, const QRegion* r) {
  return &this_ptr->operator-=(*r);
}

int qt_gui_c_QRegion_rectCount(const QRegion* this_ptr) {
  return this_ptr->rectCount();
}

void qt_gui_c_QRegion_rects_to_output(const QRegion* this_ptr, QVector< QRect >* output) {
  new(output) QVector< QRect >(this_ptr->rects());
}

void qt_gui_c_QRegion_setRects(QRegion* this_ptr, const QRect* rect, int num) {
  this_ptr->setRects(rect, num);
}

QRegion* qt_gui_c_QRegion_subtracted_as_ptr(const QRegion* this_ptr, const QRegion* r) {
  return new QRegion(this_ptr->subtracted(*r));
}

void qt_gui_c_QRegion_swap(QRegion* this_ptr, QRegion* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QRegion_translate_dx_dy(QRegion* this_ptr, int dx, int dy) {
  this_ptr->translate(dx, dy);
}

void qt_gui_c_QRegion_translate_p(QRegion* this_ptr, const QPoint* p) {
  this_ptr->translate(*p);
}

QRegion* qt_gui_c_QRegion_translated_as_ptr_dx_dy(const QRegion* this_ptr, int dx, int dy) {
  return new QRegion(this_ptr->translated(dx, dy));
}

QRegion* qt_gui_c_QRegion_translated_as_ptr_p(const QRegion* this_ptr, const QPoint* p) {
  return new QRegion(this_ptr->translated(*p));
}

QRegion* qt_gui_c_QRegion_united_as_ptr_QRect(const QRegion* this_ptr, const QRect* r) {
  return new QRegion(this_ptr->united(*r));
}

QRegion* qt_gui_c_QRegion_united_as_ptr_QRegion(const QRegion* this_ptr, const QRegion* r) {
  return new QRegion(this_ptr->united(*r));
}

QRegion* qt_gui_c_QRegion_xored_as_ptr(const QRegion* this_ptr, const QRegion* r) {
  return new QRegion(this_ptr->xored(*r));
}

