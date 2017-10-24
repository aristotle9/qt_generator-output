#ifndef QT_3D_RENDER_C_QCLEARBUFFERS_H
#define QT_3D_RENDER_C_QCLEARBUFFERS_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT Qt3DRender::QClearBuffers* qt_3d_render_c_QClearBuffers_G_dynamic_cast_Qt3DRender_QClearBuffers_ptr(Qt3DRender::QFrameGraphNode* ptr);
QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QClearBuffers_G_static_cast_QObject_ptr(Qt3DRender::QClearBuffers* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QClearBuffers* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QClearBuffers* qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QClearBuffers_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QClearBuffers* qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QClearBuffers_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QClearBuffers* qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QClearBuffers_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QFrameGraphNode* qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QClearBuffers* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QClearBuffers::BufferType qt_3d_render_c_Qt3DRender_QClearBuffers_buffers(const Qt3DRender::QClearBuffers* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QClearBuffers_clearColor_to_output(const Qt3DRender::QClearBuffers* this_ptr, QColor* output);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QClearBuffers_clearDepthValue(const Qt3DRender::QClearBuffers* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QClearBuffers_clearStencilValue(const Qt3DRender::QClearBuffers* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderTargetOutput* qt_3d_render_c_Qt3DRender_QClearBuffers_colorBuffer(const Qt3DRender::QClearBuffers* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QClearBuffers_delete(Qt3DRender::QClearBuffers* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QClearBuffers_metaObject(const Qt3DRender::QClearBuffers* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QClearBuffers* qt_3d_render_c_Qt3DRender_QClearBuffers_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QClearBuffers* qt_3d_render_c_Qt3DRender_QClearBuffers_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QClearBuffers_qt_metacall(Qt3DRender::QClearBuffers* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QClearBuffers_qt_metacast(Qt3DRender::QClearBuffers* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QClearBuffers_setBuffers(Qt3DRender::QClearBuffers* this_ptr, Qt3DRender::QClearBuffers::BufferType buffers);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QClearBuffers_setClearColor(Qt3DRender::QClearBuffers* this_ptr, const QColor* color);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QClearBuffers_setClearDepthValue(Qt3DRender::QClearBuffers* this_ptr, float clearDepthValue);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QClearBuffers_setClearStencilValue(Qt3DRender::QClearBuffers* this_ptr, int clearStencilValue);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QClearBuffers_setColorBuffer(Qt3DRender::QClearBuffers* this_ptr, Qt3DRender::QRenderTargetOutput* buffer);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QClearBuffers_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QClearBuffers_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QCLEARBUFFERS_H
