#include "qt_gui_c_QOpenGLVertexArrayObject.h"

void qt_gui_c_QOpenGLVertexArrayObject_Binder_constructor(QOpenGLVertexArrayObject* v, QOpenGLVertexArrayObject::Binder* output) {
  new(output) QOpenGLVertexArrayObject::Binder(v);
}

void qt_gui_c_QOpenGLVertexArrayObject_Binder_destructor(QOpenGLVertexArrayObject::Binder* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QOpenGLVertexArrayObject_Binder_rebind(QOpenGLVertexArrayObject::Binder* this_ptr) {
  this_ptr->rebind();
}

void qt_gui_c_QOpenGLVertexArrayObject_Binder_release(QOpenGLVertexArrayObject::Binder* this_ptr) {
  this_ptr->release();
}

QObject* qt_gui_c_QOpenGLVertexArrayObject_G_static_cast_QObject_ptr(QOpenGLVertexArrayObject* ptr) {
  return static_cast<QObject*>(ptr);
}

QOpenGLVertexArrayObject* qt_gui_c_QOpenGLVertexArrayObject_G_static_cast_QOpenGLVertexArrayObject_ptr(QObject* ptr) {
  return static_cast<QOpenGLVertexArrayObject*>(ptr);
}

void qt_gui_c_QOpenGLVertexArrayObject_bind(QOpenGLVertexArrayObject* this_ptr) {
  this_ptr->bind();
}

bool qt_gui_c_QOpenGLVertexArrayObject_create(QOpenGLVertexArrayObject* this_ptr) {
  return this_ptr->create();
}

void qt_gui_c_QOpenGLVertexArrayObject_delete(QOpenGLVertexArrayObject* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QOpenGLVertexArrayObject_destroy(QOpenGLVertexArrayObject* this_ptr) {
  this_ptr->destroy();
}

bool qt_gui_c_QOpenGLVertexArrayObject_isCreated(const QOpenGLVertexArrayObject* this_ptr) {
  return this_ptr->isCreated();
}

const QMetaObject* qt_gui_c_QOpenGLVertexArrayObject_metaObject(const QOpenGLVertexArrayObject* this_ptr) {
  return this_ptr->metaObject();
}

QOpenGLVertexArrayObject* qt_gui_c_QOpenGLVertexArrayObject_new_no_args() {
  return new QOpenGLVertexArrayObject();
}

QOpenGLVertexArrayObject* qt_gui_c_QOpenGLVertexArrayObject_new_parent(QObject* parent) {
  return new QOpenGLVertexArrayObject(parent);
}

GLuint qt_gui_c_QOpenGLVertexArrayObject_objectId(const QOpenGLVertexArrayObject* this_ptr) {
  return this_ptr->objectId();
}

int qt_gui_c_QOpenGLVertexArrayObject_qt_metacall(QOpenGLVertexArrayObject* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QOpenGLVertexArrayObject_qt_metacast(QOpenGLVertexArrayObject* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QOpenGLVertexArrayObject_release(QOpenGLVertexArrayObject* this_ptr) {
  this_ptr->release();
}

void qt_gui_c_QOpenGLVertexArrayObject_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLVertexArrayObject::trUtf8(s, c, n));
}

void qt_gui_c_QOpenGLVertexArrayObject_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLVertexArrayObject::tr(s, c, n));
}

