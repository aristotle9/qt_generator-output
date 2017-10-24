#include "qt_widgets_c_QGraphicsBlurEffect.h"

QGraphicsBlurEffect* qt_widgets_c_QGraphicsBlurEffect_G_dynamic_cast_QGraphicsBlurEffect_ptr(QGraphicsEffect* ptr) {
  return dynamic_cast<QGraphicsBlurEffect*>(ptr);
}

QGraphicsBlurEffect* qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QGraphicsBlurEffect_ptr_QGraphicsEffect(QGraphicsEffect* ptr) {
  return static_cast<QGraphicsBlurEffect*>(ptr);
}

QGraphicsBlurEffect* qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QGraphicsBlurEffect_ptr_QObject(QObject* ptr) {
  return static_cast<QGraphicsBlurEffect*>(ptr);
}

QGraphicsEffect* qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QGraphicsEffect_ptr(QGraphicsBlurEffect* ptr) {
  return static_cast<QGraphicsEffect*>(ptr);
}

QObject* qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QObject_ptr(QGraphicsBlurEffect* ptr) {
  return static_cast<QObject*>(ptr);
}

unsigned int qt_widgets_c_QGraphicsBlurEffect_blurHints(const QGraphicsBlurEffect* this_ptr) {
  return uint(this_ptr->blurHints());
}

double qt_widgets_c_QGraphicsBlurEffect_blurRadius(const QGraphicsBlurEffect* this_ptr) {
  return this_ptr->blurRadius();
}

void qt_widgets_c_QGraphicsBlurEffect_boundingRectFor_to_output(const QGraphicsBlurEffect* this_ptr, const QRectF* rect, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRectFor(*rect));
}

void qt_widgets_c_QGraphicsBlurEffect_delete(QGraphicsBlurEffect* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QGraphicsBlurEffect_metaObject(const QGraphicsBlurEffect* this_ptr) {
  return this_ptr->metaObject();
}

QGraphicsBlurEffect* qt_widgets_c_QGraphicsBlurEffect_new_no_args() {
  return new QGraphicsBlurEffect();
}

QGraphicsBlurEffect* qt_widgets_c_QGraphicsBlurEffect_new_parent(QObject* parent) {
  return new QGraphicsBlurEffect(parent);
}

int qt_widgets_c_QGraphicsBlurEffect_qt_metacall(QGraphicsBlurEffect* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGraphicsBlurEffect_qt_metacast(QGraphicsBlurEffect* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGraphicsBlurEffect_setBlurHints(QGraphicsBlurEffect* this_ptr, unsigned int hints) {
  this_ptr->setBlurHints(QFlags< QGraphicsBlurEffect::BlurHint >(hints));
}

void qt_widgets_c_QGraphicsBlurEffect_setBlurRadius(QGraphicsBlurEffect* this_ptr, double blurRadius) {
  this_ptr->setBlurRadius(blurRadius);
}

void qt_widgets_c_QGraphicsBlurEffect_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsBlurEffect::trUtf8(s, c, n));
}

void qt_widgets_c_QGraphicsBlurEffect_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsBlurEffect::tr(s, c, n));
}

