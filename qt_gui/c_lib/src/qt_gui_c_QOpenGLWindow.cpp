#include "qt_gui_c_QOpenGLWindow.h"

QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_dynamic_cast_QOpenGLWindow_ptr_QPaintDevice(QPaintDevice* ptr) {
  return dynamic_cast<QOpenGLWindow*>(ptr);
}

QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_dynamic_cast_QOpenGLWindow_ptr_QPaintDeviceWindow(QPaintDeviceWindow* ptr) {
  return dynamic_cast<QOpenGLWindow*>(ptr);
}

QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_dynamic_cast_QOpenGLWindow_ptr_QSurface(QSurface* ptr) {
  return dynamic_cast<QOpenGLWindow*>(ptr);
}

QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_dynamic_cast_QOpenGLWindow_ptr_QWindow(QWindow* ptr) {
  return dynamic_cast<QOpenGLWindow*>(ptr);
}

QObject* qt_gui_c_QOpenGLWindow_G_static_cast_QObject_ptr(QOpenGLWindow* ptr) {
  return static_cast<QObject*>(ptr);
}

QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QObject(QObject* ptr) {
  return static_cast<QOpenGLWindow*>(ptr);
}

QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QOpenGLWindow*>(ptr);
}

QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QPaintDeviceWindow(QPaintDeviceWindow* ptr) {
  return static_cast<QOpenGLWindow*>(ptr);
}

QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QSurface(QSurface* ptr) {
  return static_cast<QOpenGLWindow*>(ptr);
}

QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QWindow(QWindow* ptr) {
  return static_cast<QOpenGLWindow*>(ptr);
}

QPaintDeviceWindow* qt_gui_c_QOpenGLWindow_G_static_cast_QPaintDeviceWindow_ptr(QOpenGLWindow* ptr) {
  return static_cast<QPaintDeviceWindow*>(ptr);
}

QPaintDevice* qt_gui_c_QOpenGLWindow_G_static_cast_QPaintDevice_ptr(QOpenGLWindow* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QSurface* qt_gui_c_QOpenGLWindow_G_static_cast_QSurface_ptr(QOpenGLWindow* ptr) {
  return static_cast<QSurface*>(ptr);
}

QWindow* qt_gui_c_QOpenGLWindow_G_static_cast_QWindow_ptr(QOpenGLWindow* ptr) {
  return static_cast<QWindow*>(ptr);
}

QOpenGLContext* qt_gui_c_QOpenGLWindow_context(const QOpenGLWindow* this_ptr) {
  return this_ptr->context();
}

GLuint qt_gui_c_QOpenGLWindow_defaultFramebufferObject(const QOpenGLWindow* this_ptr) {
  return this_ptr->defaultFramebufferObject();
}

void qt_gui_c_QOpenGLWindow_delete(QOpenGLWindow* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QOpenGLWindow_doneCurrent(QOpenGLWindow* this_ptr) {
  this_ptr->doneCurrent();
}

QImage* qt_gui_c_QOpenGLWindow_grabFramebuffer_as_ptr(QOpenGLWindow* this_ptr) {
  return new QImage(this_ptr->grabFramebuffer());
}

bool qt_gui_c_QOpenGLWindow_isValid(const QOpenGLWindow* this_ptr) {
  return this_ptr->isValid();
}

void qt_gui_c_QOpenGLWindow_makeCurrent(QOpenGLWindow* this_ptr) {
  this_ptr->makeCurrent();
}

const QMetaObject* qt_gui_c_QOpenGLWindow_metaObject(const QOpenGLWindow* this_ptr) {
  return this_ptr->metaObject();
}

QOpenGLWindow* qt_gui_c_QOpenGLWindow_new_no_args() {
  return new QOpenGLWindow();
}

QOpenGLWindow* qt_gui_c_QOpenGLWindow_new_shareContext(QOpenGLContext* shareContext) {
  return new QOpenGLWindow(shareContext);
}

QOpenGLWindow* qt_gui_c_QOpenGLWindow_new_shareContext_updateBehavior(QOpenGLContext* shareContext, QOpenGLWindow::UpdateBehavior updateBehavior) {
  return new QOpenGLWindow(shareContext, updateBehavior);
}

QOpenGLWindow* qt_gui_c_QOpenGLWindow_new_shareContext_updateBehavior_parent(QOpenGLContext* shareContext, QOpenGLWindow::UpdateBehavior updateBehavior, QWindow* parent) {
  return new QOpenGLWindow(shareContext, updateBehavior, parent);
}

QOpenGLWindow* qt_gui_c_QOpenGLWindow_new_updateBehavior(QOpenGLWindow::UpdateBehavior updateBehavior) {
  return new QOpenGLWindow(updateBehavior);
}

QOpenGLWindow* qt_gui_c_QOpenGLWindow_new_updateBehavior_parent(QOpenGLWindow::UpdateBehavior updateBehavior, QWindow* parent) {
  return new QOpenGLWindow(updateBehavior, parent);
}

int qt_gui_c_QOpenGLWindow_qt_metacall(QOpenGLWindow* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QOpenGLWindow_qt_metacast(QOpenGLWindow* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

QOpenGLContext* qt_gui_c_QOpenGLWindow_shareContext(const QOpenGLWindow* this_ptr) {
  return this_ptr->shareContext();
}

void qt_gui_c_QOpenGLWindow_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLWindow::trUtf8(s, c, n));
}

void qt_gui_c_QOpenGLWindow_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLWindow::tr(s, c, n));
}

QOpenGLWindow::UpdateBehavior qt_gui_c_QOpenGLWindow_updateBehavior(const QOpenGLWindow* this_ptr) {
  return this_ptr->updateBehavior();
}

