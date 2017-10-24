#ifndef QT_3D_RENDER_C_QRENDERSETTINGS_H
#define QT_3D_RENDER_C_QRENDERSETTINGS_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QRenderSettings_G_static_cast_QObject_ptr(Qt3DRender::QRenderSettings* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QComponent* qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QRenderSettings* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QRenderSettings* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderSettings* qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DRender_QRenderSettings_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderSettings* qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DRender_QRenderSettings_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderSettings* qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DRender_QRenderSettings_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QFrameGraphNode* qt_3d_render_c_Qt3DRender_QRenderSettings_activeFrameGraph(const Qt3DRender::QRenderSettings* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderSettings_delete(Qt3DRender::QRenderSettings* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QRenderSettings_metaObject(const Qt3DRender::QRenderSettings* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderSettings* qt_3d_render_c_Qt3DRender_QRenderSettings_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderSettings* qt_3d_render_c_Qt3DRender_QRenderSettings_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickingSettings* qt_3d_render_c_Qt3DRender_QRenderSettings_pickingSettings(Qt3DRender::QRenderSettings* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QRenderSettings_qt_metacall(Qt3DRender::QRenderSettings* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QRenderSettings_qt_metacast(Qt3DRender::QRenderSettings* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderSettings::RenderPolicy qt_3d_render_c_Qt3DRender_QRenderSettings_renderPolicy(const Qt3DRender::QRenderSettings* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderSettings_setActiveFrameGraph(Qt3DRender::QRenderSettings* this_ptr, Qt3DRender::QFrameGraphNode* activeFrameGraph);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderSettings_setRenderPolicy(Qt3DRender::QRenderSettings* this_ptr, Qt3DRender::QRenderSettings::RenderPolicy renderPolicy);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderSettings_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderSettings_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QRENDERSETTINGS_H
