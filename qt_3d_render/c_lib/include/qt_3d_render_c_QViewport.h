#ifndef QT_3D_RENDER_C_QVIEWPORT_H
#define QT_3D_RENDER_C_QVIEWPORT_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT Qt3DRender::QViewport* qt_3d_render_c_QViewport_G_dynamic_cast_Qt3DRender_QViewport_ptr(Qt3DRender::QFrameGraphNode* ptr);
QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QViewport_G_static_cast_QObject_ptr(Qt3DRender::QViewport* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QViewport_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QViewport* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QFrameGraphNode* qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QViewport* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QViewport* qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QViewport_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QViewport* qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QViewport_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QViewport* qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QViewport_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QViewport_delete(Qt3DRender::QViewport* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QViewport_gamma(const Qt3DRender::QViewport* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QViewport_metaObject(const Qt3DRender::QViewport* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QViewport* qt_3d_render_c_Qt3DRender_QViewport_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QViewport* qt_3d_render_c_Qt3DRender_QViewport_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QViewport_normalizedRect_to_output(const Qt3DRender::QViewport* this_ptr, QRectF* output);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QViewport_qt_metacall(Qt3DRender::QViewport* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QViewport_qt_metacast(Qt3DRender::QViewport* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QViewport_setGamma(Qt3DRender::QViewport* this_ptr, float gamma);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QViewport_setNormalizedRect(Qt3DRender::QViewport* this_ptr, const QRectF* normalizedRect);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QViewport_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QViewport_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QVIEWPORT_H
