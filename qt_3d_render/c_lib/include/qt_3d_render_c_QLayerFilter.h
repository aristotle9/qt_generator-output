#ifndef QT_3D_RENDER_C_QLAYERFILTER_H
#define QT_3D_RENDER_C_QLAYERFILTER_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT Qt3DRender::QLayerFilter* qt_3d_render_c_QLayerFilter_G_dynamic_cast_Qt3DRender_QLayerFilter_ptr(Qt3DRender::QFrameGraphNode* ptr);
QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QLayerFilter_G_static_cast_QObject_ptr(Qt3DRender::QLayerFilter* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QLayerFilter* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QFrameGraphNode* qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QLayerFilter* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QLayerFilter* qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QLayerFilter_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QLayerFilter* qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QLayerFilter_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QLayerFilter* qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QLayerFilter_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QLayerFilter_addLayer(Qt3DRender::QLayerFilter* this_ptr, Qt3DRender::QLayer* layer);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QLayerFilter_delete(Qt3DRender::QLayerFilter* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QLayerFilter_layers_to_output(const Qt3DRender::QLayerFilter* this_ptr, QVector< Qt3DRender::QLayer* >* output);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QLayerFilter_metaObject(const Qt3DRender::QLayerFilter* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QLayerFilter* qt_3d_render_c_Qt3DRender_QLayerFilter_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QLayerFilter* qt_3d_render_c_Qt3DRender_QLayerFilter_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QLayerFilter_qt_metacall(Qt3DRender::QLayerFilter* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QLayerFilter_qt_metacast(Qt3DRender::QLayerFilter* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QLayerFilter_removeLayer(Qt3DRender::QLayerFilter* this_ptr, Qt3DRender::QLayer* layer);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QLayerFilter_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QLayerFilter_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QLAYERFILTER_H
