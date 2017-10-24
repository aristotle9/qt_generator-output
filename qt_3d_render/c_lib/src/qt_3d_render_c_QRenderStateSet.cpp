#include "qt_3d_render_c_QRenderStateSet.h"

Qt3DRender::QRenderStateSet* qt_3d_render_c_QRenderStateSet_G_dynamic_cast_Qt3DRender_QRenderStateSet_ptr(Qt3DRender::QFrameGraphNode* ptr) {
  return dynamic_cast<Qt3DRender::QRenderStateSet*>(ptr);
}

QObject* qt_3d_render_c_QRenderStateSet_G_static_cast_QObject_ptr(Qt3DRender::QRenderStateSet* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QRenderStateSet_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QRenderStateSet* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_QRenderStateSet_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QRenderStateSet* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNode*>(ptr);
}

Qt3DRender::QRenderStateSet* qt_3d_render_c_QRenderStateSet_G_static_cast_Qt3DRender_QRenderStateSet_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QRenderStateSet*>(ptr);
}

Qt3DRender::QRenderStateSet* qt_3d_render_c_QRenderStateSet_G_static_cast_Qt3DRender_QRenderStateSet_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QRenderStateSet*>(ptr);
}

Qt3DRender::QRenderStateSet* qt_3d_render_c_QRenderStateSet_G_static_cast_Qt3DRender_QRenderStateSet_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr) {
  return static_cast<Qt3DRender::QRenderStateSet*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QRenderStateSet_addRenderState(Qt3DRender::QRenderStateSet* this_ptr, Qt3DRender::QRenderState* state) {
  this_ptr->addRenderState(state);
}

void qt_3d_render_c_Qt3DRender_QRenderStateSet_delete(Qt3DRender::QRenderStateSet* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QRenderStateSet_metaObject(const Qt3DRender::QRenderStateSet* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QRenderStateSet* qt_3d_render_c_Qt3DRender_QRenderStateSet_new_no_args() {
  return new Qt3DRender::QRenderStateSet();
}

Qt3DRender::QRenderStateSet* qt_3d_render_c_Qt3DRender_QRenderStateSet_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QRenderStateSet(parent);
}

int qt_3d_render_c_Qt3DRender_QRenderStateSet_qt_metacall(Qt3DRender::QRenderStateSet* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QRenderStateSet_qt_metacast(Qt3DRender::QRenderStateSet* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QRenderStateSet_removeRenderState(Qt3DRender::QRenderStateSet* this_ptr, Qt3DRender::QRenderState* state) {
  this_ptr->removeRenderState(state);
}

void qt_3d_render_c_Qt3DRender_QRenderStateSet_renderStates_to_output(const Qt3DRender::QRenderStateSet* this_ptr, QVector< Qt3DRender::QRenderState* >* output) {
  new(output) QVector< Qt3DRender::QRenderState* >(this_ptr->renderStates());
}

void qt_3d_render_c_Qt3DRender_QRenderStateSet_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderStateSet::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QRenderStateSet_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderStateSet::tr(s, c, n));
}

