#include "qt_gui_c_QImage.h"

QImage* qt_gui_c_QImage_G_dynamic_cast_QImage_ptr(QPaintDevice* ptr) {
  return dynamic_cast<QImage*>(ptr);
}

QDataStream* qt_gui_c_QImage_G_operator_shl(QDataStream* arg1, const QImage* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_gui_c_QImage_G_operator_shl_to_output(const QDebug* arg1, const QImage* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_gui_c_QImage_G_operator_shr(QDataStream* arg1, QImage* arg2) {
  return &operator>>(*arg1, *arg2);
}

QImage* qt_gui_c_QImage_G_static_cast_QImage_ptr(QPaintDevice* ptr) {
  return static_cast<QImage*>(ptr);
}

QPaintDevice* qt_gui_c_QImage_G_static_cast_QPaintDevice_ptr(QImage* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

void qt_gui_c_QImage_G_swap(QImage* value1, QImage* value2) {
  swap(*value1, *value2);
}

bool qt_gui_c_QImage_allGray(const QImage* this_ptr) {
  return this_ptr->allGray();
}

QImage* qt_gui_c_QImage_alphaChannel_as_ptr(const QImage* this_ptr) {
  return new QImage(this_ptr->alphaChannel());
}

int qt_gui_c_QImage_bitPlaneCount(const QImage* this_ptr) {
  return this_ptr->bitPlaneCount();
}

unsigned char* qt_gui_c_QImage_bits(QImage* this_ptr) {
  return this_ptr->bits();
}

const unsigned char* qt_gui_c_QImage_bits_const(const QImage* this_ptr) {
  return this_ptr->bits();
}

int qt_gui_c_QImage_byteCount(const QImage* this_ptr) {
  return this_ptr->byteCount();
}

int qt_gui_c_QImage_bytesPerLine(const QImage* this_ptr) {
  return this_ptr->bytesPerLine();
}

qint64 qt_gui_c_QImage_cacheKey(const QImage* this_ptr) {
  return this_ptr->cacheKey();
}

unsigned int qt_gui_c_QImage_color(const QImage* this_ptr, int i) {
  return this_ptr->color(i);
}

int qt_gui_c_QImage_colorCount(const QImage* this_ptr) {
  return this_ptr->colorCount();
}

void qt_gui_c_QImage_colorTable_to_output(const QImage* this_ptr, QVector< unsigned int >* output) {
  new(output) QVector< unsigned int >(this_ptr->colorTable());
}

const unsigned char* qt_gui_c_QImage_constBits(const QImage* this_ptr) {
  return this_ptr->constBits();
}

const unsigned char* qt_gui_c_QImage_constScanLine(const QImage* this_ptr, int arg1) {
  return this_ptr->constScanLine(arg1);
}

void qt_gui_c_QImage_convert_to_QVariant_to_output(const QImage* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

QImage* qt_gui_c_QImage_copy_as_ptr_no_args(const QImage* this_ptr) {
  return new QImage(this_ptr->copy());
}

QImage* qt_gui_c_QImage_copy_as_ptr_rect(const QImage* this_ptr, const QRect* rect) {
  return new QImage(this_ptr->copy(*rect));
}

QImage* qt_gui_c_QImage_copy_as_ptr_x_y_w_h(const QImage* this_ptr, int x, int y, int w, int h) {
  return new QImage(this_ptr->copy(x, y, w, h));
}

QImage* qt_gui_c_QImage_createHeuristicMask_as_ptr_clipTight(const QImage* this_ptr, bool clipTight) {
  return new QImage(this_ptr->createHeuristicMask(clipTight));
}

QImage* qt_gui_c_QImage_createHeuristicMask_as_ptr_no_args(const QImage* this_ptr) {
  return new QImage(this_ptr->createHeuristicMask());
}

QImage* qt_gui_c_QImage_createMaskFromColor_as_ptr_color(const QImage* this_ptr, unsigned int color) {
  return new QImage(this_ptr->createMaskFromColor(color));
}

QImage* qt_gui_c_QImage_createMaskFromColor_as_ptr_color_mode(const QImage* this_ptr, unsigned int color, const Qt::MaskMode* mode) {
  return new QImage(this_ptr->createMaskFromColor(color, *mode));
}

void qt_gui_c_QImage_delete(QImage* this_ptr) {
  delete this_ptr;
}

int qt_gui_c_QImage_depth(const QImage* this_ptr) {
  return this_ptr->depth();
}

void qt_gui_c_QImage_detach(QImage* this_ptr) {
  this_ptr->detach();
}

int qt_gui_c_QImage_devType(const QImage* this_ptr) {
  return this_ptr->devType();
}

double qt_gui_c_QImage_devicePixelRatio(const QImage* this_ptr) {
  return this_ptr->devicePixelRatio();
}

int qt_gui_c_QImage_dotsPerMeterX(const QImage* this_ptr) {
  return this_ptr->dotsPerMeterX();
}

int qt_gui_c_QImage_dotsPerMeterY(const QImage* this_ptr) {
  return this_ptr->dotsPerMeterY();
}

void qt_gui_c_QImage_fill_QColor(QImage* this_ptr, const QColor* color) {
  this_ptr->fill(*color);
}

void qt_gui_c_QImage_fill_Qt_GlobalColor(QImage* this_ptr, const Qt::GlobalColor* color) {
  this_ptr->fill(*color);
}

void qt_gui_c_QImage_fill_unsigned_int(QImage* this_ptr, unsigned int pixel) {
  this_ptr->fill(pixel);
}

QImage::Format qt_gui_c_QImage_format(const QImage* this_ptr) {
  return this_ptr->format();
}

QImage* qt_gui_c_QImage_fromData_as_ptr_data(const QByteArray* data) {
  return new QImage(QImage::fromData(*data));
}

QImage* qt_gui_c_QImage_fromData_as_ptr_data_format(const QByteArray* data, const char* format) {
  return new QImage(QImage::fromData(*data, format));
}

QImage* qt_gui_c_QImage_fromData_as_ptr_data_size(const unsigned char* data, int size) {
  return new QImage(QImage::fromData(data, size));
}

QImage* qt_gui_c_QImage_fromData_as_ptr_data_size_format(const unsigned char* data, int size, const char* format) {
  return new QImage(QImage::fromData(data, size, format));
}

bool qt_gui_c_QImage_hasAlphaChannel(const QImage* this_ptr) {
  return this_ptr->hasAlphaChannel();
}

int qt_gui_c_QImage_height(const QImage* this_ptr) {
  return this_ptr->height();
}

void qt_gui_c_QImage_invertPixels_arg1(QImage* this_ptr, QImage::InvertMode arg1) {
  this_ptr->invertPixels(arg1);
}

void qt_gui_c_QImage_invertPixels_no_args(QImage* this_ptr) {
  this_ptr->invertPixels();
}

bool qt_gui_c_QImage_isDetached(const QImage* this_ptr) {
  return this_ptr->isDetached();
}

bool qt_gui_c_QImage_isGrayscale(const QImage* this_ptr) {
  return this_ptr->isGrayscale();
}

bool qt_gui_c_QImage_isNull(const QImage* this_ptr) {
  return this_ptr->isNull();
}

bool qt_gui_c_QImage_loadFromData_buf_len(QImage* this_ptr, const unsigned char* buf, int len) {
  return this_ptr->loadFromData(buf, len);
}

bool qt_gui_c_QImage_loadFromData_buf_len_format(QImage* this_ptr, const unsigned char* buf, int len, const char* format) {
  return this_ptr->loadFromData(buf, len, format);
}

bool qt_gui_c_QImage_loadFromData_data(QImage* this_ptr, const QByteArray* data) {
  return this_ptr->loadFromData(*data);
}

bool qt_gui_c_QImage_loadFromData_data_aformat(QImage* this_ptr, const QByteArray* data, const char* aformat) {
  return this_ptr->loadFromData(*data, aformat);
}

bool qt_gui_c_QImage_load_device_format(QImage* this_ptr, QIODevice* device, const char* format) {
  return this_ptr->load(device, format);
}

bool qt_gui_c_QImage_load_fileName(QImage* this_ptr, const QString* fileName) {
  return this_ptr->load(*fileName);
}

bool qt_gui_c_QImage_load_fileName_format(QImage* this_ptr, const QString* fileName, const char* format) {
  return this_ptr->load(*fileName, format);
}

QImage* qt_gui_c_QImage_mirrored_as_ptr_horizontally(const QImage* this_ptr, bool horizontally) {
  return new QImage(this_ptr->mirrored(horizontally));
}

QImage* qt_gui_c_QImage_mirrored_as_ptr_horizontally_vertically(const QImage* this_ptr, bool horizontally, bool vertically) {
  return new QImage(this_ptr->mirrored(horizontally, vertically));
}

QImage* qt_gui_c_QImage_mirrored_as_ptr_no_args(const QImage* this_ptr) {
  return new QImage(this_ptr->mirrored());
}

QImage* qt_gui_c_QImage_new_const_QImage_ref(const QImage* arg1) {
  return new QImage(*arg1);
}

QImage* qt_gui_c_QImage_new_const_QSize_ref_QImage_Format(const QSize* size, QImage::Format format) {
  return new QImage(*size, format);
}

QImage* qt_gui_c_QImage_new_const_QString_ref(const QString* fileName) {
  return new QImage(*fileName);
}

QImage* qt_gui_c_QImage_new_const_QString_ref_const_char_ptr(const QString* fileName, const char* format) {
  return new QImage(*fileName, format);
}

QImage* qt_gui_c_QImage_new_const_unsigned_char_ptr_int_int_QImage_Format(const unsigned char* data, int width, int height, QImage::Format format) {
  return new QImage(data, width, height, format);
}

QImage* qt_gui_c_QImage_new_const_unsigned_char_ptr_int_int_QImage_Format_void_func_void_ptr(const unsigned char* data, int width, int height, QImage::Format format, void (*cleanupFunction)(void*)) {
  return new QImage(data, width, height, format, cleanupFunction);
}

QImage* qt_gui_c_QImage_new_const_unsigned_char_ptr_int_int_QImage_Format_void_func_void_ptr_void_ptr(const unsigned char* data, int width, int height, QImage::Format format, void (*cleanupFunction)(void*), void* cleanupInfo) {
  return new QImage(data, width, height, format, cleanupFunction, cleanupInfo);
}

QImage* qt_gui_c_QImage_new_const_unsigned_char_ptr_int_int_int_QImage_Format(const unsigned char* data, int width, int height, int bytesPerLine, QImage::Format format) {
  return new QImage(data, width, height, bytesPerLine, format);
}

QImage* qt_gui_c_QImage_new_const_unsigned_char_ptr_int_int_int_QImage_Format_void_func_void_ptr(const unsigned char* data, int width, int height, int bytesPerLine, QImage::Format format, void (*cleanupFunction)(void*)) {
  return new QImage(data, width, height, bytesPerLine, format, cleanupFunction);
}

QImage* qt_gui_c_QImage_new_const_unsigned_char_ptr_int_int_int_QImage_Format_void_func_void_ptr_void_ptr(const unsigned char* data, int width, int height, int bytesPerLine, QImage::Format format, void (*cleanupFunction)(void*), void* cleanupInfo) {
  return new QImage(data, width, height, bytesPerLine, format, cleanupFunction, cleanupInfo);
}

QImage* qt_gui_c_QImage_new_int_int_QImage_Format(int width, int height, QImage::Format format) {
  return new QImage(width, height, format);
}

QImage* qt_gui_c_QImage_new_no_args() {
  return new QImage();
}

QImage* qt_gui_c_QImage_new_unsigned_char_ptr_int_int_QImage_Format(unsigned char* data, int width, int height, QImage::Format format) {
  return new QImage(data, width, height, format);
}

QImage* qt_gui_c_QImage_new_unsigned_char_ptr_int_int_QImage_Format_void_func_void_ptr(unsigned char* data, int width, int height, QImage::Format format, void (*cleanupFunction)(void*)) {
  return new QImage(data, width, height, format, cleanupFunction);
}

QImage* qt_gui_c_QImage_new_unsigned_char_ptr_int_int_QImage_Format_void_func_void_ptr_void_ptr(unsigned char* data, int width, int height, QImage::Format format, void (*cleanupFunction)(void*), void* cleanupInfo) {
  return new QImage(data, width, height, format, cleanupFunction, cleanupInfo);
}

QImage* qt_gui_c_QImage_new_unsigned_char_ptr_int_int_int_QImage_Format(unsigned char* data, int width, int height, int bytesPerLine, QImage::Format format) {
  return new QImage(data, width, height, bytesPerLine, format);
}

QImage* qt_gui_c_QImage_new_unsigned_char_ptr_int_int_int_QImage_Format_void_func_void_ptr(unsigned char* data, int width, int height, int bytesPerLine, QImage::Format format, void (*cleanupFunction)(void*)) {
  return new QImage(data, width, height, bytesPerLine, format, cleanupFunction);
}

QImage* qt_gui_c_QImage_new_unsigned_char_ptr_int_int_int_QImage_Format_void_func_void_ptr_void_ptr(unsigned char* data, int width, int height, int bytesPerLine, QImage::Format format, void (*cleanupFunction)(void*), void* cleanupInfo) {
  return new QImage(data, width, height, bytesPerLine, format, cleanupFunction, cleanupInfo);
}

void qt_gui_c_QImage_offset_to_output(const QImage* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->offset());
}

QImage* qt_gui_c_QImage_operator_assign(QImage* this_ptr, const QImage* arg1) {
  return &this_ptr->operator=(*arg1);
}

bool qt_gui_c_QImage_operator_eq(const QImage* this_ptr, const QImage* arg1) {
  return this_ptr->operator==(*arg1);
}

bool qt_gui_c_QImage_operator_neq(const QImage* this_ptr, const QImage* arg1) {
  return this_ptr->operator!=(*arg1);
}

QPaintEngine* qt_gui_c_QImage_paintEngine(const QImage* this_ptr) {
  return this_ptr->paintEngine();
}

void qt_gui_c_QImage_pixelColor_to_output_pt(const QImage* this_ptr, const QPoint* pt, QColor* output) {
  new(output) QColor(this_ptr->pixelColor(*pt));
}

void qt_gui_c_QImage_pixelColor_to_output_x_y(const QImage* this_ptr, int x, int y, QColor* output) {
  new(output) QColor(this_ptr->pixelColor(x, y));
}

void qt_gui_c_QImage_pixelFormat_to_output(const QImage* this_ptr, QPixelFormat* output) {
  new(output) QPixelFormat(this_ptr->pixelFormat());
}

int qt_gui_c_QImage_pixelIndex_pt(const QImage* this_ptr, const QPoint* pt) {
  return this_ptr->pixelIndex(*pt);
}

int qt_gui_c_QImage_pixelIndex_x_y(const QImage* this_ptr, int x, int y) {
  return this_ptr->pixelIndex(x, y);
}

unsigned int qt_gui_c_QImage_pixel_pt(const QImage* this_ptr, const QPoint* pt) {
  return this_ptr->pixel(*pt);
}

unsigned int qt_gui_c_QImage_pixel_x_y(const QImage* this_ptr, int x, int y) {
  return this_ptr->pixel(x, y);
}

void qt_gui_c_QImage_rect_to_output(const QImage* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->rect());
}

bool qt_gui_c_QImage_reinterpretAsFormat(QImage* this_ptr, QImage::Format f) {
  return this_ptr->reinterpretAsFormat(f);
}

QImage* qt_gui_c_QImage_rgbSwapped_as_ptr(const QImage* this_ptr) {
  return new QImage(this_ptr->rgbSwapped());
}

bool qt_gui_c_QImage_save_device(const QImage* this_ptr, QIODevice* device) {
  return this_ptr->save(device);
}

bool qt_gui_c_QImage_save_device_format(const QImage* this_ptr, QIODevice* device, const char* format) {
  return this_ptr->save(device, format);
}

bool qt_gui_c_QImage_save_device_format_quality(const QImage* this_ptr, QIODevice* device, const char* format, int quality) {
  return this_ptr->save(device, format, quality);
}

bool qt_gui_c_QImage_save_fileName(const QImage* this_ptr, const QString* fileName) {
  return this_ptr->save(*fileName);
}

bool qt_gui_c_QImage_save_fileName_format(const QImage* this_ptr, const QString* fileName, const char* format) {
  return this_ptr->save(*fileName, format);
}

bool qt_gui_c_QImage_save_fileName_format_quality(const QImage* this_ptr, const QString* fileName, const char* format, int quality) {
  return this_ptr->save(*fileName, format, quality);
}

QImage* qt_gui_c_QImage_scaledToHeight_as_ptr_h(const QImage* this_ptr, int h) {
  return new QImage(this_ptr->scaledToHeight(h));
}

QImage* qt_gui_c_QImage_scaledToHeight_as_ptr_h_mode(const QImage* this_ptr, int h, const Qt::TransformationMode* mode) {
  return new QImage(this_ptr->scaledToHeight(h, *mode));
}

QImage* qt_gui_c_QImage_scaledToWidth_as_ptr_w(const QImage* this_ptr, int w) {
  return new QImage(this_ptr->scaledToWidth(w));
}

QImage* qt_gui_c_QImage_scaledToWidth_as_ptr_w_mode(const QImage* this_ptr, int w, const Qt::TransformationMode* mode) {
  return new QImage(this_ptr->scaledToWidth(w, *mode));
}

QImage* qt_gui_c_QImage_scaled_as_ptr_s(const QImage* this_ptr, const QSize* s) {
  return new QImage(this_ptr->scaled(*s));
}

QImage* qt_gui_c_QImage_scaled_as_ptr_s_aspectMode(const QImage* this_ptr, const QSize* s, const Qt::AspectRatioMode* aspectMode) {
  return new QImage(this_ptr->scaled(*s, *aspectMode));
}

QImage* qt_gui_c_QImage_scaled_as_ptr_s_aspectMode_mode(const QImage* this_ptr, const QSize* s, const Qt::AspectRatioMode* aspectMode, const Qt::TransformationMode* mode) {
  return new QImage(this_ptr->scaled(*s, *aspectMode, *mode));
}

QImage* qt_gui_c_QImage_scaled_as_ptr_w_h(const QImage* this_ptr, int w, int h) {
  return new QImage(this_ptr->scaled(w, h));
}

QImage* qt_gui_c_QImage_scaled_as_ptr_w_h_aspectMode(const QImage* this_ptr, int w, int h, const Qt::AspectRatioMode* aspectMode) {
  return new QImage(this_ptr->scaled(w, h, *aspectMode));
}

QImage* qt_gui_c_QImage_scaled_as_ptr_w_h_aspectMode_mode(const QImage* this_ptr, int w, int h, const Qt::AspectRatioMode* aspectMode, const Qt::TransformationMode* mode) {
  return new QImage(this_ptr->scaled(w, h, *aspectMode, *mode));
}

unsigned char* qt_gui_c_QImage_scanLine(QImage* this_ptr, int arg1) {
  return this_ptr->scanLine(arg1);
}

const unsigned char* qt_gui_c_QImage_scanLine_const(const QImage* this_ptr, int arg1) {
  return this_ptr->scanLine(arg1);
}

void qt_gui_c_QImage_setAlphaChannel(QImage* this_ptr, const QImage* alphaChannel) {
  this_ptr->setAlphaChannel(*alphaChannel);
}

void qt_gui_c_QImage_setColor(QImage* this_ptr, int i, unsigned int c) {
  this_ptr->setColor(i, c);
}

void qt_gui_c_QImage_setColorCount(QImage* this_ptr, int arg1) {
  this_ptr->setColorCount(arg1);
}

void qt_gui_c_QImage_setColorTable(QImage* this_ptr, const QVector< unsigned int >* colors) {
  this_ptr->setColorTable(*colors);
}

void qt_gui_c_QImage_setDevicePixelRatio(QImage* this_ptr, double scaleFactor) {
  this_ptr->setDevicePixelRatio(scaleFactor);
}

void qt_gui_c_QImage_setDotsPerMeterX(QImage* this_ptr, int arg1) {
  this_ptr->setDotsPerMeterX(arg1);
}

void qt_gui_c_QImage_setDotsPerMeterY(QImage* this_ptr, int arg1) {
  this_ptr->setDotsPerMeterY(arg1);
}

void qt_gui_c_QImage_setOffset(QImage* this_ptr, const QPoint* arg1) {
  this_ptr->setOffset(*arg1);
}

void qt_gui_c_QImage_setPixelColor_pt_c(QImage* this_ptr, const QPoint* pt, const QColor* c) {
  this_ptr->setPixelColor(*pt, *c);
}

void qt_gui_c_QImage_setPixelColor_x_y_c(QImage* this_ptr, int x, int y, const QColor* c) {
  this_ptr->setPixelColor(x, y, *c);
}

void qt_gui_c_QImage_setPixel_pt_index_or_rgb(QImage* this_ptr, const QPoint* pt, unsigned int index_or_rgb) {
  this_ptr->setPixel(*pt, index_or_rgb);
}

void qt_gui_c_QImage_setPixel_x_y_index_or_rgb(QImage* this_ptr, int x, int y, unsigned int index_or_rgb) {
  this_ptr->setPixel(x, y, index_or_rgb);
}

void qt_gui_c_QImage_setText(QImage* this_ptr, const QString* key, const QString* value) {
  this_ptr->setText(*key, *value);
}

void qt_gui_c_QImage_size_to_output(const QImage* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->size());
}

void qt_gui_c_QImage_swap(QImage* this_ptr, QImage* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QImage_textKeys_to_output(const QImage* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->textKeys());
}

void qt_gui_c_QImage_text_to_output_key(const QImage* this_ptr, const QString* key, QString* output) {
  new(output) QString(this_ptr->text(*key));
}

void qt_gui_c_QImage_text_to_output_no_args(const QImage* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_gui_c_QImage_toPixelFormat_to_output(const QImage::Format* format, QPixelFormat* output) {
  new(output) QPixelFormat(QImage::toPixelFormat(*format));
}

QImage* qt_gui_c_QImage_transformed_as_ptr_QMatrix(const QImage* this_ptr, const QMatrix* matrix) {
  return new QImage(this_ptr->transformed(*matrix));
}

QImage* qt_gui_c_QImage_transformed_as_ptr_QMatrix_Qt_TransformationMode(const QImage* this_ptr, const QMatrix* matrix, const Qt::TransformationMode* mode) {
  return new QImage(this_ptr->transformed(*matrix, *mode));
}

QImage* qt_gui_c_QImage_transformed_as_ptr_QTransform(const QImage* this_ptr, const QTransform* matrix) {
  return new QImage(this_ptr->transformed(*matrix));
}

QImage* qt_gui_c_QImage_transformed_as_ptr_QTransform_Qt_TransformationMode(const QImage* this_ptr, const QTransform* matrix, const Qt::TransformationMode* mode) {
  return new QImage(this_ptr->transformed(*matrix, *mode));
}

void qt_gui_c_QImage_trueMatrix_to_output_QMatrix_int_int(const QMatrix* arg1, int w, int h, QMatrix* output) {
  new(output) QMatrix(QImage::trueMatrix(*arg1, w, h));
}

void qt_gui_c_QImage_trueMatrix_to_output_QTransform_int_int(const QTransform* arg1, int w, int h, QTransform* output) {
  new(output) QTransform(QImage::trueMatrix(*arg1, w, h));
}

bool qt_gui_c_QImage_valid_pt(const QImage* this_ptr, const QPoint* pt) {
  return this_ptr->valid(*pt);
}

bool qt_gui_c_QImage_valid_x_y(const QImage* this_ptr, int x, int y) {
  return this_ptr->valid(x, y);
}

int qt_gui_c_QImage_width(const QImage* this_ptr) {
  return this_ptr->width();
}

