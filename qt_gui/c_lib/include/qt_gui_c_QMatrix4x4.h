#ifndef QT_GUI_C_QMATRIX4X4_H
#define QT_GUI_C_QMATRIX4X4_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QDataStream* qt_gui_c_QMatrix4x4_G_operator_shl(QDataStream* arg1, const QMatrix4x4* arg2);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QMatrix4x4_G_operator_shr(QDataStream* arg1, QMatrix4x4* arg2);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QMatrix4x4_column_as_ptr(const QMatrix4x4* this_ptr, int index);
QT_GUI_C_EXPORT const float* qt_gui_c_QMatrix4x4_constData(const QMatrix4x4* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_convert_to_QVariant_to_output(const QMatrix4x4* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_copyDataTo(const QMatrix4x4* this_ptr, float* values);
QT_GUI_C_EXPORT float* qt_gui_c_QMatrix4x4_data(QMatrix4x4* this_ptr);
QT_GUI_C_EXPORT const float* qt_gui_c_QMatrix4x4_data_const(const QMatrix4x4* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_delete(QMatrix4x4* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QMatrix4x4_determinant(const QMatrix4x4* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_fill(QMatrix4x4* this_ptr, float value);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_flipCoordinates(QMatrix4x4* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_frustum(QMatrix4x4* this_ptr, float left, float right, float bottom, float top, float nearPlane, float farPlane);
QT_GUI_C_EXPORT QMatrix4x4* qt_gui_c_QMatrix4x4_inverted_as_ptr_invertible(const QMatrix4x4* this_ptr, bool* invertible);
QT_GUI_C_EXPORT QMatrix4x4* qt_gui_c_QMatrix4x4_inverted_as_ptr_no_args(const QMatrix4x4* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QMatrix4x4_isAffine(const QMatrix4x4* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QMatrix4x4_isIdentity(const QMatrix4x4* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_lookAt(QMatrix4x4* this_ptr, const QVector3D* eye, const QVector3D* center, const QVector3D* up);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_mapRect_to_output_QRect(const QMatrix4x4* this_ptr, const QRect* rect, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_mapRect_to_output_QRectF(const QMatrix4x4* this_ptr, const QRectF* rect, QRectF* output);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QMatrix4x4_mapVector_as_ptr(const QMatrix4x4* this_ptr, const QVector3D* vector);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QMatrix4x4_map_as_ptr_QVector3D(const QMatrix4x4* this_ptr, const QVector3D* point);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QMatrix4x4_map_as_ptr_QVector4D(const QMatrix4x4* this_ptr, const QVector4D* point);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_map_to_output_QPoint(const QMatrix4x4* this_ptr, const QPoint* point, QPoint* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_map_to_output_QPointF(const QMatrix4x4* this_ptr, const QPointF* point, QPointF* output);
QT_GUI_C_EXPORT QMatrix4x4* qt_gui_c_QMatrix4x4_new_m11_m12_m13_m14_m21_m22_m23_m24_m31_m32_m33_m34_m41_m42_m43_m44(float m11, float m12, float m13, float m14, float m21, float m22, float m23, float m24, float m31, float m32, float m33, float m34, float m41, float m42, float m43, float m44);
QT_GUI_C_EXPORT QMatrix4x4* qt_gui_c_QMatrix4x4_new_matrix(const QMatrix* matrix);
QT_GUI_C_EXPORT QMatrix4x4* qt_gui_c_QMatrix4x4_new_no_args();
QT_GUI_C_EXPORT QMatrix4x4* qt_gui_c_QMatrix4x4_new_transform(const QTransform* transform);
QT_GUI_C_EXPORT QMatrix4x4* qt_gui_c_QMatrix4x4_new_values(const float* values);
QT_GUI_C_EXPORT QMatrix4x4* qt_gui_c_QMatrix4x4_new_values_cols_rows(const float* values, int cols, int rows);
QT_GUI_C_EXPORT QMatrix4x4* qt_gui_c_QMatrix4x4_operator_add_assign(QMatrix4x4* this_ptr, const QMatrix4x4* other);
QT_GUI_C_EXPORT float* qt_gui_c_QMatrix4x4_operator_call(QMatrix4x4* this_ptr, int row, int column);
QT_GUI_C_EXPORT const float* qt_gui_c_QMatrix4x4_operator_call_const(const QMatrix4x4* this_ptr, int row, int column);
QT_GUI_C_EXPORT QMatrix4x4* qt_gui_c_QMatrix4x4_operator_div_assign(QMatrix4x4* this_ptr, float divisor);
QT_GUI_C_EXPORT bool qt_gui_c_QMatrix4x4_operator_eq(const QMatrix4x4* this_ptr, const QMatrix4x4* other);
QT_GUI_C_EXPORT QMatrix4x4* qt_gui_c_QMatrix4x4_operator_mul_assign_factor(QMatrix4x4* this_ptr, float factor);
QT_GUI_C_EXPORT QMatrix4x4* qt_gui_c_QMatrix4x4_operator_mul_assign_other(QMatrix4x4* this_ptr, const QMatrix4x4* other);
QT_GUI_C_EXPORT bool qt_gui_c_QMatrix4x4_operator_neq(const QMatrix4x4* this_ptr, const QMatrix4x4* other);
QT_GUI_C_EXPORT QMatrix4x4* qt_gui_c_QMatrix4x4_operator_sub_assign(QMatrix4x4* this_ptr, const QMatrix4x4* other);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_optimize(QMatrix4x4* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_ortho_QRect(QMatrix4x4* this_ptr, const QRect* rect);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_ortho_QRectF(QMatrix4x4* this_ptr, const QRectF* rect);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_ortho_float_float_float_float_float_float(QMatrix4x4* this_ptr, float left, float right, float bottom, float top, float nearPlane, float farPlane);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_perspective(QMatrix4x4* this_ptr, float verticalAngle, float aspectRatio, float nearPlane, float farPlane);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_rotate_angle_vector(QMatrix4x4* this_ptr, float angle, const QVector3D* vector);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_rotate_angle_x_y(QMatrix4x4* this_ptr, float angle, float x, float y);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_rotate_angle_x_y_z(QMatrix4x4* this_ptr, float angle, float x, float y, float z);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_rotate_quaternion(QMatrix4x4* this_ptr, const QQuaternion* quaternion);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QMatrix4x4_row_as_ptr(const QMatrix4x4* this_ptr, int index);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_scale_factor(QMatrix4x4* this_ptr, float factor);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_scale_vector(QMatrix4x4* this_ptr, const QVector3D* vector);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_scale_x_y(QMatrix4x4* this_ptr, float x, float y);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_scale_x_y_z(QMatrix4x4* this_ptr, float x, float y, float z);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_setColumn(QMatrix4x4* this_ptr, int index, const QVector4D* value);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_setRow(QMatrix4x4* this_ptr, int index, const QVector4D* value);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_setToIdentity(QMatrix4x4* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_toAffine_to_output(const QMatrix4x4* this_ptr, QMatrix* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_toTransform_to_output_distanceToPlane(const QMatrix4x4* this_ptr, float distanceToPlane, QTransform* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_toTransform_to_output_no_args(const QMatrix4x4* this_ptr, QTransform* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_translate_vector(QMatrix4x4* this_ptr, const QVector3D* vector);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_translate_x_y(QMatrix4x4* this_ptr, float x, float y);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_translate_x_y_z(QMatrix4x4* this_ptr, float x, float y, float z);
QT_GUI_C_EXPORT QMatrix4x4* qt_gui_c_QMatrix4x4_transposed_as_ptr(const QMatrix4x4* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_viewport_left_bottom_width_height(QMatrix4x4* this_ptr, float left, float bottom, float width, float height);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_viewport_left_bottom_width_height_nearPlane(QMatrix4x4* this_ptr, float left, float bottom, float width, float height, float nearPlane);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_viewport_left_bottom_width_height_nearPlane_farPlane(QMatrix4x4* this_ptr, float left, float bottom, float width, float height, float nearPlane, float farPlane);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix4x4_viewport_rect(QMatrix4x4* this_ptr, const QRectF* rect);

} // extern "C"

#endif // QT_GUI_C_QMATRIX4X4_H
