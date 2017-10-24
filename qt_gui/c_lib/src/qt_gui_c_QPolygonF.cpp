#include "qt_gui_c_QPolygonF.h"

QPolygonF* qt_gui_c_QPolygonF_G_static_cast_QPolygonF_ptr(QVector< QPointF >* ptr) {
  return static_cast<QPolygonF*>(ptr);
}

QVector< QPointF >* qt_gui_c_QPolygonF_G_static_cast_QVector_QPointF_ptr(QPolygonF* ptr) {
  return static_cast<QVector< QPointF >*>(ptr);
}

void qt_gui_c_QPolygonF_boundingRect_to_output(const QPolygonF* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect());
}

void qt_gui_c_QPolygonF_constructor_QPolygon(const QPolygon* a, QPolygonF* output) {
  new(output) QPolygonF(*a);
}

void qt_gui_c_QPolygonF_constructor_QPolygonF(const QPolygonF* a, QPolygonF* output) {
  new(output) QPolygonF(*a);
}

void qt_gui_c_QPolygonF_constructor_QRectF(const QRectF* r, QPolygonF* output) {
  new(output) QPolygonF(*r);
}

void qt_gui_c_QPolygonF_constructor_QVector_QPointF(const QVector< QPointF >* v, QPolygonF* output) {
  new(output) QPolygonF(*v);
}

void qt_gui_c_QPolygonF_constructor_int(int size, QPolygonF* output) {
  new(output) QPolygonF(size);
}

void qt_gui_c_QPolygonF_constructor_no_args(QPolygonF* output) {
  new(output) QPolygonF();
}

bool qt_gui_c_QPolygonF_containsPoint(const QPolygonF* this_ptr, const QPointF* pt, const Qt::FillRule* fillRule) {
  return this_ptr->containsPoint(*pt, *fillRule);
}

void qt_gui_c_QPolygonF_convert_to_QVariant_to_output(const QPolygonF* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

void qt_gui_c_QPolygonF_destructor(QPolygonF* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QPolygonF_intersected_to_output(const QPolygonF* this_ptr, const QPolygonF* r, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->intersected(*r));
}

bool qt_gui_c_QPolygonF_isClosed(const QPolygonF* this_ptr) {
  return this_ptr->isClosed();
}

QPolygonF* qt_gui_c_QPolygonF_operator_assign(QPolygonF* this_ptr, const QPolygonF* other) {
  return &this_ptr->operator=(*other);
}

void qt_gui_c_QPolygonF_subtracted_to_output(const QPolygonF* this_ptr, const QPolygonF* r, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->subtracted(*r));
}

void qt_gui_c_QPolygonF_swap(QPolygonF* this_ptr, QPolygonF* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QPolygonF_toPolygon_to_output(const QPolygonF* this_ptr, QPolygon* output) {
  new(output) QPolygon(this_ptr->toPolygon());
}

void qt_gui_c_QPolygonF_translate_dx_dy(QPolygonF* this_ptr, double dx, double dy) {
  this_ptr->translate(dx, dy);
}

void qt_gui_c_QPolygonF_translate_offset(QPolygonF* this_ptr, const QPointF* offset) {
  this_ptr->translate(*offset);
}

void qt_gui_c_QPolygonF_translated_to_output_dx_dy(const QPolygonF* this_ptr, double dx, double dy, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->translated(dx, dy));
}

void qt_gui_c_QPolygonF_translated_to_output_offset(const QPolygonF* this_ptr, const QPointF* offset, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->translated(*offset));
}

void qt_gui_c_QPolygonF_united_to_output(const QPolygonF* this_ptr, const QPolygonF* r, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->united(*r));
}

