#ifndef QT_3D_RENDER_C_QCAMERA_H
#define QT_3D_RENDER_C_QCAMERA_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QCamera_G_static_cast_QObject_ptr(Qt3DRender::QCamera* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QEntity* qt_3d_render_c_QCamera_G_static_cast_Qt3DCore_QEntity_ptr(Qt3DRender::QCamera* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QCamera_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QCamera* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QCamera* qt_3d_render_c_QCamera_G_static_cast_Qt3DRender_QCamera_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QCamera* qt_3d_render_c_QCamera_G_static_cast_Qt3DRender_QCamera_ptr_Qt3DCore_QEntity(Qt3DCore::QEntity* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QCamera* qt_3d_render_c_QCamera_G_static_cast_Qt3DRender_QCamera_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCamera_aspectRatio(const Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCamera_bottom(const Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_delete(Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCamera_exposure(const Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCamera_farPlane(const Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCamera_fieldOfView(const Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCamera_left(const Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QCameraLens* qt_3d_render_c_Qt3DRender_QCamera_lens(const Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QCamera_metaObject(const Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCamera_nearPlane(const Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QCamera* qt_3d_render_c_Qt3DRender_QCamera_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QCamera* qt_3d_render_c_Qt3DRender_QCamera_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_panAboutViewCenter_angle(Qt3DRender::QCamera* this_ptr, float angle);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_panAboutViewCenter_angle_axis(Qt3DRender::QCamera* this_ptr, float angle, const QVector3D* axis);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_panRotation_to_output(const Qt3DRender::QCamera* this_ptr, float angle, QQuaternion* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_pan_angle(Qt3DRender::QCamera* this_ptr, float angle);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_pan_angle_axis(Qt3DRender::QCamera* this_ptr, float angle, const QVector3D* axis);
QT_3D_RENDER_C_EXPORT QVector3D* qt_3d_render_c_Qt3DRender_QCamera_position_as_ptr(const Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT QMatrix4x4* qt_3d_render_c_Qt3DRender_QCamera_projectionMatrix_as_ptr(const Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QCamera_qt_metacall(Qt3DRender::QCamera* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QCamera_qt_metacast(Qt3DRender::QCamera* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCamera_right(const Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_roll(Qt3DRender::QCamera* this_ptr, float angle);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_rollAboutViewCenter(Qt3DRender::QCamera* this_ptr, float angle);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_rollRotation_to_output(const Qt3DRender::QCamera* this_ptr, float angle, QQuaternion* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_rotate(Qt3DRender::QCamera* this_ptr, const QQuaternion* q);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_rotateAboutViewCenter(Qt3DRender::QCamera* this_ptr, const QQuaternion* q);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_rotation_to_output(const Qt3DRender::QCamera* this_ptr, float angle, const QVector3D* axis, QQuaternion* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_setAspectRatio(Qt3DRender::QCamera* this_ptr, float aspectRatio);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_setBottom(Qt3DRender::QCamera* this_ptr, float bottom);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_setExposure(Qt3DRender::QCamera* this_ptr, float exposure);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_setFarPlane(Qt3DRender::QCamera* this_ptr, float farPlane);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_setFieldOfView(Qt3DRender::QCamera* this_ptr, float fieldOfView);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_setLeft(Qt3DRender::QCamera* this_ptr, float left);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_setNearPlane(Qt3DRender::QCamera* this_ptr, float nearPlane);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_setPosition(Qt3DRender::QCamera* this_ptr, const QVector3D* position);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_setProjectionMatrix(Qt3DRender::QCamera* this_ptr, const QMatrix4x4* projectionMatrix);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_setProjectionType(Qt3DRender::QCamera* this_ptr, const Qt3DRender::QCameraLens::ProjectionType* type);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_setRight(Qt3DRender::QCamera* this_ptr, float right);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_setTop(Qt3DRender::QCamera* this_ptr, float top);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_setUpVector(Qt3DRender::QCamera* this_ptr, const QVector3D* upVector);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_setViewCenter(Qt3DRender::QCamera* this_ptr, const QVector3D* viewCenter);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_tilt(Qt3DRender::QCamera* this_ptr, float angle);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_tiltAboutViewCenter(Qt3DRender::QCamera* this_ptr, float angle);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_tiltRotation_to_output(const Qt3DRender::QCamera* this_ptr, float angle, QQuaternion* output);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCamera_top(const Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT Qt3DCore::QTransform* qt_3d_render_c_Qt3DRender_QCamera_transform(const Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_translateWorld_vWorld(Qt3DRender::QCamera* this_ptr, const QVector3D* vWorld);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_translateWorld_vWorld_option(Qt3DRender::QCamera* this_ptr, const QVector3D* vWorld, Qt3DRender::QCamera::CameraTranslationOption option);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_translate_vLocal(Qt3DRender::QCamera* this_ptr, const QVector3D* vLocal);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCamera_translate_vLocal_option(Qt3DRender::QCamera* this_ptr, const QVector3D* vLocal, Qt3DRender::QCamera::CameraTranslationOption option);
QT_3D_RENDER_C_EXPORT QVector3D* qt_3d_render_c_Qt3DRender_QCamera_upVector_as_ptr(const Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT QVector3D* qt_3d_render_c_Qt3DRender_QCamera_viewCenter_as_ptr(const Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT QMatrix4x4* qt_3d_render_c_Qt3DRender_QCamera_viewMatrix_as_ptr(const Qt3DRender::QCamera* this_ptr);
QT_3D_RENDER_C_EXPORT QVector3D* qt_3d_render_c_Qt3DRender_QCamera_viewVector_as_ptr(const Qt3DRender::QCamera* this_ptr);

} // extern "C"

#endif // QT_3D_RENDER_C_QCAMERA_H
