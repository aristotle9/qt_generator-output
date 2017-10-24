#include "qt_widgets_c_QGraphicsTransform.h"

QGraphicsTransform* qt_widgets_c_QGraphicsTransform_G_static_cast_QGraphicsTransform_ptr(QObject* ptr) {
  return static_cast<QGraphicsTransform*>(ptr);
}

QObject* qt_widgets_c_QGraphicsTransform_G_static_cast_QObject_ptr(QGraphicsTransform* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QGraphicsTransform_applyTo(const QGraphicsTransform* this_ptr, QMatrix4x4* matrix) {
  this_ptr->applyTo(matrix);
}

void qt_widgets_c_QGraphicsTransform_delete(QGraphicsTransform* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QGraphicsTransform_metaObject(const QGraphicsTransform* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QGraphicsTransform_qt_metacall(QGraphicsTransform* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGraphicsTransform_qt_metacast(QGraphicsTransform* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGraphicsTransform_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsTransform::trUtf8(s, c, n));
}

void qt_widgets_c_QGraphicsTransform_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsTransform::tr(s, c, n));
}

