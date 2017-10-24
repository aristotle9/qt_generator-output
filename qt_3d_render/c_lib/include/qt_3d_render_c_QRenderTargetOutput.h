#ifndef QT_3D_RENDER_C_QRENDERTARGETOUTPUT_H
#define QT_3D_RENDER_C_QRENDERTARGETOUTPUT_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QRenderTargetOutput_G_static_cast_QObject_ptr(Qt3DRender::QRenderTargetOutput* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QRenderTargetOutput_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QRenderTargetOutput* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderTargetOutput* qt_3d_render_c_QRenderTargetOutput_G_static_cast_Qt3DRender_QRenderTargetOutput_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderTargetOutput* qt_3d_render_c_QRenderTargetOutput_G_static_cast_Qt3DRender_QRenderTargetOutput_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderTargetOutput::AttachmentPoint qt_3d_render_c_Qt3DRender_QRenderTargetOutput_attachmentPoint(const Qt3DRender::QRenderTargetOutput* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderTargetOutput_delete(Qt3DRender::QRenderTargetOutput* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QRenderTargetOutput_layer(const Qt3DRender::QRenderTargetOutput* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QRenderTargetOutput_metaObject(const Qt3DRender::QRenderTargetOutput* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QRenderTargetOutput_mipLevel(const Qt3DRender::QRenderTargetOutput* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderTargetOutput* qt_3d_render_c_Qt3DRender_QRenderTargetOutput_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QRenderTargetOutput* qt_3d_render_c_Qt3DRender_QRenderTargetOutput_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QRenderTargetOutput_qt_metacall(Qt3DRender::QRenderTargetOutput* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QRenderTargetOutput_qt_metacast(Qt3DRender::QRenderTargetOutput* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderTargetOutput_setAttachmentPoint(Qt3DRender::QRenderTargetOutput* this_ptr, Qt3DRender::QRenderTargetOutput::AttachmentPoint attachmentPoint);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderTargetOutput_setFace(Qt3DRender::QRenderTargetOutput* this_ptr, const Qt3DRender::QAbstractTexture::CubeMapFace* face);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderTargetOutput_setLayer(Qt3DRender::QRenderTargetOutput* this_ptr, int layer);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderTargetOutput_setMipLevel(Qt3DRender::QRenderTargetOutput* this_ptr, int level);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderTargetOutput_setTexture(Qt3DRender::QRenderTargetOutput* this_ptr, Qt3DRender::QAbstractTexture* texture);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractTexture* qt_3d_render_c_Qt3DRender_QRenderTargetOutput_texture(const Qt3DRender::QRenderTargetOutput* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderTargetOutput_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QRenderTargetOutput_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QRENDERTARGETOUTPUT_H
