#ifndef QT_GUI_C_QQUATERNION_H
#define QT_GUI_C_QQUATERNION_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QVector3D* qt_gui_c_QQuaternion_G_operator_mul_as_ptr(const QQuaternion* quaternion, const QVector3D* vec);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QQuaternion_G_operator_shl(QDataStream* arg1, const QQuaternion* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_G_operator_shl_to_output(const QDebug* dbg, const QQuaternion* q, QDebug* output);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QQuaternion_G_operator_shr(QDataStream* arg1, QQuaternion* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_conjugate_to_output(const QQuaternion* this_ptr, QQuaternion* output);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_conjugated_to_output(const QQuaternion* this_ptr, QQuaternion* output);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_constructor_no_args(QQuaternion* output);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_constructor_scalar_vector(float scalar, const QVector3D* vector, QQuaternion* output);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_constructor_scalar_xpos_ypos_zpos(float scalar, float xpos, float ypos, float zpos, QQuaternion* output);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_constructor_vector(const QVector4D* vector, QQuaternion* output);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_convert_to_QVariant_to_output(const QQuaternion* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_destructor(QQuaternion* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QQuaternion_dotProduct(const QQuaternion* q1, const QQuaternion* q2);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_fromAxes_to_output(const QVector3D* xAxis, const QVector3D* yAxis, const QVector3D* zAxis, QQuaternion* output);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_fromAxisAndAngle_to_output_axis_angle(const QVector3D* axis, float angle, QQuaternion* output);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_fromAxisAndAngle_to_output_x_y_z_angle(float x, float y, float z, float angle, QQuaternion* output);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_fromDirection_to_output(const QVector3D* direction, const QVector3D* up, QQuaternion* output);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_fromEulerAngles_to_output_eulerAngles(const QVector3D* eulerAngles, QQuaternion* output);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_fromEulerAngles_to_output_pitch_yaw_roll(float pitch, float yaw, float roll, QQuaternion* output);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_getAxes(const QQuaternion* this_ptr, QVector3D* xAxis, QVector3D* yAxis, QVector3D* zAxis);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_getAxisAndAngle_axis_angle(const QQuaternion* this_ptr, QVector3D* axis, float* angle);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_getAxisAndAngle_x_y_z_angle(const QQuaternion* this_ptr, float* x, float* y, float* z, float* angle);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_getEulerAngles(const QQuaternion* this_ptr, float* pitch, float* yaw, float* roll);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_inverted_to_output(const QQuaternion* this_ptr, QQuaternion* output);
QT_GUI_C_EXPORT bool qt_gui_c_QQuaternion_isIdentity(const QQuaternion* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QQuaternion_isNull(const QQuaternion* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QQuaternion_length(const QQuaternion* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QQuaternion_lengthSquared(const QQuaternion* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_nlerp_to_output(const QQuaternion* q1, const QQuaternion* q2, float t, QQuaternion* output);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_normalize(QQuaternion* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_normalized_to_output(const QQuaternion* this_ptr, QQuaternion* output);
QT_GUI_C_EXPORT QQuaternion* qt_gui_c_QQuaternion_operator_add_assign(QQuaternion* this_ptr, const QQuaternion* quaternion);
QT_GUI_C_EXPORT QQuaternion* qt_gui_c_QQuaternion_operator_div_assign(QQuaternion* this_ptr, float divisor);
QT_GUI_C_EXPORT QQuaternion* qt_gui_c_QQuaternion_operator_mul_assign_factor(QQuaternion* this_ptr, float factor);
QT_GUI_C_EXPORT QQuaternion* qt_gui_c_QQuaternion_operator_mul_assign_quaternion(QQuaternion* this_ptr, const QQuaternion* quaternion);
QT_GUI_C_EXPORT QQuaternion* qt_gui_c_QQuaternion_operator_sub_assign(QQuaternion* this_ptr, const QQuaternion* quaternion);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QQuaternion_rotatedVector_as_ptr(const QQuaternion* this_ptr, const QVector3D* vector);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_rotationTo_to_output(const QVector3D* from, const QVector3D* to, QQuaternion* output);
QT_GUI_C_EXPORT float qt_gui_c_QQuaternion_scalar(const QQuaternion* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_setScalar(QQuaternion* this_ptr, float scalar);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_setVector_vector(QQuaternion* this_ptr, const QVector3D* vector);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_setVector_x_y_z(QQuaternion* this_ptr, float x, float y, float z);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_setX(QQuaternion* this_ptr, float x);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_setY(QQuaternion* this_ptr, float y);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_setZ(QQuaternion* this_ptr, float z);
QT_GUI_C_EXPORT void qt_gui_c_QQuaternion_slerp_to_output(const QQuaternion* q1, const QQuaternion* q2, float t, QQuaternion* output);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QQuaternion_toEulerAngles_as_ptr(const QQuaternion* this_ptr);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QQuaternion_toVector4D_as_ptr(const QQuaternion* this_ptr);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QQuaternion_vector_as_ptr(const QQuaternion* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QQuaternion_x(const QQuaternion* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QQuaternion_y(const QQuaternion* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QQuaternion_z(const QQuaternion* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QQUATERNION_H
