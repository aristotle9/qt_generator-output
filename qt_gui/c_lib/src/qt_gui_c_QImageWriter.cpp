#include "qt_gui_c_QImageWriter.h"

bool qt_gui_c_QImageWriter_canWrite(const QImageWriter* this_ptr) {
  return this_ptr->canWrite();
}

int qt_gui_c_QImageWriter_compression(const QImageWriter* this_ptr) {
  return this_ptr->compression();
}

void qt_gui_c_QImageWriter_delete(QImageWriter* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QImageWriter_description_to_output(const QImageWriter* this_ptr, QString* output) {
  new(output) QString(this_ptr->description());
}

QIODevice* qt_gui_c_QImageWriter_device(const QImageWriter* this_ptr) {
  return this_ptr->device();
}

QImageWriter::ImageWriterError qt_gui_c_QImageWriter_error(const QImageWriter* this_ptr) {
  return this_ptr->error();
}

void qt_gui_c_QImageWriter_errorString_to_output(const QImageWriter* this_ptr, QString* output) {
  new(output) QString(this_ptr->errorString());
}

void qt_gui_c_QImageWriter_fileName_to_output(const QImageWriter* this_ptr, QString* output) {
  new(output) QString(this_ptr->fileName());
}

void qt_gui_c_QImageWriter_format_to_output(const QImageWriter* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->format());
}

float qt_gui_c_QImageWriter_gamma(const QImageWriter* this_ptr) {
  return this_ptr->gamma();
}

QImageWriter* qt_gui_c_QImageWriter_new_device_format(QIODevice* device, const QByteArray* format) {
  return new QImageWriter(device, *format);
}

QImageWriter* qt_gui_c_QImageWriter_new_fileName(const QString* fileName) {
  return new QImageWriter(*fileName);
}

QImageWriter* qt_gui_c_QImageWriter_new_fileName_format(const QString* fileName, const QByteArray* format) {
  return new QImageWriter(*fileName, *format);
}

QImageWriter* qt_gui_c_QImageWriter_new_no_args() {
  return new QImageWriter();
}

bool qt_gui_c_QImageWriter_optimizedWrite(const QImageWriter* this_ptr) {
  return this_ptr->optimizedWrite();
}

bool qt_gui_c_QImageWriter_progressiveScanWrite(const QImageWriter* this_ptr) {
  return this_ptr->progressiveScanWrite();
}

int qt_gui_c_QImageWriter_quality(const QImageWriter* this_ptr) {
  return this_ptr->quality();
}

void qt_gui_c_QImageWriter_setCompression(QImageWriter* this_ptr, int compression) {
  this_ptr->setCompression(compression);
}

void qt_gui_c_QImageWriter_setDescription(QImageWriter* this_ptr, const QString* description) {
  this_ptr->setDescription(*description);
}

void qt_gui_c_QImageWriter_setDevice(QImageWriter* this_ptr, QIODevice* device) {
  this_ptr->setDevice(device);
}

void qt_gui_c_QImageWriter_setFileName(QImageWriter* this_ptr, const QString* fileName) {
  this_ptr->setFileName(*fileName);
}

void qt_gui_c_QImageWriter_setFormat(QImageWriter* this_ptr, const QByteArray* format) {
  this_ptr->setFormat(*format);
}

void qt_gui_c_QImageWriter_setGamma(QImageWriter* this_ptr, float gamma) {
  this_ptr->setGamma(gamma);
}

void qt_gui_c_QImageWriter_setOptimizedWrite(QImageWriter* this_ptr, bool optimize) {
  this_ptr->setOptimizedWrite(optimize);
}

void qt_gui_c_QImageWriter_setProgressiveScanWrite(QImageWriter* this_ptr, bool progressive) {
  this_ptr->setProgressiveScanWrite(progressive);
}

void qt_gui_c_QImageWriter_setQuality(QImageWriter* this_ptr, int quality) {
  this_ptr->setQuality(quality);
}

void qt_gui_c_QImageWriter_setSubType(QImageWriter* this_ptr, const QByteArray* type) {
  this_ptr->setSubType(*type);
}

void qt_gui_c_QImageWriter_setText(QImageWriter* this_ptr, const QString* key, const QString* text) {
  this_ptr->setText(*key, *text);
}

void qt_gui_c_QImageWriter_subType_to_output(const QImageWriter* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->subType());
}

void qt_gui_c_QImageWriter_supportedImageFormats_to_output(QList< QByteArray >* output) {
  new(output) QList< QByteArray >(QImageWriter::supportedImageFormats());
}

void qt_gui_c_QImageWriter_supportedMimeTypes_to_output(QList< QByteArray >* output) {
  new(output) QList< QByteArray >(QImageWriter::supportedMimeTypes());
}

void qt_gui_c_QImageWriter_supportedSubTypes_to_output(const QImageWriter* this_ptr, QList< QByteArray >* output) {
  new(output) QList< QByteArray >(this_ptr->supportedSubTypes());
}

bool qt_gui_c_QImageWriter_supportsOption(const QImageWriter* this_ptr, const QImageIOHandler::ImageOption* option) {
  return this_ptr->supportsOption(*option);
}

void qt_gui_c_QImageWriter_trUtf8_to_output(const char* sourceText, const char* disambiguation, int n, QString* output) {
  new(output) QString(QImageWriter::trUtf8(sourceText, disambiguation, n));
}

void qt_gui_c_QImageWriter_tr_to_output(const char* sourceText, const char* disambiguation, int n, QString* output) {
  new(output) QString(QImageWriter::tr(sourceText, disambiguation, n));
}

bool qt_gui_c_QImageWriter_write(QImageWriter* this_ptr, const QImage* image) {
  return this_ptr->write(*image);
}

