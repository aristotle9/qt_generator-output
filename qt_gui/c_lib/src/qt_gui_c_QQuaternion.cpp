#include "qt_gui_c_QQuaternion.h"

QVector3D* qt_gui_c_QQuaternion_G_operator_mul_as_ptr(const QQuaternion* quaternion, const QVector3D* vec) {
  return new QVector3D(operator*(*quaternion, *vec));
}

QDataStream* qt_gui_c_QQuaternion_G_operator_shl(QDataStream* arg1, const QQuaternion* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_gui_c_QQuaternion_G_operator_shl_to_output(const QDebug* dbg, const QQuaternion* q, QDebug* output) {
  new(output) QDebug(operator<<(*dbg, *q));
}

QDataStream* qt_gui_c_QQuaternion_G_operator_shr(QDataStream* arg1, QQuaternion* arg2) {
  return &operator>>(*arg1, *arg2);
}

void qt_gui_c_QQuaternion_conjugate_to_output(const QQuaternion* this_ptr, QQuaternion* output) {
  new(output) QQuaternion(this_ptr->conjugate());
}

void qt_gui_c_QQuaternion_conjugated_to_output(const QQuaternion* this_ptr, QQuaternion* output) {
  new(output) QQuaternion(this_ptr->conjugated());
}

void qt_gui_c_QQuaternion_constructor_no_args(QQuaternion* output) {
  new(output) QQuaternion();
}

void qt_gui_c_QQuaternion_constructor_scalar_vector(float scalar, const QVector3D* vector, QQuaternion* output) {
  new(output) QQuaternion(scalar, *vector);
}

void qt_gui_c_QQuaternion_constructor_scalar_xpos_ypos_zpos(float scalar, float xpos, float ypos, float zpos, QQuaternion* output) {
  new(output) QQuaternion(scalar, xpos, ypos, zpos);
}

void qt_gui_c_QQuaternion_constructor_vector(const QVector4D* vector, QQuaternion* output) {
  new(output) QQuaternion(*vector);
}

void qt_gui_c_QQuaternion_convert_to_QVariant_to_output(const QQuaternion* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

void qt_gui_c_QQuaternion_destructor(QQuaternion* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

float qt_gui_c_QQuaternion_dotProduct(const QQuaternion* q1, const QQuaternion* q2) {
  return QQuaternion::dotProduct(*q1, *q2);
}

void qt_gui_c_QQuaternion_fromAxes_to_output(const QVector3D* xAxis, const QVector3D* yAxis, const QVector3D* zAxis, QQuaternion* output) {
  new(output) QQuaternion(QQuaternion::fromAxes(*xAxis, *yAxis, *zAxis));
}

void qt_gui_c_QQuaternion_fromAxisAndAngle_to_output_axis_angle(const QVector3D* axis, float angle, QQuaternion* output) {
  new(output) QQuaternion(QQuaternion::fromAxisAndAngle(*axis, angle));
}

void qt_gui_c_QQuaternion_fromAxisAndAngle_to_output_x_y_z_angle(float x, float y, float z, float angle, QQuaternion* output) {
  new(output) QQuaternion(QQuaternion::fromAxisAndAngle(x, y, z, angle));
}

void qt_gui_c_QQuaternion_fromDirection_to_output(const QVector3D* direction, const QVector3D* up, QQuaternion* output) {
  new(output) QQuaternion(QQuaternion::fromDirection(*direction, *up));
}

void qt_gui_c_QQuaternion_fromEulerAngles_to_output_eulerAngles(const QVector3D* eulerAngles, QQuaternion* output) {
  new(output) QQuaternion(QQuaternion::fromEulerAngles(*eulerAngles));
}

void qt_gui_c_QQuaternion_fromEulerAngles_to_output_pitch_yaw_roll(float pitch, float yaw, float roll, QQuaternion* output) {
  new(output) QQuaternion(QQuaternion::fromEulerAngles(pitch, yaw, roll));
}

void qt_gui_c_QQuaternion_getAxes(const QQuaternion* this_ptr, QVector3D* xAxis, QVector3D* yAxis, QVector3D* zAxis) {
  this_ptr->getAxes(xAxis, yAxis, zAxis);
}

void qt_gui_c_QQuaternion_getAxisAndAngle_axis_angle(const QQuaternion* this_ptr, QVector3D* axis, float* angle) {
  this_ptr->getAxisAndAngle(axis, angle);
}

void qt_gui_c_QQuaternion_getAxisAndAngle_x_y_z_angle(const QQuaternion* this_ptr, float* x, float* y, float* z, float* angle) {
  this_ptr->getAxisAndAngle(x, y, z, angle);
}

void qt_gui_c_QQuaternion_getEulerAngles(const QQuaternion* this_ptr, float* pitch, float* yaw, float* roll) {
  this_ptr->getEulerAngles(pitch, yaw, roll);
}

void qt_gui_c_QQuaternion_inverted_to_output(const QQuaternion* this_ptr, QQuaternion* output) {
  new(output) QQuaternion(this_ptr->inverted());
}

bool qt_gui_c_QQuaternion_isIdentity(const QQuaternion* this_ptr) {
  return this_ptr->isIdentity();
}

bool qt_gui_c_QQuaternion_isNull(const QQuaternion* this_ptr) {
  return this_ptr->isNull();
}

float qt_gui_c_QQuaternion_length(const QQuaternion* this_ptr) {
  return this_ptr->length();
}

float qt_gui_c_QQuaternion_lengthSquared(const QQuaternion* this_ptr) {
  return this_ptr->lengthSquared();
}

void qt_gui_c_QQuaternion_nlerp_to_output(const QQuaternion* q1, const QQuaternion* q2, float t, QQuaternion* output) {
  new(output) QQuaternion(QQuaternion::nlerp(*q1, *q2, t));
}

void qt_gui_c_QQuaternion_normalize(QQuaternion* this_ptr) {
  this_ptr->normalize();
}

void qt_gui_c_QQuaternion_normalized_to_output(const QQuaternion* this_ptr, QQuaternion* output) {
  new(output) QQuaternion(this_ptr->normalized());
}

QQuaternion* qt_gui_c_QQuaternion_operator_add_assign(QQuaternion* this_ptr, const QQuaternion* quaternion) {
  return &this_ptr->operator+=(*quaternion);
}

QQuaternion* qt_gui_c_QQuaternion_operator_div_assign(QQuaternion* this_ptr, float divisor) {
  return &this_ptr->operator/=(divisor);
}

QQuaternion* qt_gui_c_QQuaternion_operator_mul_assign_factor(QQuaternion* this_ptr, float factor) {
  return &this_ptr->operator*=(factor);
}

QQuaternion* qt_gui_c_QQuaternion_operator_mul_assign_quaternion(QQuaternion* this_ptr, const QQuaternion* quaternion) {
  return &this_ptr->operator*=(*quaternion);
}

QQuaternion* qt_gui_c_QQuaternion_operator_sub_assign(QQuaternion* this_ptr, const QQuaternion* quaternion) {
  return &this_ptr->operator-=(*quaternion);
}

QVector3D* qt_gui_c_QQuaternion_rotatedVector_as_ptr(const QQuaternion* this_ptr, const QVector3D* vector) {
  return new QVector3D(this_ptr->rotatedVector(*vector));
}

void qt_gui_c_QQuaternion_rotationTo_to_output(const QVector3D* from, const QVector3D* to, QQuaternion* output) {
  new(output) QQuaternion(QQuaternion::rotationTo(*from, *to));
}

float qt_gui_c_QQuaternion_scalar(const QQuaternion* this_ptr) {
  return this_ptr->scalar();
}

void qt_gui_c_QQuaternion_setScalar(QQuaternion* this_ptr, float scalar) {
  this_ptr->setScalar(scalar);
}

void qt_gui_c_QQuaternion_setVector_vector(QQuaternion* this_ptr, const QVector3D* vector) {
  this_ptr->setVector(*vector);
}

void qt_gui_c_QQuaternion_setVector_x_y_z(QQuaternion* this_ptr, float x, float y, float z) {
  this_ptr->setVector(x, y, z);
}

void qt_gui_c_QQuaternion_setX(QQuaternion* this_ptr, float x) {
  this_ptr->setX(x);
}

void qt_gui_c_QQuaternion_setY(QQuaternion* this_ptr, float y) {
  this_ptr->setY(y);
}

void qt_gui_c_QQuaternion_setZ(QQuaternion* this_ptr, float z) {
  this_ptr->setZ(z);
}

void qt_gui_c_QQuaternion_slerp_to_output(const QQuaternion* q1, const QQuaternion* q2, float t, QQuaternion* output) {
  new(output) QQuaternion(QQuaternion::slerp(*q1, *q2, t));
}

QVector3D* qt_gui_c_QQuaternion_toEulerAngles_as_ptr(const QQuaternion* this_ptr) {
  return new QVector3D(this_ptr->toEulerAngles());
}

QVector4D* qt_gui_c_QQuaternion_toVector4D_as_ptr(const QQuaternion* this_ptr) {
  return new QVector4D(this_ptr->toVector4D());
}

QVector3D* qt_gui_c_QQuaternion_vector_as_ptr(const QQuaternion* this_ptr) {
  return new QVector3D(this_ptr->vector());
}

float qt_gui_c_QQuaternion_x(const QQuaternion* this_ptr) {
  return this_ptr->x();
}

float qt_gui_c_QQuaternion_y(const QQuaternion* this_ptr) {
  return this_ptr->y();
}

float qt_gui_c_QQuaternion_z(const QQuaternion* this_ptr) {
  return this_ptr->z();
}

