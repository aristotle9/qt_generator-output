#include "qt_3d_render_c_QBufferCapture.h"

Qt3DRender::QBufferCapture* qt_3d_render_c_QBufferCapture_G_dynamic_cast_Qt3DRender_QBufferCapture_ptr(Qt3DRender::QFrameGraphNode* ptr) {
  return dynamic_cast<Qt3DRender::QBufferCapture*>(ptr);
}

QObject* qt_3d_render_c_QBufferCapture_G_static_cast_QObject_ptr(Qt3DRender::QBufferCapture* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QBufferCapture_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QBufferCapture* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QBufferCapture* qt_3d_render_c_QBufferCapture_G_static_cast_Qt3DRender_QBufferCapture_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QBufferCapture*>(ptr);
}

Qt3DRender::QBufferCapture* qt_3d_render_c_QBufferCapture_G_static_cast_Qt3DRender_QBufferCapture_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QBufferCapture*>(ptr);
}

Qt3DRender::QBufferCapture* qt_3d_render_c_QBufferCapture_G_static_cast_Qt3DRender_QBufferCapture_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr) {
  return static_cast<Qt3DRender::QBufferCapture*>(ptr);
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_QBufferCapture_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QBufferCapture* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNode*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QBufferCapture_delete(Qt3DRender::QBufferCapture* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QBufferCapture_metaObject(const Qt3DRender::QBufferCapture* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QBufferCapture* qt_3d_render_c_Qt3DRender_QBufferCapture_new_no_args() {
  return new Qt3DRender::QBufferCapture();
}

Qt3DRender::QBufferCapture* qt_3d_render_c_Qt3DRender_QBufferCapture_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QBufferCapture(parent);
}

int qt_3d_render_c_Qt3DRender_QBufferCapture_qt_metacall(Qt3DRender::QBufferCapture* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QBufferCapture_qt_metacast(Qt3DRender::QBufferCapture* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QBufferCapture_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QBufferCapture::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QBufferCapture_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QBufferCapture::tr(s, c, n));
}

