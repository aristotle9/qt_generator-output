#include "qt_widgets_c_QGesture.h"

void qt_widgets_c_QGesture_G_operator_shl_to_output_QDebug_QGesture(const QDebug* arg1, const QGesture* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, arg2));
}

void qt_widgets_c_QGesture_G_operator_shl_to_output_QDebug_QGestureEvent(const QDebug* arg1, const QGestureEvent* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, arg2));
}

QGesture* qt_widgets_c_QGesture_G_static_cast_QGesture_ptr(QObject* ptr) {
  return static_cast<QGesture*>(ptr);
}

QObject* qt_widgets_c_QGesture_G_static_cast_QObject_ptr(QGesture* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QGesture_delete(QGesture* this_ptr) {
  delete this_ptr;
}

QGesture::GestureCancelPolicy qt_widgets_c_QGesture_gestureCancelPolicy(const QGesture* this_ptr) {
  return this_ptr->gestureCancelPolicy();
}

bool qt_widgets_c_QGesture_hasHotSpot(const QGesture* this_ptr) {
  return this_ptr->hasHotSpot();
}

void qt_widgets_c_QGesture_hotSpot_to_output(const QGesture* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->hotSpot());
}

const QMetaObject* qt_widgets_c_QGesture_metaObject(const QGesture* this_ptr) {
  return this_ptr->metaObject();
}

QGesture* qt_widgets_c_QGesture_new_no_args() {
  return new QGesture();
}

QGesture* qt_widgets_c_QGesture_new_parent(QObject* parent) {
  return new QGesture(parent);
}

int qt_widgets_c_QGesture_qt_metacall(QGesture* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGesture_qt_metacast(QGesture* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGesture_setGestureCancelPolicy(QGesture* this_ptr, QGesture::GestureCancelPolicy policy) {
  this_ptr->setGestureCancelPolicy(policy);
}

void qt_widgets_c_QGesture_setHotSpot(QGesture* this_ptr, const QPointF* value) {
  this_ptr->setHotSpot(*value);
}

void qt_widgets_c_QGesture_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGesture::trUtf8(s, c, n));
}

void qt_widgets_c_QGesture_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGesture::tr(s, c, n));
}

void qt_widgets_c_QGesture_unsetHotSpot(QGesture* this_ptr) {
  this_ptr->unsetHotSpot();
}

