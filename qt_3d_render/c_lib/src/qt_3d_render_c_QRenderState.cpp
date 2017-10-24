#include "qt_3d_render_c_QRenderState.h"

QObject* qt_3d_render_c_QRenderState_G_static_cast_QObject_ptr(Qt3DRender::QRenderState* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QRenderState_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QRenderState* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QRenderState* qt_3d_render_c_QRenderState_G_static_cast_Qt3DRender_QRenderState_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QRenderState*>(ptr);
}

Qt3DRender::QRenderState* qt_3d_render_c_QRenderState_G_static_cast_Qt3DRender_QRenderState_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QRenderState*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QRenderState_delete(Qt3DRender::QRenderState* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QRenderState_metaObject(const Qt3DRender::QRenderState* this_ptr) {
  return this_ptr->metaObject();
}

int qt_3d_render_c_Qt3DRender_QRenderState_qt_metacall(Qt3DRender::QRenderState* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QRenderState_qt_metacast(Qt3DRender::QRenderState* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QRenderState_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderState::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QRenderState_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderState::tr(s, c, n));
}

