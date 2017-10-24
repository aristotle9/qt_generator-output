#ifndef QT_3D_CORE_C_QTRANSFORM_H
#define QT_3D_CORE_C_QTRANSFORM_H

#include "qt_3d_core_c_global.h"

extern "C" {

QT_3D_CORE_C_EXPORT Qt3DCore::QTransform* qt_3d_core_c_QTransform_G_dynamic_cast_Qt3DCore_QTransform_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QTransform* qt_3d_core_c_QTransform_G_dynamic_cast_Qt3DCore_QTransform_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_CORE_C_EXPORT QObject* qt_3d_core_c_QTransform_G_static_cast_QObject_ptr(Qt3DCore::QTransform* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QComponent* qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DCore::QTransform* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QNode* qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QNode_ptr(Qt3DCore::QTransform* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QTransform* qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QTransform_ptr_QObject(QObject* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QTransform* qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QTransform_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QTransform* qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QTransform_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_delete(Qt3DCore::QTransform* this_ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_fromAxesAndAngles_to_output_axis1_angle1_axis2_angle2(const QVector3D* axis1, float angle1, const QVector3D* axis2, float angle2, QQuaternion* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_fromAxesAndAngles_to_output_axis1_angle1_axis2_angle2_axis3_angle3(const QVector3D* axis1, float angle1, const QVector3D* axis2, float angle2, const QVector3D* axis3, float angle3, QQuaternion* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_fromAxisAndAngle_to_output_axis_angle(const QVector3D* axis, float angle, QQuaternion* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_fromAxisAndAngle_to_output_x_y_z_angle(float x, float y, float z, float angle, QQuaternion* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_fromEulerAngles_to_output_eulerAngles(const QVector3D* eulerAngles, QQuaternion* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_fromEulerAngles_to_output_pitch_yaw_roll(float pitch, float yaw, float roll, QQuaternion* output);
QT_3D_CORE_C_EXPORT QMatrix4x4* qt_3d_core_c_Qt3DCore_QTransform_matrix_as_ptr(const Qt3DCore::QTransform* this_ptr);
QT_3D_CORE_C_EXPORT const QMetaObject* qt_3d_core_c_Qt3DCore_QTransform_metaObject(const Qt3DCore::QTransform* this_ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QTransform* qt_3d_core_c_Qt3DCore_QTransform_new_no_args();
QT_3D_CORE_C_EXPORT Qt3DCore::QTransform* qt_3d_core_c_Qt3DCore_QTransform_new_parent(Qt3DCore::QNode* parent);
QT_3D_CORE_C_EXPORT int qt_3d_core_c_Qt3DCore_QTransform_qt_metacall(Qt3DCore::QTransform* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_CORE_C_EXPORT void* qt_3d_core_c_Qt3DCore_QTransform_qt_metacast(Qt3DCore::QTransform* this_ptr, const char* arg1);
QT_3D_CORE_C_EXPORT QMatrix4x4* qt_3d_core_c_Qt3DCore_QTransform_rotateAround_as_ptr(const QVector3D* point, float angle, const QVector3D* axis);
QT_3D_CORE_C_EXPORT float qt_3d_core_c_Qt3DCore_QTransform_rotationX(const Qt3DCore::QTransform* this_ptr);
QT_3D_CORE_C_EXPORT float qt_3d_core_c_Qt3DCore_QTransform_rotationY(const Qt3DCore::QTransform* this_ptr);
QT_3D_CORE_C_EXPORT float qt_3d_core_c_Qt3DCore_QTransform_rotationZ(const Qt3DCore::QTransform* this_ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_rotation_to_output(const Qt3DCore::QTransform* this_ptr, QQuaternion* output);
QT_3D_CORE_C_EXPORT float qt_3d_core_c_Qt3DCore_QTransform_scale(const Qt3DCore::QTransform* this_ptr);
QT_3D_CORE_C_EXPORT QVector3D* qt_3d_core_c_Qt3DCore_QTransform_scale3D_as_ptr(const Qt3DCore::QTransform* this_ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_setMatrix(Qt3DCore::QTransform* this_ptr, const QMatrix4x4* matrix);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_setRotation(Qt3DCore::QTransform* this_ptr, const QQuaternion* rotation);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_setRotationX(Qt3DCore::QTransform* this_ptr, float rotationX);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_setRotationY(Qt3DCore::QTransform* this_ptr, float rotationY);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_setRotationZ(Qt3DCore::QTransform* this_ptr, float rotationZ);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_setScale(Qt3DCore::QTransform* this_ptr, float scale);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_setScale3D(Qt3DCore::QTransform* this_ptr, const QVector3D* scale);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_setTranslation(Qt3DCore::QTransform* this_ptr, const QVector3D* translation);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QTransform_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_CORE_C_EXPORT QVector3D* qt_3d_core_c_Qt3DCore_QTransform_translation_as_ptr(const Qt3DCore::QTransform* this_ptr);

} // extern "C"

#endif // QT_3D_CORE_C_QTRANSFORM_H
