#ifndef QT_3D_RENDER_C_QRENDERPASS_H
#define QT_3D_RENDER_C_QRENDERPASS_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QRenderPass_G_static_cast_QObject_ptr(Qt3DRender::QRenderPass* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QRenderPass_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QRenderPass* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderPass* qt_3d_render_c_QRenderPass_G_static_cast_Qt3DRender_QRenderPass_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderPass* qt_3d_render_c_QRenderPass_G_static_cast_Qt3DRender_QRenderPass_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPass_addFilterKey(Qt3DRender::QRenderPass* this_ptr, Qt3DRender::QFilterKey* filterKey);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPass_addParameter(Qt3DRender::QRenderPass* this_ptr, Qt3DRender::QParameter* p);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPass_addRenderState(Qt3DRender::QRenderPass* this_ptr, Qt3DRender::QRenderState* state);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPass_delete(Qt3DRender::QRenderPass* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPass_filterKeys_to_output(const Qt3DRender::QRenderPass* this_ptr, QVector< Qt3DRender::QFilterKey* >* output);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QRenderPass_metaObject(const Qt3DRender::QRenderPass* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderPass* qt_3d_render_c_Qt3DRender_QRenderPass_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderPass* qt_3d_render_c_Qt3DRender_QRenderPass_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPass_parameters_to_output(const Qt3DRender::QRenderPass* this_ptr, QVector< Qt3DRender::QParameter* >* output);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QRenderPass_qt_metacall(Qt3DRender::QRenderPass* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QRenderPass_qt_metacast(Qt3DRender::QRenderPass* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPass_removeFilterKey(Qt3DRender::QRenderPass* this_ptr, Qt3DRender::QFilterKey* filterKey);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPass_removeParameter(Qt3DRender::QRenderPass* this_ptr, Qt3DRender::QParameter* p);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPass_removeRenderState(Qt3DRender::QRenderPass* this_ptr, Qt3DRender::QRenderState* state);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPass_renderStates_to_output(const Qt3DRender::QRenderPass* this_ptr, QVector< Qt3DRender::QRenderState* >* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPass_setShaderProgram(Qt3DRender::QRenderPass* this_ptr, Qt3DRender::QShaderProgram* shaderProgram);
QT_3D_RENDER_C_EXPORT Qt3DRender::QShaderProgram* qt_3d_render_c_Qt3DRender_QRenderPass_shaderProgram(const Qt3DRender::QRenderPass* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPass_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderPass_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QRENDERPASS_H
