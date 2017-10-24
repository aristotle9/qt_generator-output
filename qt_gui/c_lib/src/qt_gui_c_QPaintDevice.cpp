#include "qt_gui_c_QPaintDevice.h"

int qt_gui_c_QPaintDevice_colorCount(const QPaintDevice* this_ptr) {
  return this_ptr->colorCount();
}

void qt_gui_c_QPaintDevice_delete(QPaintDevice* this_ptr) {
  delete this_ptr;
}

int qt_gui_c_QPaintDevice_depth(const QPaintDevice* this_ptr) {
  return this_ptr->depth();
}

int qt_gui_c_QPaintDevice_devType(const QPaintDevice* this_ptr) {
  return this_ptr->devType();
}

int qt_gui_c_QPaintDevice_devicePixelRatio(const QPaintDevice* this_ptr) {
  return this_ptr->devicePixelRatio();
}

double qt_gui_c_QPaintDevice_devicePixelRatioF(const QPaintDevice* this_ptr) {
  return this_ptr->devicePixelRatioF();
}

double qt_gui_c_QPaintDevice_devicePixelRatioFScale() {
  return QPaintDevice::devicePixelRatioFScale();
}

int qt_gui_c_QPaintDevice_height(const QPaintDevice* this_ptr) {
  return this_ptr->height();
}

int qt_gui_c_QPaintDevice_heightMM(const QPaintDevice* this_ptr) {
  return this_ptr->heightMM();
}

int qt_gui_c_QPaintDevice_logicalDpiX(const QPaintDevice* this_ptr) {
  return this_ptr->logicalDpiX();
}

int qt_gui_c_QPaintDevice_logicalDpiY(const QPaintDevice* this_ptr) {
  return this_ptr->logicalDpiY();
}

QPaintEngine* qt_gui_c_QPaintDevice_paintEngine(const QPaintDevice* this_ptr) {
  return this_ptr->paintEngine();
}

bool qt_gui_c_QPaintDevice_paintingActive(const QPaintDevice* this_ptr) {
  return this_ptr->paintingActive();
}

int qt_gui_c_QPaintDevice_physicalDpiX(const QPaintDevice* this_ptr) {
  return this_ptr->physicalDpiX();
}

int qt_gui_c_QPaintDevice_physicalDpiY(const QPaintDevice* this_ptr) {
  return this_ptr->physicalDpiY();
}

int qt_gui_c_QPaintDevice_width(const QPaintDevice* this_ptr) {
  return this_ptr->width();
}

int qt_gui_c_QPaintDevice_widthMM(const QPaintDevice* this_ptr) {
  return this_ptr->widthMM();
}

