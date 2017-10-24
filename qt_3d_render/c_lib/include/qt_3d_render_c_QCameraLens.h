#ifndef QT_3D_RENDER_C_QCAMERALENS_H
#define QT_3D_RENDER_C_QCAMERALENS_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QCameraLens_G_static_cast_QObject_ptr(Qt3DRender::QCameraLens* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QComponent* qt_3d_render_c_QCameraLens_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QCameraLens* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QCameraLens_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QCameraLens* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QCameraLens* qt_3d_render_c_QCameraLens_G_static_cast_Qt3DRender_QCameraLens_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QCameraLens* qt_3d_render_c_QCameraLens_G_static_cast_Qt3DRender_QCameraLens_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QCameraLens* qt_3d_render_c_QCameraLens_G_static_cast_Qt3DRender_QCameraLens_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCameraLens_aspectRatio(const Qt3DRender::QCameraLens* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCameraLens_bottom(const Qt3DRender::QCameraLens* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCameraLens_delete(Qt3DRender::QCameraLens* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCameraLens_exposure(const Qt3DRender::QCameraLens* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCameraLens_farPlane(const Qt3DRender::QCameraLens* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCameraLens_fieldOfView(const Qt3DRender::QCameraLens* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCameraLens_left(const Qt3DRender::QCameraLens* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QCameraLens_metaObject(const Qt3DRender::QCameraLens* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCameraLens_nearPlane(const Qt3DRender::QCameraLens* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QCameraLens* qt_3d_render_c_Qt3DRender_QCameraLens_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QCameraLens* qt_3d_render_c_Qt3DRender_QCameraLens_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT QMatrix4x4* qt_3d_render_c_Qt3DRender_QCameraLens_projectionMatrix_as_ptr(const Qt3DRender::QCameraLens* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QCameraLens::ProjectionType qt_3d_render_c_Qt3DRender_QCameraLens_projectionType(const Qt3DRender::QCameraLens* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QCameraLens_qt_metacall(Qt3DRender::QCameraLens* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QCameraLens_qt_metacast(Qt3DRender::QCameraLens* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCameraLens_right(const Qt3DRender::QCameraLens* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCameraLens_setAspectRatio(Qt3DRender::QCameraLens* this_ptr, float aspectRatio);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCameraLens_setBottom(Qt3DRender::QCameraLens* this_ptr, float bottom);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCameraLens_setExposure(Qt3DRender::QCameraLens* this_ptr, float exposure);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCameraLens_setFarPlane(Qt3DRender::QCameraLens* this_ptr, float farPlane);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCameraLens_setFieldOfView(Qt3DRender::QCameraLens* this_ptr, float fieldOfView);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCameraLens_setFrustumProjection(Qt3DRender::QCameraLens* this_ptr, float left, float right, float bottom, float top, float nearPlane, float farPlane);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCameraLens_setLeft(Qt3DRender::QCameraLens* this_ptr, float left);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCameraLens_setNearPlane(Qt3DRender::QCameraLens* this_ptr, float nearPlane);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCameraLens_setOrthographicProjection(Qt3DRender::QCameraLens* this_ptr, float left, float right, float bottom, float top, float nearPlane, float farPlane);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCameraLens_setPerspectiveProjection(Qt3DRender::QCameraLens* this_ptr, float fieldOfView, float aspect, float nearPlane, float farPlane);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCameraLens_setProjectionMatrix(Qt3DRender::QCameraLens* this_ptr, const QMatrix4x4* projectionMatrix);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCameraLens_setProjectionType(Qt3DRender::QCameraLens* this_ptr, Qt3DRender::QCameraLens::ProjectionType projectionType);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCameraLens_setRight(Qt3DRender::QCameraLens* this_ptr, float right);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCameraLens_setTop(Qt3DRender::QCameraLens* this_ptr, float top);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QCameraLens_top(const Qt3DRender::QCameraLens* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCameraLens_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QCameraLens_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QCAMERALENS_H
