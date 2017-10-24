#include "qt_gui_c_QTransform.h"

void qt_gui_c_QTransform_G_operator_add_to_output(const QTransform* a, double n, QTransform* output) {
  new(output) QTransform(operator+(*a, n));
}

void qt_gui_c_QTransform_G_operator_div_to_output(const QTransform* a, double n, QTransform* output) {
  new(output) QTransform(operator/(*a, n));
}

QRegion* qt_gui_c_QTransform_G_operator_mul_as_ptr(const QRegion* r, const QTransform* m) {
  return new QRegion(operator*(*r, *m));
}

void qt_gui_c_QTransform_G_operator_mul_to_output_QLineF_QTransform(const QLineF* l, const QTransform* m, QLineF* output) {
  new(output) QLineF(operator*(*l, *m));
}

void qt_gui_c_QTransform_G_operator_mul_to_output_QLine_QTransform(const QLine* l, const QTransform* m, QLine* output) {
  new(output) QLine(operator*(*l, *m));
}

void qt_gui_c_QTransform_G_operator_mul_to_output_QPainterPath_QTransform(const QPainterPath* p, const QTransform* m, QPainterPath* output) {
  new(output) QPainterPath(operator*(*p, *m));
}

void qt_gui_c_QTransform_G_operator_mul_to_output_QPointF_QTransform(const QPointF* p, const QTransform* m, QPointF* output) {
  new(output) QPointF(operator*(*p, *m));
}

void qt_gui_c_QTransform_G_operator_mul_to_output_QPoint_QTransform(const QPoint* p, const QTransform* m, QPoint* output) {
  new(output) QPoint(operator*(*p, *m));
}

void qt_gui_c_QTransform_G_operator_mul_to_output_QPolygonF_QTransform(const QPolygonF* a, const QTransform* m, QPolygonF* output) {
  new(output) QPolygonF(operator*(*a, *m));
}

void qt_gui_c_QTransform_G_operator_mul_to_output_QPolygon_QTransform(const QPolygon* a, const QTransform* m, QPolygon* output) {
  new(output) QPolygon(operator*(*a, *m));
}

void qt_gui_c_QTransform_G_operator_mul_to_output_QTransform_double(const QTransform* a, double n, QTransform* output) {
  new(output) QTransform(operator*(*a, n));
}

