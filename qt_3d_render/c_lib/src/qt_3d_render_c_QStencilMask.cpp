#include "qt_3d_render_c_QStencilMask.h"

Qt3DRender::QStencilMask* qt_3d_render_c_QStencilMask_G_dynamic_cast_Qt3DRender_QStencilMask_ptr(Qt3DRender::QRenderState* ptr) {
  return dynamic_cast<Qt3DRender::QStencilMask*>(ptr);
}

QObject* qt_3d_render_c_QStencilMask_G_static_cast_QObject_ptr(Qt3DRender::QStencilMask* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QStencilMask_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QStencilMask* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QRenderState* qt_3d_render_c_QStencilMask_G_static_cast_Qt3DRender_QRenderState_ptr(Qt3DRender::QStencilMask* ptr) {
  return static_cast<Qt3DRender::QRenderState*>(ptr);
}

Qt3DRender::QStencilMask* qt_3d_render_c_QStencilMask_G_static_cast_Qt3DRender_QStencilMask_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QStencilMask*>(ptr);
}

Qt3DRender::QStencilMask* qt_3d_render_c_QStencilMask_G_static_cast_Qt3DRender_QStencilMask_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QStencilMask*>(ptr);
}

Qt3DRender::QStencilMask* qt_3d_render_c_QStencilMask_G_static_cast_Qt3DRender_QStencilMask_ptr_Qt3DRender_QRenderState(Qt3DRender::QRenderState* ptr) {
  return static_cast<Qt3DRender::QStencilMask*>(ptr);
}

unsigned int qt_3d_render_c_Qt3DRender_QStencilMask_backOutputMask(const Qt3DRender::QStencilMask* this_ptr) {
  return this_ptr->backOutputMask();
}

void qt_3d_render_c_Qt3DRender_QStencilMask_delete(Qt3DRender::QStencilMask* this_ptr) {
  delete this_ptr;
}

unsigned int qt_3d_render_c_Qt3DRender_QStencilMask_frontOutputMask(const Qt3DRender::QStencilMask* this_ptr) {
  return this_ptr->frontOutputMask();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QStencilMask_metaObject(const Qt3DRender::QStencilMask* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QStencilMask* qt_3d_render_c_Qt3DRender_QStencilMask_new_no_args() {
  return new Qt3DRender::QStencilMask();
}

Qt3DRender::QStencilMask* qt_3d_render_c_Qt3DRender_QStencilMask_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QStencilMask(parent);
}

int qt_3d_render_c_Qt3DRender_QStencilMask_qt_metacall(Qt3DRender::QStencilMask* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QStencilMask_qt_metacast(Qt3DRender::QStencilMask* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QStencilMask_setBackOutputMask(Qt3DRender::QStencilMask* this_ptr, unsigned int backOutputMask) {
  this_ptr->setBackOutputMask(backOutputMask);
}

void qt_3d_render_c_Qt3DRender_QStencilMask_setFrontOutputMask(Qt3DRender::QStencilMask* this_ptr, unsigned int frontOutputMask) {
  this_ptr->setFrontOutputMask(frontOutputMask);
}

void qt_3d_render_c_Qt3DRender_QStencilMask_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QStencilMask::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QStencilMask_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QStencilMask::tr(s, c, n));
}

