#include "qt_3d_render_c_QComputeCommand.h"

QObject* qt_3d_render_c_QComputeCommand_G_static_cast_QObject_ptr(Qt3DRender::QComputeCommand* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_QComputeCommand_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QComputeCommand* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QComputeCommand_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QComputeCommand* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QComputeCommand* qt_3d_render_c_QComputeCommand_G_static_cast_Qt3DRender_QComputeCommand_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QComputeCommand*>(ptr);
}

Qt3DRender::QComputeCommand* qt_3d_render_c_QComputeCommand_G_static_cast_Qt3DRender_QComputeCommand_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DRender::QComputeCommand*>(ptr);
}

Qt3DRender::QComputeCommand* qt_3d_render_c_QComputeCommand_G_static_cast_Qt3DRender_QComputeCommand_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QComputeCommand*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QComputeCommand_delete(Qt3DRender::QComputeCommand* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QComputeCommand_metaObject(const Qt3DRender::QComputeCommand* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QComputeCommand* qt_3d_render_c_Qt3DRender_QComputeCommand_new_no_args() {
  return new Qt3DRender::QComputeCommand();
}

Qt3DRender::QComputeCommand* qt_3d_render_c_Qt3DRender_QComputeCommand_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QComputeCommand(parent);
}

int qt_3d_render_c_Qt3DRender_QComputeCommand_qt_metacall(Qt3DRender::QComputeCommand* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QComputeCommand_qt_metacast(Qt3DRender::QComputeCommand* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QComputeCommand_setWorkGroupX(Qt3DRender::QComputeCommand* this_ptr, int workGroupX) {
  this_ptr->setWorkGroupX(workGroupX);
}

void qt_3d_render_c_Qt3DRender_QComputeCommand_setWorkGroupY(Qt3DRender::QComputeCommand* this_ptr, int workGroupY) {
  this_ptr->setWorkGroupY(workGroupY);
}

void qt_3d_render_c_Qt3DRender_QComputeCommand_setWorkGroupZ(Qt3DRender::QComputeCommand* this_ptr, int workGroupZ) {
  this_ptr->setWorkGroupZ(workGroupZ);
}

void qt_3d_render_c_Qt3DRender_QComputeCommand_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QComputeCommand::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QComputeCommand_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QComputeCommand::tr(s, c, n));
}

int qt_3d_render_c_Qt3DRender_QComputeCommand_workGroupX(const Qt3DRender::QComputeCommand* this_ptr) {
  return this_ptr->workGroupX();
}

int qt_3d_render_c_Qt3DRender_QComputeCommand_workGroupY(const Qt3DRender::QComputeCommand* this_ptr) {
  return this_ptr->workGroupY();
}

int qt_3d_render_c_Qt3DRender_QComputeCommand_workGroupZ(const Qt3DRender::QComputeCommand* this_ptr) {
  return this_ptr->workGroupZ();
}

