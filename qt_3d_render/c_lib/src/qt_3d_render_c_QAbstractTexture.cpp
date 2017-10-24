#include "qt_3d_render_c_QAbstractTexture.h"

QObject* qt_3d_render_c_QAbstractTexture_G_static_cast_QObject_ptr(Qt3DRender::QAbstractTexture* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QAbstractTexture_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QAbstractTexture* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QAbstractTexture* qt_3d_render_c_QAbstractTexture_G_static_cast_Qt3DRender_QAbstractTexture_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QAbstractTexture*>(ptr);
}

Qt3DRender::QAbstractTexture* qt_3d_render_c_QAbstractTexture_G_static_cast_Qt3DRender_QAbstractTexture_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QAbstractTexture*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_addTextureImage(Qt3DRender::QAbstractTexture* this_ptr, Qt3DRender::QAbstractTextureImage* textureImage) {
  this_ptr->addTextureImage(textureImage);
}

Qt3DRender::QAbstractTexture::ComparisonFunction qt_3d_render_c_Qt3DRender_QAbstractTexture_comparisonFunction(const Qt3DRender::QAbstractTexture* this_ptr) {
  return this_ptr->comparisonFunction();
}

Qt3DRender::QAbstractTexture::ComparisonMode qt_3d_render_c_Qt3DRender_QAbstractTexture_comparisonMode(const Qt3DRender::QAbstractTexture* this_ptr) {
  return this_ptr->comparisonMode();
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_dataGenerator_to_output(const Qt3DRender::QAbstractTexture* this_ptr, QSharedPointer< Qt3DRender::QTextureGenerator >* output) {
  new(output) QSharedPointer< Qt3DRender::QTextureGenerator >(this_ptr->dataGenerator());
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_delete(Qt3DRender::QAbstractTexture* this_ptr) {
  delete this_ptr;
}

int qt_3d_render_c_Qt3DRender_QAbstractTexture_depth(const Qt3DRender::QAbstractTexture* this_ptr) {
  return this_ptr->depth();
}

Qt3DRender::QAbstractTexture::TextureFormat qt_3d_render_c_Qt3DRender_QAbstractTexture_format(const Qt3DRender::QAbstractTexture* this_ptr) {
  return this_ptr->format();
}

bool qt_3d_render_c_Qt3DRender_QAbstractTexture_generateMipMaps(const Qt3DRender::QAbstractTexture* this_ptr) {
  return this_ptr->generateMipMaps();
}

int qt_3d_render_c_Qt3DRender_QAbstractTexture_height(const Qt3DRender::QAbstractTexture* this_ptr) {
  return this_ptr->height();
}

int qt_3d_render_c_Qt3DRender_QAbstractTexture_layers(const Qt3DRender::QAbstractTexture* this_ptr) {
  return this_ptr->layers();
}

Qt3DRender::QAbstractTexture::Filter qt_3d_render_c_Qt3DRender_QAbstractTexture_magnificationFilter(const Qt3DRender::QAbstractTexture* this_ptr) {
  return this_ptr->magnificationFilter();
}

float qt_3d_render_c_Qt3DRender_QAbstractTexture_maximumAnisotropy(const Qt3DRender::QAbstractTexture* this_ptr) {
  return this_ptr->maximumAnisotropy();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QAbstractTexture_metaObject(const Qt3DRender::QAbstractTexture* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QAbstractTexture::Filter qt_3d_render_c_Qt3DRender_QAbstractTexture_minificationFilter(const Qt3DRender::QAbstractTexture* this_ptr) {
  return this_ptr->minificationFilter();
}

int qt_3d_render_c_Qt3DRender_QAbstractTexture_qt_metacall(Qt3DRender::QAbstractTexture* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QAbstractTexture_qt_metacast(Qt3DRender::QAbstractTexture* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_removeTextureImage(Qt3DRender::QAbstractTexture* this_ptr, Qt3DRender::QAbstractTextureImage* textureImage) {
  this_ptr->removeTextureImage(textureImage);
}

int qt_3d_render_c_Qt3DRender_QAbstractTexture_samples(const Qt3DRender::QAbstractTexture* this_ptr) {
  return this_ptr->samples();
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_setComparisonFunction(Qt3DRender::QAbstractTexture* this_ptr, Qt3DRender::QAbstractTexture::ComparisonFunction function) {
  this_ptr->setComparisonFunction(function);
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_setComparisonMode(Qt3DRender::QAbstractTexture* this_ptr, Qt3DRender::QAbstractTexture::ComparisonMode mode) {
  this_ptr->setComparisonMode(mode);
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_setDepth(Qt3DRender::QAbstractTexture* this_ptr, int depth) {
  this_ptr->setDepth(depth);
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_setFormat(Qt3DRender::QAbstractTexture* this_ptr, Qt3DRender::QAbstractTexture::TextureFormat format) {
  this_ptr->setFormat(format);
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_setGenerateMipMaps(Qt3DRender::QAbstractTexture* this_ptr, bool gen) {
  this_ptr->setGenerateMipMaps(gen);
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_setHeight(Qt3DRender::QAbstractTexture* this_ptr, int height) {
  this_ptr->setHeight(height);
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_setLayers(Qt3DRender::QAbstractTexture* this_ptr, int layers) {
  this_ptr->setLayers(layers);
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_setMagnificationFilter(Qt3DRender::QAbstractTexture* this_ptr, Qt3DRender::QAbstractTexture::Filter f) {
  this_ptr->setMagnificationFilter(f);
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_setMaximumAnisotropy(Qt3DRender::QAbstractTexture* this_ptr, float anisotropy) {
  this_ptr->setMaximumAnisotropy(anisotropy);
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_setMinificationFilter(Qt3DRender::QAbstractTexture* this_ptr, Qt3DRender::QAbstractTexture::Filter f) {
  this_ptr->setMinificationFilter(f);
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_setSamples(Qt3DRender::QAbstractTexture* this_ptr, int samples) {
  this_ptr->setSamples(samples);
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_setSize_width(Qt3DRender::QAbstractTexture* this_ptr, int width) {
  this_ptr->setSize(width);
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_setSize_width_height(Qt3DRender::QAbstractTexture* this_ptr, int width, int height) {
  this_ptr->setSize(width, height);
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_setSize_width_height_depth(Qt3DRender::QAbstractTexture* this_ptr, int width, int height, int depth) {
  this_ptr->setSize(width, height, depth);
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_setWidth(Qt3DRender::QAbstractTexture* this_ptr, int width) {
  this_ptr->setWidth(width);
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_setWrapMode(Qt3DRender::QAbstractTexture* this_ptr, const Qt3DRender::QTextureWrapMode* wrapMode) {
  this_ptr->setWrapMode(*wrapMode);
}

Qt3DRender::QAbstractTexture::Status qt_3d_render_c_Qt3DRender_QAbstractTexture_status(const Qt3DRender::QAbstractTexture* this_ptr) {
  return this_ptr->status();
}

Qt3DRender::QAbstractTexture::Target qt_3d_render_c_Qt3DRender_QAbstractTexture_target(const Qt3DRender::QAbstractTexture* this_ptr) {
  return this_ptr->target();
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_textureImages_to_output(const Qt3DRender::QAbstractTexture* this_ptr, QVector< Qt3DRender::QAbstractTextureImage* >* output) {
  new(output) QVector< Qt3DRender::QAbstractTextureImage* >(this_ptr->textureImages());
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QAbstractTexture::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QAbstractTexture_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QAbstractTexture::tr(s, c, n));
}

int qt_3d_render_c_Qt3DRender_QAbstractTexture_width(const Qt3DRender::QAbstractTexture* this_ptr) {
  return this_ptr->width();
}

Qt3DRender::QTextureWrapMode* qt_3d_render_c_Qt3DRender_QAbstractTexture_wrapMode(Qt3DRender::QAbstractTexture* this_ptr) {
  return this_ptr->wrapMode();
}

