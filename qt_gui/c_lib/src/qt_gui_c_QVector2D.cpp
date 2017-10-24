#include "qt_gui_c_QVector2D.h"

QDataStream* qt_gui_c_QVector2D_G_operator_shl(QDataStream* arg1, const QVector2D* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_gui_c_QVector2D_G_operator_shl_to_output(const QDebug* dbg, const QVector2D* vector, QDebug* output) {
  new(output) QDebug(operator<<(*dbg, *vector));
}

QDataStream* qt_gui_c_QVector2D_G_operator_shr(QDataStream* arg1, QVector2D* arg2) {
  return &operator>>(*arg1, *arg2);
}

void qt_gui_c_QVector2D_convert_to_QVariant_to_output(const QVector2D* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

void qt_gui_c_QVector2D_delete(QVector2D* this_ptr) {
  delete this_ptr;
}

float qt_gui_c_QVector2D_distanceToLine(const QVector2D* this_ptr, const QVector2D* point, const QVector2D* direction) {
  return this_ptr->distanceToLine(*point, *direction);
}

float qt_gui_c_QVector2D_distanceToPoint(const QVector2D* this_ptr, const QVector2D* point) {
  return this_ptr->distanceToPoint(*point);
}

float qt_gui_c_QVector2D_dotProduct(const QVector2D* v1, const QVector2D* v2) {
  return QVector2D::dotProduct(*v1, *v2);
}

bool qt_gui_c_QVector2D_isNull(const QVector2D* this_ptr) {
  return this_ptr->isNull();
}

float qt_gui_c_QVector2D_length(const QVector2D* this_ptr) {
  return this_ptr->length();
}

float qt_gui_c_QVector2D_lengthSquared(const QVector2D* this_ptr) {
  return this_ptr->lengthSquared();
}

QVector2D* qt_gui_c_QVector2D_new_QPoint(const QPoint* point) {
  return new QVector2D(*point);
}

QVector2D* qt_gui_c_QVector2D_new_QPointF(const QPointF* point) {
  return new QVector2D(*point);
}

QVector2D* qt_gui_c_QVector2D_new_QVector3D(const QVector3D* vector) {
  return new QVector2D(*vector);
}

QVector2D* qt_gui_c_QVector2D_new_QVector4D(const QVector4D* vector) {
  return new QVector2D(*vector);
}

QVector2D* qt_gui_c_QVector2D_new_float_float(float xpos, float ypos) {
  return new QVector2D(xpos, ypos);
}

QVector2D* qt_gui_c_QVector2D_new_no_args() {
  return new QVector2D();
}

void qt_gui_c_QVector2D_normalize(QVector2D* this_ptr) {
  this_ptr->normalize();
}

QVector2D* qt_gui_c_QVector2D_normalized_as_ptr(const QVector2D* this_ptr) {
  return new QVector2D(this_ptr->normalized());
}

QVector2D* qt_gui_c_QVector2D_operator_add_assign(QVector2D* this_ptr, const QVector2D* vector) {
  return &this_ptr->operator+=(*vector);
}

QVector2D* qt_gui_c_QVector2D_operator_div_assign_divisor(QVector2D* this_ptr, float divisor) {
  return &this_ptr->operator/=(divisor);
}

QVector2D* qt_gui_c_QVector2D_operator_div_assign_vector(QVector2D* this_ptr, const QVector2D* vector) {
  return &this_ptr->operator/=(*vector);
}

float* qt_gui_c_QVector2D_operator_index(QVector2D* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

float qt_gui_c_QVector2D_operator_index_const(const QVector2D* this_ptr, int i) {
  return this_ptr->operator[](i);
}

QVector2D* qt_gui_c_QVector2D_operator_mul_assign_factor(QVector2D* this_ptr, float factor) {
  return &this_ptr->operator*=(factor);
}

QVector2D* qt_gui_c_QVector2D_operator_mul_assign_vector(QVector2D* this_ptr, const QVector2D* vector) {
  return &this_ptr->operator*=(*vector);
}

QVector2D* qt_gui_c_QVector2D_operator_sub_assign(QVector2D* this_ptr, const QVector2D* vector) {
  return &this_ptr->operator-=(*vector);
}

void qt_gui_c_QVector2D_setX(QVector2D* this_ptr, float x) {
  this_ptr->setX(x);
}

void qt_gui_c_QVector2D_setY(QVector2D* this_ptr, float y) {
  this_ptr->setY(y);
}

void qt_gui_c_QVector2D_toPointF_to_output(const QVector2D* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->toPointF());
}

void qt_gui_c_QVector2D_toPoint_to_output(const QVector2D* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->toPoint());
}

QVector3D* qt_gui_c_QVector2D_toVector3D_as_ptr(const QVector2D* this_ptr) {
  return new QVector3D(this_ptr->toVector3D());
}

QVector4D* qt_gui_c_QVector2D_toVector4D_as_ptr(const QVector2D* this_ptr) {
  return new QVector4D(this_ptr->toVector4D());
}

float qt_gui_c_QVector2D_x(const QVector2D* this_ptr) {
  return this_ptr->x();
}

float qt_gui_c_QVector2D_y(const QVector2D* this_ptr) {
  return this_ptr->y();
}

