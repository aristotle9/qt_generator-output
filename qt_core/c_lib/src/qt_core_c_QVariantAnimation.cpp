#include "qt_core_c_QVariantAnimation.h"

QVariantAnimation* qt_core_c_QVariantAnimation_G_dynamic_cast_QVariantAnimation_ptr_QAbstractAnimation(QAbstractAnimation* ptr) {
  return dynamic_cast<QVariantAnimation*>(ptr);
}

QVariantAnimation* qt_core_c_QVariantAnimation_G_dynamic_cast_QVariantAnimation_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QVariantAnimation*>(ptr);
}

QAbstractAnimation* qt_core_c_QVariantAnimation_G_static_cast_QAbstractAnimation_ptr(QVariantAnimation* ptr) {
  return static_cast<QAbstractAnimation*>(ptr);
}

QObject* qt_core_c_QVariantAnimation_G_static_cast_QObject_ptr(QVariantAnimation* ptr) {
  return static_cast<QObject*>(ptr);
}

QVariantAnimation* qt_core_c_QVariantAnimation_G_static_cast_QVariantAnimation_ptr_QAbstractAnimation(QAbstractAnimation* ptr) {
  return static_cast<QVariantAnimation*>(ptr);
}

QVariantAnimation* qt_core_c_QVariantAnimation_G_static_cast_QVariantAnimation_ptr_QObject(QObject* ptr) {
  return static_cast<QVariantAnimation*>(ptr);
}

void qt_core_c_QVariantAnimation_currentValue_to_output(const QVariantAnimation* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->currentValue());
}

void qt_core_c_QVariantAnimation_delete(QVariantAnimation* this_ptr) {
  delete this_ptr;
}

int qt_core_c_QVariantAnimation_duration(const QVariantAnimation* this_ptr) {
  return this_ptr->duration();
}

void qt_core_c_QVariantAnimation_easingCurve_to_output(const QVariantAnimation* this_ptr, QEasingCurve* output) {
  new(output) QEasingCurve(this_ptr->easingCurve());
}

void qt_core_c_QVariantAnimation_endValue_to_output(const QVariantAnimation* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->endValue());
}

void qt_core_c_QVariantAnimation_keyValueAt_to_output(const QVariantAnimation* this_ptr, double step, QVariant* output) {
  new(output) QVariant(this_ptr->keyValueAt(step));
}

void qt_core_c_QVariantAnimation_keyValues_to_output(const QVariantAnimation* this_ptr, QVector< QPair< double, QVariant > >* output) {
  new(output) QVector< QPair< double, QVariant > >(this_ptr->keyValues());
}

const QMetaObject* qt_core_c_QVariantAnimation_metaObject(const QVariantAnimation* this_ptr) {
  return this_ptr->metaObject();
}

QVariantAnimation* qt_core_c_QVariantAnimation_new_no_args() {
  return new QVariantAnimation();
}

QVariantAnimation* qt_core_c_QVariantAnimation_new_parent(QObject* parent) {
  return new QVariantAnimation(parent);
}

void qt_core_c_QVariantAnimation_setDuration(QVariantAnimation* this_ptr, int msecs) {
  this_ptr->setDuration(msecs);
}

void qt_core_c_QVariantAnimation_setEasingCurve(QVariantAnimation* this_ptr, const QEasingCurve* easing) {
  this_ptr->setEasingCurve(*easing);
}

void qt_core_c_QVariantAnimation_setEndValue(QVariantAnimation* this_ptr, const QVariant* value) {
  this_ptr->setEndValue(*value);
}

void qt_core_c_QVariantAnimation_setKeyValueAt(QVariantAnimation* this_ptr, double step, const QVariant* value) {
  this_ptr->setKeyValueAt(step, *value);
}

void qt_core_c_QVariantAnimation_setKeyValues(QVariantAnimation* this_ptr, const QVector< QPair< double, QVariant > >* values) {
  this_ptr->setKeyValues(*values);
}

void qt_core_c_QVariantAnimation_setStartValue(QVariantAnimation* this_ptr, const QVariant* value) {
  this_ptr->setStartValue(*value);
}

void qt_core_c_QVariantAnimation_startValue_to_output(const QVariantAnimation* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->startValue());
}

void qt_core_c_QVariantAnimation_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QVariantAnimation::trUtf8(s, c, n));
}

void qt_core_c_QVariantAnimation_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QVariantAnimation::tr(s, c, n));
}

