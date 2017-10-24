#include "qt_gui_c_QOpenGLTimerQuery.h"

QObject* qt_gui_c_QOpenGLTimerQuery_G_static_cast_QObject_ptr(QOpenGLTimerQuery* ptr) {
  return static_cast<QObject*>(ptr);
}

QOpenGLTimerQuery* qt_gui_c_QOpenGLTimerQuery_G_static_cast_QOpenGLTimerQuery_ptr(QObject* ptr) {
  return static_cast<QOpenGLTimerQuery*>(ptr);
}

void qt_gui_c_QOpenGLTimerQuery_begin(QOpenGLTimerQuery* this_ptr) {
  this_ptr->begin();
}

bool qt_gui_c_QOpenGLTimerQuery_create(QOpenGLTimerQuery* this_ptr) {
  return this_ptr->create();
}

void qt_gui_c_QOpenGLTimerQuery_delete(QOpenGLTimerQuery* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QOpenGLTimerQuery_destroy(QOpenGLTimerQuery* this_ptr) {
  this_ptr->destroy();
}

void qt_gui_c_QOpenGLTimerQuery_end(QOpenGLTimerQuery* this_ptr) {
  this_ptr->end();
}

bool qt_gui_c_QOpenGLTimerQuery_isCreated(const QOpenGLTimerQuery* this_ptr) {
  return this_ptr->isCreated();
}

bool qt_gui_c_QOpenGLTimerQuery_isResultAvailable(const QOpenGLTimerQuery* this_ptr) {
  return this_ptr->isResultAvailable();
}

const QMetaObject* qt_gui_c_QOpenGLTimerQuery_metaObject(const QOpenGLTimerQuery* this_ptr) {
  return this_ptr->metaObject();
}

QOpenGLTimerQuery* qt_gui_c_QOpenGLTimerQuery_new_no_args() {
  return new QOpenGLTimerQuery();
}

QOpenGLTimerQuery* qt_gui_c_QOpenGLTimerQuery_new_parent(QObject* parent) {
  return new QOpenGLTimerQuery(parent);
}

GLuint qt_gui_c_QOpenGLTimerQuery_objectId(const QOpenGLTimerQuery* this_ptr) {
  return this_ptr->objectId();
}

int qt_gui_c_QOpenGLTimerQuery_qt_metacall(QOpenGLTimerQuery* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QOpenGLTimerQuery_qt_metacast(QOpenGLTimerQuery* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QOpenGLTimerQuery_recordTimestamp(QOpenGLTimerQuery* this_ptr) {
  this_ptr->recordTimestamp();
}

void qt_gui_c_QOpenGLTimerQuery_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLTimerQuery::trUtf8(s, c, n));
}

void qt_gui_c_QOpenGLTimerQuery_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLTimerQuery::tr(s, c, n));
}

GLuint64 qt_gui_c_QOpenGLTimerQuery_waitForResult(const QOpenGLTimerQuery* this_ptr) {
  return this_ptr->waitForResult();
}

GLuint64 qt_gui_c_QOpenGLTimerQuery_waitForTimestamp(const QOpenGLTimerQuery* this_ptr) {
  return this_ptr->waitForTimestamp();
}

