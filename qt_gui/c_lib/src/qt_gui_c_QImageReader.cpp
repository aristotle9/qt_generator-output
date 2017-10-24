#include "qt_gui_c_QImageReader.h"

bool qt_gui_c_QImageReader_autoDetectImageFormat(const QImageReader* this_ptr) {
  return this_ptr->autoDetectImageFormat();
}

bool qt_gui_c_QImageReader_autoTransform(const QImageReader* this_ptr) {
  return this_ptr->autoTransform();
}

void qt_gui_c_QImageReader_backgroundColor_to_output(const QImageReader* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->backgroundColor());
}

bool qt_gui_c_QImageReader_canRead(const QImageReader* this_ptr) {
  return this_ptr->canRead();
}

void qt_gui_c_QImageReader_clipRect_to_output(const QImageReader* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->clipRect());
}

void qt_gui_c_QImageReader_constructor_device(QIODevice* device, QImageReader* output) {
  new(output) QImageReader(device);
}

void qt_gui_c_QImageReader_constructor_device_format(QIODevice* device, const QByteArray* format, QImageReader* output) {
  new(output) QImageReader(device, *format);
}

void qt_gui_c_QImageReader_constructor_fileName(const QString* fileName, QImageReader* output) {
  new(output) QImageReader(*fileName);
}

void qt_gui_c_QImageReader_constructor_fileName_format(const QString* fileName, const QByteArray* format, QImageReader* output) {
  new(output) QImageReader(*fileName, *format);
}

void qt_gui_c_QImageReader_constructor_no_args(QImageReader* output) {
  new(output) QImageReader();
}

int qt_gui_c_QImageReader_currentImageNumber(const QImageReader* this_ptr) {
  return this_ptr->currentImageNumber();
}

void qt_gui_c_QImageReader_currentImageRect_to_output(const QImageReader* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->currentImageRect());
}

bool qt_gui_c_QImageReader_decideFormatFromContent(const QImageReader* this_ptr) {
  return this_ptr->decideFormatFromContent();
}

void qt_gui_c_QImageReader_destructor(QImageReader* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

QIODevice* qt_gui_c_QImageReader_device(const QImageReader* this_ptr) {
  return this_ptr->device();
}

QImageReader::ImageReaderError qt_gui_c_QImageReader_error(const QImageReader* this_ptr) {
  return this_ptr->error();
}

void qt_gui_c_QImageReader_errorString_to_output(const QImageReader* this_ptr, QString* output) {
  new(output) QString(this_ptr->errorString());
}

void qt_gui_c_QImageReader_fileName_to_output(const QImageReader* this_ptr, QString* output) {
  new(output) QString(this_ptr->fileName());
}

void qt_gui_c_QImageReader_format_to_output(const QImageReader* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->format());
}

float qt_gui_c_QImageReader_gamma(const QImageReader* this_ptr) {
  return this_ptr->gamma();
}

int qt_gui_c_QImageReader_imageCount(const QImageReader* this_ptr) {
  return this_ptr->imageCount();
}

void qt_gui_c_QImageReader_imageFormat_to_output_device(QIODevice* device, QByteArray* output) {
  new(output) QByteArray(QImageReader::imageFormat(device));
}

void qt_gui_c_QImageReader_imageFormat_to_output_fileName(const QString* fileName, QByteArray* output) {
  new(output) QByteArray(QImageReader::imageFormat(*fileName));
}

bool qt_gui_c_QImageReader_jumpToImage(QImageReader* this_ptr, int imageNumber) {
  return this_ptr->jumpToImage(imageNumber);
}

bool qt_gui_c_QImageReader_jumpToNextImage(QImageReader* this_ptr) {
  return this_ptr->jumpToNextImage();
}

int qt_gui_c_QImageReader_loopCount(const QImageReader* this_ptr) {
  return this_ptr->loopCount();
}

int qt_gui_c_QImageReader_nextImageDelay(const QImageReader* this_ptr) {
  return this_ptr->nextImageDelay();
}

int qt_gui_c_QImageReader_quality(const QImageReader* this_ptr) {
  return this_ptr->quality();
}

bool qt_gui_c_QImageReader_read(QImageReader* this_ptr, QImage* image) {
  return this_ptr->read(image);
}

