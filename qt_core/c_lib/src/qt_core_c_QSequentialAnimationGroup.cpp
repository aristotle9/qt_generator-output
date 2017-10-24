#include "qt_core_c_QSequentialAnimationGroup.h"

QSequentialAnimationGroup* qt_core_c_QSequentialAnimationGroup_G_dynamic_cast_QSequentialAnimationGroup_ptr_QAbstractAnimation(QAbstractAnimation* ptr) {
  return dynamic_cast<QSequentialAnimationGroup*>(ptr);
}

QSequentialAnimationGroup* qt_core_c_QSequentialAnimationGroup_G_dynamic_cast_QSequentialAnimationGroup_ptr_QAnimationGroup(QAnimationGroup* ptr) {
  return dynamic_cast<QSequentialAnimationGroup*>(ptr);
}

QSequentialAnimationGroup* qt_core_c_QSequentialAnimationGroup_G_dynamic_cast_QSequentialAnimationGroup_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QSequentialAnimationGroup*>(ptr);
}

QAbstractAnimation* qt_core_c_QSequentialAnimationGroup_G_static_cast_QAbstractAnimation_ptr(QSequentialAnimationGroup* ptr) {
  return static_cast<QAbstractAnimation*>(ptr);
}

QAnimationGroup* qt_core_c_QSequentialAnimationGroup_G_static_cast_QAnimationGroup_ptr(QSequentialAnimationGroup* ptr) {
  return static_cast<QAnimationGroup*>(ptr);
}

QObject* qt_core_c_QSequentialAnimationGroup_G_static_cast_QObject_ptr(QSequentialAnimationGroup* ptr) {
  return static_cast<QObject*>(ptr);
}

QSequentialAnimationGroup* qt_core_c_QSequentialAnimationGroup_G_static_cast_QSequentialAnimationGroup_ptr_QAbstractAnimation(QAbstractAnimation* ptr) {
  return static_cast<QSequentialAnimationGroup*>(ptr);
}

QSequentialAnimationGroup* qt_core_c_QSequentialAnimationGroup_G_static_cast_QSequentialAnimationGroup_ptr_QAnimationGroup(QAnimationGroup* ptr) {
  return static_cast<QSequentialAnimationGroup*>(ptr);
}

QSequentialAnimationGroup* qt_core_c_QSequentialAnimationGroup_G_static_cast_QSequentialAnimationGroup_ptr_QObject(QObject* ptr) {
  return static_cast<QSequentialAnimationGroup*>(ptr);
}

QPauseAnimation* qt_core_c_QSequentialAnimationGroup_addPause(QSequentialAnimationGroup* this_ptr, int msecs) {
  return this_ptr->addPause(msecs);
}

QAbstractAnimation* qt_core_c_QSequentialAnimationGroup_currentAnimation(const QSequentialAnimationGroup* this_ptr) {
  return this_ptr->currentAnimation();
}

void qt_core_c_QSequentialAnimationGroup_delete(QSequentialAnimationGroup* this_ptr) {
  delete this_ptr;
}

int qt_core_c_QSequentialAnimationGroup_duration(const QSequentialAnimationGroup* this_ptr) {
  return this_ptr->duration();
}

QPauseAnimation* qt_core_c_QSequentialAnimationGroup_insertPause(QSequentialAnimationGroup* this_ptr, int index, int msecs) {
  return this_ptr->insertPause(index, msecs);
}

const QMetaObject* qt_core_c_QSequentialAnimationGroup_metaObject(const QSequentialAnimationGroup* this_ptr) {
  return this_ptr->metaObject();
}

QSequentialAnimationGroup* qt_core_c_QSequentialAnimationGroup_new_no_args() {
  return new QSequentialAnimationGroup();
}

QSequentialAnimationGroup* qt_core_c_QSequentialAnimationGroup_new_parent(QObject* parent) {
  return new QSequentialAnimationGroup(parent);
}

void qt_core_c_QSequentialAnimationGroup_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSequentialAnimationGroup::trUtf8(s, c, n));
}

void qt_core_c_QSequentialAnimationGroup_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSequentialAnimationGroup::tr(s, c, n));
}

