#include "qt_3d_core_c_QAbstractAspect.h"

QObject* qt_3d_core_c_QAbstractAspect_G_static_cast_QObject_ptr(Qt3DCore::QAbstractAspect* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QAbstractAspect* qt_3d_core_c_QAbstractAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(QObject* ptr) {
  return static_cast<Qt3DCore::QAbstractAspect*>(ptr);
}

void qt_3d_core_c_Qt3DCore_QAbstractAspect_delete(Qt3DCore::QAbstractAspect* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_core_c_Qt3DCore_QAbstractAspect_metaObject(const Qt3DCore::QAbstractAspect* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DCore::QAbstractAspect* qt_3d_core_c_Qt3DCore_QAbstractAspect_new_no_args() {
  return new Qt3DCore::QAbstractAspect();
}

Qt3DCore::QAbstractAspect* qt_3d_core_c_Qt3DCore_QAbstractAspect_new_parent(QObject* parent) {
  return new Qt3DCore::QAbstractAspect(parent);
}

int qt_3d_core_c_Qt3DCore_QAbstractAspect_qt_metacall(Qt3DCore::QAbstractAspect* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_core_c_Qt3DCore_QAbstractAspect_qt_metacast(Qt3DCore::QAbstractAspect* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_core_c_Qt3DCore_QAbstractAspect_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DCore::QAbstractAspect::trUtf8(s, c, n));
}

void qt_3d_core_c_Qt3DCore_QAbstractAspect_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DCore::QAbstractAspect::tr(s, c, n));
}

