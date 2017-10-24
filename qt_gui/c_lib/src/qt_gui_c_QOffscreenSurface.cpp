#include "qt_gui_c_QOffscreenSurface.h"

QOffscreenSurface* qt_gui_c_QOffscreenSurface_G_dynamic_cast_QOffscreenSurface_ptr(QSurface* ptr) {
  return dynamic_cast<QOffscreenSurface*>(ptr);
}

QObject* qt_gui_c_QOffscreenSurface_G_static_cast_QObject_ptr(QOffscreenSurface* ptr) {
  return static_cast<QObject*>(ptr);
}

QOffscreenSurface* qt_gui_c_QOffscreenSurface_G_static_cast_QOffscreenSurface_ptr_QObject(QObject* ptr) {
  return static_cast<QOffscreenSurface*>(ptr);
}

QOffscreenSurface* qt_gui_c_QOffscreenSurface_G_static_cast_QOffscreenSurface_ptr_QSurface(QSurface* ptr) {
  return static_cast<QOffscreenSurface*>(ptr);
}

QSurface* qt_gui_c_QOffscreenSurface_G_static_cast_QSurface_ptr(QOffscreenSurface* ptr) {
  return static_cast<QSurface*>(ptr);
}

void qt_gui_c_QOffscreenSurface_create(QOffscreenSurface* this_ptr) {
  this_ptr->create();
}

void qt_gui_c_QOffscreenSurface_delete(QOffscreenSurface* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QOffscreenSurface_destroy(QOffscreenSurface* this_ptr) {
  this_ptr->destroy();
}

void qt_gui_c_QOffscreenSurface_format_to_output(const QOffscreenSurface* this_ptr, QSurfaceFormat* output) {
  new(output) QSurfaceFormat(this_ptr->format());
}

bool qt_gui_c_QOffscreenSurface_isValid(const QOffscreenSurface* this_ptr) {
  return this_ptr->isValid();
}

const QMetaObject* qt_gui_c_QOffscreenSurface_metaObject(const QOffscreenSurface* this_ptr) {
  return this_ptr->metaObject();
}

void* qt_gui_c_QOffscreenSurface_nativeHandle(const QOffscreenSurface* this_ptr) {
  return this_ptr->nativeHandle();
}

QOffscreenSurface* qt_gui_c_QOffscreenSurface_new_no_args() {
  return new QOffscreenSurface();
}

QOffscreenSurface* qt_gui_c_QOffscreenSurface_new_screen(QScreen* screen) {
  return new QOffscreenSurface(screen);
}

int qt_gui_c_QOffscreenSurface_qt_metacall(QOffscreenSurface* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QOffscreenSurface_qt_metacast(QOffscreenSurface* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QOffscreenSurface_requestedFormat_to_output(const QOffscreenSurface* this_ptr, QSurfaceFormat* output) {
  new(output) QSurfaceFormat(this_ptr->requestedFormat());
}

QScreen* qt_gui_c_QOffscreenSurface_screen(const QOffscreenSurface* this_ptr) {
  return this_ptr->screen();
}

void qt_gui_c_QOffscreenSurface_setFormat(QOffscreenSurface* this_ptr, const QSurfaceFormat* format) {
  this_ptr->setFormat(*format);
}

void qt_gui_c_QOffscreenSurface_setNativeHandle(QOffscreenSurface* this_ptr, void* handle) {
  this_ptr->setNativeHandle(handle);
}

void qt_gui_c_QOffscreenSurface_setScreen(QOffscreenSurface* this_ptr, QScreen* screen) {
  this_ptr->setScreen(screen);
}

void qt_gui_c_QOffscreenSurface_size_to_output(const QOffscreenSurface* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->size());
}

QSurface::SurfaceType qt_gui_c_QOffscreenSurface_surfaceType(const QOffscreenSurface* this_ptr) {
  return this_ptr->surfaceType();
}

void qt_gui_c_QOffscreenSurface_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOffscreenSurface::trUtf8(s, c, n));
}

void qt_gui_c_QOffscreenSurface_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOffscreenSurface::tr(s, c, n));
}

