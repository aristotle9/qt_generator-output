#include "qt_gui_c_QPolygon.h"

QDataStream* qt_gui_c_QPolygon_G_operator_shl_stream_array(QDataStream* stream, const QPolygonF* array) {
  return &operator<<(*stream, *array);
}

QDataStream* qt_gui_c_QPolygon_G_operator_shl_stream_polygon(QDataStream* stream, const QPolygon* polygon) {
  return &operator<<(*stream, *polygon);
}

void qt_gui_c_QPolygon_G_operator_shl_to_output_QDebug_QPolygon(const QDebug* arg1, const QPolygon* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

void qt_gui_c_QPolygon_G_operator_shl_to_output_QDebug_QPolygonF(const QDebug* arg1, const QPolygonF* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_gui_c_QPolygon_G_operator_shr_stream_array(QDataStream* stream, QPolygonF* array) {
  return &operator>>(*stream, *array);
}

QDataStream* qt_gui_c_QPolygon_G_operator_shr_stream_polygon(QDataStream* stream, QPolygon* polygon) {
  return &operator>>(*stream, *polygon);
}

QPolygon* qt_gui_c_QPolygon_G_static_cast_QPolygon_ptr(QVector< QPoint >* ptr) {
  return static_cast<QPolygon*>(ptr);
}

QVector< QPoint >* qt_gui_c_QPolygon_G_static_cast_QVector_QPoint_ptr(QPolygon* ptr) {
  return static_cast<QVector< QPoint >*>(ptr);
}

void qt_gui_c_QPolygon_G_swap_QPolygonF_QPolygonF(QPolygonF* value1, QPolygonF* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QPolygon_G_swap_QPolygon_QPolygon(QPolygon* value1, QPolygon* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QPolygon_boundingRect_to_output(const QPolygon* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->boundingRect());
}

void qt_gui_c_QPolygon_constructor_nPoints_points(int nPoints, const int* points, QPolygon* output) {
  new(output) QPolygon(nPoints, points);
}

void qt_gui_c_QPolygon_constructor_no_args(QPolygon* output) {
  new(output) QPolygon();
}

void qt_gui_c_QPolygon_constructor_other(const QPolygon* other, QPolygon* output) {
  new(output) QPolygon(*other);
}

void qt_gui_c_QPolygon_constructor_r(const QRect* r, QPolygon* output) {
  new(output) QPolygon(*r);
}

void qt_gui_c_QPolygon_constructor_r_closed(const QRect* r, bool closed, QPolygon* output) {
  new(output) QPolygon(*r, closed);
}

void qt_gui_c_QPolygon_constructor_size(int size, QPolygon* output) {
  new(output) QPolygon(size);
}

void qt_gui_c_QPolygon_constructor_v(const QVector< QPoint >* v, QPolygon* output) {
  new(output) QPolygon(*v);
}

bool qt_gui_c_QPolygon_containsPoint(const QPolygon* this_ptr, const QPoint* pt, const Qt::FillRule* fillRule) {
  return this_ptr->containsPoint(*pt, *fillRule);
}

void qt_gui_c_QPolygon_convert_to_QVariant_to_output(const QPolygon* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

void qt_gui_c_QPolygon_destructor(QPolygon* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QPolygon_intersected_to_output(const QPolygon* this_ptr, const QPolygon* r, QPolygon* output) {
  new(output) QPolygon(this_ptr->intersected(*r));
}

QPolygon* qt_gui_c_QPolygon_operator_assign(QPolygon* this_ptr, const QPolygon* other) {
  return &this_ptr->operator=(*other);
}

void qt_gui_c_QPolygon_point(const QPolygon* this_ptr, int i, int* x, int* y) {
  this_ptr->point(i, x, y);
}

void qt_gui_c_QPolygon_point_to_output(const QPolygon* this_ptr, int i, QPoint* output) {
  new(output) QPoint(this_ptr->point(i));
}

void qt_gui_c_QPolygon_putPoints_index_nPoints_from(QPolygon* this_ptr, int index, int nPoints, const QPolygon* from) {
  this_ptr->putPoints(index, nPoints, *from);
}

void qt_gui_c_QPolygon_putPoints_index_nPoints_from_fromIndex(QPolygon* this_ptr, int index, int nPoints, const QPolygon* from, int fromIndex) {
  this_ptr->putPoints(index, nPoints, *from, fromIndex);
}

void qt_gui_c_QPolygon_putPoints_index_nPoints_points(QPolygon* this_ptr, int index, int nPoints, const int* points) {
  this_ptr->putPoints(index, nPoints, points);
}

void qt_gui_c_QPolygon_setPoint_index_p(QPolygon* this_ptr, int index, const QPoint* p) {
  this_ptr->setPoint(index, *p);
}

void qt_gui_c_QPolygon_setPoint_index_x_y(QPolygon* this_ptr, int index, int x, int y) {
  this_ptr->setPoint(index, x, y);
}

void qt_gui_c_QPolygon_setPoints(QPolygon* this_ptr, int nPoints, const int* points) {
  this_ptr->setPoints(nPoints, points);
}

void qt_gui_c_QPolygon_subtracted_to_output(const QPolygon* this_ptr, const QPolygon* r, QPolygon* output) {
  new(output) QPolygon(this_ptr->subtracted(*r));
}

void qt_gui_c_QPolygon_swap(QPolygon* this_ptr, QPolygon* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QPolygon_translate_dx_dy(QPolygon* this_ptr, int dx, int dy) {
  this_ptr->translate(dx, dy);
}

void qt_gui_c_QPolygon_translate_offset(QPolygon* this_ptr, const QPoint* offset) {
  this_ptr->translate(*offset);
}

void qt_gui_c_QPolygon_translated_to_output_dx_dy(const QPolygon* this_ptr, int dx, int dy, QPolygon* output) {
  new(output) QPolygon(this_ptr->translated(dx, dy));
}

void qt_gui_c_QPolygon_translated_to_output_offset(const QPolygon* this_ptr, const QPoint* offset, QPolygon* output) {
  new(output) QPolygon(this_ptr->translated(*offset));
}

void qt_gui_c_QPolygon_united_to_output(const QPolygon* this_ptr, const QPolygon* r, QPolygon* output) {
  new(output) QPolygon(this_ptr->united(*r));
}

