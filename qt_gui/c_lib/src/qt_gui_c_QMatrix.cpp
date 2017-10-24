#include "qt_gui_c_QMatrix.h"

QRegion* qt_gui_c_QMatrix_G_operator_mul_as_ptr(const QRegion* r, const QMatrix* m) {
  return new QRegion(operator*(*r, *m));
}

void qt_gui_c_QMatrix_G_operator_mul_to_output_QLineF_QMatrix(const QLineF* l, const QMatrix* m, QLineF* output) {
  new(output) QLineF(operator*(*l, *m));
}

void qt_gui_c_QMatrix_G_operator_mul_to_output_QLine_QMatrix(const QLine* l, const QMatrix* m, QLine* output) {
  new(output) QLine(operator*(*l, *m));
}

void qt_gui_c_QMatrix_G_operator_mul_to_output_QPainterPath_QMatrix(const QPainterPath* p, const QMatrix* m, QPainterPath* output) {
  new(output) QPainterPath(operator*(*p, *m));
}

void qt_gui_c_QMatrix_G_operator_mul_to_output_QPointF_QMatrix(const QPointF* p, const QMatrix* m, QPointF* output) {
  new(output) QPointF(operator*(*p, *m));
}

void qt_gui_c_QMatrix_G_operator_mul_to_output_QPoint_QMatrix(const QPoint* p, const QMatrix* m, QPoint* output) {
  new(output) QPoint(operator*(*p, *m));
}

void qt_gui_c_QMatrix_G_operator_mul_to_output_QPolygonF_QMatrix(const QPolygonF* a, const QMatrix* m, QPolygonF* output) {
  new(output) QPolygonF(operator*(*a, *m));
}

void qt_gui_c_QMatrix_G_operator_mul_to_output_QPolygon_QMatrix(const QPolygon* a, const QMatrix* m, QPolygon* output) {
  new(output) QPolygon(operator*(*a, *m));
}

