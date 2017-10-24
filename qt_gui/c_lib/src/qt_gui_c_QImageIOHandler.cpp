#include "qt_gui_c_QImageIOHandler.h"

bool qt_gui_c_QImageIOHandler_canRead(const QImageIOHandler* this_ptr) {
  return this_ptr->canRead();
}

int qt_gui_c_QImageIOHandler_currentImageNumber(const QImageIOHandler* this_ptr) {
  return this_ptr->currentImageNumber();
}

void qt_gui_c_QImageIOHandler_currentImageRect_to_output(const QImageIOHandler* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->currentImageRect());
}

void qt_gui_c_QImageIOHandler_delete(QImageIOHandler* this_ptr) {
  delete this_ptr;
}

QIODevice* qt_gui_c_QImageIOHandler_device(const QImageIOHandler* this_ptr) {
  return this_ptr->device();
}

void qt_gui_c_QImageIOHandler_format_to_output(const QImageIOHandler* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->format());
}

int qt_gui_c_QImageIOHandler_imageCount(const QImageIOHandler* this_ptr) {
  return this_ptr->imageCount();
}

bool qt_gui_c_QImageIOHandler_jumpToImage(QImageIOHandler* this_ptr, int imageNumber) {
  return this_ptr->jumpToImage(imageNumber);
}

bool qt_gui_c_QImageIOHandler_jumpToNextImage(QImageIOHandler* this_ptr) {
  return this_ptr->jumpToNextImage();
}

int qt_gui_c_QImageIOHandler_loopCount(const QImageIOHandler* this_ptr) {
  return this_ptr->loopCount();
}

void qt_gui_c_QImageIOHandler_name_to_output(const QImageIOHandler* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->name());
}

int qt_gui_c_QImageIOHandler_nextImageDelay(const QImageIOHandler* this_ptr) {
  return this_ptr->nextImageDelay();
}

void qt_gui_c_QImageIOHandler_option_to_output(const QImageIOHandler* this_ptr, QImageIOHandler::ImageOption option, QVariant* output) {
  new(output) QVariant(this_ptr->option(option));
}

bool qt_gui_c_QImageIOHandler_read(QImageIOHandler* this_ptr, QImage* image) {
  return this_ptr->read(image);
}

void qt_gui_c_QImageIOHandler_setDevice(QImageIOHandler* this_ptr, QIODevice* device) {
  this_ptr->setDevice(device);
}

void qt_gui_c_QImageIOHandler_setFormat(QImageIOHandler* this_ptr, const QByteArray* format) {
  this_ptr->setFormat(*format);
}

void qt_gui_c_QImageIOHandler_setFormat_const(const QImageIOHandler* this_ptr, const QByteArray* format) {
  this_ptr->setFormat(*format);
}

void qt_gui_c_QImageIOHandler_setOption(QImageIOHandler* this_ptr, QImageIOHandler::ImageOption option, const QVariant* value) {
  this_ptr->setOption(option, *value);
}

bool qt_gui_c_QImageIOHandler_supportsOption(const QImageIOHandler* this_ptr, QImageIOHandler::ImageOption option) {
  return this_ptr->supportsOption(option);
}

bool qt_gui_c_QImageIOHandler_write(QImageIOHandler* this_ptr, const QImage* image) {
  return this_ptr->write(*image);
}

