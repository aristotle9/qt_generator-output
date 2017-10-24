#include "qt_widgets_c_QPanGesture.h"

QPanGesture* qt_widgets_c_QPanGesture_G_dynamic_cast_QPanGesture_ptr(QGesture* ptr) {
  return dynamic_cast<QPanGesture*>(ptr);
}

QGesture* qt_widgets_c_QPanGesture_G_static_cast_QGesture_ptr(QPanGesture* ptr) {
  return static_cast<QGesture*>(ptr);
}

QObject* qt_widgets_c_QPanGesture_G_static_cast_QObject_ptr(QPanGesture* ptr) {
  return static_cast<QObject*>(ptr);
}

QPanGesture* qt_widgets_c_QPanGesture_G_static_cast_QPanGesture_ptr_QGesture(QGesture* ptr) {
  return static_cast<QPanGesture*>(ptr);
}

QPanGesture* qt_widgets_c_QPanGesture_G_static_cast_QPanGesture_ptr_QObject(QObject* ptr) {
  return static_cast<QPanGesture*>(ptr);
}

double qt_widgets_c_QPanGesture_acceleration(const QPanGesture* this_ptr) {
  return this_ptr->acceleration();
}

void qt_widgets_c_QPanGesture_delete(QPanGesture* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QPanGesture_delta_to_output(const QPanGesture* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->delta());
}

void qt_widgets_c_QPanGesture_lastOffset_to_output(const QPanGesture* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->lastOffset());
}

const QMetaObject* qt_widgets_c_QPanGesture_metaObject(const QPanGesture* this_ptr) {
  return this_ptr->metaObject();
}

QPanGesture* qt_widgets_c_QPanGesture_new_no_args() {
  return new QPanGesture();
}

QPanGesture* qt_widgets_c_QPanGesture_new_parent(QObject* parent) {
  return new QPanGesture(parent);
}

void qt_widgets_c_QPanGesture_offset_to_output(const QPanGesture* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->offset());
}

int qt_widgets_c_QPanGesture_qt_metacall(QPanGesture* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QPanGesture_qt_metacast(QPanGesture* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QPanGesture_setAcceleration(QPanGesture* this_ptr, double value) {
  this_ptr->setAcceleration(value);
}

void qt_widgets_c_QPanGesture_setLastOffset(QPanGesture* this_ptr, const QPointF* value) {
  this_ptr->setLastOffset(*value);
}

void qt_widgets_c_QPanGesture_setOffset(QPanGesture* this_ptr, const QPointF* value) {
  this_ptr->setOffset(*value);
}

void qt_widgets_c_QPanGesture_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPanGesture::trUtf8(s, c, n));
}

void qt_widgets_c_QPanGesture_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPanGesture::tr(s, c, n));
}

