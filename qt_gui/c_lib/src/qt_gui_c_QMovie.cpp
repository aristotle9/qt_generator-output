#include "qt_gui_c_QMovie.h"

QMovie* qt_gui_c_QMovie_G_static_cast_QMovie_ptr(QObject* ptr) {
  return static_cast<QMovie*>(ptr);
}

QObject* qt_gui_c_QMovie_G_static_cast_QObject_ptr(QMovie* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_gui_c_QMovie_backgroundColor_to_output(const QMovie* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->backgroundColor());
}

QMovie::CacheMode qt_gui_c_QMovie_cacheMode(const QMovie* this_ptr) {
  return this_ptr->cacheMode();
}

int qt_gui_c_QMovie_currentFrameNumber(const QMovie* this_ptr) {
  return this_ptr->currentFrameNumber();
}

QImage* qt_gui_c_QMovie_currentImage_as_ptr(const QMovie* this_ptr) {
  return new QImage(this_ptr->currentImage());
}

QPixmap* qt_gui_c_QMovie_currentPixmap_as_ptr(const QMovie* this_ptr) {
  return new QPixmap(this_ptr->currentPixmap());
}

void qt_gui_c_QMovie_delete(QMovie* this_ptr) {
  delete this_ptr;
}

QIODevice* qt_gui_c_QMovie_device(const QMovie* this_ptr) {
  return this_ptr->device();
}

void qt_gui_c_QMovie_fileName_to_output(const QMovie* this_ptr, QString* output) {
  new(output) QString(this_ptr->fileName());
}

void qt_gui_c_QMovie_format_to_output(const QMovie* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->format());
}

int qt_gui_c_QMovie_frameCount(const QMovie* this_ptr) {
  return this_ptr->frameCount();
}

void qt_gui_c_QMovie_frameRect_to_output(const QMovie* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->frameRect());
}

bool qt_gui_c_QMovie_isValid(const QMovie* this_ptr) {
  return this_ptr->isValid();
}

bool qt_gui_c_QMovie_jumpToFrame(QMovie* this_ptr, int frameNumber) {
  return this_ptr->jumpToFrame(frameNumber);
}

bool qt_gui_c_QMovie_jumpToNextFrame(QMovie* this_ptr) {
  return this_ptr->jumpToNextFrame();
}

int qt_gui_c_QMovie_loopCount(const QMovie* this_ptr) {
  return this_ptr->loopCount();
}

const QMetaObject* qt_gui_c_QMovie_metaObject(const QMovie* this_ptr) {
  return this_ptr->metaObject();
}

QMovie* qt_gui_c_QMovie_new_device(QIODevice* device) {
  return new QMovie(device);
}

QMovie* qt_gui_c_QMovie_new_device_format(QIODevice* device, const QByteArray* format) {
  return new QMovie(device, *format);
}

QMovie* qt_gui_c_QMovie_new_device_format_parent(QIODevice* device, const QByteArray* format, QObject* parent) {
  return new QMovie(device, *format, parent);
}

QMovie* qt_gui_c_QMovie_new_fileName(const QString* fileName) {
  return new QMovie(*fileName);
}

QMovie* qt_gui_c_QMovie_new_fileName_format(const QString* fileName, const QByteArray* format) {
  return new QMovie(*fileName, *format);
}

QMovie* qt_gui_c_QMovie_new_fileName_format_parent(const QString* fileName, const QByteArray* format, QObject* parent) {
  return new QMovie(*fileName, *format, parent);
}

QMovie* qt_gui_c_QMovie_new_no_args() {
  return new QMovie();
}

QMovie* qt_gui_c_QMovie_new_parent(QObject* parent) {
  return new QMovie(parent);
}

int qt_gui_c_QMovie_nextFrameDelay(const QMovie* this_ptr) {
  return this_ptr->nextFrameDelay();
}

int qt_gui_c_QMovie_qt_metacall(QMovie* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QMovie_qt_metacast(QMovie* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QMovie_scaledSize_to_output(QMovie* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->scaledSize());
}

void qt_gui_c_QMovie_setBackgroundColor(QMovie* this_ptr, const QColor* color) {
  this_ptr->setBackgroundColor(*color);
}

void qt_gui_c_QMovie_setCacheMode(QMovie* this_ptr, QMovie::CacheMode mode) {
  this_ptr->setCacheMode(mode);
}

void qt_gui_c_QMovie_setDevice(QMovie* this_ptr, QIODevice* device) {
  this_ptr->setDevice(device);
}

void qt_gui_c_QMovie_setFileName(QMovie* this_ptr, const QString* fileName) {
  this_ptr->setFileName(*fileName);
}

void qt_gui_c_QMovie_setFormat(QMovie* this_ptr, const QByteArray* format) {
  this_ptr->setFormat(*format);
}

void qt_gui_c_QMovie_setPaused(QMovie* this_ptr, bool paused) {
  this_ptr->setPaused(paused);
}

void qt_gui_c_QMovie_setScaledSize(QMovie* this_ptr, const QSize* size) {
  this_ptr->setScaledSize(*size);
}

void qt_gui_c_QMovie_setSpeed(QMovie* this_ptr, int percentSpeed) {
  this_ptr->setSpeed(percentSpeed);
}

int qt_gui_c_QMovie_speed(const QMovie* this_ptr) {
  return this_ptr->speed();
}

void qt_gui_c_QMovie_start(QMovie* this_ptr) {
  this_ptr->start();
}

QMovie::MovieState qt_gui_c_QMovie_state(const QMovie* this_ptr) {
  return this_ptr->state();
}

void qt_gui_c_QMovie_stop(QMovie* this_ptr) {
  this_ptr->stop();
}

void qt_gui_c_QMovie_supportedFormats_to_output(QList< QByteArray >* output) {
  new(output) QList< QByteArray >(QMovie::supportedFormats());
}

void qt_gui_c_QMovie_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMovie::trUtf8(s, c, n));
}

void qt_gui_c_QMovie_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMovie::tr(s, c, n));
}

