#ifndef QT_3D_RENDER_C_QRENDERPASSFILTER_H
#define QT_3D_RENDER_C_QRENDERPASSFILTER_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderPassFilter* qt_3d_render_c_QRenderPassFilter_G_dynamic_cast_Qt3DRender_QRenderPassFilter_ptr(Qt3DRender::QFrameGraphNode* ptr);
QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QRenderPassFilter_G_static_cast_QObject_ptr(Qt3DRender::QRenderPassFilter* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QRenderPassFilter* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QFrameGraphNode* qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QRenderPassFilter* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderPassFilter* qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QRenderPassFilter_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderPassFilter* qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QRenderPassFilter_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderPassFilter* qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QRenderPassFilter_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPassFilter_addMatch(Qt3DRender::QRenderPassFilter* this_ptr, Qt3DRender::QFilterKey* filterKey);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPassFilter_addParameter(Qt3DRender::QRenderPassFilter* this_ptr, Qt3DRender::QParameter* parameter);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPassFilter_delete(Qt3DRender::QRenderPassFilter* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPassFilter_matchAny_to_output(const Qt3DRender::QRenderPassFilter* this_ptr, QVector< Qt3DRender::QFilterKey* >* output);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QRenderPassFilter_metaObject(const Qt3DRender::QRenderPassFilter* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderPassFilter* qt_3d_render_c_Qt3DRender_QRenderPassFilter_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderPassFilter* qt_3d_render_c_Qt3DRender_QRenderPassFilter_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPassFilter_parameters_to_output(const Qt3DRender::QRenderPassFilter* this_ptr, QVector< Qt3DRender::QParameter* >* output);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QRenderPassFilter_qt_metacall(Qt3DRender::QRenderPassFilter* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QRenderPassFilter_qt_metacast(Qt3DRender::QRenderPassFilter* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPassFilter_removeMatch(Qt3DRender::QRenderPassFilter* this_ptr, Qt3DRender::QFilterKey* filterKey);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPassFilter_removeParameter(Qt3DRender::QRenderPassFilter* this_ptr, Qt3DRender::QParameter* parameter);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPassFilter_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPassFilter_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QRENDERPASSFILTER_H
