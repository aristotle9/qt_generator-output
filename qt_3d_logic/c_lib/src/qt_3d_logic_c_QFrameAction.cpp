#include "qt_3d_logic_c_QFrameAction.h"

QObject* qt_3d_logic_c_QFrameAction_G_static_cast_QObject_ptr(Qt3DLogic::QFrameAction* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_logic_c_QFrameAction_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DLogic::QFrameAction* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_logic_c_QFrameAction_G_static_cast_Qt3DCore_QNode_ptr(Qt3DLogic::QFrameAction* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DLogic::QFrameAction* qt_3d_logic_c_QFrameAction_G_static_cast_Qt3DLogic_QFrameAction_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DLogic::QFrameAction*>(ptr);
}

Qt3DLogic::QFrameAction* qt_3d_logic_c_QFrameAction_G_static_cast_Qt3DLogic_QFrameAction_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DLogic::QFrameAction*>(ptr);
}

Qt3DLogic::QFrameAction* qt_3d_logic_c_QFrameAction_G_static_cast_Qt3DLogic_QFrameAction_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DLogic::QFrameAction*>(ptr);
}

void qt_3d_logic_c_Qt3DLogic_QFrameAction_delete(Qt3DLogic::QFrameAction* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_logic_c_Qt3DLogic_QFrameAction_metaObject(const Qt3DLogic::QFrameAction* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DLogic::QFrameAction* qt_3d_logic_c_Qt3DLogic_QFrameAction_new_no_args() {
  return new Qt3DLogic::QFrameAction();
}

Qt3DLogic::QFrameAction* qt_3d_logic_c_Qt3DLogic_QFrameAction_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DLogic::QFrameAction(parent);
}

int qt_3d_logic_c_Qt3DLogic_QFrameAction_qt_metacall(Qt3DLogic::QFrameAction* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_logic_c_Qt3DLogic_QFrameAction_qt_metacast(Qt3DLogic::QFrameAction* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_logic_c_Qt3DLogic_QFrameAction_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DLogic::QFrameAction::trUtf8(s, c, n));
}

void qt_3d_logic_c_Qt3DLogic_QFrameAction_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DLogic::QFrameAction::tr(s, c, n));
}

