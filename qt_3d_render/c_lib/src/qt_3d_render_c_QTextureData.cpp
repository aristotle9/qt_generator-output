#include "qt_3d_render_c_QTextureData.h"

void qt_3d_render_c_Qt3DRender_QTextureData_addImageData(Qt3DRender::QTextureData* this_ptr, const QSharedPointer< Qt3DRender::QTextureImageData >* imageData) {
  this_ptr->addImageData(*imageData);
}

void qt_3d_render_c_Qt3DRender_QTextureData_constructor(Qt3DRender::QTextureData* output) {
  new(output) Qt3DRender::QTextureData();
}

int qt_3d_render_c_Qt3DRender_QTextureData_depth(const Qt3DRender::QTextureData* this_ptr) {
  return this_ptr->depth();
}

void qt_3d_render_c_Qt3DRender_QTextureData_destructor(Qt3DRender::QTextureData* this_ptr) {
  qt_3d_render_c_call_destructor(this_ptr);
}

int qt_3d_render_c_Qt3DRender_QTextureData_height(const Qt3DRender::QTextureData* this_ptr) {
  return this_ptr->height();
}

void qt_3d_render_c_Qt3DRender_QTextureData_imageData_to_output(const Qt3DRender::QTextureData* this_ptr, QVector< QSharedPointer< Qt3DRender::QTextureImageData > >* output) {
  new(output) QVector< QSharedPointer< Qt3DRender::QTextureImageData > >(this_ptr->imageData());
}

bool qt_3d_render_c_Qt3DRender_QTextureData_isAutoMipMapGenerationEnabled(const Qt3DRender::QTextureData* this_ptr) {
  return this_ptr->isAutoMipMapGenerationEnabled();
}

int qt_3d_render_c_Qt3DRender_QTextureData_layers(const Qt3DRender::QTextureData* this_ptr) {
  return this_ptr->layers();
}

float qt_3d_render_c_Qt3DRender_QTextureData_maximumAnisotropy(const Qt3DRender::QTextureData* this_ptr) {
  return this_ptr->maximumAnisotropy();
}

void qt_3d_render_c_Qt3DRender_QTextureData_setAutoMipMapGenerationEnabled(Qt3DRender::QTextureData* this_ptr, bool isAutoMipMapGenerationEnabled) {
  this_ptr->setAutoMipMapGenerationEnabled(isAutoMipMapGenerationEnabled);
}

void qt_3d_render_c_Qt3DRender_QTextureData_setComparisonFunction(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QAbstractTexture::ComparisonFunction* comparisonFunction) {
  this_ptr->setComparisonFunction(*comparisonFunction);
}

void qt_3d_render_c_Qt3DRender_QTextureData_setComparisonMode(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QAbstractTexture::ComparisonMode* comparisonMode) {
  this_ptr->setComparisonMode(*comparisonMode);
}

void qt_3d_render_c_Qt3DRender_QTextureData_setDepth(Qt3DRender::QTextureData* this_ptr, int depth) {
  this_ptr->setDepth(depth);
}

void qt_3d_render_c_Qt3DRender_QTextureData_setFormat(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QAbstractTexture::TextureFormat* arg1) {
  this_ptr->setFormat(*arg1);
}

void qt_3d_render_c_Qt3DRender_QTextureData_setHeight(Qt3DRender::QTextureData* this_ptr, int height) {
  this_ptr->setHeight(height);
}

void qt_3d_render_c_Qt3DRender_QTextureData_setLayers(Qt3DRender::QTextureData* this_ptr, int layers) {
  this_ptr->setLayers(layers);
}

void qt_3d_render_c_Qt3DRender_QTextureData_setMagnificationFilter(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QAbstractTexture::Filter* filter) {
  this_ptr->setMagnificationFilter(*filter);
}

void qt_3d_render_c_Qt3DRender_QTextureData_setMaximumAnisotropy(Qt3DRender::QTextureData* this_ptr, float maximumAnisotropy) {
  this_ptr->setMaximumAnisotropy(maximumAnisotropy);
}

void qt_3d_render_c_Qt3DRender_QTextureData_setMinificationFilter(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QAbstractTexture::Filter* filter) {
  this_ptr->setMinificationFilter(*filter);
}

void qt_3d_render_c_Qt3DRender_QTextureData_setTarget(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QAbstractTexture::Target* target) {
  this_ptr->setTarget(*target);
}

void qt_3d_render_c_Qt3DRender_QTextureData_setWidth(Qt3DRender::QTextureData* this_ptr, int width) {
  this_ptr->setWidth(width);
}

void qt_3d_render_c_Qt3DRender_QTextureData_setWrapModeX(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QTextureWrapMode::WrapMode* wrapModeX) {
  this_ptr->setWrapModeX(*wrapModeX);
}

void qt_3d_render_c_Qt3DRender_QTextureData_setWrapModeY(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QTextureWrapMode::WrapMode* wrapModeY) {
  this_ptr->setWrapModeY(*wrapModeY);
}

void qt_3d_render_c_Qt3DRender_QTextureData_setWrapModeZ(Qt3DRender::QTextureData* this_ptr, const Qt3DRender::QTextureWrapMode::WrapMode* wrapModeZ) {
  this_ptr->setWrapModeZ(*wrapModeZ);
}

int qt_3d_render_c_Qt3DRender_QTextureData_width(const Qt3DRender::QTextureData* this_ptr) {
  return this_ptr->width();
}

