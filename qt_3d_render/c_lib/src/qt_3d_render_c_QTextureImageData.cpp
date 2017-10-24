#include "qt_3d_render_c_QTextureImageData.h"

void qt_3d_render_c_Qt3DRender_QTextureImageData_cleanup(Qt3DRender::QTextureImageData* this_ptr) {
  this_ptr->cleanup();
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_constructor(Qt3DRender::QTextureImageData* output) {
  new(output) Qt3DRender::QTextureImageData();
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_data_to_output_layer(const Qt3DRender::QTextureImageData* this_ptr, int layer, QByteArray* output) {
  new(output) QByteArray(this_ptr->data(layer));
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_data_to_output_layer_face(const Qt3DRender::QTextureImageData* this_ptr, int layer, int face, QByteArray* output) {
  new(output) QByteArray(this_ptr->data(layer, face));
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_data_to_output_layer_face_mipmapLevel(const Qt3DRender::QTextureImageData* this_ptr, int layer, int face, int mipmapLevel, QByteArray* output) {
  new(output) QByteArray(this_ptr->data(layer, face, mipmapLevel));
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_data_to_output_no_args(const Qt3DRender::QTextureImageData* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->data());
}

int qt_3d_render_c_Qt3DRender_QTextureImageData_depth(const Qt3DRender::QTextureImageData* this_ptr) {
  return this_ptr->depth();
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_destructor(Qt3DRender::QTextureImageData* this_ptr) {
  qt_3d_render_c_call_destructor(this_ptr);
}

int qt_3d_render_c_Qt3DRender_QTextureImageData_faces(const Qt3DRender::QTextureImageData* this_ptr) {
  return this_ptr->faces();
}

int qt_3d_render_c_Qt3DRender_QTextureImageData_height(const Qt3DRender::QTextureImageData* this_ptr) {
  return this_ptr->height();
}

bool qt_3d_render_c_Qt3DRender_QTextureImageData_isCompressed(const Qt3DRender::QTextureImageData* this_ptr) {
  return this_ptr->isCompressed();
}

int qt_3d_render_c_Qt3DRender_QTextureImageData_layers(const Qt3DRender::QTextureImageData* this_ptr) {
  return this_ptr->layers();
}

int qt_3d_render_c_Qt3DRender_QTextureImageData_mipLevels(const Qt3DRender::QTextureImageData* this_ptr) {
  return this_ptr->mipLevels();
}

Qt3DRender::QTextureImageData* qt_3d_render_c_Qt3DRender_QTextureImageData_operator_assign(Qt3DRender::QTextureImageData* this_ptr, const Qt3DRender::QTextureImageData* other) {
  return &this_ptr->operator=(*other);
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_setData_data_blockSize(Qt3DRender::QTextureImageData* this_ptr, const QByteArray* data, int blockSize) {
  this_ptr->setData(*data, blockSize);
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_setData_data_blockSize_isCompressed(Qt3DRender::QTextureImageData* this_ptr, const QByteArray* data, int blockSize, bool isCompressed) {
  this_ptr->setData(*data, blockSize, isCompressed);
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_setDepth(Qt3DRender::QTextureImageData* this_ptr, int depth) {
  this_ptr->setDepth(depth);
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_setFaces(Qt3DRender::QTextureImageData* this_ptr, int faces) {
  this_ptr->setFaces(faces);
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_setFormat(Qt3DRender::QTextureImageData* this_ptr, const QOpenGLTexture::TextureFormat* format) {
  this_ptr->setFormat(*format);
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_setHeight(Qt3DRender::QTextureImageData* this_ptr, int height) {
  this_ptr->setHeight(height);
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_setImage(Qt3DRender::QTextureImageData* this_ptr, const QImage* arg1) {
  this_ptr->setImage(*arg1);
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_setLayers(Qt3DRender::QTextureImageData* this_ptr, int layers) {
  this_ptr->setLayers(layers);
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_setMipLevels(Qt3DRender::QTextureImageData* this_ptr, int mipLevels) {
  this_ptr->setMipLevels(mipLevels);
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_setPixelFormat(Qt3DRender::QTextureImageData* this_ptr, const QOpenGLTexture::PixelFormat* pixelFormat) {
  this_ptr->setPixelFormat(*pixelFormat);
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_setPixelType(Qt3DRender::QTextureImageData* this_ptr, const QOpenGLTexture::PixelType* pixelType) {
  this_ptr->setPixelType(*pixelType);
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_setTarget(Qt3DRender::QTextureImageData* this_ptr, const QOpenGLTexture::Target* target) {
  this_ptr->setTarget(*target);
}

void qt_3d_render_c_Qt3DRender_QTextureImageData_setWidth(Qt3DRender::QTextureImageData* this_ptr, int width) {
  this_ptr->setWidth(width);
}

int qt_3d_render_c_Qt3DRender_QTextureImageData_width(const Qt3DRender::QTextureImageData* this_ptr) {
  return this_ptr->width();
}