QDataStream* qt_gui_c_QMatrix_G_operator_shl(QDataStream* arg1, const QMatrix* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_gui_c_QMatrix_G_operator_shl_to_output(const QDebug* arg1, const QMatrix* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_gui_c_QMatrix_G_operator_shr(QDataStream* arg1, QMatrix* arg2) {
  return &operator>>(*arg1, *arg2);
}

bool qt_gui_c_QMatrix_G_qFuzzyCompare(const QMatrix* m1, const QMatrix* m2) {
  return qFuzzyCompare(*m1, *m2);
}

unsigned int qt_gui_c_QMatrix_G_qHash_key(const QMatrix* key) {
  return qHash(*key);
}

unsigned int qt_gui_c_QMatrix_G_qHash_key_seed(const QMatrix* key, unsigned int seed) {
  return qHash(*key, seed);
}

void qt_gui_c_QMatrix_constructor_m11_m12_m21_m22_dx_dy(double m11, double m12, double m21, double m22, double dx, double dy, QMatrix* output) {
  new(output) QMatrix(m11, m12, m21, m22, dx, dy);
}

void qt_gui_c_QMatrix_constructor_no_args(QMatrix* output) {
  new(output) QMatrix();
}

void qt_gui_c_QMatrix_constructor_other(const QMatrix* other, QMatrix* output) {
  new(output) QMatrix(*other);
}

void qt_gui_c_QMatrix_convert_to_QVariant_to_output(const QMatrix* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

void qt_gui_c_QMatrix_destructor(QMatrix* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

double qt_gui_c_QMatrix_determinant(const QMatrix* this_ptr) {
  return this_ptr->determinant();
}

double qt_gui_c_QMatrix_dx(const QMatrix* this_ptr) {
  return this_ptr->dx();
}

double qt_gui_c_QMatrix_dy(const QMatrix* this_ptr) {
  return this_ptr->dy();
}

void qt_gui_c_QMatrix_inverted_to_output_invertible(const QMatrix* this_ptr, bool* invertible, QMatrix* output) {
  new(output) QMatrix(this_ptr->inverted(invertible));
}

void qt_gui_c_QMatrix_inverted_to_output_no_args(const QMatrix* this_ptr, QMatrix* output) {
  new(output) QMatrix(this_ptr->inverted());
}

bool qt_gui_c_QMatrix_isIdentity(const QMatrix* this_ptr) {
  return this_ptr->isIdentity();
}

bool qt_gui_c_QMatrix_isInvertible(const QMatrix* this_ptr) {
  return this_ptr->isInvertible();
}

double qt_gui_c_QMatrix_m11(const QMatrix* this_ptr) {
  return this_ptr->m11();
}

double qt_gui_c_QMatrix_m12(const QMatrix* this_ptr) {
  return this_ptr->m12();
}

double qt_gui_c_QMatrix_m21(const QMatrix* this_ptr) {
  return this_ptr->m21();
}

double qt_gui_c_QMatrix_m22(const QMatrix* this_ptr) {
  return this_ptr->m22();
}

void qt_gui_c_QMatrix_mapRect_to_output_QRect(const QMatrix* this_ptr, const QRect* arg1, QRect* output) {
  new(output) QRect(this_ptr->mapRect(*arg1));
}

void qt_gui_c_QMatrix_mapRect_to_output_QRectF(const QMatrix* this_ptr, const QRectF* arg1, QRectF* output) {
  new(output) QRectF(this_ptr->mapRect(*arg1));
}

void qt_gui_c_QMatrix_mapToPolygon_to_output(const QMatrix* this_ptr, const QRect* r, QPolygon* output) {
  new(output) QPolygon(this_ptr->mapToPolygon(*r));
}

QRegion* qt_gui_c_QMatrix_map_as_ptr(const QMatrix* this_ptr, const QRegion* r) {
  return new QRegion(this_ptr->map(*r));
}

void qt_gui_c_QMatrix_map_double_double_double_double(const QMatrix* this_ptr, double x, double y, double* tx, double* ty) {
  this_ptr->map(x, y, tx, ty);
}

void qt_gui_c_QMatrix_map_int_int_int_int(const QMatrix* this_ptr, int x, int y, int* tx, int* ty) {
  this_ptr->map(x, y, tx, ty);
}

void qt_gui_c_QMatrix_map_to_output_QLine(const QMatrix* this_ptr, const QLine* l, QLine* output) {
  new(output) QLine(this_ptr->map(*l));
}

void qt_gui_c_QMatrix_map_to_output_QLineF(const QMatrix* this_ptr, const QLineF* l, QLineF* output) {
  new(output) QLineF(this_ptr->map(*l));
}

void qt_gui_c_QMatrix_map_to_output_QPainterPath(const QMatrix* this_ptr, const QPainterPath* p, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->map(*p));
}

void qt_gui_c_QMatrix_map_to_output_QPoint(const QMatrix* this_ptr, const QPoint* p, QPoint* output) {
  new(output) QPoint(this_ptr->map(*p));
}

void qt_gui_c_QMatrix_map_to_output_QPointF(const QMatrix* this_ptr, const QPointF* p, QPointF* output) {
  new(output) QPointF(this_ptr->map(*p));
}

void qt_gui_c_QMatrix_map_to_output_QPolygon(const QMatrix* this_ptr, const QPolygon* a, QPolygon* output) {
  new(output) QPolygon(this_ptr->map(*a));
}

void qt_gui_c_QMatrix_map_to_output_QPolygonF(const QMatrix* this_ptr, const QPolygonF* a, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->map(*a));
}

QMatrix* qt_gui_c_QMatrix_operator_assign(QMatrix* this_ptr, const QMatrix* arg1) {
  return &this_ptr->operator=(*arg1);
}

bool qt_gui_c_QMatrix_operator_eq(const QMatrix* this_ptr, const QMatrix* arg1) {
  return this_ptr->operator==(*arg1);
}

QMatrix* qt_gui_c_QMatrix_operator_mul_assign(QMatrix* this_ptr, const QMatrix* arg1) {
  return &this_ptr->operator*=(*arg1);
}

void qt_gui_c_QMatrix_operator_mul_to_output(const QMatrix* this_ptr, const QMatrix* o, QMatrix* output) {
  new(output) QMatrix(this_ptr->operator*(*o));
}

bool qt_gui_c_QMatrix_operator_neq(const QMatrix* this_ptr, const QMatrix* arg1) {
  return this_ptr->operator!=(*arg1);
}

void qt_gui_c_QMatrix_reset(QMatrix* this_ptr) {
  this_ptr->reset();
}

QMatrix* qt_gui_c_QMatrix_rotate(QMatrix* this_ptr, double a) {
  return &this_ptr->rotate(a);
}

QMatrix* qt_gui_c_QMatrix_scale(QMatrix* this_ptr, double sx, double sy) {
  return &this_ptr->scale(sx, sy);
}

void qt_gui_c_QMatrix_setMatrix(QMatrix* this_ptr, double m11, double m12, double m21, double m22, double dx, double dy) {
  this_ptr->setMatrix(m11, m12, m21, m22, dx, dy);
}

QMatrix* qt_gui_c_QMatrix_shear(QMatrix* this_ptr, double sh, double sv) {
  return &this_ptr->shear(sh, sv);
}

QMatrix* qt_gui_c_QMatrix_translate(QMatrix* this_ptr, double dx, double dy) {
  return &this_ptr->translate(dx, dy);
}

