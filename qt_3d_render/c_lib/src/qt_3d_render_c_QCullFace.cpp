#include "qt_3d_render_c_QCullFace.h"

Qt3DRender::QCullFace* qt_3d_render_c_QCullFace_G_dynamic_cast_Qt3DRender_QCullFace_ptr(Qt3DRender::QRenderState* ptr) {
  return dynamic_cast<Qt3DRender::QCullFace*>(ptr);
}

QObject* qt_3d_render_c_QCullFace_G_static_cast_QObject_ptr(Qt3DRender::QCullFace* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QCullFace_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QCullFace* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QCullFace* qt_3d_render_c_QCullFace_G_static_cast_Qt3DRender_QCullFace_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QCullFace*>(ptr);
}

Qt3DRender::QCullFace* qt_3d_render_c_QCullFace_G_static_cast_Qt3DRender_QCullFace_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QCullFace*>(ptr);
}

Qt3DRender::QCullFace* qt_3d_render_c_QCullFace_G_static_cast_Qt3DRender_QCullFace_ptr_Qt3DRender_QRenderState(Qt3DRender::QRenderState* ptr) {
  return static_cast<Qt3DRender::QCullFace*>(ptr);
}

Qt3DRender::QRenderState* qt_3d_render_c_QCullFace_G_static_cast_Qt3DRender_QRenderState_ptr(Qt3DRender::QCullFace* ptr) {
  return static_cast<Qt3DRender::QRenderState*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QCullFace_delete(Qt3DRender::QCullFace* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QCullFace_metaObject(const Qt3DRender::QCullFace* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QCullFace::CullingMode qt_3d_render_c_Qt3DRender_QCullFace_mode(const Qt3DRender::QCullFace* this_ptr) {
  return this_ptr->mode();
}

Qt3DRender::QCullFace* qt_3d_render_c_Qt3DRender_QCullFace_new_no_args() {
  return new Qt3DRender::QCullFace();
}

Qt3DRender::QCullFace* qt_3d_render_c_Qt3DRender_QCullFace_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QCullFace(parent);
}

int qt_3d_render_c_Qt3DRender_QCullFace_qt_metacall(Qt3DRender::QCullFace* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QCullFace_qt_metacast(Qt3DRender::QCullFace* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QCullFace_setMode(Qt3DRender::QCullFace* this_ptr, Qt3DRender::QCullFace::CullingMode mode) {
  this_ptr->setMode(mode);
}

void qt_3d_render_c_Qt3DRender_QCullFace_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QCullFace::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QCullFace_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QCullFace::tr(s, c, n));
}

