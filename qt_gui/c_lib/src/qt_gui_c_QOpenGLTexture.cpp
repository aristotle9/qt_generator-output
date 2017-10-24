#include "qt_gui_c_QOpenGLTexture.h"

void qt_gui_c_QOpenGLTexture_allocateStorage_no_args(QOpenGLTexture* this_ptr) {
  this_ptr->allocateStorage();
}

void qt_gui_c_QOpenGLTexture_allocateStorage_pixelFormat_pixelType(QOpenGLTexture* this_ptr, QOpenGLTexture::PixelFormat pixelFormat, QOpenGLTexture::PixelType pixelType) {
  this_ptr->allocateStorage(pixelFormat, pixelType);
}

void qt_gui_c_QOpenGLTexture_bind_no_args(QOpenGLTexture* this_ptr) {
  this_ptr->bind();
}

void qt_gui_c_QOpenGLTexture_bind_unit(QOpenGLTexture* this_ptr, unsigned int unit) {
  this_ptr->bind(unit);
}

void qt_gui_c_QOpenGLTexture_bind_unit_reset(QOpenGLTexture* this_ptr, unsigned int unit, QOpenGLTexture::TextureUnitReset reset) {
  this_ptr->bind(unit, reset);
}

void qt_gui_c_QOpenGLTexture_borderColor_float(const QOpenGLTexture* this_ptr, float* border) {
  this_ptr->borderColor(border);
}

void qt_gui_c_QOpenGLTexture_borderColor_int(const QOpenGLTexture* this_ptr, int* border) {
  this_ptr->borderColor(border);
}

void qt_gui_c_QOpenGLTexture_borderColor_to_output(const QOpenGLTexture* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->borderColor());
}

void qt_gui_c_QOpenGLTexture_borderColor_unsigned_int(const QOpenGLTexture* this_ptr, unsigned int* border) {
  this_ptr->borderColor(border);
}

GLuint qt_gui_c_QOpenGLTexture_boundTextureId_target(QOpenGLTexture::BindingTarget target) {
  return QOpenGLTexture::boundTextureId(target);
}

GLuint qt_gui_c_QOpenGLTexture_boundTextureId_unit_target(unsigned int unit, QOpenGLTexture::BindingTarget target) {
  return QOpenGLTexture::boundTextureId(unit, target);
}

QOpenGLTexture::ComparisonFunction qt_gui_c_QOpenGLTexture_comparisonFunction(const QOpenGLTexture* this_ptr) {
  return this_ptr->comparisonFunction();
}

QOpenGLTexture::ComparisonMode qt_gui_c_QOpenGLTexture_comparisonMode(const QOpenGLTexture* this_ptr) {
  return this_ptr->comparisonMode();
}

bool qt_gui_c_QOpenGLTexture_create(QOpenGLTexture* this_ptr) {
  return this_ptr->create();
}

QOpenGLTexture* qt_gui_c_QOpenGLTexture_createTextureView(const QOpenGLTexture* this_ptr, QOpenGLTexture::Target target, QOpenGLTexture::TextureFormat viewFormat, int minimumMipmapLevel, int maximumMipmapLevel, int minimumLayer, int maximumLayer) {
  return this_ptr->createTextureView(target, viewFormat, minimumMipmapLevel, maximumMipmapLevel, minimumLayer, maximumLayer);
}

void qt_gui_c_QOpenGLTexture_delete(QOpenGLTexture* this_ptr) {
  delete this_ptr;
}

int qt_gui_c_QOpenGLTexture_depth(const QOpenGLTexture* this_ptr) {
  return this_ptr->depth();
}

QOpenGLTexture::DepthStencilMode qt_gui_c_QOpenGLTexture_depthStencilMode(const QOpenGLTexture* this_ptr) {
  return this_ptr->depthStencilMode();
}

void qt_gui_c_QOpenGLTexture_destroy(QOpenGLTexture* this_ptr) {
  this_ptr->destroy();
}

int qt_gui_c_QOpenGLTexture_faces(const QOpenGLTexture* this_ptr) {
  return this_ptr->faces();
}

QOpenGLTexture::TextureFormat qt_gui_c_QOpenGLTexture_format(const QOpenGLTexture* this_ptr) {
  return this_ptr->format();
}

