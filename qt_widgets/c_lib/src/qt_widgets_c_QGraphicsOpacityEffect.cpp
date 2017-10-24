#include "qt_widgets_c_QGraphicsOpacityEffect.h"

QGraphicsOpacityEffect* qt_widgets_c_QGraphicsOpacityEffect_G_dynamic_cast_QGraphicsOpacityEffect_ptr(QGraphicsEffect* ptr) {
  return dynamic_cast<QGraphicsOpacityEffect*>(ptr);
}

QGraphicsEffect* qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QGraphicsEffect_ptr(QGraphicsOpacityEffect* ptr) {
  return static_cast<QGraphicsEffect*>(ptr);
}

QGraphicsOpacityEffect* qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QGraphicsOpacityEffect_ptr_QGraphicsEffect(QGraphicsEffect* ptr) {
  return static_cast<QGraphicsOpacityEffect*>(ptr);
}

QGraphicsOpacityEffect* qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QGraphicsOpacityEffect_ptr_QObject(QObject* ptr) {
  return static_cast<QGraphicsOpacityEffect*>(ptr);
}

QObject* qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QObject_ptr(QGraphicsOpacityEffect* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QGraphicsOpacityEffect_delete(QGraphicsOpacityEffect* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QGraphicsOpacityEffect_metaObject(const QGraphicsOpacityEffect* this_ptr) {
  return this_ptr->metaObject();
}

QGraphicsOpacityEffect* qt_widgets_c_QGraphicsOpacityEffect_new_no_args() {
  return new QGraphicsOpacityEffect();
}

QGraphicsOpacityEffect* qt_widgets_c_QGraphicsOpacityEffect_new_parent(QObject* parent) {
  return new QGraphicsOpacityEffect(parent);
}

double qt_widgets_c_QGraphicsOpacityEffect_opacity(const QGraphicsOpacityEffect* this_ptr) {
  return this_ptr->opacity();
}

void qt_widgets_c_QGraphicsOpacityEffect_opacityMask_to_output(const QGraphicsOpacityEffect* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->opacityMask());
}

int qt_widgets_c_QGraphicsOpacityEffect_qt_metacall(QGraphicsOpacityEffect* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGraphicsOpacityEffect_qt_metacast(QGraphicsOpacityEffect* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGraphicsOpacityEffect_setOpacity(QGraphicsOpacityEffect* this_ptr, double opacity) {
  this_ptr->setOpacity(opacity);
}

void qt_widgets_c_QGraphicsOpacityEffect_setOpacityMask(QGraphicsOpacityEffect* this_ptr, const QBrush* mask) {
  this_ptr->setOpacityMask(*mask);
}

void qt_widgets_c_QGraphicsOpacityEffect_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsOpacityEffect::trUtf8(s, c, n));
}

void qt_widgets_c_QGraphicsOpacityEffect_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsOpacityEffect::tr(s, c, n));
}

