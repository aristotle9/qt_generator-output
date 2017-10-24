#include "qt_gui_c_QTouchDevice.h"

void qt_gui_c_QTouchDevice_G_operator_shl_to_output(const QDebug* arg1, const QTouchDevice* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, arg2));
}

unsigned int qt_gui_c_QTouchDevice_capabilities(const QTouchDevice* this_ptr) {
  return uint(this_ptr->capabilities());
}

void qt_gui_c_QTouchDevice_delete(QTouchDevice* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QTouchDevice_devices_to_output(QList< const QTouchDevice* >* output) {
  new(output) QList< const QTouchDevice* >(QTouchDevice::devices());
}

int qt_gui_c_QTouchDevice_maximumTouchPoints(const QTouchDevice* this_ptr) {
  return this_ptr->maximumTouchPoints();
}

void qt_gui_c_QTouchDevice_name_to_output(const QTouchDevice* this_ptr, QString* output) {
  new(output) QString(this_ptr->name());
}

QTouchDevice* qt_gui_c_QTouchDevice_new() {
  return new QTouchDevice();
}

void qt_gui_c_QTouchDevice_setCapabilities(QTouchDevice* this_ptr, unsigned int caps) {
  this_ptr->setCapabilities(QFlags< QTouchDevice::CapabilityFlag >(caps));
}

void qt_gui_c_QTouchDevice_setMaximumTouchPoints(QTouchDevice* this_ptr, int max) {
  this_ptr->setMaximumTouchPoints(max);
}

void qt_gui_c_QTouchDevice_setName(QTouchDevice* this_ptr, const QString* name) {
  this_ptr->setName(*name);
}

void qt_gui_c_QTouchDevice_setType(QTouchDevice* this_ptr, QTouchDevice::DeviceType devType) {
  this_ptr->setType(devType);
}

QTouchDevice::DeviceType qt_gui_c_QTouchDevice_type(const QTouchDevice* this_ptr) {
  return this_ptr->type();
}

