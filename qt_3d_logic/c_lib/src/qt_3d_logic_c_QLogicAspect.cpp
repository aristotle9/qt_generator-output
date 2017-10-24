#include "qt_3d_logic_c_QLogicAspect.h"

QObject* qt_3d_logic_c_QLogicAspect_G_static_cast_QObject_ptr(Qt3DLogic::QLogicAspect* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QAbstractAspect* qt_3d_logic_c_QLogicAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(Qt3DLogic::QLogicAspect* ptr) {
  return static_cast<Qt3DCore::QAbstractAspect*>(ptr);
}

Qt3DLogic::QLogicAspect* qt_3d_logic_c_QLogicAspect_G_static_cast_Qt3DLogic_QLogicAspect_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DLogic::QLogicAspect*>(ptr);
}

Qt3DLogic::QLogicAspect* qt_3d_logic_c_QLogicAspect_G_static_cast_Qt3DLogic_QLogicAspect_ptr_Qt3DCore_QAbstractAspect(Qt3DCore::QAbstractAspect* ptr) {
  return static_cast<Qt3DLogic::QLogicAspect*>(ptr);
}

void qt_3d_logic_c_Qt3DLogic_QLogicAspect_delete(Qt3DLogic::QLogicAspect* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_logic_c_Qt3DLogic_QLogicAspect_metaObject(const Qt3DLogic::QLogicAspect* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DLogic::QLogicAspect* qt_3d_logic_c_Qt3DLogic_QLogicAspect_new_no_args() {
  return new Qt3DLogic::QLogicAspect();
}

Qt3DLogic::QLogicAspect* qt_3d_logic_c_Qt3DLogic_QLogicAspect_new_parent(QObject* parent) {
  return new Qt3DLogic::QLogicAspect(parent);
}

int qt_3d_logic_c_Qt3DLogic_QLogicAspect_qt_metacall(Qt3DLogic::QLogicAspect* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_logic_c_Qt3DLogic_QLogicAspect_qt_metacast(Qt3DLogic::QLogicAspect* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_logic_c_Qt3DLogic_QLogicAspect_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DLogic::QLogicAspect::trUtf8(s, c, n));
}

void qt_3d_logic_c_Qt3DLogic_QLogicAspect_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DLogic::QLogicAspect::tr(s, c, n));
}

