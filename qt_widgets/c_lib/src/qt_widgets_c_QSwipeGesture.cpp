#include "qt_widgets_c_QSwipeGesture.h"

QSwipeGesture* qt_widgets_c_QSwipeGesture_G_dynamic_cast_QSwipeGesture_ptr(QGesture* ptr) {
  return dynamic_cast<QSwipeGesture*>(ptr);
}

QGesture* qt_widgets_c_QSwipeGesture_G_static_cast_QGesture_ptr(QSwipeGesture* ptr) {
  return static_cast<QGesture*>(ptr);
}

QObject* qt_widgets_c_QSwipeGesture_G_static_cast_QObject_ptr(QSwipeGesture* ptr) {
  return static_cast<QObject*>(ptr);
}

QSwipeGesture* qt_widgets_c_QSwipeGesture_G_static_cast_QSwipeGesture_ptr_QGesture(QGesture* ptr) {
  return static_cast<QSwipeGesture*>(ptr);
}

QSwipeGesture* qt_widgets_c_QSwipeGesture_G_static_cast_QSwipeGesture_ptr_QObject(QObject* ptr) {
  return static_cast<QSwipeGesture*>(ptr);
}

void qt_widgets_c_QSwipeGesture_delete(QSwipeGesture* this_ptr) {
  delete this_ptr;
}

QSwipeGesture::SwipeDirection qt_widgets_c_QSwipeGesture_horizontalDirection(const QSwipeGesture* this_ptr) {
  return this_ptr->horizontalDirection();
}

const QMetaObject* qt_widgets_c_QSwipeGesture_metaObject(const QSwipeGesture* this_ptr) {
  return this_ptr->metaObject();
}

QSwipeGesture* qt_widgets_c_QSwipeGesture_new_no_args() {
  return new QSwipeGesture();
}

QSwipeGesture* qt_widgets_c_QSwipeGesture_new_parent(QObject* parent) {
  return new QSwipeGesture(parent);
}

int qt_widgets_c_QSwipeGesture_qt_metacall(QSwipeGesture* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QSwipeGesture_qt_metacast(QSwipeGesture* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QSwipeGesture_setSwipeAngle(QSwipeGesture* this_ptr, double value) {
  this_ptr->setSwipeAngle(value);
}

double qt_widgets_c_QSwipeGesture_swipeAngle(const QSwipeGesture* this_ptr) {
  return this_ptr->swipeAngle();
}

void qt_widgets_c_QSwipeGesture_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSwipeGesture::trUtf8(s, c, n));
}

void qt_widgets_c_QSwipeGesture_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSwipeGesture::tr(s, c, n));
}

QSwipeGesture::SwipeDirection qt_widgets_c_QSwipeGesture_verticalDirection(const QSwipeGesture* this_ptr) {
  return this_ptr->verticalDirection();
}