QDataStream* qt_gui_c_QTransform_G_operator_shl(QDataStream* arg1, const QTransform* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_gui_c_QTransform_G_operator_shl_to_output(const QDebug* arg1, const QTransform* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_gui_c_QTransform_G_operator_shr(QDataStream* arg1, QTransform* arg2) {
  return &operator>>(*arg1, *arg2);
}

void qt_gui_c_QTransform_G_operator_sub_to_output(const QTransform* a, double n, QTransform* output) {
  new(output) QTransform(operator-(*a, n));
}

bool qt_gui_c_QTransform_G_qFuzzyCompare(const QTransform* t1, const QTransform* t2) {
  return qFuzzyCompare(*t1, *t2);
}

unsigned int qt_gui_c_QTransform_G_qHash_key(const QTransform* key) {
  return qHash(*key);
}

unsigned int qt_gui_c_QTransform_G_qHash_key_seed(const QTransform* key, unsigned int seed) {
  return qHash(*key, seed);
}

void qt_gui_c_QTransform_adjoint_to_output(const QTransform* this_ptr, QTransform* output) {
  new(output) QTransform(this_ptr->adjoint());
}

void qt_gui_c_QTransform_constructor_h11_h12_h13_h21_h22_h23_h31_h32(double h11, double h12, double h13, double h21, double h22, double h23, double h31, double h32, QTransform* output) {
  new(output) QTransform(h11, h12, h13, h21, h22, h23, h31, h32);
}

void qt_gui_c_QTransform_constructor_h11_h12_h13_h21_h22_h23_h31_h32_h33(double h11, double h12, double h13, double h21, double h22, double h23, double h31, double h32, double h33, QTransform* output) {
  new(output) QTransform(h11, h12, h13, h21, h22, h23, h31, h32, h33);
}

void qt_gui_c_QTransform_constructor_h11_h12_h21_h22_dx_dy(double h11, double h12, double h21, double h22, double dx, double dy, QTransform* output) {
  new(output) QTransform(h11, h12, h21, h22, dx, dy);
}

void qt_gui_c_QTransform_constructor_mtx(const QMatrix* mtx, QTransform* output) {
  new(output) QTransform(*mtx);
}

void qt_gui_c_QTransform_constructor_no_args(QTransform* output) {
  new(output) QTransform();
}

void qt_gui_c_QTransform_constructor_other(const QTransform* other, QTransform* output) {
  new(output) QTransform(*other);
}

void qt_gui_c_QTransform_convert_to_QVariant_to_output(const QTransform* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

void qt_gui_c_QTransform_destructor(QTransform* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

double qt_gui_c_QTransform_det(const QTransform* this_ptr) {
  return this_ptr->det();
}

double qt_gui_c_QTransform_determinant(const QTransform* this_ptr) {
  return this_ptr->determinant();
}

double qt_gui_c_QTransform_dx(const QTransform* this_ptr) {
  return this_ptr->dx();
}

double qt_gui_c_QTransform_dy(const QTransform* this_ptr) {
  return this_ptr->dy();
}

void qt_gui_c_QTransform_fromScale_to_output(double dx, double dy, QTransform* output) {
  new(output) QTransform(QTransform::fromScale(dx, dy));
}

void qt_gui_c_QTransform_fromTranslate_to_output(double dx, double dy, QTransform* output) {
  new(output) QTransform(QTransform::fromTranslate(dx, dy));
}

void qt_gui_c_QTransform_inverted_to_output_invertible(const QTransform* this_ptr, bool* invertible, QTransform* output) {
  new(output) QTransform(this_ptr->inverted(invertible));
}

void qt_gui_c_QTransform_inverted_to_output_no_args(const QTransform* this_ptr, QTransform* output) {
  new(output) QTransform(this_ptr->inverted());
}

bool qt_gui_c_QTransform_isAffine(const QTransform* this_ptr) {
  return this_ptr->isAffine();
}

bool qt_gui_c_QTransform_isIdentity(const QTransform* this_ptr) {
  return this_ptr->isIdentity();
}

bool qt_gui_c_QTransform_isInvertible(const QTransform* this_ptr) {
  return this_ptr->isInvertible();
}

bool qt_gui_c_QTransform_isRotating(const QTransform* this_ptr) {
  return this_ptr->isRotating();
}

bool qt_gui_c_QTransform_isScaling(const QTransform* this_ptr) {
  return this_ptr->isScaling();
}

bool qt_gui_c_QTransform_isTranslating(const QTransform* this_ptr) {
  return this_ptr->isTranslating();
}

double qt_gui_c_QTransform_m11(const QTransform* this_ptr) {
  return this_ptr->m11();
}

double qt_gui_c_QTransform_m12(const QTransform* this_ptr) {
  return this_ptr->m12();
}

double qt_gui_c_QTransform_m13(const QTransform* this_ptr) {
  return this_ptr->m13();
}

double qt_gui_c_QTransform_m21(const QTransform* this_ptr) {
  return this_ptr->m21();
}

double qt_gui_c_QTransform_m22(const QTransform* this_ptr) {
  return this_ptr->m22();
}

double qt_gui_c_QTransform_m23(const QTransform* this_ptr) {
  return this_ptr->m23();
}

double qt_gui_c_QTransform_m31(const QTransform* this_ptr) {
  return this_ptr->m31();
}

double qt_gui_c_QTransform_m32(const QTransform* this_ptr) {
  return this_ptr->m32();
}

double qt_gui_c_QTransform_m33(const QTransform* this_ptr) {
  return this_ptr->m33();
}

void qt_gui_c_QTransform_mapRect_to_output_QRect(const QTransform* this_ptr, const QRect* arg1, QRect* output) {
  new(output) QRect(this_ptr->mapRect(*arg1));
}

void qt_gui_c_QTransform_mapRect_to_output_QRectF(const QTransform* this_ptr, const QRectF* arg1, QRectF* output) {
  new(output) QRectF(this_ptr->mapRect(*arg1));
}

void qt_gui_c_QTransform_mapToPolygon_to_output(const QTransform* this_ptr, const QRect* r, QPolygon* output) {
  new(output) QPolygon(this_ptr->mapToPolygon(*r));
}

QRegion* qt_gui_c_QTransform_map_as_ptr(const QTransform* this_ptr, const QRegion* r) {
  return new QRegion(this_ptr->map(*r));
}

void qt_gui_c_QTransform_map_double_double_double_double(const QTransform* this_ptr, double x, double y, double* tx, double* ty) {
  this_ptr->map(x, y, tx, ty);
}

void qt_gui_c_QTransform_map_int_int_int_int(const QTransform* this_ptr, int x, int y, int* tx, int* ty) {
  this_ptr->map(x, y, tx, ty);
}

void qt_gui_c_QTransform_map_to_output_QLine(const QTransform* this_ptr, const QLine* l, QLine* output) {
  new(output) QLine(this_ptr->map(*l));
}

void qt_gui_c_QTransform_map_to_output_QLineF(const QTransform* this_ptr, const QLineF* l, QLineF* output) {
  new(output) QLineF(this_ptr->map(*l));
}

void qt_gui_c_QTransform_map_to_output_QPainterPath(const QTransform* this_ptr, const QPainterPath* p, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->map(*p));
}

void qt_gui_c_QTransform_map_to_output_QPoint(const QTransform* this_ptr, const QPoint* p, QPoint* output) {
  new(output) QPoint(this_ptr->map(*p));
}

void qt_gui_c_QTransform_map_to_output_QPointF(const QTransform* this_ptr, const QPointF* p, QPointF* output) {
  new(output) QPointF(this_ptr->map(*p));
}

void qt_gui_c_QTransform_map_to_output_QPolygon(const QTransform* this_ptr, const QPolygon* a, QPolygon* output) {
  new(output) QPolygon(this_ptr->map(*a));
}

void qt_gui_c_QTransform_map_to_output_QPolygonF(const QTransform* this_ptr, const QPolygonF* a, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->map(*a));
}

QTransform* qt_gui_c_QTransform_operator_add_assign(QTransform* this_ptr, double div) {
  return &this_ptr->operator+=(div);
}

QTransform* qt_gui_c_QTransform_operator_assign(QTransform* this_ptr, const QTransform* arg1) {
  return &this_ptr->operator=(*arg1);
}

QTransform* qt_gui_c_QTransform_operator_div_assign(QTransform* this_ptr, double div) {
  return &this_ptr->operator/=(div);
}

bool qt_gui_c_QTransform_operator_eq(const QTransform* this_ptr, const QTransform* arg1) {
  return this_ptr->operator==(*arg1);
}

QTransform* qt_gui_c_QTransform_operator_mul_assign_arg1(QTransform* this_ptr, const QTransform* arg1) {
  return &this_ptr->operator*=(*arg1);
}

QTransform* qt_gui_c_QTransform_operator_mul_assign_div(QTransform* this_ptr, double div) {
  return &this_ptr->operator*=(div);
}

void qt_gui_c_QTransform_operator_mul_to_output(const QTransform* this_ptr, const QTransform* o, QTransform* output) {
  new(output) QTransform(this_ptr->operator*(*o));
}

bool qt_gui_c_QTransform_operator_neq(const QTransform* this_ptr, const QTransform* arg1) {
  return this_ptr->operator!=(*arg1);
}

QTransform* qt_gui_c_QTransform_operator_sub_assign(QTransform* this_ptr, double div) {
  return &this_ptr->operator-=(div);
}

bool qt_gui_c_QTransform_quadToQuad(const QPolygonF* one, const QPolygonF* two, QTransform* result) {
  return QTransform::quadToQuad(*one, *two, *result);
}

bool qt_gui_c_QTransform_quadToSquare(const QPolygonF* quad, QTransform* result) {
  return QTransform::quadToSquare(*quad, *result);
}

void qt_gui_c_QTransform_reset(QTransform* this_ptr) {
  this_ptr->reset();
}

QTransform* qt_gui_c_QTransform_rotateRadians_a(QTransform* this_ptr, double a) {
  return &this_ptr->rotateRadians(a);
}

QTransform* qt_gui_c_QTransform_rotateRadians_a_axis(QTransform* this_ptr, double a, const Qt::Axis* axis) {
  return &this_ptr->rotateRadians(a, *axis);
}

QTransform* qt_gui_c_QTransform_rotate_a(QTransform* this_ptr, double a) {
  return &this_ptr->rotate(a);
}

QTransform* qt_gui_c_QTransform_rotate_a_axis(QTransform* this_ptr, double a, const Qt::Axis* axis) {
  return &this_ptr->rotate(a, *axis);
}

QTransform* qt_gui_c_QTransform_scale(QTransform* this_ptr, double sx, double sy) {
  return &this_ptr->scale(sx, sy);
}

void qt_gui_c_QTransform_setMatrix(QTransform* this_ptr, double m11, double m12, double m13, double m21, double m22, double m23, double m31, double m32, double m33) {
  this_ptr->setMatrix(m11, m12, m13, m21, m22, m23, m31, m32, m33);
}

QTransform* qt_gui_c_QTransform_shear(QTransform* this_ptr, double sh, double sv) {
  return &this_ptr->shear(sh, sv);
}

bool qt_gui_c_QTransform_squareToQuad(const QPolygonF* square, QTransform* result) {
  return QTransform::squareToQuad(*square, *result);
}

const QMatrix* qt_gui_c_QTransform_toAffine(const QTransform* this_ptr) {
  return &this_ptr->toAffine();
}

QTransform* qt_gui_c_QTransform_translate(QTransform* this_ptr, double dx, double dy) {
  return &this_ptr->translate(dx, dy);
}

void qt_gui_c_QTransform_transposed_to_output(const QTransform* this_ptr, QTransform* output) {
  new(output) QTransform(this_ptr->transposed());
}

QTransform::TransformationType qt_gui_c_QTransform_type(const QTransform* this_ptr) {
  return this_ptr->type();
}

