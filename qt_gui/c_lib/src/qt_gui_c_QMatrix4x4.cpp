#include "qt_gui_c_QMatrix4x4.h"

QDataStream* qt_gui_c_QMatrix4x4_G_operator_shl(QDataStream* arg1, const QMatrix4x4* arg2) {
  return &operator<<(*arg1, *arg2);
}

QDataStream* qt_gui_c_QMatrix4x4_G_operator_shr(QDataStream* arg1, QMatrix4x4* arg2) {
  return &operator>>(*arg1, *arg2);
}

QVector4D* qt_gui_c_QMatrix4x4_column_as_ptr(const QMatrix4x4* this_ptr, int index) {
  return new QVector4D(this_ptr->column(index));
}

const float* qt_gui_c_QMatrix4x4_constData(const QMatrix4x4* this_ptr) {
  return this_ptr->constData();
}

void qt_gui_c_QMatrix4x4_convert_to_QVariant_to_output(const QMatrix4x4* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

void qt_gui_c_QMatrix4x4_copyDataTo(const QMatrix4x4* this_ptr, float* values) {
  this_ptr->copyDataTo(values);
}

float* qt_gui_c_QMatrix4x4_data(QMatrix4x4* this_ptr) {
  return this_ptr->data();
}

const float* qt_gui_c_QMatrix4x4_data_const(const QMatrix4x4* this_ptr) {
  return this_ptr->data();
}

void qt_gui_c_QMatrix4x4_delete(QMatrix4x4* this_ptr) {
  delete this_ptr;
}

double qt_gui_c_QMatrix4x4_determinant(const QMatrix4x4* this_ptr) {
  return this_ptr->determinant();
}

void qt_gui_c_QMatrix4x4_fill(QMatrix4x4* this_ptr, float value) {
  this_ptr->fill(value);
}

void qt_gui_c_QMatrix4x4_flipCoordinates(QMatrix4x4* this_ptr) {
  this_ptr->flipCoordinates();
}

void qt_gui_c_QMatrix4x4_frustum(QMatrix4x4* this_ptr, float left, float right, float bottom, float top, float nearPlane, float farPlane) {
  this_ptr->frustum(left, right, bottom, top, nearPlane, farPlane);
}

QMatrix4x4* qt_gui_c_QMatrix4x4_inverted_as_ptr_invertible(const QMatrix4x4* this_ptr, bool* invertible) {
  return new QMatrix4x4(this_ptr->inverted(invertible));
}

QMatrix4x4* qt_gui_c_QMatrix4x4_inverted_as_ptr_no_args(const QMatrix4x4* this_ptr) {
  return new QMatrix4x4(this_ptr->inverted());
}

bool qt_gui_c_QMatrix4x4_isAffine(const QMatrix4x4* this_ptr) {
  return this_ptr->isAffine();
}

bool qt_gui_c_QMatrix4x4_isIdentity(const QMatrix4x4* this_ptr) {
  return this_ptr->isIdentity();
}

void qt_gui_c_QMatrix4x4_lookAt(QMatrix4x4* this_ptr, const QVector3D* eye, const QVector3D* center, const QVector3D* up) {
  this_ptr->lookAt(*eye, *center, *up);
}

void qt_gui_c_QMatrix4x4_mapRect_to_output_QRect(const QMatrix4x4* this_ptr, const QRect* rect, QRect* output) {
  new(output) QRect(this_ptr->mapRect(*rect));
}

void qt_gui_c_QMatrix4x4_mapRect_to_output_QRectF(const QMatrix4x4* this_ptr, const QRectF* rect, QRectF* output) {
  new(output) QRectF(this_ptr->mapRect(*rect));
}

QVector3D* qt_gui_c_QMatrix4x4_mapVector_as_ptr(const QMatrix4x4* this_ptr, const QVector3D* vector) {
  return new QVector3D(this_ptr->mapVector(*vector));
}

QVector3D* qt_gui_c_QMatrix4x4_map_as_ptr_QVector3D(const QMatrix4x4* this_ptr, const QVector3D* point) {
  return new QVector3D(this_ptr->map(*point));
}

QVector4D* qt_gui_c_QMatrix4x4_map_as_ptr_QVector4D(const QMatrix4x4* this_ptr, const QVector4D* point) {
  return new QVector4D(this_ptr->map(*point));
}

void qt_gui_c_QMatrix4x4_map_to_output_QPoint(const QMatrix4x4* this_ptr, const QPoint* point, QPoint* output) {
  new(output) QPoint(this_ptr->map(*point));
}

void qt_gui_c_QMatrix4x4_map_to_output_QPointF(const QMatrix4x4* this_ptr, const QPointF* point, QPointF* output) {
  new(output) QPointF(this_ptr->map(*point));
}

QMatrix4x4* qt_gui_c_QMatrix4x4_new_m11_m12_m13_m14_m21_m22_m23_m24_m31_m32_m33_m34_m41_m42_m43_m44(float m11, float m12, float m13, float m14, float m21, float m22, float m23, float m24, float m31, float m32, float m33, float m34, float m41, float m42, float m43, float m44) {
  return new QMatrix4x4(m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, m41, m42, m43, m44);
}

QMatrix4x4* qt_gui_c_QMatrix4x4_new_matrix(const QMatrix* matrix) {
  return new QMatrix4x4(*matrix);
}

QMatrix4x4* qt_gui_c_QMatrix4x4_new_no_args() {
  return new QMatrix4x4();
}

QMatrix4x4* qt_gui_c_QMatrix4x4_new_transform(const QTransform* transform) {
  return new QMatrix4x4(*transform);
}

QMatrix4x4* qt_gui_c_QMatrix4x4_new_values(const float* values) {
  return new QMatrix4x4(values);
}

QMatrix4x4* qt_gui_c_QMatrix4x4_new_values_cols_rows(const float* values, int cols, int rows) {
  return new QMatrix4x4(values, cols, rows);
}

QMatrix4x4* qt_gui_c_QMatrix4x4_operator_add_assign(QMatrix4x4* this_ptr, const QMatrix4x4* other) {
  return &this_ptr->operator+=(*other);
}

float* qt_gui_c_QMatrix4x4_operator_call(QMatrix4x4* this_ptr, int row, int column) {
  return &this_ptr->operator()(row, column);
}

const float* qt_gui_c_QMatrix4x4_operator_call_const(const QMatrix4x4* this_ptr, int row, int column) {
  return &this_ptr->operator()(row, column);
}

QMatrix4x4* qt_gui_c_QMatrix4x4_operator_div_assign(QMatrix4x4* this_ptr, float divisor) {
  return &this_ptr->operator/=(divisor);
}

bool qt_gui_c_QMatrix4x4_operator_eq(const QMatrix4x4* this_ptr, const QMatrix4x4* other) {
  return this_ptr->operator==(*other);
}

QMatrix4x4* qt_gui_c_QMatrix4x4_operator_mul_assign_factor(QMatrix4x4* this_ptr, float factor) {
  return &this_ptr->operator*=(factor);
}

QMatrix4x4* qt_gui_c_QMatrix4x4_operator_mul_assign_other(QMatrix4x4* this_ptr, const QMatrix4x4* other) {
  return &this_ptr->operator*=(*other);
}

bool qt_gui_c_QMatrix4x4_operator_neq(const QMatrix4x4* this_ptr, const QMatrix4x4* other) {
  return this_ptr->operator!=(*other);
}

QMatrix4x4* qt_gui_c_QMatrix4x4_operator_sub_assign(QMatrix4x4* this_ptr, const QMatrix4x4* other) {
  return &this_ptr->operator-=(*other);
}

void qt_gui_c_QMatrix4x4_optimize(QMatrix4x4* this_ptr) {
  this_ptr->optimize();
}

void qt_gui_c_QMatrix4x4_ortho_QRect(QMatrix4x4* this_ptr, const QRect* rect) {
  this_ptr->ortho(*rect);
}

void qt_gui_c_QMatrix4x4_ortho_QRectF(QMatrix4x4* this_ptr, const QRectF* rect) {
  this_ptr->ortho(*rect);
}

void qt_gui_c_QMatrix4x4_ortho_float_float_float_float_float_float(QMatrix4x4* this_ptr, float left, float right, float bottom, float top, float nearPlane, float farPlane) {
  this_ptr->ortho(left, right, bottom, top, nearPlane, farPlane);
}

void qt_gui_c_QMatrix4x4_perspective(QMatrix4x4* this_ptr, float verticalAngle, float aspectRatio, float nearPlane, float farPlane) {
  this_ptr->perspective(verticalAngle, aspectRatio, nearPlane, farPlane);
}

void qt_gui_c_QMatrix4x4_rotate_angle_vector(QMatrix4x4* this_ptr, float angle, const QVector3D* vector) {
  this_ptr->rotate(angle, *vector);
}

void qt_gui_c_QMatrix4x4_rotate_angle_x_y(QMatrix4x4* this_ptr, float angle, float x, float y) {
  this_ptr->rotate(angle, x, y);
}

void qt_gui_c_QMatrix4x4_rotate_angle_x_y_z(QMatrix4x4* this_ptr, float angle, float x, float y, float z) {
  this_ptr->rotate(angle, x, y, z);
}

void qt_gui_c_QMatrix4x4_rotate_quaternion(QMatrix4x4* this_ptr, const QQuaternion* quaternion) {
  this_ptr->rotate(*quaternion);
}

QVector4D* qt_gui_c_QMatrix4x4_row_as_ptr(const QMatrix4x4* this_ptr, int index) {
  return new QVector4D(this_ptr->row(index));
}

void qt_gui_c_QMatrix4x4_scale_factor(QMatrix4x4* this_ptr, float factor) {
  this_ptr->scale(factor);
}

void qt_gui_c_QMatrix4x4_scale_vector(QMatrix4x4* this_ptr, const QVector3D* vector) {
  this_ptr->scale(*vector);
}

void qt_gui_c_QMatrix4x4_scale_x_y(QMatrix4x4* this_ptr, float x, float y) {
  this_ptr->scale(x, y);
}

void qt_gui_c_QMatrix4x4_scale_x_y_z(QMatrix4x4* this_ptr, float x, float y, float z) {
  this_ptr->scale(x, y, z);
}

void qt_gui_c_QMatrix4x4_setColumn(QMatrix4x4* this_ptr, int index, const QVector4D* value) {
  this_ptr->setColumn(index, *value);
}

void qt_gui_c_QMatrix4x4_setRow(QMatrix4x4* this_ptr, int index, const QVector4D* value) {
  this_ptr->setRow(index, *value);
}

void qt_gui_c_QMatrix4x4_setToIdentity(QMatrix4x4* this_ptr) {
  this_ptr->setToIdentity();
}

void qt_gui_c_QMatrix4x4_toAffine_to_output(const QMatrix4x4* this_ptr, QMatrix* output) {
  new(output) QMatrix(this_ptr->toAffine());
}

void qt_gui_c_QMatrix4x4_toTransform_to_output_distanceToPlane(const QMatrix4x4* this_ptr, float distanceToPlane, QTransform* output) {
  new(output) QTransform(this_ptr->toTransform(distanceToPlane));
}

void qt_gui_c_QMatrix4x4_toTransform_to_output_no_args(const QMatrix4x4* this_ptr, QTransform* output) {
  new(output) QTransform(this_ptr->toTransform());
}

void qt_gui_c_QMatrix4x4_translate_vector(QMatrix4x4* this_ptr, const QVector3D* vector) {
  this_ptr->translate(*vector);
}

void qt_gui_c_QMatrix4x4_translate_x_y(QMatrix4x4* this_ptr, float x, float y) {
  this_ptr->translate(x, y);
}

void qt_gui_c_QMatrix4x4_translate_x_y_z(QMatrix4x4* this_ptr, float x, float y, float z) {
  this_ptr->translate(x, y, z);
}

QMatrix4x4* qt_gui_c_QMatrix4x4_transposed_as_ptr(const QMatrix4x4* this_ptr) {
  return new QMatrix4x4(this_ptr->transposed());
}

void qt_gui_c_QMatrix4x4_viewport_left_bottom_width_height(QMatrix4x4* this_ptr, float left, float bottom, float width, float height) {
  this_ptr->viewport(left, bottom, width, height);
}

void qt_gui_c_QMatrix4x4_viewport_left_bottom_width_height_nearPlane(QMatrix4x4* this_ptr, float left, float bottom, float width, float height, float nearPlane) {
  this_ptr->viewport(left, bottom, width, height, nearPlane);
}

void qt_gui_c_QMatrix4x4_viewport_left_bottom_width_height_nearPlane_farPlane(QMatrix4x4* this_ptr, float left, float bottom, float width, float height, float nearPlane, float farPlane) {
  this_ptr->viewport(left, bottom, width, height, nearPlane, farPlane);
}

void qt_gui_c_QMatrix4x4_viewport_rect(QMatrix4x4* this_ptr, const QRectF* rect) {
  this_ptr->viewport(*rect);
}

