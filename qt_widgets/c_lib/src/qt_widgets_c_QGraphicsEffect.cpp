#include "qt_widgets_c_QGraphicsEffect.h"

QGraphicsEffect* qt_widgets_c_QGraphicsEffect_G_static_cast_QGraphicsEffect_ptr(QObject* ptr) {
  return static_cast<QGraphicsEffect*>(ptr);
}

QObject* qt_widgets_c_QGraphicsEffect_G_static_cast_QObject_ptr(QGraphicsEffect* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QGraphicsEffect_boundingRectFor_to_output(const QGraphicsEffect* this_ptr, const QRectF* sourceRect, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRectFor(*sourceRect));
}

void qt_widgets_c_QGraphicsEffect_boundingRect_to_output(const QGraphicsEffect* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect());
}

void qt_widgets_c_QGraphicsEffect_delete(QGraphicsEffect* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QGraphicsEffect_isEnabled(const QGraphicsEffect* this_ptr) {
  return this_ptr->isEnabled();
}

const QMetaObject* qt_widgets_c_QGraphicsEffect_metaObject(const QGraphicsEffect* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QGraphicsEffect_qt_metacall(QGraphicsEffect* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGraphicsEffect_qt_metacast(QGraphicsEffect* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGraphicsEffect_setEnabled(QGraphicsEffect* this_ptr, bool enable) {
  this_ptr->setEnabled(enable);
}

void qt_widgets_c_QGraphicsEffect_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsEffect::trUtf8(s, c, n));
}

void qt_widgets_c_QGraphicsEffect_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsEffect::tr(s, c, n));
}

void qt_widgets_c_QGraphicsEffect_update(QGraphicsEffect* this_ptr) {
  this_ptr->update();
}

