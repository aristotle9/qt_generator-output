#include "qt_widgets_c_QTapGesture.h"

QTapGesture* qt_widgets_c_QTapGesture_G_dynamic_cast_QTapGesture_ptr(QGesture* ptr) {
  return dynamic_cast<QTapGesture*>(ptr);
}

QGesture* qt_widgets_c_QTapGesture_G_static_cast_QGesture_ptr(QTapGesture* ptr) {
  return static_cast<QGesture*>(ptr);
}

QObject* qt_widgets_c_QTapGesture_G_static_cast_QObject_ptr(QTapGesture* ptr) {
  return static_cast<QObject*>(ptr);
}

QTapGesture* qt_widgets_c_QTapGesture_G_static_cast_QTapGesture_ptr_QGesture(QGesture* ptr) {
  return static_cast<QTapGesture*>(ptr);
}

QTapGesture* qt_widgets_c_QTapGesture_G_static_cast_QTapGesture_ptr_QObject(QObject* ptr) {
  return static_cast<QTapGesture*>(ptr);
}

void qt_widgets_c_QTapGesture_delete(QTapGesture* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QTapGesture_metaObject(const QTapGesture* this_ptr) {
  return this_ptr->metaObject();
}

QTapGesture* qt_widgets_c_QTapGesture_new_no_args() {
  return new QTapGesture();
}

QTapGesture* qt_widgets_c_QTapGesture_new_parent(QObject* parent) {
  return new QTapGesture(parent);
}

void qt_widgets_c_QTapGesture_position_to_output(const QTapGesture* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->position());
}

int qt_widgets_c_QTapGesture_qt_metacall(QTapGesture* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QTapGesture_qt_metacast(QTapGesture* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QTapGesture_setPosition(QTapGesture* this_ptr, const QPointF* pos) {
  this_ptr->setPosition(*pos);
}

void qt_widgets_c_QTapGesture_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTapGesture::trUtf8(s, c, n));
}

void qt_widgets_c_QTapGesture_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTapGesture::tr(s, c, n));
}

