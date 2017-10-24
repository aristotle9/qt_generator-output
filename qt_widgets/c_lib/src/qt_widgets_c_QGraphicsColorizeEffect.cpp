#include "qt_widgets_c_QGraphicsColorizeEffect.h"

QGraphicsColorizeEffect* qt_widgets_c_QGraphicsColorizeEffect_G_dynamic_cast_QGraphicsColorizeEffect_ptr(QGraphicsEffect* ptr) {
  return dynamic_cast<QGraphicsColorizeEffect*>(ptr);
}

QGraphicsColorizeEffect* qt_widgets_c_QGraphicsColorizeEffect_G_static_cast_QGraphicsColorizeEffect_ptr_QGraphicsEffect(QGraphicsEffect* ptr) {
  return static_cast<QGraphicsColorizeEffect*>(ptr);
}

QGraphicsColorizeEffect* qt_widgets_c_QGraphicsColorizeEffect_G_static_cast_QGraphicsColorizeEffect_ptr_QObject(QObject* ptr) {
  return static_cast<QGraphicsColorizeEffect*>(ptr);
}

QGraphicsEffect* qt_widgets_c_QGraphicsColorizeEffect_G_static_cast_QGraphicsEffect_ptr(QGraphicsColorizeEffect* ptr) {
  return static_cast<QGraphicsEffect*>(ptr);
}

QObject* qt_widgets_c_QGraphicsColorizeEffect_G_static_cast_QObject_ptr(QGraphicsColorizeEffect* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QGraphicsColorizeEffect_color_to_output(const QGraphicsColorizeEffect* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->color());
}

void qt_widgets_c_QGraphicsColorizeEffect_delete(QGraphicsColorizeEffect* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QGraphicsColorizeEffect_metaObject(const QGraphicsColorizeEffect* this_ptr) {
  return this_ptr->metaObject();
}

QGraphicsColorizeEffect* qt_widgets_c_QGraphicsColorizeEffect_new_no_args() {
  return new QGraphicsColorizeEffect();
}

QGraphicsColorizeEffect* qt_widgets_c_QGraphicsColorizeEffect_new_parent(QObject* parent) {
  return new QGraphicsColorizeEffect(parent);
}

int qt_widgets_c_QGraphicsColorizeEffect_qt_metacall(QGraphicsColorizeEffect* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGraphicsColorizeEffect_qt_metacast(QGraphicsColorizeEffect* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGraphicsColorizeEffect_setColor(QGraphicsColorizeEffect* this_ptr, const QColor* c) {
  this_ptr->setColor(*c);
}

void qt_widgets_c_QGraphicsColorizeEffect_setStrength(QGraphicsColorizeEffect* this_ptr, double strength) {
  this_ptr->setStrength(strength);
}

double qt_widgets_c_QGraphicsColorizeEffect_strength(const QGraphicsColorizeEffect* this_ptr) {
  return this_ptr->strength();
}

void qt_widgets_c_QGraphicsColorizeEffect_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsColorizeEffect::trUtf8(s, c, n));
}

void qt_widgets_c_QGraphicsColorizeEffect_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsColorizeEffect::tr(s, c, n));
}

