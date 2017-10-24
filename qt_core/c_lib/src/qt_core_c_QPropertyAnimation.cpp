#include "qt_core_c_QPropertyAnimation.h"

QPropertyAnimation* qt_core_c_QPropertyAnimation_G_dynamic_cast_QPropertyAnimation_ptr_QAbstractAnimation(QAbstractAnimation* ptr) {
  return dynamic_cast<QPropertyAnimation*>(ptr);
}

QPropertyAnimation* qt_core_c_QPropertyAnimation_G_dynamic_cast_QPropertyAnimation_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QPropertyAnimation*>(ptr);
}

QPropertyAnimation* qt_core_c_QPropertyAnimation_G_dynamic_cast_QPropertyAnimation_ptr_QVariantAnimation(QVariantAnimation* ptr) {
  return dynamic_cast<QPropertyAnimation*>(ptr);
}

QAbstractAnimation* qt_core_c_QPropertyAnimation_G_static_cast_QAbstractAnimation_ptr(QPropertyAnimation* ptr) {
  return static_cast<QAbstractAnimation*>(ptr);
}

QObject* qt_core_c_QPropertyAnimation_G_static_cast_QObject_ptr(QPropertyAnimation* ptr) {
  return static_cast<QObject*>(ptr);
}

QPropertyAnimation* qt_core_c_QPropertyAnimation_G_static_cast_QPropertyAnimation_ptr_QAbstractAnimation(QAbstractAnimation* ptr) {
  return static_cast<QPropertyAnimation*>(ptr);
}

QPropertyAnimation* qt_core_c_QPropertyAnimation_G_static_cast_QPropertyAnimation_ptr_QObject(QObject* ptr) {
  return static_cast<QPropertyAnimation*>(ptr);
}

QPropertyAnimation* qt_core_c_QPropertyAnimation_G_static_cast_QPropertyAnimation_ptr_QVariantAnimation(QVariantAnimation* ptr) {
  return static_cast<QPropertyAnimation*>(ptr);
}

QVariantAnimation* qt_core_c_QPropertyAnimation_G_static_cast_QVariantAnimation_ptr(QPropertyAnimation* ptr) {
  return static_cast<QVariantAnimation*>(ptr);
}

void qt_core_c_QPropertyAnimation_delete(QPropertyAnimation* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_core_c_QPropertyAnimation_metaObject(const QPropertyAnimation* this_ptr) {
  return this_ptr->metaObject();
}

QPropertyAnimation* qt_core_c_QPropertyAnimation_new_no_args() {
  return new QPropertyAnimation();
}

QPropertyAnimation* qt_core_c_QPropertyAnimation_new_parent(QObject* parent) {
  return new QPropertyAnimation(parent);
}

QPropertyAnimation* qt_core_c_QPropertyAnimation_new_target_propertyName(QObject* target, const QByteArray* propertyName) {
  return new QPropertyAnimation(target, *propertyName);
}

QPropertyAnimation* qt_core_c_QPropertyAnimation_new_target_propertyName_parent(QObject* target, const QByteArray* propertyName, QObject* parent) {
  return new QPropertyAnimation(target, *propertyName, parent);
}

void qt_core_c_QPropertyAnimation_propertyName_to_output(const QPropertyAnimation* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->propertyName());
}

void qt_core_c_QPropertyAnimation_setPropertyName(QPropertyAnimation* this_ptr, const QByteArray* propertyName) {
  this_ptr->setPropertyName(*propertyName);
}

void qt_core_c_QPropertyAnimation_setTargetObject(QPropertyAnimation* this_ptr, QObject* target) {
  this_ptr->setTargetObject(target);
}

QObject* qt_core_c_QPropertyAnimation_targetObject(const QPropertyAnimation* this_ptr) {
  return this_ptr->targetObject();
}

void qt_core_c_QPropertyAnimation_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPropertyAnimation::trUtf8(s, c, n));
}

void qt_core_c_QPropertyAnimation_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPropertyAnimation::tr(s, c, n));
}

