#include "qt_widgets_c_QOpenGLWidget.h"

QOpenGLWidget* qt_widgets_c_QOpenGLWidget_G_dynamic_cast_QOpenGLWidget_ptr(QWidget* ptr) {
  return dynamic_cast<QOpenGLWidget*>(ptr);
}

QObject* qt_widgets_c_QOpenGLWidget_G_static_cast_QObject_ptr(QOpenGLWidget* ptr) {
  return static_cast<QObject*>(ptr);
}

QOpenGLWidget* qt_widgets_c_QOpenGLWidget_G_static_cast_QOpenGLWidget_ptr_QObject(QObject* ptr) {
  return static_cast<QOpenGLWidget*>(ptr);
}

QOpenGLWidget* qt_widgets_c_QOpenGLWidget_G_static_cast_QOpenGLWidget_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QOpenGLWidget*>(ptr);
}

QOpenGLWidget* qt_widgets_c_QOpenGLWidget_G_static_cast_QOpenGLWidget_ptr_QWidget(QWidget* ptr) {
  return static_cast<QOpenGLWidget*>(ptr);
}

QPaintDevice* qt_widgets_c_QOpenGLWidget_G_static_cast_QPaintDevice_ptr(QOpenGLWidget* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QOpenGLWidget_G_static_cast_QWidget_ptr(QOpenGLWidget* ptr) {
  return static_cast<QWidget*>(ptr);
}

QOpenGLContext* qt_widgets_c_QOpenGLWidget_context(const QOpenGLWidget* this_ptr) {
  return this_ptr->context();
}

GLuint qt_widgets_c_QOpenGLWidget_defaultFramebufferObject(const QOpenGLWidget* this_ptr) {
  return this_ptr->defaultFramebufferObject();
}

void qt_widgets_c_QOpenGLWidget_delete(QOpenGLWidget* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QOpenGLWidget_doneCurrent(QOpenGLWidget* this_ptr) {
  this_ptr->doneCurrent();
}

void qt_widgets_c_QOpenGLWidget_format_to_output(const QOpenGLWidget* this_ptr, QSurfaceFormat* output) {
  new(output) QSurfaceFormat(this_ptr->format());
}

QImage* qt_widgets_c_QOpenGLWidget_grabFramebuffer_as_ptr(QOpenGLWidget* this_ptr) {
  return new QImage(this_ptr->grabFramebuffer());
}

bool qt_widgets_c_QOpenGLWidget_isValid(const QOpenGLWidget* this_ptr) {
  return this_ptr->isValid();
}

void qt_widgets_c_QOpenGLWidget_makeCurrent(QOpenGLWidget* this_ptr) {
  this_ptr->makeCurrent();
}

const QMetaObject* qt_widgets_c_QOpenGLWidget_metaObject(const QOpenGLWidget* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QOpenGLWidget_qt_metacall(QOpenGLWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QOpenGLWidget_qt_metacast(QOpenGLWidget* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QOpenGLWidget_setFormat(QOpenGLWidget* this_ptr, const QSurfaceFormat* format) {
  this_ptr->setFormat(*format);
}

void qt_widgets_c_QOpenGLWidget_setUpdateBehavior(QOpenGLWidget* this_ptr, QOpenGLWidget::UpdateBehavior updateBehavior) {
  this_ptr->setUpdateBehavior(updateBehavior);
}

void qt_widgets_c_QOpenGLWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLWidget::trUtf8(s, c, n));
}

void qt_widgets_c_QOpenGLWidget_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLWidget::tr(s, c, n));
}

QOpenGLWidget::UpdateBehavior qt_widgets_c_QOpenGLWidget_updateBehavior(const QOpenGLWidget* this_ptr) {
  return this_ptr->updateBehavior();
}