void qt_gui_c_QOpenGLTexture_generateMipMaps_baseLevel(QOpenGLTexture* this_ptr, int baseLevel) {
  this_ptr->generateMipMaps(baseLevel);
}

void qt_gui_c_QOpenGLTexture_generateMipMaps_baseLevel_resetBaseLevel(QOpenGLTexture* this_ptr, int baseLevel, bool resetBaseLevel) {
  this_ptr->generateMipMaps(baseLevel, resetBaseLevel);
}

void qt_gui_c_QOpenGLTexture_generateMipMaps_no_args(QOpenGLTexture* this_ptr) {
  this_ptr->generateMipMaps();
}

bool qt_gui_c_QOpenGLTexture_hasFeature(QOpenGLTexture::Feature feature) {
  return QOpenGLTexture::hasFeature(feature);
}

int qt_gui_c_QOpenGLTexture_height(const QOpenGLTexture* this_ptr) {
  return this_ptr->height();
}

bool qt_gui_c_QOpenGLTexture_isAutoMipMapGenerationEnabled(const QOpenGLTexture* this_ptr) {
  return this_ptr->isAutoMipMapGenerationEnabled();
}

bool qt_gui_c_QOpenGLTexture_isBound_no_args(const QOpenGLTexture* this_ptr) {
  return this_ptr->isBound();
}

bool qt_gui_c_QOpenGLTexture_isBound_unit(QOpenGLTexture* this_ptr, unsigned int unit) {
  return this_ptr->isBound(unit);
}

bool qt_gui_c_QOpenGLTexture_isCreated(const QOpenGLTexture* this_ptr) {
  return this_ptr->isCreated();
}

bool qt_gui_c_QOpenGLTexture_isFixedSamplePositions(const QOpenGLTexture* this_ptr) {
  return this_ptr->isFixedSamplePositions();
}

bool qt_gui_c_QOpenGLTexture_isStorageAllocated(const QOpenGLTexture* this_ptr) {
  return this_ptr->isStorageAllocated();
}

bool qt_gui_c_QOpenGLTexture_isTextureView(const QOpenGLTexture* this_ptr) {
  return this_ptr->isTextureView();
}

int qt_gui_c_QOpenGLTexture_layers(const QOpenGLTexture* this_ptr) {
  return this_ptr->layers();
}

void qt_gui_c_QOpenGLTexture_levelOfDetailRange_to_output(const QOpenGLTexture* this_ptr, QPair< float, float >* output) {
  new(output) QPair< float, float >(this_ptr->levelOfDetailRange());
}

float qt_gui_c_QOpenGLTexture_levelofDetailBias(const QOpenGLTexture* this_ptr) {
  return this_ptr->levelofDetailBias();
}

QOpenGLTexture::Filter qt_gui_c_QOpenGLTexture_magnificationFilter(const QOpenGLTexture* this_ptr) {
  return this_ptr->magnificationFilter();
}

float qt_gui_c_QOpenGLTexture_maximumAnisotropy(const QOpenGLTexture* this_ptr) {
  return this_ptr->maximumAnisotropy();
}

float qt_gui_c_QOpenGLTexture_maximumLevelOfDetail(const QOpenGLTexture* this_ptr) {
  return this_ptr->maximumLevelOfDetail();
}

int qt_gui_c_QOpenGLTexture_maximumMipLevels(const QOpenGLTexture* this_ptr) {
  return this_ptr->maximumMipLevels();
}

void qt_gui_c_QOpenGLTexture_minMagFilters_to_output(const QOpenGLTexture* this_ptr, QPair< QOpenGLTexture::Filter, QOpenGLTexture::Filter >* output) {
  new(output) QPair< QOpenGLTexture::Filter, QOpenGLTexture::Filter >(this_ptr->minMagFilters());
}

QOpenGLTexture::Filter qt_gui_c_QOpenGLTexture_minificationFilter(const QOpenGLTexture* this_ptr) {
  return this_ptr->minificationFilter();
}

float qt_gui_c_QOpenGLTexture_minimumLevelOfDetail(const QOpenGLTexture* this_ptr) {
  return this_ptr->minimumLevelOfDetail();
}

int qt_gui_c_QOpenGLTexture_mipBaseLevel(const QOpenGLTexture* this_ptr) {
  return this_ptr->mipBaseLevel();
}

