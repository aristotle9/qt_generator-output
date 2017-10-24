#include "qt_core_c_QPauseAnimation.h"

QPauseAnimation* qt_core_c_QPauseAnimation_G_dynamic_cast_QPauseAnimation_ptr_QAbstractAnimation(QAbstractAnimation* ptr) {
  return dynamic_cast<QPauseAnimation*>(ptr);
}

QPauseAnimation* qt_core_c_QPauseAnimation_G_dynamic_cast_QPauseAnimation_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QPauseAnimation*>(ptr);
}

QAbstractAnimation* qt_core_c_QPauseAnimation_G_static_cast_QAbstractAnimation_ptr(QPauseAnimation* ptr) {
  return static_cast<QAbstractAnimation*>(ptr);
}

QObject* qt_core_c_QPauseAnimation_G_static_cast_QObject_ptr(QPauseAnimation* ptr) {
  return static_cast<QObject*>(ptr);
}

QPauseAnimation* qt_core_c_QPauseAnimation_G_static_cast_QPauseAnimation_ptr_QAbstractAnimation(QAbstractAnimation* ptr) {
  return static_cast<QPauseAnimation*>(ptr);
}

QPauseAnimation* qt_core_c_QPauseAnimation_G_static_cast_QPauseAnimation_ptr_QObject(QObject* ptr) {
  return static_cast<QPauseAnimation*>(ptr);
}

void qt_core_c_QPauseAnimation_delete(QPauseAnimation* this_ptr) {
  delete this_ptr;
}

int qt_core_c_QPauseAnimation_duration(const QPauseAnimation* this_ptr) {
  return this_ptr->duration();
}

const QMetaObject* qt_core_c_QPauseAnimation_metaObject(const QPauseAnimation* this_ptr) {
  return this_ptr->metaObject();
}

QPauseAnimation* qt_core_c_QPauseAnimation_new_msecs(int msecs) {
  return new QPauseAnimation(msecs);
}

QPauseAnimation* qt_core_c_QPauseAnimation_new_msecs_parent(int msecs, QObject* parent) {
  return new QPauseAnimation(msecs, parent);
}

QPauseAnimation* qt_core_c_QPauseAnimation_new_no_args() {
  return new QPauseAnimation();
}

QPauseAnimation* qt_core_c_QPauseAnimation_new_parent(QObject* parent) {
  return new QPauseAnimation(parent);
}

void qt_core_c_QPauseAnimation_setDuration(QPauseAnimation* this_ptr, int msecs) {
  this_ptr->setDuration(msecs);
}

void qt_core_c_QPauseAnimation_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPauseAnimation::trUtf8(s, c, n));
}

void qt_core_c_QPauseAnimation_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPauseAnimation::tr(s, c, n));
}

