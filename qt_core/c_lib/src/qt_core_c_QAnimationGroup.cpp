#include "qt_core_c_QAnimationGroup.h"

QAnimationGroup* qt_core_c_QAnimationGroup_G_dynamic_cast_QAnimationGroup_ptr_QAbstractAnimation(QAbstractAnimation* ptr) {
  return dynamic_cast<QAnimationGroup*>(ptr);
}

QAnimationGroup* qt_core_c_QAnimationGroup_G_dynamic_cast_QAnimationGroup_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QAnimationGroup*>(ptr);
}

QAbstractAnimation* qt_core_c_QAnimationGroup_G_static_cast_QAbstractAnimation_ptr(QAnimationGroup* ptr) {
  return static_cast<QAbstractAnimation*>(ptr);
}

QAnimationGroup* qt_core_c_QAnimationGroup_G_static_cast_QAnimationGroup_ptr_QAbstractAnimation(QAbstractAnimation* ptr) {
  return static_cast<QAnimationGroup*>(ptr);
}

QAnimationGroup* qt_core_c_QAnimationGroup_G_static_cast_QAnimationGroup_ptr_QObject(QObject* ptr) {
  return static_cast<QAnimationGroup*>(ptr);
}

QObject* qt_core_c_QAnimationGroup_G_static_cast_QObject_ptr(QAnimationGroup* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_core_c_QAnimationGroup_addAnimation(QAnimationGroup* this_ptr, QAbstractAnimation* animation) {
  this_ptr->addAnimation(animation);
}

QAbstractAnimation* qt_core_c_QAnimationGroup_animationAt(const QAnimationGroup* this_ptr, int index) {
  return this_ptr->animationAt(index);
}

int qt_core_c_QAnimationGroup_animationCount(const QAnimationGroup* this_ptr) {
  return this_ptr->animationCount();
}

void qt_core_c_QAnimationGroup_clear(QAnimationGroup* this_ptr) {
  this_ptr->clear();
}

void qt_core_c_QAnimationGroup_delete(QAnimationGroup* this_ptr) {
  delete this_ptr;
}

int qt_core_c_QAnimationGroup_indexOfAnimation(const QAnimationGroup* this_ptr, QAbstractAnimation* animation) {
  return this_ptr->indexOfAnimation(animation);
}

void qt_core_c_QAnimationGroup_insertAnimation(QAnimationGroup* this_ptr, int index, QAbstractAnimation* animation) {
  this_ptr->insertAnimation(index, animation);
}

const QMetaObject* qt_core_c_QAnimationGroup_metaObject(const QAnimationGroup* this_ptr) {
  return this_ptr->metaObject();
}

void qt_core_c_QAnimationGroup_removeAnimation(QAnimationGroup* this_ptr, QAbstractAnimation* animation) {
  this_ptr->removeAnimation(animation);
}

QAbstractAnimation* qt_core_c_QAnimationGroup_takeAnimation(QAnimationGroup* this_ptr, int index) {
  return this_ptr->takeAnimation(index);
}

void qt_core_c_QAnimationGroup_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAnimationGroup::trUtf8(s, c, n));
}

void qt_core_c_QAnimationGroup_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAnimationGroup::tr(s, c, n));
}

