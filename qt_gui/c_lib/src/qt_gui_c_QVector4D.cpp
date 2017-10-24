#include "qt_gui_c_QVector4D.h"

QDataStream* qt_gui_c_QVector4D_G_operator_shl(QDataStream* arg1, const QVector4D* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_gui_c_QVector4D_G_operator_shl_to_output(const QDebug* dbg, const QVector4D* vector, QDebug* output) {
  new(output) QDebug(operator<<(*dbg, *vector));
}

QDataStream* qt_gui_c_QVector4D_G_operator_shr(QDataStream* arg1, QVector4D* arg2) {
  return &operator>>(*arg1, *arg2);
}

void qt_gui_c_QVector4D_convert_to_QVariant_to_output(const QVector4D* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

void qt_gui_c_QVector4D_delete(QVector4D* this_ptr) {
  delete this_ptr;
}

float qt_gui_c_QVector4D_dotProduct(const QVector4D* v1, const QVector4D* v2) {
  return QVector4D::dotProduct(*v1, *v2);
}

bool qt_gui_c_QVector4D_isNull(const QVector4D* this_ptr) {
  return this_ptr->isNull();
}

float qt_gui_c_QVector4D_length(const QVector4D* this_ptr) {
  return this_ptr->length();
}

float qt_gui_c_QVector4D_lengthSquared(const QVector4D* this_ptr) {
  return this_ptr->lengthSquared();
}

QVector4D* qt_gui_c_QVector4D_new_QPoint(const QPoint* point) {
  return new QVector4D(*point);
}

QVector4D* qt_gui_c_QVector4D_new_QPointF(const QPointF* point) {
  return new QVector4D(*point);
}

QVector4D* qt_gui_c_QVector4D_new_QVector2D(const QVector2D* vector) {
  return new QVector4D(*vector);
}

QVector4D* qt_gui_c_QVector4D_new_QVector2D_float_float(const QVector2D* vector, float zpos, float wpos) {
  return new QVector4D(*vector, zpos, wpos);
}

QVector4D* qt_gui_c_QVector4D_new_QVector3D(const QVector3D* vector) {
  return new QVector4D(*vector);
}

QVector4D* qt_gui_c_QVector4D_new_QVector3D_float(const QVector3D* vector, float wpos) {
  return new QVector4D(*vector, wpos);
}

QVector4D* qt_gui_c_QVector4D_new_float_float_float_float(float xpos, float ypos, float zpos, float wpos) {
  return new QVector4D(xpos, ypos, zpos, wpos);
}

QVector4D* qt_gui_c_QVector4D_new_no_args() {
  return new QVector4D();
}

void qt_gui_c_QVector4D_normalize(QVector4D* this_ptr) {
  this_ptr->normalize();
}

QVector4D* qt_gui_c_QVector4D_normalized_as_ptr(const QVector4D* this_ptr) {
  return new QVector4D(this_ptr->normalized());
}

QVector4D* qt_gui_c_QVector4D_operator_add_assign(QVector4D* this_ptr, const QVector4D* vector) {
  return &this_ptr->operator+=(*vector);
}

QVector4D* qt_gui_c_QVector4D_operator_div_assign_divisor(QVector4D* this_ptr, float divisor) {
  return &this_ptr->operator/=(divisor);
}

QVector4D* qt_gui_c_QVector4D_operator_div_assign_vector(QVector4D* this_ptr, const QVector4D* vector) {
  return &this_ptr->operator/=(*vector);
}

float* qt_gui_c_QVector4D_operator_index(QVector4D* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

float qt_gui_c_QVector4D_operator_index_const(const QVector4D* this_ptr, int i) {
  return this_ptr->operator[](i);
}

QVector4D* qt_gui_c_QVector4D_operator_mul_assign_factor(QVector4D* this_ptr, float factor) {
  return &this_ptr->operator*=(factor);
}

QVector4D* qt_gui_c_QVector4D_operator_mul_assign_vector(QVector4D* this_ptr, const QVector4D* vector) {
  return &this_ptr->operator*=(*vector);
}

QVector4D* qt_gui_c_QVector4D_operator_sub_assign(QVector4D* this_ptr, const QVector4D* vector) {
  return &this_ptr->operator-=(*vector);
}

void qt_gui_c_QVector4D_setW(QVector4D* this_ptr, float w) {
  this_ptr->setW(w);
}

void qt_gui_c_QVector4D_setX(QVector4D* this_ptr, float x) {
  this_ptr->setX(x);
}

void qt_gui_c_QVector4D_setY(QVector4D* this_ptr, float y) {
  this_ptr->setY(y);
}

void qt_gui_c_QVector4D_setZ(QVector4D* this_ptr, float z) {
  this_ptr->setZ(z);
}

void qt_gui_c_QVector4D_toPointF_to_output(const QVector4D* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->toPointF());
}

void qt_gui_c_QVector4D_toPoint_to_output(const QVector4D* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->toPoint());
}

QVector2D* qt_gui_c_QVector4D_toVector2DAffine_as_ptr(const QVector4D* this_ptr) {
  return new QVector2D(this_ptr->toVector2DAffine());
}

QVector2D* qt_gui_c_QVector4D_toVector2D_as_ptr(const QVector4D* this_ptr) {
  return new QVector2D(this_ptr->toVector2D());
}

QVector3D* qt_gui_c_QVector4D_toVector3DAffine_as_ptr(const QVector4D* this_ptr) {
  return new QVector3D(this_ptr->toVector3DAffine());
}

QVector3D* qt_gui_c_QVector4D_toVector3D_as_ptr(const QVector4D* this_ptr) {
  return new QVector3D(this_ptr->toVector3D());
}

float qt_gui_c_QVector4D_w(const QVector4D* this_ptr) {
  return this_ptr->w();
}

float qt_gui_c_QVector4D_x(const QVector4D* this_ptr) {
  return this_ptr->x();
}

float qt_gui_c_QVector4D_y(const QVector4D* this_ptr) {
  return this_ptr->y();
}

float qt_gui_c_QVector4D_z(const QVector4D* this_ptr) {
  return this_ptr->z();
}