void qt_gui_c_QOpenGLTexture_mipLevelRange_to_output(const QOpenGLTexture* this_ptr, QPair< int, int >* output) {
  new(output) QPair< int, int >(this_ptr->mipLevelRange());
}

int qt_gui_c_QOpenGLTexture_mipLevels(const QOpenGLTexture* this_ptr) {
  return this_ptr->mipLevels();
}

int qt_gui_c_QOpenGLTexture_mipMaxLevel(const QOpenGLTexture* this_ptr) {
  return this_ptr->mipMaxLevel();
}

QOpenGLTexture* qt_gui_c_QOpenGLTexture_new_image(const QImage* image) {
  return new QOpenGLTexture(*image);
}

QOpenGLTexture* qt_gui_c_QOpenGLTexture_new_image_genMipMaps(const QImage* image, QOpenGLTexture::MipMapGeneration genMipMaps) {
  return new QOpenGLTexture(*image, genMipMaps);
}

QOpenGLTexture* qt_gui_c_QOpenGLTexture_new_target(QOpenGLTexture::Target target) {
  return new QOpenGLTexture(target);
}

void qt_gui_c_QOpenGLTexture_release_no_args(QOpenGLTexture* this_ptr) {
  this_ptr->release();
}

void qt_gui_c_QOpenGLTexture_release_unit(QOpenGLTexture* this_ptr, unsigned int unit) {
  this_ptr->release(unit);
}

void qt_gui_c_QOpenGLTexture_release_unit_reset(QOpenGLTexture* this_ptr, unsigned int unit, QOpenGLTexture::TextureUnitReset reset) {
  this_ptr->release(unit, reset);
}

int qt_gui_c_QOpenGLTexture_samples(const QOpenGLTexture* this_ptr) {
  return this_ptr->samples();
}

void qt_gui_c_QOpenGLTexture_setAutoMipMapGenerationEnabled(QOpenGLTexture* this_ptr, bool enabled) {
  this_ptr->setAutoMipMapGenerationEnabled(enabled);
}

void qt_gui_c_QOpenGLTexture_setBorderColor_QColor(QOpenGLTexture* this_ptr, const QColor* color) {
  this_ptr->setBorderColor(*color);
}

void qt_gui_c_QOpenGLTexture_setBorderColor_float_float_float_float(QOpenGLTexture* this_ptr, float r, float g, float b, float a) {
  this_ptr->setBorderColor(r, g, b, a);
}

void qt_gui_c_QOpenGLTexture_setBorderColor_int_int_int_int(QOpenGLTexture* this_ptr, int r, int g, int b, int a) {
  this_ptr->setBorderColor(r, g, b, a);
}

void qt_gui_c_QOpenGLTexture_setBorderColor_unsigned_int_unsigned_int_unsigned_int_unsigned_int(QOpenGLTexture* this_ptr, unsigned int r, unsigned int g, unsigned int b, unsigned int a) {
  this_ptr->setBorderColor(r, g, b, a);
}

void qt_gui_c_QOpenGLTexture_setComparisonFunction(QOpenGLTexture* this_ptr, QOpenGLTexture::ComparisonFunction function) {
  this_ptr->setComparisonFunction(function);
}

