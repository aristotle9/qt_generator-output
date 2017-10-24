#ifndef QT_3D_RENDER_C_QTEXTUREDATA_H
#define QT_3D_RENDER_C_QTEXTUREDATA_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_addImageData(Qt3DRender::QTextureData* this_ptr, const QSharedPointer< Qt3DRender::QTextureImageData >* imageData);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_constructor(Qt3DRender::QTextureData* output);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QTextureData_depth(const Qt3DRender::QTextureData* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_destructor(Qt3DRender::QTextureData* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QTextureData_height(const Qt3DRender::QTextureData* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_imageData_to_output(const Qt3DRender::QTextureData* this_ptr, QVector< QSharedPointer< Qt3DRender::QTextureImageData > >* output);
QT_3D_RENDER_C_EXPORT bool qt_3d_render_c_Qt3DRender_QTextureData_isAutoMipMapGenerationEnabled(const Qt3DRender::QTextureData* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QTextureData_layers(const Qt3DRender::QTextureData* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QTextureData_maximumAnisotropy(const Qt3DRender::QTextureData* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_setAutoMipMapGenerationEnabled(Qt3DRender::QTextureData* this_ptr, bool isAutoMipMapGenerationEnabled);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_setComparisonFunction(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QAbstractTexture::ComparisonFunction* comparisonFunction);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_setComparisonMode(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QAbstractTexture::ComparisonMode* comparisonMode);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_setDepth(Qt3DRender::QTextureData* this_ptr, int depth);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_setFormat(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QAbstractTexture::TextureFormat* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_setHeight(Qt3DRender::QTextureData* this_ptr, int height);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_setLayers(Qt3DRender::QTextureData* this_ptr, int layers);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_setMagnificationFilter(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QAbstractTexture::Filter* filter);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_setMaximumAnisotropy(Qt3DRender::QTextureData* this_ptr, float maximumAnisotropy);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_setMinificationFilter(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QAbstractTexture::Filter* filter);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_setTarget(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QAbstractTexture::Target* target);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_setWidth(Qt3DRender::QTextureData* this_ptr, int width);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_setWrapModeX(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QTextureWrapMode::WrapMode* wrapModeX);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_setWrapModeY(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QTextureWrapMode::WrapMode* wrapModeY);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTextureData_setWrapModeZ(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QTextureWrapMode::WrapMode* wrapModeZ);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QTextureData_width(const Qt3DRender::QTextureData* this_ptr);

} // extern "C"

#endif // QT_3D_RENDER_C_QTEXTUREDATA_H
