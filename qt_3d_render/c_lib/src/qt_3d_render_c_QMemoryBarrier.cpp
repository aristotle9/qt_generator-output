#include "qt_3d_render_c_QMemoryBarrier.h"

Qt3DRender::QMemoryBarrier* qt_3d_render_c_QMemoryBarrier_G_dynamic_cast_Qt3DRender_QMemoryBarrier_ptr(Qt3DRender::QFrameGraphNode* ptr) {
  return dynamic_cast<Qt3DRender::QMemoryBarrier*>(ptr);
}

QObject* qt_3d_render_c_QMemoryBarrier_G_static_cast_QObject_ptr(Qt3DRender::QMemoryBarrier* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QMemoryBarrier_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QMemoryBarrier* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_QMemoryBarrier_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QMemoryBarrier* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNode*>(ptr);
}

Qt3DRender::QMemoryBarrier* qt_3d_render_c_QMemoryBarrier_G_static_cast_Qt3DRender_QMemoryBarrier_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QMemoryBarrier*>(ptr);
}

Qt3DRender::QMemoryBarrier* qt_3d_render_c_QMemoryBarrier_G_static_cast_Qt3DRender_QMemoryBarrier_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QMemoryBarrier*>(ptr);
}

Qt3DRender::QMemoryBarrier* qt_3d_render_c_QMemoryBarrier_G_static_cast_Qt3DRender_QMemoryBarrier_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr) {
  return static_cast<Qt3DRender::QMemoryBarrier*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QMemoryBarrier_delete(Qt3DRender::QMemoryBarrier* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QMemoryBarrier_metaObject(const Qt3DRender::QMemoryBarrier* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QMemoryBarrier* qt_3d_render_c_Qt3DRender_QMemoryBarrier_new_no_args() {
  return new Qt3DRender::QMemoryBarrier();
}

Qt3DRender::QMemoryBarrier* qt_3d_render_c_Qt3DRender_QMemoryBarrier_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QMemoryBarrier(parent);
}

int qt_3d_render_c_Qt3DRender_QMemoryBarrier_qt_metacall(Qt3DRender::QMemoryBarrier* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QMemoryBarrier_qt_metacast(Qt3DRender::QMemoryBarrier* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QMemoryBarrier_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QMemoryBarrier::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QMemoryBarrier_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QMemoryBarrier::tr(s, c, n));
}

unsigned int qt_3d_render_c_Qt3DRender_QMemoryBarrier_waitOperations(const Qt3DRender::QMemoryBarrier* this_ptr) {
  return uint(this_ptr->waitOperations());
}

