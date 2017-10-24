#ifndef QT_3D_RENDER_C_QTEXTUREIMAGE_H
#define QT_3D_RENDER_C_QTEXTUREIMAGE_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT Qt3DRender::QTextureImage* qt_3d_render_c_QTextureImage_G_dynamic_cast_Qt3DRender_QTextureImage_ptr(Qt3DRender::QAbstractTextureImage* ptr);
QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QTextureImage_G_static_cast_QObject_ptr(Qt3DRender::QTextureImage* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QTextureImage_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QTextureImage* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractTextureImage* qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QAbstractTextureImage_ptr(Qt3DRender::QTextureImage* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QTextureImage* qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QTextureImage_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QTextureImage* qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QTextureImage_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QTextureImage* qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QTextureImage_ptr_Qt3DRender_QAbstractTextureImage(Qt3DRender::QAbstractTextureImage* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImage_delete(Qt3DRender::QTextureImage* this_ptr);
QT_3D_RENDER_C_EXPORT bool qt_3d_render_c_Qt3DRender_QTextureImage_isMirrored(const Qt3DRender::QTextureImage* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QTextureImage_metaObject(const Qt3DRender::QTextureImage* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QTextureImage* qt_3d_render_c_Qt3DRender_QTextureImage_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QTextureImage* qt_3d_render_c_Qt3DRender_QTextureImage_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QTextureImage_qt_metacall(Qt3DRender::QTextureImage* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QTextureImage_qt_metacast(Qt3DRender::QTextureImage* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImage_setMirrored(Qt3DRender::QTextureImage* this_ptr, bool mirrored);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImage_setSource(Qt3DRender::QTextureImage* this_ptr, const QUrl* source);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImage_source_to_output(const Qt3DRender::QTextureImage* this_ptr, QUrl* output);
QT_3D_RENDER_C_EXPORT Qt3DRender::QTextureImage::Status qt_3d_render_c_Qt3DRender_QTextureImage_status(const Qt3DRender::QTextureImage* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImage_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImage_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QTEXTUREIMAGE_H
