#include "qt_3d_render_c_QFrontFace.h"

Qt3DRender::QFrontFace* qt_3d_render_c_QFrontFace_G_dynamic_cast_Qt3DRender_QFrontFace_ptr(Qt3DRender::QRenderState* ptr) {
  return dynamic_cast<Qt3DRender::QFrontFace*>(ptr);
}

QObject* qt_3d_render_c_QFrontFace_G_static_cast_QObject_ptr(Qt3DRender::QFrontFace* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QFrontFace_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QFrontFace* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QFrontFace* qt_3d_render_c_QFrontFace_G_static_cast_Qt3DRender_QFrontFace_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QFrontFace*>(ptr);
}

Qt3DRender::QFrontFace* qt_3d_render_c_QFrontFace_G_static_cast_Qt3DRender_QFrontFace_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QFrontFace*>(ptr);
}

Qt3DRender::QFrontFace* qt_3d_render_c_QFrontFace_G_static_cast_Qt3DRender_QFrontFace_ptr_Qt3DRender_QRenderState(Qt3DRender::QRenderState* ptr) {
  return static_cast<Qt3DRender::QFrontFace*>(ptr);
}

Qt3DRender::QRenderState* qt_3d_render_c_QFrontFace_G_static_cast_Qt3DRender_QRenderState_ptr(Qt3DRender::QFrontFace* ptr) {
  return static_cast<Qt3DRender::QRenderState*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QFrontFace_delete(Qt3DRender::QFrontFace* this_ptr) {
  delete this_ptr;
}

Qt3DRender::QFrontFace::WindingDirection qt_3d_render_c_Qt3DRender_QFrontFace_direction(const Qt3DRender::QFrontFace* this_ptr) {
  return this_ptr->direction();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QFrontFace_metaObject(const Qt3DRender::QFrontFace* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QFrontFace* qt_3d_render_c_Qt3DRender_QFrontFace_new_no_args() {
  return new Qt3DRender::QFrontFace();
}

Qt3DRender::QFrontFace* qt_3d_render_c_Qt3DRender_QFrontFace_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QFrontFace(parent);
}

int qt_3d_render_c_Qt3DRender_QFrontFace_qt_metacall(Qt3DRender::QFrontFace* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QFrontFace_qt_metacast(Qt3DRender::QFrontFace* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QFrontFace_setDirection(Qt3DRender::QFrontFace* this_ptr, Qt3DRender::QFrontFace::WindingDirection direction) {
  this_ptr->setDirection(direction);
}

void qt_3d_render_c_Qt3DRender_QFrontFace_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QFrontFace::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QFrontFace_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QFrontFace::tr(s, c, n));
}

