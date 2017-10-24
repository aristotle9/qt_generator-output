#include "qt_gui_c_QOpenGLPaintDevice.h"

QOpenGLPaintDevice* qt_gui_c_QOpenGLPaintDevice_G_dynamic_cast_QOpenGLPaintDevice_ptr(QPaintDevice* ptr) {
  return dynamic_cast<QOpenGLPaintDevice*>(ptr);
}

QOpenGLPaintDevice* qt_gui_c_QOpenGLPaintDevice_G_static_cast_QOpenGLPaintDevice_ptr(QPaintDevice* ptr) {
  return static_cast<QOpenGLPaintDevice*>(ptr);
}

QPaintDevice* qt_gui_c_QOpenGLPaintDevice_G_static_cast_QPaintDevice_ptr(QOpenGLPaintDevice* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QOpenGLContext* qt_gui_c_QOpenGLPaintDevice_context(const QOpenGLPaintDevice* this_ptr) {
  return this_ptr->context();
}

void qt_gui_c_QOpenGLPaintDevice_delete(QOpenGLPaintDevice* this_ptr) {
  delete this_ptr;
}

int qt_gui_c_QOpenGLPaintDevice_devType(const QOpenGLPaintDevice* this_ptr) {
  return this_ptr->devType();
}

double qt_gui_c_QOpenGLPaintDevice_dotsPerMeterX(const QOpenGLPaintDevice* this_ptr) {
  return this_ptr->dotsPerMeterX();
}

double qt_gui_c_QOpenGLPaintDevice_dotsPerMeterY(const QOpenGLPaintDevice* this_ptr) {
  return this_ptr->dotsPerMeterY();
}

void qt_gui_c_QOpenGLPaintDevice_ensureActiveTarget(QOpenGLPaintDevice* this_ptr) {
  this_ptr->ensureActiveTarget();
}

QOpenGLPaintDevice* qt_gui_c_QOpenGLPaintDevice_new_no_args() {
  return new QOpenGLPaintDevice();
}

QOpenGLPaintDevice* qt_gui_c_QOpenGLPaintDevice_new_size(const QSize* size) {
  return new QOpenGLPaintDevice(*size);
}

QOpenGLPaintDevice* qt_gui_c_QOpenGLPaintDevice_new_width_height(int width, int height) {
  return new QOpenGLPaintDevice(width, height);
}

QPaintEngine* qt_gui_c_QOpenGLPaintDevice_paintEngine(const QOpenGLPaintDevice* this_ptr) {
  return this_ptr->paintEngine();
}

bool qt_gui_c_QOpenGLPaintDevice_paintFlipped(const QOpenGLPaintDevice* this_ptr) {
  return this_ptr->paintFlipped();
}

void qt_gui_c_QOpenGLPaintDevice_setDevicePixelRatio(QOpenGLPaintDevice* this_ptr, double devicePixelRatio) {
  this_ptr->setDevicePixelRatio(devicePixelRatio);
}

void qt_gui_c_QOpenGLPaintDevice_setDotsPerMeterX(QOpenGLPaintDevice* this_ptr, double arg1) {
  this_ptr->setDotsPerMeterX(arg1);
}

void qt_gui_c_QOpenGLPaintDevice_setDotsPerMeterY(QOpenGLPaintDevice* this_ptr, double arg1) {
  this_ptr->setDotsPerMeterY(arg1);
}

void qt_gui_c_QOpenGLPaintDevice_setPaintFlipped(QOpenGLPaintDevice* this_ptr, bool flipped) {
  this_ptr->setPaintFlipped(flipped);
}

void qt_gui_c_QOpenGLPaintDevice_setSize(QOpenGLPaintDevice* this_ptr, const QSize* size) {
  this_ptr->setSize(*size);
}

void qt_gui_c_QOpenGLPaintDevice_size_to_output(const QOpenGLPaintDevice* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->size());
}

