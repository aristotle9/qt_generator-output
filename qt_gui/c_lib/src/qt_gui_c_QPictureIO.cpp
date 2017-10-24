#include "qt_gui_c_QPictureIO.h"

void qt_gui_c_QPictureIO_constructor_fileName_format(const QString* fileName, const char* format, QPictureIO* output) {
  new(output) QPictureIO(*fileName, format);
}

void qt_gui_c_QPictureIO_constructor_ioDevice_format(QIODevice* ioDevice, const char* format, QPictureIO* output) {
  new(output) QPictureIO(ioDevice, format);
}

void qt_gui_c_QPictureIO_constructor_no_args(QPictureIO* output) {
  new(output) QPictureIO();
}

void qt_gui_c_QPictureIO_defineIOHandler(const char* format, const char* header, const char* flags, void (*read_picture)(QPictureIO*), void (*write_picture)(QPictureIO*)) {
  QPictureIO::defineIOHandler(format, header, flags, read_picture, write_picture);
}

void qt_gui_c_QPictureIO_description_to_output(const QPictureIO* this_ptr, QString* output) {
  new(output) QString(this_ptr->description());
}

void qt_gui_c_QPictureIO_destructor(QPictureIO* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QPictureIO_fileName_to_output(const QPictureIO* this_ptr, QString* output) {
  new(output) QString(this_ptr->fileName());
}

const char* qt_gui_c_QPictureIO_format(const QPictureIO* this_ptr) {
  return this_ptr->format();
}

float qt_gui_c_QPictureIO_gamma(const QPictureIO* this_ptr) {
  return this_ptr->gamma();
}

void qt_gui_c_QPictureIO_inputFormats_to_output(QList< QByteArray >* output) {
  new(output) QList< QByteArray >(QPictureIO::inputFormats());
}

QIODevice* qt_gui_c_QPictureIO_ioDevice(const QPictureIO* this_ptr) {
  return this_ptr->ioDevice();
}

void qt_gui_c_QPictureIO_outputFormats_to_output(QList< QByteArray >* output) {
  new(output) QList< QByteArray >(QPictureIO::outputFormats());
}

const char* qt_gui_c_QPictureIO_parameters(const QPictureIO* this_ptr) {
  return this_ptr->parameters();
}

const QPicture* qt_gui_c_QPictureIO_picture(const QPictureIO* this_ptr) {
  return &this_ptr->picture();
}

void qt_gui_c_QPictureIO_pictureFormat_to_output_arg1(QIODevice* arg1, QByteArray* output) {
  new(output) QByteArray(QPictureIO::pictureFormat(arg1));
}

void qt_gui_c_QPictureIO_pictureFormat_to_output_fileName(const QString* fileName, QByteArray* output) {
  new(output) QByteArray(QPictureIO::pictureFormat(*fileName));
}

int qt_gui_c_QPictureIO_quality(const QPictureIO* this_ptr) {
  return this_ptr->quality();
}

bool qt_gui_c_QPictureIO_read(QPictureIO* this_ptr) {
  return this_ptr->read();
}

void qt_gui_c_QPictureIO_setDescription(QPictureIO* this_ptr, const QString* arg1) {
  this_ptr->setDescription(*arg1);
}

void qt_gui_c_QPictureIO_setFileName(QPictureIO* this_ptr, const QString* arg1) {
  this_ptr->setFileName(*arg1);
}

void qt_gui_c_QPictureIO_setFormat(QPictureIO* this_ptr, const char* arg1) {
  this_ptr->setFormat(arg1);
}

void qt_gui_c_QPictureIO_setGamma(QPictureIO* this_ptr, float arg1) {
  this_ptr->setGamma(arg1);
}

void qt_gui_c_QPictureIO_setIODevice(QPictureIO* this_ptr, QIODevice* arg1) {
  this_ptr->setIODevice(arg1);
}

void qt_gui_c_QPictureIO_setParameters(QPictureIO* this_ptr, const char* arg1) {
  this_ptr->setParameters(arg1);
}

void qt_gui_c_QPictureIO_setPicture(QPictureIO* this_ptr, const QPicture* arg1) {
  this_ptr->setPicture(*arg1);
}

void qt_gui_c_QPictureIO_setQuality(QPictureIO* this_ptr, int arg1) {
  this_ptr->setQuality(arg1);
}

void qt_gui_c_QPictureIO_setStatus(QPictureIO* this_ptr, int arg1) {
  this_ptr->setStatus(arg1);
}

int qt_gui_c_QPictureIO_status(const QPictureIO* this_ptr) {
  return this_ptr->status();
}

bool qt_gui_c_QPictureIO_write(QPictureIO* this_ptr) {
  return this_ptr->write();
}

