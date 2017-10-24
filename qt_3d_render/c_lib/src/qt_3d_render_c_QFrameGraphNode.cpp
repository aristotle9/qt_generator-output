#include "qt_3d_render_c_QFrameGraphNode.h"

QObject* qt_3d_render_c_QFrameGraphNode_G_static_cast_QObject_ptr(Qt3DRender::QFrameGraphNode* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QFrameGraphNode_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QFrameGraphNode* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_QFrameGraphNode_G_static_cast_Qt3DRender_QFrameGraphNode_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNode*>(ptr);
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_QFrameGraphNode_G_static_cast_Qt3DRender_QFrameGraphNode_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNode*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QFrameGraphNode_delete(Qt3DRender::QFrameGraphNode* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QFrameGraphNode_metaObject(const Qt3DRender::QFrameGraphNode* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_Qt3DRender_QFrameGraphNode_new_no_args() {
  return new Qt3DRender::QFrameGraphNode();
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_Qt3DRender_QFrameGraphNode_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QFrameGraphNode(parent);
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_Qt3DRender_QFrameGraphNode_parentFrameGraphNode(const Qt3DRender::QFrameGraphNode* this_ptr) {
  return this_ptr->parentFrameGraphNode();
}

int qt_3d_render_c_Qt3DRender_QFrameGraphNode_qt_metacall(Qt3DRender::QFrameGraphNode* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QFrameGraphNode_qt_metacast(Qt3DRender::QFrameGraphNode* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QFrameGraphNode_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QFrameGraphNode::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QFrameGraphNode_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QFrameGraphNode::tr(s, c, n));
}

