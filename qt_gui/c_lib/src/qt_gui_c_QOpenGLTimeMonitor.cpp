#include "qt_gui_c_QOpenGLTimeMonitor.h"

QObject* qt_gui_c_QOpenGLTimeMonitor_G_static_cast_QObject_ptr(QOpenGLTimeMonitor* ptr) {
  return static_cast<QObject*>(ptr);
}

QOpenGLTimeMonitor* qt_gui_c_QOpenGLTimeMonitor_G_static_cast_QOpenGLTimeMonitor_ptr(QObject* ptr) {
  return static_cast<QOpenGLTimeMonitor*>(ptr);
}

bool qt_gui_c_QOpenGLTimeMonitor_create(QOpenGLTimeMonitor* this_ptr) {
  return this_ptr->create();
}

void qt_gui_c_QOpenGLTimeMonitor_delete(QOpenGLTimeMonitor* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QOpenGLTimeMonitor_destroy(QOpenGLTimeMonitor* this_ptr) {
  this_ptr->destroy();
}

bool qt_gui_c_QOpenGLTimeMonitor_isCreated(const QOpenGLTimeMonitor* this_ptr) {
  return this_ptr->isCreated();
}

bool qt_gui_c_QOpenGLTimeMonitor_isResultAvailable(const QOpenGLTimeMonitor* this_ptr) {
  return this_ptr->isResultAvailable();
}

const QMetaObject* qt_gui_c_QOpenGLTimeMonitor_metaObject(const QOpenGLTimeMonitor* this_ptr) {
  return this_ptr->metaObject();
}

QOpenGLTimeMonitor* qt_gui_c_QOpenGLTimeMonitor_new_no_args() {
  return new QOpenGLTimeMonitor();
}

QOpenGLTimeMonitor* qt_gui_c_QOpenGLTimeMonitor_new_parent(QObject* parent) {
  return new QOpenGLTimeMonitor(parent);
}

void qt_gui_c_QOpenGLTimeMonitor_objectIds_to_output(const QOpenGLTimeMonitor* this_ptr, QVector< GLuint >* output) {
  new(output) QVector< GLuint >(this_ptr->objectIds());
}

int qt_gui_c_QOpenGLTimeMonitor_qt_metacall(QOpenGLTimeMonitor* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QOpenGLTimeMonitor_qt_metacast(QOpenGLTimeMonitor* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

int qt_gui_c_QOpenGLTimeMonitor_recordSample(QOpenGLTimeMonitor* this_ptr) {
  return this_ptr->recordSample();
}

void qt_gui_c_QOpenGLTimeMonitor_reset(QOpenGLTimeMonitor* this_ptr) {
  this_ptr->reset();
}

int qt_gui_c_QOpenGLTimeMonitor_sampleCount(const QOpenGLTimeMonitor* this_ptr) {
  return this_ptr->sampleCount();
}

void qt_gui_c_QOpenGLTimeMonitor_setSampleCount(QOpenGLTimeMonitor* this_ptr, int sampleCount) {
  this_ptr->setSampleCount(sampleCount);
}

void qt_gui_c_QOpenGLTimeMonitor_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLTimeMonitor::trUtf8(s, c, n));
}

void qt_gui_c_QOpenGLTimeMonitor_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLTimeMonitor::tr(s, c, n));
}

void qt_gui_c_QOpenGLTimeMonitor_waitForIntervals_to_output(const QOpenGLTimeMonitor* this_ptr, QVector< GLuint64 >* output) {
  new(output) QVector< GLuint64 >(this_ptr->waitForIntervals());
}

void qt_gui_c_QOpenGLTimeMonitor_waitForSamples_to_output(const QOpenGLTimeMonitor* this_ptr, QVector< GLuint64 >* output) {
  new(output) QVector< GLuint64 >(this_ptr->waitForSamples());
}

