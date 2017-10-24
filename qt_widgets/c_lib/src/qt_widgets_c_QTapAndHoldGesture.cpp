#include "qt_widgets_c_QTapAndHoldGesture.h"

QTapAndHoldGesture* qt_widgets_c_QTapAndHoldGesture_G_dynamic_cast_QTapAndHoldGesture_ptr(QGesture* ptr) {
  return dynamic_cast<QTapAndHoldGesture*>(ptr);
}

QGesture* qt_widgets_c_QTapAndHoldGesture_G_static_cast_QGesture_ptr(QTapAndHoldGesture* ptr) {
  return static_cast<QGesture*>(ptr);
}

QObject* qt_widgets_c_QTapAndHoldGesture_G_static_cast_QObject_ptr(QTapAndHoldGesture* ptr) {
  return static_cast<QObject*>(ptr);
}

QTapAndHoldGesture* qt_widgets_c_QTapAndHoldGesture_G_static_cast_QTapAndHoldGesture_ptr_QGesture(QGesture* ptr) {
  return static_cast<QTapAndHoldGesture*>(ptr);
}

QTapAndHoldGesture* qt_widgets_c_QTapAndHoldGesture_G_static_cast_QTapAndHoldGesture_ptr_QObject(QObject* ptr) {
  return static_cast<QTapAndHoldGesture*>(ptr);
}

void qt_widgets_c_QTapAndHoldGesture_delete(QTapAndHoldGesture* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QTapAndHoldGesture_metaObject(const QTapAndHoldGesture* this_ptr) {
  return this_ptr->metaObject();
}

QTapAndHoldGesture* qt_widgets_c_QTapAndHoldGesture_new_no_args() {
  return new QTapAndHoldGesture();
}

QTapAndHoldGesture* qt_widgets_c_QTapAndHoldGesture_new_parent(QObject* parent) {
  return new QTapAndHoldGesture(parent);
}

void qt_widgets_c_QTapAndHoldGesture_position_to_output(const QTapAndHoldGesture* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->position());
}

int qt_widgets_c_QTapAndHoldGesture_qt_metacall(QTapAndHoldGesture* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QTapAndHoldGesture_qt_metacast(QTapAndHoldGesture* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QTapAndHoldGesture_setPosition(QTapAndHoldGesture* this_ptr, const QPointF* pos) {
  this_ptr->setPosition(*pos);
}

void qt_widgets_c_QTapAndHoldGesture_setTimeout(int msecs) {
  QTapAndHoldGesture::setTimeout(msecs);
}

int qt_widgets_c_QTapAndHoldGesture_timeout() {
  return QTapAndHoldGesture::timeout();
}

void qt_widgets_c_QTapAndHoldGesture_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTapAndHoldGesture::trUtf8(s, c, n));
}

void qt_widgets_c_QTapAndHoldGesture_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTapAndHoldGesture::tr(s, c, n));
}