QImage* qt_gui_c_QImageReader_read_as_ptr(QImageReader* this_ptr) {
  return new QImage(this_ptr->read());
}

void qt_gui_c_QImageReader_scaledClipRect_to_output(const QImageReader* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->scaledClipRect());
}

void qt_gui_c_QImageReader_scaledSize_to_output(const QImageReader* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->scaledSize());
}

void qt_gui_c_QImageReader_setAutoDetectImageFormat(QImageReader* this_ptr, bool enabled) {
  this_ptr->setAutoDetectImageFormat(enabled);
}

void qt_gui_c_QImageReader_setAutoTransform(QImageReader* this_ptr, bool enabled) {
  this_ptr->setAutoTransform(enabled);
}

void qt_gui_c_QImageReader_setBackgroundColor(QImageReader* this_ptr, const QColor* color) {
  this_ptr->setBackgroundColor(*color);
}

void qt_gui_c_QImageReader_setClipRect(QImageReader* this_ptr, const QRect* rect) {
  this_ptr->setClipRect(*rect);
}

void qt_gui_c_QImageReader_setDecideFormatFromContent(QImageReader* this_ptr, bool ignored) {
  this_ptr->setDecideFormatFromContent(ignored);
}

void qt_gui_c_QImageReader_setDevice(QImageReader* this_ptr, QIODevice* device) {
  this_ptr->setDevice(device);
}

void qt_gui_c_QImageReader_setFileName(QImageReader* this_ptr, const QString* fileName) {
  this_ptr->setFileName(*fileName);
}

void qt_gui_c_QImageReader_setFormat(QImageReader* this_ptr, const QByteArray* format) {
  this_ptr->setFormat(*format);
}

void qt_gui_c_QImageReader_setGamma(QImageReader* this_ptr, float gamma) {
  this_ptr->setGamma(gamma);
}

void qt_gui_c_QImageReader_setQuality(QImageReader* this_ptr, int quality) {
  this_ptr->setQuality(quality);
}

void qt_gui_c_QImageReader_setScaledClipRect(QImageReader* this_ptr, const QRect* rect) {
  this_ptr->setScaledClipRect(*rect);
}

void qt_gui_c_QImageReader_setScaledSize(QImageReader* this_ptr, const QSize* size) {
  this_ptr->setScaledSize(*size);
}

void qt_gui_c_QImageReader_size_to_output(const QImageReader* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->size());
}

void qt_gui_c_QImageReader_subType_to_output(const QImageReader* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->subType());
}

void qt_gui_c_QImageReader_supportedImageFormats_to_output(QList< QByteArray >* output) {
  new(output) QList< QByteArray >(QImageReader::supportedImageFormats());
}

void qt_gui_c_QImageReader_supportedMimeTypes_to_output(QList< QByteArray >* output) {
  new(output) QList< QByteArray >(QImageReader::supportedMimeTypes());
}

void qt_gui_c_QImageReader_supportedSubTypes_to_output(const QImageReader* this_ptr, QList< QByteArray >* output) {
  new(output) QList< QByteArray >(this_ptr->supportedSubTypes());
}

bool qt_gui_c_QImageReader_supportsAnimation(const QImageReader* this_ptr) {
  return this_ptr->supportsAnimation();
}

bool qt_gui_c_QImageReader_supportsOption(const QImageReader* this_ptr, const QImageIOHandler::ImageOption* option) {
  return this_ptr->supportsOption(*option);
}

void qt_gui_c_QImageReader_textKeys_to_output(const QImageReader* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->textKeys());
}

void qt_gui_c_QImageReader_text_to_output(const QImageReader* this_ptr, const QString* key, QString* output) {
  new(output) QString(this_ptr->text(*key));
}

void qt_gui_c_QImageReader_trUtf8_to_output(const char* sourceText, const char* disambiguation, int n, QString* output) {
  new(output) QString(QImageReader::trUtf8(sourceText, disambiguation, n));
}

void qt_gui_c_QImageReader_tr_to_output(const char* sourceText, const char* disambiguation, int n, QString* output) {
  new(output) QString(QImageReader::tr(sourceText, disambiguation, n));
}

