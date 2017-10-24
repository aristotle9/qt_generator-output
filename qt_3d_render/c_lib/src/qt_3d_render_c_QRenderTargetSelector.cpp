#include "qt_3d_render_c_QRenderTargetSelector.h"

Qt3DRender::QRenderTargetSelector* qt_3d_render_c_QRenderTargetSelector_G_dynamic_cast_Qt3DRender_QRenderTargetSelector_ptr(Qt3DRender::QFrameGraphNode* ptr) {
  return dynamic_cast<Qt3DRender::QRenderTargetSelector*>(ptr);
}

QObject* qt_3d_render_c_QRenderTargetSelector_G_static_cast_QObject_ptr(Qt3DRender::QRenderTargetSelector* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QRenderTargetSelector_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QRenderTargetSelector* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_QRenderTargetSelector_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QRenderTargetSelector* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNode*>(ptr);
}

Qt3DRender::QRenderTargetSelector* qt_3d_render_c_QRenderTargetSelector_G_static_cast_Qt3DRender_QRenderTargetSelector_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QRenderTargetSelector*>(ptr);
}

Qt3DRender::QRenderTargetSelector* qt_3d_render_c_QRenderTargetSelector_G_static_cast_Qt3DRender_QRenderTargetSelector_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QRenderTargetSelector*>(ptr);
}

Qt3DRender::QRenderTargetSelector* qt_3d_render_c_QRenderTargetSelector_G_static_cast_Qt3DRender_QRenderTargetSelector_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr) {
  return static_cast<Qt3DRender::QRenderTargetSelector*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QRenderTargetSelector_delete(Qt3DRender::QRenderTargetSelector* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QRenderTargetSelector_metaObject(const Qt3DRender::QRenderTargetSelector* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QRenderTargetSelector* qt_3d_render_c_Qt3DRender_QRenderTargetSelector_new_no_args() {
  return new Qt3DRender::QRenderTargetSelector();
}

Qt3DRender::QRenderTargetSelector* qt_3d_render_c_Qt3DRender_QRenderTargetSelector_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QRenderTargetSelector(parent);
}

void qt_3d_render_c_Qt3DRender_QRenderTargetSelector_outputs_to_output(const Qt3DRender::QRenderTargetSelector* this_ptr, QVector< Qt3DRender::QRenderTargetOutput::AttachmentPoint >* output) {
  new(output) QVector< Qt3DRender::QRenderTargetOutput::AttachmentPoint >(this_ptr->outputs());
}

int qt_3d_render_c_Qt3DRender_QRenderTargetSelector_qt_metacall(Qt3DRender::QRenderTargetSelector* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QRenderTargetSelector_qt_metacast(Qt3DRender::QRenderTargetSelector* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QRenderTargetSelector_setOutputs(Qt3DRender::QRenderTargetSelector* this_ptr, const QVector< Qt3DRender::QRenderTargetOutput::AttachmentPoint >* buffers) {
  this_ptr->setOutputs(*buffers);
}

void qt_3d_render_c_Qt3DRender_QRenderTargetSelector_setTarget(Qt3DRender::QRenderTargetSelector* this_ptr, Qt3DRender::QRenderTarget* target) {
  this_ptr->setTarget(target);
}

Qt3DRender::QRenderTarget* qt_3d_render_c_Qt3DRender_QRenderTargetSelector_target(const Qt3DRender::QRenderTargetSelector* this_ptr) {
  return this_ptr->target();
}

void qt_3d_render_c_Qt3DRender_QRenderTargetSelector_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderTargetSelector::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QRenderTargetSelector_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderTargetSelector::tr(s, c, n));
}

