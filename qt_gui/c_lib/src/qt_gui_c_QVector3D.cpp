#include "qt_gui_c_QVector3D.h"

QDataStream* qt_gui_c_QVector3D_G_operator_shl(QDataStream* arg1, const QVector3D* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_gui_c_QVector3D_G_operator_shl_to_output(const QDebug* dbg, const QVector3D* vector, QDebug* output) {
  new(output) QDebug(operator<<(*dbg, *vector));
}

QDataStream* qt_gui_c_QVector3D_G_operator_shr(QDataStream* arg1, QVector3D* arg2) {
  return &operator>>(*arg1, *arg2);
}

void qt_gui_c_QVector3D_convert_to_QVariant_to_output(const QVector3D* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

QVector3D* qt_gui_c_QVector3D_crossProduct_as_ptr(const QVector3D* v1, const QVector3D* v2) {
  return new QVector3D(QVector3D::crossProduct(*v1, *v2));
}

void qt_gui_c_QVector3D_delete(QVector3D* this_ptr) {
  delete this_ptr;
}

float qt_gui_c_QVector3D_distanceToLine(const QVector3D* this_ptr, const QVector3D* point, const QVector3D* direction) {
  return this_ptr->distanceToLine(*point, *direction);
}

float qt_gui_c_QVector3D_distanceToPlane_plane1_plane2_plane3(const QVector3D* this_ptr, const QVector3D* plane1, const QVector3D* plane2, const QVector3D* plane3) {
  return this_ptr->distanceToPlane(*plane1, *plane2, *plane3);
}

float qt_gui_c_QVector3D_distanceToPlane_plane_normal(const QVector3D* this_ptr, const QVector3D* plane, const QVector3D* normal) {
  return this_ptr->distanceToPlane(*plane, *normal);
}

float qt_gui_c_QVector3D_distanceToPoint(const QVector3D* this_ptr, const QVector3D* point) {
  return this_ptr->distanceToPoint(*point);
}

float qt_gui_c_QVector3D_dotProduct(const QVector3D* v1, const QVector3D* v2) {
  return QVector3D::dotProduct(*v1, *v2);
}

bool qt_gui_c_QVector3D_isNull(const QVector3D* this_ptr) {
  return this_ptr->isNull();
}

float qt_gui_c_QVector3D_length(const QVector3D* this_ptr) {
  return this_ptr->length();
}

float qt_gui_c_QVector3D_lengthSquared(const QVector3D* this_ptr) {
  return this_ptr->lengthSquared();
}

QVector3D* qt_gui_c_QVector3D_new_QPoint(const QPoint* point) {
  return new QVector3D(*point);
}

QVector3D* qt_gui_c_QVector3D_new_QPointF(const QPointF* point) {
  return new QVector3D(*point);
}

QVector3D* qt_gui_c_QVector3D_new_QVector2D(const QVector2D* vector) {
  return new QVector3D(*vector);
}

QVector3D* qt_gui_c_QVector3D_new_QVector2D_float(const QVector2D* vector, float zpos) {
  return new QVector3D(*vector, zpos);
}

QVector3D* qt_gui_c_QVector3D_new_QVector4D(const QVector4D* vector) {
  return new QVector3D(*vector);
}

QVector3D* qt_gui_c_QVector3D_new_float_float_float(float xpos, float ypos, float zpos) {
  return new QVector3D(xpos, ypos, zpos);
}

QVector3D* qt_gui_c_QVector3D_new_no_args() {
  return new QVector3D();
}

QVector3D* qt_gui_c_QVector3D_normal_as_ptr_v1_v2(const QVector3D* v1, const QVector3D* v2) {
  return new QVector3D(QVector3D::normal(*v1, *v2));
}

QVector3D* qt_gui_c_QVector3D_normal_as_ptr_v1_v2_v3(const QVector3D* v1, const QVector3D* v2, const QVector3D* v3) {
  return new QVector3D(QVector3D::normal(*v1, *v2, *v3));
}

void qt_gui_c_QVector3D_normalize(QVector3D* this_ptr) {
  this_ptr->normalize();
}

QVector3D* qt_gui_c_QVector3D_normalized_as_ptr(const QVector3D* this_ptr) {
  return new QVector3D(this_ptr->normalized());
}

QVector3D* qt_gui_c_QVector3D_operator_add_assign(QVector3D* this_ptr, const QVector3D* vector) {
  return &this_ptr->operator+=(*vector);
}

QVector3D* qt_gui_c_QVector3D_operator_div_assign_divisor(QVector3D* this_ptr, float divisor) {
  return &this_ptr->operator/=(divisor);
}

QVector3D* qt_gui_c_QVector3D_operator_div_assign_vector(QVector3D* this_ptr, const QVector3D* vector) {
  return &this_ptr->operator/=(*vector);
}

float* qt_gui_c_QVector3D_operator_index(QVector3D* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

float qt_gui_c_QVector3D_operator_index_const(const QVector3D* this_ptr, int i) {
  return this_ptr->operator[](i);
}

QVector3D* qt_gui_c_QVector3D_operator_mul_assign_factor(QVector3D* this_ptr, float factor) {
  return &this_ptr->operator*=(factor);
}

QVector3D* qt_gui_c_QVector3D_operator_mul_assign_vector(QVector3D* this_ptr, const QVector3D* vector) {
  return &this_ptr->operator*=(*vector);
}

QVector3D* qt_gui_c_QVector3D_operator_sub_assign(QVector3D* this_ptr, const QVector3D* vector) {
  return &this_ptr->operator-=(*vector);
}

QVector3D* qt_gui_c_QVector3D_project_as_ptr(const QVector3D* this_ptr, const QMatrix4x4* modelView, const QMatrix4x4* projection, const QRect* viewport) {
  return new QVector3D(this_ptr->project(*modelView, *projection, *viewport));
}

void qt_gui_c_QVector3D_setX(QVector3D* this_ptr, float x) {
  this_ptr->setX(x);
}

void qt_gui_c_QVector3D_setY(QVector3D* this_ptr, float y) {
  this_ptr->setY(y);
}

void qt_gui_c_QVector3D_setZ(QVector3D* this_ptr, float z) {
  this_ptr->setZ(z);
}

void qt_gui_c_QVector3D_toPointF_to_output(const QVector3D* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->toPointF());
}

void qt_gui_c_QVector3D_toPoint_to_output(const QVector3D* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->toPoint());
}

QVector2D* qt_gui_c_QVector3D_toVector2D_as_ptr(const QVector3D* this_ptr) {
  return new QVector2D(this_ptr->toVector2D());
}

QVector4D* qt_gui_c_QVector3D_toVector4D_as_ptr(const QVector3D* this_ptr) {
  return new QVector4D(this_ptr->toVector4D());
}

QVector3D* qt_gui_c_QVector3D_unproject_as_ptr(const QVector3D* this_ptr, const QMatrix4x4* modelView, const QMatrix4x4* projection, const QRect* viewport) {
  return new QVector3D(this_ptr->unproject(*modelView, *projection, *viewport));
}

float qt_gui_c_QVector3D_x(const QVector3D* this_ptr) {
  return this_ptr->x();
}

float qt_gui_c_QVector3D_y(const QVector3D* this_ptr) {
  return this_ptr->y();
}

float qt_gui_c_QVector3D_z(const QVector3D* this_ptr) {
  return this_ptr->z();
}

