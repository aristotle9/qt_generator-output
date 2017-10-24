#include "qt_3d_render_c_QBlendEquation.h"

Qt3DRender::QBlendEquation* qt_3d_render_c_QBlendEquation_G_dynamic_cast_Qt3DRender_QBlendEquation_ptr(Qt3DRender::QRenderState* ptr) {
  return dynamic_cast<Qt3DRender::QBlendEquation*>(ptr);
}

QObject* qt_3d_render_c_QBlendEquation_G_static_cast_QObject_ptr(Qt3DRender::QBlendEquation* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QBlendEquation_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QBlendEquation* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QBlendEquation* qt_3d_render_c_QBlendEquation_G_static_cast_Qt3DRender_QBlendEquation_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QBlendEquation*>(ptr);
}

Qt3DRender::QBlendEquation* qt_3d_render_c_QBlendEquation_G_static_cast_Qt3DRender_QBlendEquation_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QBlendEquation*>(ptr);
}

Qt3DRender::QBlendEquation* qt_3d_render_c_QBlendEquation_G_static_cast_Qt3DRender_QBlendEquation_ptr_Qt3DRender_QRenderState(Qt3DRender::QRenderState* ptr) {
  return static_cast<Qt3DRender::QBlendEquation*>(ptr);
}

Qt3DRender::QRenderState* qt_3d_render_c_QBlendEquation_G_static_cast_Qt3DRender_QRenderState_ptr(Qt3DRender::QBlendEquation* ptr) {
  return static_cast<Qt3DRender::QRenderState*>(ptr);
}

Qt3DRender::QBlendEquation::BlendFunction qt_3d_render_c_Qt3DRender_QBlendEquation_blendFunction(const Qt3DRender::QBlendEquation* this_ptr) {
  return this_ptr->blendFunction();
}

void qt_3d_render_c_Qt3DRender_QBlendEquation_delete(Qt3DRender::QBlendEquation* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QBlendEquation_metaObject(const Qt3DRender::QBlendEquation* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QBlendEquation* qt_3d_render_c_Qt3DRender_QBlendEquation_new_no_args() {
  return new Qt3DRender::QBlendEquation();
}

Qt3DRender::QBlendEquation* qt_3d_render_c_Qt3DRender_QBlendEquation_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QBlendEquation(parent);
}

int qt_3d_render_c_Qt3DRender_QBlendEquation_qt_metacall(Qt3DRender::QBlendEquation* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QBlendEquation_qt_metacast(Qt3DRender::QBlendEquation* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QBlendEquation_setBlendFunction(Qt3DRender::QBlendEquation* this_ptr, Qt3DRender::QBlendEquation::BlendFunction blendFunction) {
  this_ptr->setBlendFunction(blendFunction);
}

void qt_3d_render_c_Qt3DRender_QBlendEquation_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QBlendEquation::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QBlendEquation_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QBlendEquation::tr(s, c, n));
}

