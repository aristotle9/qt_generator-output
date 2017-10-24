#ifndef QT_3D_RENDER_C_QTEXTURE_H
#define QT_3D_RENDER_C_QTEXTURE_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT Qt3DRender::QTextureLoader* qt_3d_render_c_QTexture_G_dynamic_cast_Qt3DRender_QTextureLoader_ptr(Qt3DRender::QAbstractTexture* ptr);
QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QTexture_G_static_cast_QObject_ptr(Qt3DRender::QTextureLoader* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QTexture_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QTextureLoader* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractTexture* qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QAbstractTexture_ptr(Qt3DRender::QTextureLoader* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QTextureLoader* qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QTextureLoader_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QTextureLoader* qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QTextureLoader_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QTextureLoader* qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QTextureLoader_ptr_Qt3DRender_QAbstractTexture(Qt3DRender::QAbstractTexture* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureLoader_delete(Qt3DRender::QTextureLoader* this_ptr);
QT_3D_RENDER_C_EXPORT bool qt_3d_render_c_Qt3DRender_QTextureLoader_isMirrored(const Qt3DRender::QTextureLoader* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QTextureLoader_metaObject(const Qt3DRender::QTextureLoader* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QTextureLoader* qt_3d_render_c_Qt3DRender_QTextureLoader_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QTextureLoader* qt_3d_render_c_Qt3DRender_QTextureLoader_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QTextureLoader_qt_metacall(Qt3DRender::QTextureLoader* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QTextureLoader_qt_metacast(Qt3DRender::QTextureLoader* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureLoader_setMirrored(Qt3DRender::QTextureLoader* this_ptr, bool mirrored);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureLoader_setSource(Qt3DRender::QTextureLoader* this_ptr, const QUrl* source);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureLoader_source_to_output(const Qt3DRender::QTextureLoader* this_ptr, QUrl* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureLoader_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureLoader_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QTEXTURE_H