void qt_gui_c_QOpenGLTexture_setComparisonMode(QOpenGLTexture* this_ptr, QOpenGLTexture::ComparisonMode mode) {
  this_ptr->setComparisonMode(mode);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_const_void_ptr(QOpenGLTexture* this_ptr, int dataSize, const void* data) {
  this_ptr->setCompressedData(dataSize, data);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, int dataSize, const void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setCompressedData(dataSize, data, options);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_int_QOpenGLTexture_CubeMapFace_int_const_void_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, QOpenGLTexture::CubeMapFace cubeFace, int dataSize, const void* data) {
  this_ptr->setCompressedData(mipLevel, layer, cubeFace, dataSize, data);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_int_QOpenGLTexture_CubeMapFace_int_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, QOpenGLTexture::CubeMapFace cubeFace, int dataSize, const void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setCompressedData(mipLevel, layer, cubeFace, dataSize, data, options);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_int_QOpenGLTexture_CubeMapFace_int_void_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, QOpenGLTexture::CubeMapFace cubeFace, int dataSize, void* data) {
  this_ptr->setCompressedData(mipLevel, layer, cubeFace, dataSize, data);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_int_QOpenGLTexture_CubeMapFace_int_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, QOpenGLTexture::CubeMapFace cubeFace, int dataSize, void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setCompressedData(mipLevel, layer, cubeFace, dataSize, data, options);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_int_const_void_ptr(QOpenGLTexture* this_ptr, int mipLevel, int dataSize, const void* data) {
  this_ptr->setCompressedData(mipLevel, dataSize, data);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_int_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, int mipLevel, int dataSize, const void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setCompressedData(mipLevel, dataSize, data, options);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_int_int_QOpenGLTexture_CubeMapFace_int_const_void_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, int layerCount, QOpenGLTexture::CubeMapFace cubeFace, int dataSize, const void* data) {
  this_ptr->setCompressedData(mipLevel, layer, layerCount, cubeFace, dataSize, data);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_int_int_QOpenGLTexture_CubeMapFace_int_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, int layerCount, QOpenGLTexture::CubeMapFace cubeFace, int dataSize, const void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setCompressedData(mipLevel, layer, layerCount, cubeFace, dataSize, data, options);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_int_int_const_void_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, int dataSize, const void* data) {
  this_ptr->setCompressedData(mipLevel, layer, dataSize, data);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_int_int_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, int dataSize, const void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setCompressedData(mipLevel, layer, dataSize, data, options);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_int_int_void_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, int dataSize, void* data) {
  this_ptr->setCompressedData(mipLevel, layer, dataSize, data);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_int_int_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, int dataSize, void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setCompressedData(mipLevel, layer, dataSize, data, options);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_int_void_ptr(QOpenGLTexture* this_ptr, int mipLevel, int dataSize, void* data) {
  this_ptr->setCompressedData(mipLevel, dataSize, data);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_int_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, int mipLevel, int dataSize, void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setCompressedData(mipLevel, dataSize, data, options);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_void_ptr(QOpenGLTexture* this_ptr, int dataSize, void* data) {
  this_ptr->setCompressedData(dataSize, data);
}

void qt_gui_c_QOpenGLTexture_setCompressedData_int_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, int dataSize, void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setCompressedData(dataSize, data, options);
}

void qt_gui_c_QOpenGLTexture_setData_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr(QOpenGLTexture* this_ptr, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data) {
  this_ptr->setData(sourceFormat, sourceType, data);
}

void qt_gui_c_QOpenGLTexture_setData_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setData(sourceFormat, sourceType, data, options);
}

void qt_gui_c_QOpenGLTexture_setData_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_void_ptr(QOpenGLTexture* this_ptr, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, void* data) {
  this_ptr->setData(sourceFormat, sourceType, data);
}

void qt_gui_c_QOpenGLTexture_setData_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setData(sourceFormat, sourceType, data, options);
}

void qt_gui_c_QOpenGLTexture_setData_const_QImage_ref(QOpenGLTexture* this_ptr, const QImage* image) {
  this_ptr->setData(*image);
}

void qt_gui_c_QOpenGLTexture_setData_const_QImage_ref_QOpenGLTexture_MipMapGeneration(QOpenGLTexture* this_ptr, const QImage* image, QOpenGLTexture::MipMapGeneration genMipMaps) {
  this_ptr->setData(*image, genMipMaps);
}

void qt_gui_c_QOpenGLTexture_setData_int_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr(QOpenGLTexture* this_ptr, int mipLevel, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data) {
  this_ptr->setData(mipLevel, sourceFormat, sourceType, data);
}

void qt_gui_c_QOpenGLTexture_setData_int_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, int mipLevel, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setData(mipLevel, sourceFormat, sourceType, data, options);
}

void qt_gui_c_QOpenGLTexture_setData_int_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_void_ptr(QOpenGLTexture* this_ptr, int mipLevel, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, void* data) {
  this_ptr->setData(mipLevel, sourceFormat, sourceType, data);
}

void qt_gui_c_QOpenGLTexture_setData_int_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, int mipLevel, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setData(mipLevel, sourceFormat, sourceType, data, options);
}

void qt_gui_c_QOpenGLTexture_setData_int_int_QOpenGLTexture_CubeMapFace_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, QOpenGLTexture::CubeMapFace cubeFace, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data) {
  this_ptr->setData(mipLevel, layer, cubeFace, sourceFormat, sourceType, data);
}

