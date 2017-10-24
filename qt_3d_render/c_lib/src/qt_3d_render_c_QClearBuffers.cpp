#include "qt_3d_render_c_QClearBuffers.h"

Qt3DRender::QClearBuffers* qt_3d_render_c_QClearBuffers_G_dynamic_cast_Qt3DRender_QClearBuffers_ptr(Qt3DRender::QFrameGraphNode* ptr) {
  return dynamic_cast<Qt3DRender::QClearBuffers*>(ptr);
}

QObject* qt_3d_render_c_QClearBuffers_G_static_cast_QObject_ptr(Qt3DRender::QClearBuffers* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QClearBuffers* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QClearBuffers* qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QClearBuffers_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QClearBuffers*>(ptr);
}

Qt3DRender::QClearBuffers* qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QClearBuffers_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QClearBuffers*>(ptr);
}

Qt3DRender::QClearBuffers* qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QClearBuffers_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr) {
  return static_cast<Qt3DRender::QClearBuffers*>(ptr);
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QClearBuffers* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNode*>(ptr);
}

Qt3DRender::QClearBuffers::BufferType qt_3d_render_c_Qt3DRender_QClearBuffers_buffers(const Qt3DRender::QClearBuffers* this_ptr) {
  return this_ptr->buffers();
}

void qt_3d_render_c_Qt3DRender_QClearBuffers_clearColor_to_output(const Qt3DRender::QClearBuffers* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->clearColor());
}

float qt_3d_render_c_Qt3DRender_QClearBuffers_clearDepthValue(const Qt3DRender::QClearBuffers* this_ptr) {
  return this_ptr->clearDepthValue();
}

int qt_3d_render_c_Qt3DRender_QClearBuffers_clearStencilValue(const Qt3DRender::QClearBuffers* this_ptr) {
  return this_ptr->clearStencilValue();
}

Qt3DRender::QRenderTargetOutput* qt_3d_render_c_Qt3DRender_QClearBuffers_colorBuffer(const Qt3DRender::QClearBuffers* this_ptr) {
  return this_ptr->colorBuffer();
}

void qt_3d_render_c_Qt3DRender_QClearBuffers_delete(Qt3DRender::QClearBuffers* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QClearBuffers_metaObject(const Qt3DRender::QClearBuffers* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QClearBuffers* qt_3d_render_c_Qt3DRender_QClearBuffers_new_no_args() {
  return new Qt3DRender::QClearBuffers();
}

Qt3DRender::QClearBuffers* qt_3d_render_c_Qt3DRender_QClearBuffers_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QClearBuffers(parent);
}

int qt_3d_render_c_Qt3DRender_QClearBuffers_qt_metacall(Qt3DRender::QClearBuffers* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QClearBuffers_qt_metacast(Qt3DRender::QClearBuffers* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QClearBuffers_setBuffers(Qt3DRender::QClearBuffers* this_ptr, Qt3DRender::QClearBuffers::BufferType buffers) {
  this_ptr->setBuffers(buffers);
}

void qt_3d_render_c_Qt3DRender_QClearBuffers_setClearColor(Qt3DRender::QClearBuffers* this_ptr, const QColor* color) {
  this_ptr->setClearColor(*color);
}

void qt_3d_render_c_Qt3DRender_QClearBuffers_setClearDepthValue(Qt3DRender::QClearBuffers* this_ptr, float clearDepthValue) {
  this_ptr->setClearDepthValue(clearDepthValue);
}

void qt_3d_render_c_Qt3DRender_QClearBuffers_setClearStencilValue(Qt3DRender::QClearBuffers* this_ptr, int clearStencilValue) {
  this_ptr->setClearStencilValue(clearStencilValue);
}

void qt_3d_render_c_Qt3DRender_QClearBuffers_setColorBuffer(Qt3DRender::QClearBuffers* this_ptr, Qt3DRender::QRenderTargetOutput* buffer) {
  this_ptr->setColorBuffer(buffer);
}

void qt_3d_render_c_Qt3DRender_QClearBuffers_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QClearBuffers::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QClearBuffers_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QClearBuffers::tr(s, c, n));
}

