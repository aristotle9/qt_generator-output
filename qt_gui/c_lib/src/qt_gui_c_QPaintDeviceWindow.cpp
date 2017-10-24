#include "qt_gui_c_QPaintDeviceWindow.h"

QPaintDeviceWindow* qt_gui_c_QPaintDeviceWindow_G_dynamic_cast_QPaintDeviceWindow_ptr_QPaintDevice(QPaintDevice* ptr) {
  return dynamic_cast<QPaintDeviceWindow*>(ptr);
}

QPaintDeviceWindow* qt_gui_c_QPaintDeviceWindow_G_dynamic_cast_QPaintDeviceWindow_ptr_QSurface(QSurface* ptr) {
  return dynamic_cast<QPaintDeviceWindow*>(ptr);
}

QPaintDeviceWindow* qt_gui_c_QPaintDeviceWindow_G_dynamic_cast_QPaintDeviceWindow_ptr_QWindow(QWindow* ptr) {
  return dynamic_cast<QPaintDeviceWindow*>(ptr);
}

QObject* qt_gui_c_QPaintDeviceWindow_G_static_cast_QObject_ptr(QPaintDeviceWindow* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDeviceWindow* qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDeviceWindow_ptr_QObject(QObject* ptr) {
  return static_cast<QPaintDeviceWindow*>(ptr);
}

QPaintDeviceWindow* qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDeviceWindow_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QPaintDeviceWindow*>(ptr);
}

QPaintDeviceWindow* qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDeviceWindow_ptr_QSurface(QSurface* ptr) {
  return static_cast<QPaintDeviceWindow*>(ptr);
}

QPaintDeviceWindow* qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDeviceWindow_ptr_QWindow(QWindow* ptr) {
  return static_cast<QPaintDeviceWindow*>(ptr);
}

QPaintDevice* qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDevice_ptr(QPaintDeviceWindow* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QSurface* qt_gui_c_QPaintDeviceWindow_G_static_cast_QSurface_ptr(QPaintDeviceWindow* ptr) {
  return static_cast<QSurface*>(ptr);
}

QWindow* qt_gui_c_QPaintDeviceWindow_G_static_cast_QWindow_ptr(QPaintDeviceWindow* ptr) {
  return static_cast<QWindow*>(ptr);
}

void qt_gui_c_QPaintDeviceWindow_delete(QPaintDeviceWindow* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_gui_c_QPaintDeviceWindow_metaObject(const QPaintDeviceWindow* this_ptr) {
  return this_ptr->metaObject();
}

int qt_gui_c_QPaintDeviceWindow_qt_metacall(QPaintDeviceWindow* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QPaintDeviceWindow_qt_metacast(QPaintDeviceWindow* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QPaintDeviceWindow_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPaintDeviceWindow::trUtf8(s, c, n));
}

void qt_gui_c_QPaintDeviceWindow_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPaintDeviceWindow::tr(s, c, n));
}

void qt_gui_c_QPaintDeviceWindow_update_no_args(QPaintDeviceWindow* this_ptr) {
  this_ptr->update();
}

void qt_gui_c_QPaintDeviceWindow_update_rect(QPaintDeviceWindow* this_ptr, const QRect* rect) {
  this_ptr->update(*rect);
}

void qt_gui_c_QPaintDeviceWindow_update_region(QPaintDeviceWindow* this_ptr, const QRegion* region) {
  this_ptr->update(*region);
}