void qt_gui_c_QOpenGLTexture_setData_int_int_QOpenGLTexture_CubeMapFace_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, QOpenGLTexture::CubeMapFace cubeFace, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setData(mipLevel, layer, cubeFace, sourceFormat, sourceType, data, options);
}

void qt_gui_c_QOpenGLTexture_setData_int_int_QOpenGLTexture_CubeMapFace_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_void_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, QOpenGLTexture::CubeMapFace cubeFace, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, void* data) {
  this_ptr->setData(mipLevel, layer, cubeFace, sourceFormat, sourceType, data);
}

void qt_gui_c_QOpenGLTexture_setData_int_int_QOpenGLTexture_CubeMapFace_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, QOpenGLTexture::CubeMapFace cubeFace, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setData(mipLevel, layer, cubeFace, sourceFormat, sourceType, data, options);
}

void qt_gui_c_QOpenGLTexture_setData_int_int_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data) {
  this_ptr->setData(mipLevel, layer, sourceFormat, sourceType, data);
}

void qt_gui_c_QOpenGLTexture_setData_int_int_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setData(mipLevel, layer, sourceFormat, sourceType, data, options);
}

void qt_gui_c_QOpenGLTexture_setData_int_int_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_void_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, void* data) {
  this_ptr->setData(mipLevel, layer, sourceFormat, sourceType, data);
}

void qt_gui_c_QOpenGLTexture_setData_int_int_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setData(mipLevel, layer, sourceFormat, sourceType, data, options);
}

void qt_gui_c_QOpenGLTexture_setData_int_int_int_QOpenGLTexture_CubeMapFace_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, int layerCount, QOpenGLTexture::CubeMapFace cubeFace, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data) {
  this_ptr->setData(mipLevel, layer, layerCount, cubeFace, sourceFormat, sourceType, data);
}

void qt_gui_c_QOpenGLTexture_setData_int_int_int_QOpenGLTexture_CubeMapFace_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(QOpenGLTexture* this_ptr, int mipLevel, int layer, int layerCount, QOpenGLTexture::CubeMapFace cubeFace, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data, const QOpenGLPixelTransferOptions* options) {
  this_ptr->setData(mipLevel, layer, layerCount, cubeFace, sourceFormat, sourceType, data, options);
}

void qt_gui_c_QOpenGLTexture_setDepthStencilMode(QOpenGLTexture* this_ptr, QOpenGLTexture::DepthStencilMode mode) {
  this_ptr->setDepthStencilMode(mode);
}

void qt_gui_c_QOpenGLTexture_setFixedSamplePositions(QOpenGLTexture* this_ptr, bool fixed) {
  this_ptr->setFixedSamplePositions(fixed);
}

void qt_gui_c_QOpenGLTexture_setFormat(QOpenGLTexture* this_ptr, QOpenGLTexture::TextureFormat format) {
  this_ptr->setFormat(format);
}

void qt_gui_c_QOpenGLTexture_setLayers(QOpenGLTexture* this_ptr, int layers) {
  this_ptr->setLayers(layers);
}

void qt_gui_c_QOpenGLTexture_setLevelOfDetailRange(QOpenGLTexture* this_ptr, float min, float max) {
  this_ptr->setLevelOfDetailRange(min, max);
}

void qt_gui_c_QOpenGLTexture_setLevelofDetailBias(QOpenGLTexture* this_ptr, float bias) {
  this_ptr->setLevelofDetailBias(bias);
}

void qt_gui_c_QOpenGLTexture_setMagnificationFilter(QOpenGLTexture* this_ptr, QOpenGLTexture::Filter filter) {
  this_ptr->setMagnificationFilter(filter);
}

void qt_gui_c_QOpenGLTexture_setMaximumAnisotropy(QOpenGLTexture* this_ptr, float anisotropy) {
  this_ptr->setMaximumAnisotropy(anisotropy);
}

void qt_gui_c_QOpenGLTexture_setMaximumLevelOfDetail(QOpenGLTexture* this_ptr, float value) {
  this_ptr->setMaximumLevelOfDetail(value);
}

