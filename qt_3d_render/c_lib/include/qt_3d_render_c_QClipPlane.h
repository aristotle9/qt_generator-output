#ifndef QT_3D_RENDER_C_QCLIPPLANE_H
#define QT_3D_RENDER_C_QCLIPPLANE_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT Qt3DRender::QClipPlane* qt_3d_render_c_QClipPlane_G_dynamic_cast_Qt3DRender_QClipPlane_ptr(Qt3DRender::QRenderState* ptr);
QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QClipPlane_G_static_cast_QObject_ptr(Qt3DRender::QClipPlane* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QClipPlane_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QClipPlane* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QClipPlane* qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QClipPlane_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QClipPlane* qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QClipPlane_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QClipPlane* qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QClipPlane_ptr_Qt3DRender_QRenderState(Qt3DRender::QRenderState* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderState* qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QRenderState_ptr(Qt3DRender::QClipPlane* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QClipPlane_delete(Qt3DRender::QClipPlane* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QClipPlane_distance(const Qt3DRender::QClipPlane* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QClipPlane_metaObject(const Qt3DRender::QClipPlane* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QClipPlane* qt_3d_render_c_Qt3DRender_QClipPlane_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QClipPlane* qt_3d_render_c_Qt3DRender_QClipPlane_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT QVector3D* qt_3d_render_c_Qt3DRender_QClipPlane_normal_as_ptr(const Qt3DRender::QClipPlane* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QClipPlane_planeIndex(const Qt3DRender::QClipPlane* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QClipPlane_qt_metacall(Qt3DRender::QClipPlane* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QClipPlane_qt_metacast(Qt3DRender::QClipPlane* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QClipPlane_setDistance(Qt3DRender::QClipPlane* this_ptr, float arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QClipPlane_setNormal(Qt3DRender::QClipPlane* this_ptr, const QVector3D* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QClipPlane_setPlaneIndex(Qt3DRender::QClipPlane* this_ptr, int arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QClipPlane_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QClipPlane_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QCLIPPLANE_H
