#ifndef QT_3D_RENDER_C_QTEXTUREIMAGEDATA_H
#define QT_3D_RENDER_C_QTEXTUREIMAGEDATA_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_cleanup(Qt3DRender::QTextureImageData* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_constructor(Qt3DRender::QTextureImageData* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_data_to_output_layer(const Qt3DRender::QTextureImageData* this_ptr, int layer, QByteArray* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_data_to_output_layer_face(const Qt3DRender::QTextureImageData* this_ptr, int layer, int face, QByteArray* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_data_to_output_layer_face_mipmapLevel(const Qt3DRender::QTextureImageData* this_ptr, int layer, int face, int mipmapLevel, QByteArray* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_data_to_output_no_args(const Qt3DRender::QTextureImageData* this_ptr, QByteArray* output);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QTextureImageData_depth(const Qt3DRender::QTextureImageData* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_destructor(Qt3DRender::QTextureImageData* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QTextureImageData_faces(const Qt3DRender::QTextureImageData* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QTextureImageData_height(const Qt3DRender::QTextureImageData* this_ptr);
QT_3D_RENDER_C_EXPORT bool qt_3d_render_c_Qt3DRender_QTextureImageData_isCompressed(const Qt3DRender::QTextureImageData* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QTextureImageData_layers(const Qt3DRender::QTextureImageData* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QTextureImageData_mipLevels(const Qt3DRender::QTextureImageData* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QTextureImageData* qt_3d_render_c_Qt3DRender_QTextureImageData_operator_assign(Qt3DRender::QTextureImageData* this_ptr, const Qt3DRender::QTextureImageData* other);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_setData_data_blockSize(Qt3DRender::QTextureImageData* this_ptr, const QByteArray* data, int blockSize);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_setData_data_blockSize_isCompressed(Qt3DRender::QTextureImageData* this_ptr, const QByteArray* data, int blockSize, bool isCompressed);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_setDepth(Qt3DRender::QTextureImageData* this_ptr, int depth);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_setFaces(Qt3DRender::QTextureImageData* this_ptr, int faces);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_setFormat(Qt3DRender::QTextureImageData* this_ptr, const QOpenGLTexture::TextureFormat* format);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_setHeight(Qt3DRender::QTextureImageData* this_ptr, int height);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_setImage(Qt3DRender::QTextureImageData* this_ptr, const QImage* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_setLayers(Qt3DRender::QTextureImageData* this_ptr, int layers);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_setMipLevels(Qt3DRender::QTextureImageData* this_ptr, int mipLevels);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_setPixelFormat(Qt3DRender::QTextureImageData* this_ptr, const QOpenGLTexture::PixelFormat* pixelFormat);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_setPixelType(Qt3DRender::QTextureImageData* this_ptr, const QOpenGLTexture::PixelType* pixelType);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_setTarget(Qt3DRender::QTextureImageData* this_ptr, const QOpenGLTexture::Target* target);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureImageData_setWidth(Qt3DRender::QTextureImageData* this_ptr, int width);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QTextureImageData_width(const Qt3DRender::QTextureImageData* this_ptr);

} // extern "C"

#endif // QT_3D_RENDER_C_QTEXTUREIMAGEDATA_H
