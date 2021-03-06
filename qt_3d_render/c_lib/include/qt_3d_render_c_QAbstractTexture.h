#ifndef QT_3D_RENDER_C_QABSTRACTTEXTURE_H
#define QT_3D_RENDER_C_QABSTRACTTEXTURE_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QAbstractTexture_G_static_cast_QObject_ptr(Qt3DRender::QAbstractTexture* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QAbstractTexture_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QAbstractTexture* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractTexture* qt_3d_render_c_QAbstractTexture_G_static_cast_Qt3DRender_QAbstractTexture_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractTexture* qt_3d_render_c_QAbstractTexture_G_static_cast_Qt3DRender_QAbstractTexture_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_addTextureImage(Qt3DRender::QAbstractTexture* this_ptr, Qt3DRender::QAbstractTextureImage* textureImage);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractTexture::ComparisonFunction qt_3d_render_c_Qt3DRender_QAbstractTexture_comparisonFunction(const Qt3DRender::QAbstractTexture* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractTexture::ComparisonMode qt_3d_render_c_Qt3DRender_QAbstractTexture_comparisonMode(const Qt3DRender::QAbstractTexture* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_dataGenerator_to_output(const Qt3DRender::QAbstractTexture* this_ptr, QSharedPointer< Qt3DRender::QTextureGenerator >* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_delete(Qt3DRender::QAbstractTexture* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QAbstractTexture_depth(const Qt3DRender::QAbstractTexture* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractTexture::TextureFormat qt_3d_render_c_Qt3DRender_QAbstractTexture_format(const Qt3DRender::QAbstractTexture* this_ptr);
QT_3D_RENDER_C_EXPORT bool qt_3d_render_c_Qt3DRender_QAbstractTexture_generateMipMaps(const Qt3DRender::QAbstractTexture* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QAbstractTexture_height(const Qt3DRender::QAbstractTexture* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QAbstractTexture_layers(const Qt3DRender::QAbstractTexture* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractTexture::Filter qt_3d_render_c_Qt3DRender_QAbstractTexture_magnificationFilter(const Qt3DRender::QAbstractTexture* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QAbstractTexture_maximumAnisotropy(const Qt3DRender::QAbstractTexture* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QAbstractTexture_metaObject(const Qt3DRender::QAbstractTexture* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractTexture::Filter qt_3d_render_c_Qt3DRender_QAbstractTexture_minificationFilter(const Qt3DRender::QAbstractTexture* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QAbstractTexture_qt_metacall(Qt3DRender::QAbstractTexture* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QAbstractTexture_qt_metacast(Qt3DRender::QAbstractTexture* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_removeTextureImage(Qt3DRender::QAbstractTexture* this_ptr, Qt3DRender::QAbstractTextureImage* textureImage);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QAbstractTexture_samples(const Qt3DRender::QAbstractTexture* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_setComparisonFunction(Qt3DRender::QAbstractTexture* this_ptr, Qt3DRender::QAbstractTexture::ComparisonFunction function);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_setComparisonMode(Qt3DRender::QAbstractTexture* this_ptr, Qt3DRender::QAbstractTexture::ComparisonMode mode);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_setDepth(Qt3DRender::QAbstractTexture* this_ptr, int depth);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_setFormat(Qt3DRender::QAbstractTexture* this_ptr, Qt3DRender::QAbstractTexture::TextureFormat format);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_setGenerateMipMaps(Qt3DRender::QAbstractTexture* this_ptr, bool gen);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_setHeight(Qt3DRender::QAbstractTexture* this_ptr, int height);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_setLayers(Qt3DRender::QAbstractTexture* this_ptr, int layers);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_setMagnificationFilter(Qt3DRender::QAbstractTexture* this_ptr, Qt3DRender::QAbstractTexture::Filter f);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_setMaximumAnisotropy(Qt3DRender::QAbstractTexture* this_ptr, float anisotropy);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_setMinificationFilter(Qt3DRender::QAbstractTexture* this_ptr, Qt3DRender::QAbstractTexture::Filter f);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_setSamples(Qt3DRender::QAbstractTexture* this_ptr, int samples);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_setSize_width(Qt3DRender::QAbstractTexture* this_ptr, int width);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_setSize_width_height(Qt3DRender::QAbstractTexture* this_ptr, int width, int height);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_setSize_width_height_depth(Qt3DRender::QAbstractTexture* this_ptr, int width, int height, int depth);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_setWidth(Qt3DRender::QAbstractTexture* this_ptr, int width);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_setWrapMode(Qt3DRender::QAbstractTexture* this_ptr, const Qt3DRender::QTextureWrapMode* wrapMode);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractTexture::Status qt_3d_render_c_Qt3DRender_QAbstractTexture_status(const Qt3DRender::QAbstractTexture* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractTexture::Target qt_3d_render_c_Qt3DRender_QAbstractTexture_target(const Qt3DRender::QAbstractTexture* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_textureImages_to_output(const Qt3DRender::QAbstractTexture* this_ptr, QVector< Qt3DRender::QAbstractTextureImage* >* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractTexture_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QAbstractTexture_width(const Qt3DRender::QAbstractTexture* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QTextureWrapMode* qt_3d_render_c_Qt3DRender_QAbstractTexture_wrapMode(Qt3DRender::QAbstractTexture* this_ptr);

} // extern "C"

#endif // QT_3D_RENDER_C_QABSTRACTTEXTURE_H
