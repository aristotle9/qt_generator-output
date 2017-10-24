#include "qt_widgets_c_QGraphicsScale.h"

QGraphicsScale* qt_widgets_c_QGraphicsScale_G_dynamic_cast_QGraphicsScale_ptr(QGraphicsTransform* ptr) {
  return dynamic_cast<QGraphicsScale*>(ptr);
}

QGraphicsScale* qt_widgets_c_QGraphicsScale_G_static_cast_QGraphicsScale_ptr_QGraphicsTransform(QGraphicsTransform* ptr) {
  return static_cast<QGraphicsScale*>(ptr);
}

QGraphicsScale* qt_widgets_c_QGraphicsScale_G_static_cast_QGraphicsScale_ptr_QObject(QObject* ptr) {
  return static_cast<QGraphicsScale*>(ptr);
}

QGraphicsTransform* qt_widgets_c_QGraphicsScale_G_static_cast_QGraphicsTransform_ptr(QGraphicsScale* ptr) {
  return static_cast<QGraphicsTransform*>(ptr);
}

QObject* qt_widgets_c_QGraphicsScale_G_static_cast_QObject_ptr(QGraphicsScale* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QGraphicsScale_applyTo(const QGraphicsScale* this_ptr, QMatrix4x4* matrix) {
  this_ptr->applyTo(matrix);
}

void qt_widgets_c_QGraphicsScale_delete(QGraphicsScale* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QGraphicsScale_metaObject(const QGraphicsScale* this_ptr) {
  return this_ptr->metaObject();
}

QGraphicsScale* qt_widgets_c_QGraphicsScale_new_no_args() {
  return new QGraphicsScale();
}

QGraphicsScale* qt_widgets_c_QGraphicsScale_new_parent(QObject* parent) {
  return new QGraphicsScale(parent);
}

QVector3D* qt_widgets_c_QGraphicsScale_origin_as_ptr(const QGraphicsScale* this_ptr) {
  return new QVector3D(this_ptr->origin());
}

int qt_widgets_c_QGraphicsScale_qt_metacall(QGraphicsScale* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGraphicsScale_qt_metacast(QGraphicsScale* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGraphicsScale_setOrigin(QGraphicsScale* this_ptr, const QVector3D* point) {
  this_ptr->setOrigin(*point);
}

void qt_widgets_c_QGraphicsScale_setXScale(QGraphicsScale* this_ptr, double arg1) {
  this_ptr->setXScale(arg1);
}

void qt_widgets_c_QGraphicsScale_setYScale(QGraphicsScale* this_ptr, double arg1) {
  this_ptr->setYScale(arg1);
}

void qt_widgets_c_QGraphicsScale_setZScale(QGraphicsScale* this_ptr, double arg1) {
  this_ptr->setZScale(arg1);
}

void qt_widgets_c_QGraphicsScale_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsScale::trUtf8(s, c, n));
}

void qt_widgets_c_QGraphicsScale_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsScale::tr(s, c, n));
}

double qt_widgets_c_QGraphicsScale_xScale(const QGraphicsScale* this_ptr) {
  return this_ptr->xScale();
}

double qt_widgets_c_QGraphicsScale_yScale(const QGraphicsScale* this_ptr) {
  return this_ptr->yScale();
}

double qt_widgets_c_QGraphicsScale_zScale(const QGraphicsScale* this_ptr) {
  return this_ptr->zScale();
}

