#include "qt_widgets_c_QGraphicsDropShadowEffect.h"

QGraphicsDropShadowEffect* qt_widgets_c_QGraphicsDropShadowEffect_G_dynamic_cast_QGraphicsDropShadowEffect_ptr(QGraphicsEffect* ptr) {
  return dynamic_cast<QGraphicsDropShadowEffect*>(ptr);
}

QGraphicsDropShadowEffect* qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QGraphicsDropShadowEffect_ptr_QGraphicsEffect(QGraphicsEffect* ptr) {
  return static_cast<QGraphicsDropShadowEffect*>(ptr);
}

QGraphicsDropShadowEffect* qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QGraphicsDropShadowEffect_ptr_QObject(QObject* ptr) {
  return static_cast<QGraphicsDropShadowEffect*>(ptr);
}

QGraphicsEffect* qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QGraphicsEffect_ptr(QGraphicsDropShadowEffect* ptr) {
  return static_cast<QGraphicsEffect*>(ptr);
}

QObject* qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QObject_ptr(QGraphicsDropShadowEffect* ptr) {
  return static_cast<QObject*>(ptr);
}

double qt_widgets_c_QGraphicsDropShadowEffect_blurRadius(const QGraphicsDropShadowEffect* this_ptr) {
  return this_ptr->blurRadius();
}

void qt_widgets_c_QGraphicsDropShadowEffect_boundingRectFor_to_output(const QGraphicsDropShadowEffect* this_ptr, const QRectF* rect, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRectFor(*rect));
}

void qt_widgets_c_QGraphicsDropShadowEffect_color_to_output(const QGraphicsDropShadowEffect* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->color());
}

void qt_widgets_c_QGraphicsDropShadowEffect_delete(QGraphicsDropShadowEffect* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QGraphicsDropShadowEffect_metaObject(const QGraphicsDropShadowEffect* this_ptr) {
  return this_ptr->metaObject();
}

QGraphicsDropShadowEffect* qt_widgets_c_QGraphicsDropShadowEffect_new_no_args() {
  return new QGraphicsDropShadowEffect();
}

QGraphicsDropShadowEffect* qt_widgets_c_QGraphicsDropShadowEffect_new_parent(QObject* parent) {
  return new QGraphicsDropShadowEffect(parent);
}

void qt_widgets_c_QGraphicsDropShadowEffect_offset_to_output(const QGraphicsDropShadowEffect* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->offset());
}

int qt_widgets_c_QGraphicsDropShadowEffect_qt_metacall(QGraphicsDropShadowEffect* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGraphicsDropShadowEffect_qt_metacast(QGraphicsDropShadowEffect* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGraphicsDropShadowEffect_setBlurRadius(QGraphicsDropShadowEffect* this_ptr, double blurRadius) {
  this_ptr->setBlurRadius(blurRadius);
}

void qt_widgets_c_QGraphicsDropShadowEffect_setColor(QGraphicsDropShadowEffect* this_ptr, const QColor* color) {
  this_ptr->setColor(*color);
}

void qt_widgets_c_QGraphicsDropShadowEffect_setOffset_d(QGraphicsDropShadowEffect* this_ptr, double d) {
  this_ptr->setOffset(d);
}

void qt_widgets_c_QGraphicsDropShadowEffect_setOffset_dx_dy(QGraphicsDropShadowEffect* this_ptr, double dx, double dy) {
  this_ptr->setOffset(dx, dy);
}

void qt_widgets_c_QGraphicsDropShadowEffect_setOffset_ofs(QGraphicsDropShadowEffect* this_ptr, const QPointF* ofs) {
  this_ptr->setOffset(*ofs);
}

void qt_widgets_c_QGraphicsDropShadowEffect_setXOffset(QGraphicsDropShadowEffect* this_ptr, double dx) {
  this_ptr->setXOffset(dx);
}

void qt_widgets_c_QGraphicsDropShadowEffect_setYOffset(QGraphicsDropShadowEffect* this_ptr, double dy) {
  this_ptr->setYOffset(dy);
}

void qt_widgets_c_QGraphicsDropShadowEffect_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsDropShadowEffect::trUtf8(s, c, n));
}

void qt_widgets_c_QGraphicsDropShadowEffect_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsDropShadowEffect::tr(s, c, n));
}

double qt_widgets_c_QGraphicsDropShadowEffect_xOffset(const QGraphicsDropShadowEffect* this_ptr) {
  return this_ptr->xOffset();
}

double qt_widgets_c_QGraphicsDropShadowEffect_yOffset(const QGraphicsDropShadowEffect* this_ptr) {
  return this_ptr->yOffset();
}