void qt_gui_c_QOpenGLTexture_setMinMagFilters(QOpenGLTexture* this_ptr, QOpenGLTexture::Filter minificationFilter, QOpenGLTexture::Filter magnificationFilter) {
  this_ptr->setMinMagFilters(minificationFilter, magnificationFilter);
}

void qt_gui_c_QOpenGLTexture_setMinificationFilter(QOpenGLTexture* this_ptr, QOpenGLTexture::Filter filter) {
  this_ptr->setMinificationFilter(filter);
}

void qt_gui_c_QOpenGLTexture_setMinimumLevelOfDetail(QOpenGLTexture* this_ptr, float value) {
  this_ptr->setMinimumLevelOfDetail(value);
}

void qt_gui_c_QOpenGLTexture_setMipBaseLevel(QOpenGLTexture* this_ptr, int baseLevel) {
  this_ptr->setMipBaseLevel(baseLevel);
}

void qt_gui_c_QOpenGLTexture_setMipLevelRange(QOpenGLTexture* this_ptr, int baseLevel, int maxLevel) {
  this_ptr->setMipLevelRange(baseLevel, maxLevel);
}

void qt_gui_c_QOpenGLTexture_setMipLevels(QOpenGLTexture* this_ptr, int levels) {
  this_ptr->setMipLevels(levels);
}

void qt_gui_c_QOpenGLTexture_setMipMaxLevel(QOpenGLTexture* this_ptr, int maxLevel) {
  this_ptr->setMipMaxLevel(maxLevel);
}

void qt_gui_c_QOpenGLTexture_setSamples(QOpenGLTexture* this_ptr, int samples) {
  this_ptr->setSamples(samples);
}

void qt_gui_c_QOpenGLTexture_setSize_width(QOpenGLTexture* this_ptr, int width) {
  this_ptr->setSize(width);
}

void qt_gui_c_QOpenGLTexture_setSize_width_height(QOpenGLTexture* this_ptr, int width, int height) {
  this_ptr->setSize(width, height);
}

void qt_gui_c_QOpenGLTexture_setSize_width_height_depth(QOpenGLTexture* this_ptr, int width, int height, int depth) {
  this_ptr->setSize(width, height, depth);
}

void qt_gui_c_QOpenGLTexture_setSwizzleMask_component_value(QOpenGLTexture* this_ptr, QOpenGLTexture::SwizzleComponent component, QOpenGLTexture::SwizzleValue value) {
  this_ptr->setSwizzleMask(component, value);
}

void qt_gui_c_QOpenGLTexture_setSwizzleMask_r_g_b_a(QOpenGLTexture* this_ptr, QOpenGLTexture::SwizzleValue r, QOpenGLTexture::SwizzleValue g, QOpenGLTexture::SwizzleValue b, QOpenGLTexture::SwizzleValue a) {
  this_ptr->setSwizzleMask(r, g, b, a);
}

void qt_gui_c_QOpenGLTexture_setWrapMode_direction_mode(QOpenGLTexture* this_ptr, QOpenGLTexture::CoordinateDirection direction, QOpenGLTexture::WrapMode mode) {
  this_ptr->setWrapMode(direction, mode);
}

void qt_gui_c_QOpenGLTexture_setWrapMode_mode(QOpenGLTexture* this_ptr, QOpenGLTexture::WrapMode mode) {
  this_ptr->setWrapMode(mode);
}

QOpenGLTexture::SwizzleValue qt_gui_c_QOpenGLTexture_swizzleMask(const QOpenGLTexture* this_ptr, QOpenGLTexture::SwizzleComponent component) {
  return this_ptr->swizzleMask(component);
}

QOpenGLTexture::Target qt_gui_c_QOpenGLTexture_target(const QOpenGLTexture* this_ptr) {
  return this_ptr->target();
}

GLuint qt_gui_c_QOpenGLTexture_textureId(const QOpenGLTexture* this_ptr) {
  return this_ptr->textureId();
}

int qt_gui_c_QOpenGLTexture_width(const QOpenGLTexture* this_ptr) {
  return this_ptr->width();
}

QOpenGLTexture::WrapMode qt_gui_c_QOpenGLTexture_wrapMode(const QOpenGLTexture* this_ptr, QOpenGLTexture::CoordinateDirection direction) {
  return this_ptr->wrapMode(direction);
}

