#include "qt_widgets_c_QGraphicsRotation.h"

QGraphicsRotation* qt_widgets_c_QGraphicsRotation_G_dynamic_cast_QGraphicsRotation_ptr(QGraphicsTransform* ptr) {
  return dynamic_cast<QGraphicsRotation*>(ptr);
}

QGraphicsRotation* qt_widgets_c_QGraphicsRotation_G_static_cast_QGraphicsRotation_ptr_QGraphicsTransform(QGraphicsTransform* ptr) {
  return static_cast<QGraphicsRotation*>(ptr);
}

QGraphicsRotation* qt_widgets_c_QGraphicsRotation_G_static_cast_QGraphicsRotation_ptr_QObject(QObject* ptr) {
  return static_cast<QGraphicsRotation*>(ptr);
}

QGraphicsTransform* qt_widgets_c_QGraphicsRotation_G_static_cast_QGraphicsTransform_ptr(QGraphicsRotation* ptr) {
  return static_cast<QGraphicsTransform*>(ptr);
}

QObject* qt_widgets_c_QGraphicsRotation_G_static_cast_QObject_ptr(QGraphicsRotation* ptr) {
  return static_cast<QObject*>(ptr);
}

double qt_widgets_c_QGraphicsRotation_angle(const QGraphicsRotation* this_ptr) {
  return this_ptr->angle();
}

void qt_widgets_c_QGraphicsRotation_applyTo(const QGraphicsRotation* this_ptr, QMatrix4x4* matrix) {
  this_ptr->applyTo(matrix);
}

QVector3D* qt_widgets_c_QGraphicsRotation_axis_as_ptr(const QGraphicsRotation* this_ptr) {
  return new QVector3D(this_ptr->axis());
}

void qt_widgets_c_QGraphicsRotation_delete(QGraphicsRotation* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QGraphicsRotation_metaObject(const QGraphicsRotation* this_ptr) {
  return this_ptr->metaObject();
}

QGraphicsRotation* qt_widgets_c_QGraphicsRotation_new_no_args() {
  return new QGraphicsRotation();
}

QGraphicsRotation* qt_widgets_c_QGraphicsRotation_new_parent(QObject* parent) {
  return new QGraphicsRotation(parent);
}

QVector3D* qt_widgets_c_QGraphicsRotation_origin_as_ptr(const QGraphicsRotation* this_ptr) {
  return new QVector3D(this_ptr->origin());
}

int qt_widgets_c_QGraphicsRotation_qt_metacall(QGraphicsRotation* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGraphicsRotation_qt_metacast(QGraphicsRotation* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGraphicsRotation_setAngle(QGraphicsRotation* this_ptr, double arg1) {
  this_ptr->setAngle(arg1);
}

void qt_widgets_c_QGraphicsRotation_setAxis_QVector3D(QGraphicsRotation* this_ptr, const QVector3D* axis) {
  this_ptr->setAxis(*axis);
}

void qt_widgets_c_QGraphicsRotation_setAxis_Qt_Axis(QGraphicsRotation* this_ptr, const Qt::Axis* axis) {
  this_ptr->setAxis(*axis);
}

void qt_widgets_c_QGraphicsRotation_setOrigin(QGraphicsRotation* this_ptr, const QVector3D* point) {
  this_ptr->setOrigin(*point);
}

void qt_widgets_c_QGraphicsRotation_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsRotation::trUtf8(s, c, n));
}

void qt_widgets_c_QGraphicsRotation_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsRotation::tr(s, c, n));
}

