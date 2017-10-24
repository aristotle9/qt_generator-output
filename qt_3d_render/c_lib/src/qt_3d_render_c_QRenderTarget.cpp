#include "qt_3d_render_c_QRenderTarget.h"

QObject* qt_3d_render_c_QRenderTarget_G_static_cast_QObject_ptr(Qt3DRender::QRenderTarget* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_QRenderTarget_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QRenderTarget* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QRenderTarget_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QRenderTarget* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QRenderTarget* qt_3d_render_c_QRenderTarget_G_static_cast_Qt3DRender_QRenderTarget_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QRenderTarget*>(ptr);
}

Qt3DRender::QRenderTarget* qt_3d_render_c_QRenderTarget_G_static_cast_Qt3DRender_QRenderTarget_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DRender::QRenderTarget*>(ptr);
}

Qt3DRender::QRenderTarget* qt_3d_render_c_QRenderTarget_G_static_cast_Qt3DRender_QRenderTarget_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QRenderTarget*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QRenderTarget_addOutput(Qt3DRender::QRenderTarget* this_ptr, Qt3DRender::QRenderTargetOutput* output) {
  this_ptr->addOutput(output);
}

void qt_3d_render_c_Qt3DRender_QRenderTarget_delete(Qt3DRender::QRenderTarget* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QRenderTarget_metaObject(const Qt3DRender::QRenderTarget* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QRenderTarget* qt_3d_render_c_Qt3DRender_QRenderTarget_new_no_args() {
  return new Qt3DRender::QRenderTarget();
}

Qt3DRender::QRenderTarget* qt_3d_render_c_Qt3DRender_QRenderTarget_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QRenderTarget(parent);
}

void qt_3d_render_c_Qt3DRender_QRenderTarget_outputs_to_output(const Qt3DRender::QRenderTarget* this_ptr, QVector< Qt3DRender::QRenderTargetOutput* >* output) {
  new(output) QVector< Qt3DRender::QRenderTargetOutput* >(this_ptr->outputs());
}

int qt_3d_render_c_Qt3DRender_QRenderTarget_qt_metacall(Qt3DRender::QRenderTarget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QRenderTarget_qt_metacast(Qt3DRender::QRenderTarget* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QRenderTarget_removeOutput(Qt3DRender::QRenderTarget* this_ptr, Qt3DRender::QRenderTargetOutput* output) {
  this_ptr->removeOutput(output);
}

void qt_3d_render_c_Qt3DRender_QRenderTarget_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderTarget::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QRenderTarget_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderTarget::tr(s, c, n));
}

