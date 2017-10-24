#include "qt_3d_render_c_QDepthTest.h"

Qt3DRender::QDepthTest* qt_3d_render_c_QDepthTest_G_dynamic_cast_Qt3DRender_QDepthTest_ptr(Qt3DRender::QRenderState* ptr) {
  return dynamic_cast<Qt3DRender::QDepthTest*>(ptr);
}

QObject* qt_3d_render_c_QDepthTest_G_static_cast_QObject_ptr(Qt3DRender::QDepthTest* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QDepthTest_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QDepthTest* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QDepthTest* qt_3d_render_c_QDepthTest_G_static_cast_Qt3DRender_QDepthTest_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QDepthTest*>(ptr);
}

Qt3DRender::QDepthTest* qt_3d_render_c_QDepthTest_G_static_cast_Qt3DRender_QDepthTest_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QDepthTest*>(ptr);
}

Qt3DRender::QDepthTest* qt_3d_render_c_QDepthTest_G_static_cast_Qt3DRender_QDepthTest_ptr_Qt3DRender_QRenderState(Qt3DRender::QRenderState* ptr) {
  return static_cast<Qt3DRender::QDepthTest*>(ptr);
}

Qt3DRender::QRenderState* qt_3d_render_c_QDepthTest_G_static_cast_Qt3DRender_QRenderState_ptr(Qt3DRender::QDepthTest* ptr) {
  return static_cast<Qt3DRender::QRenderState*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QDepthTest_delete(Qt3DRender::QDepthTest* this_ptr) {
  delete this_ptr;
}

Qt3DRender::QDepthTest::DepthFunction qt_3d_render_c_Qt3DRender_QDepthTest_depthFunction(const Qt3DRender::QDepthTest* this_ptr) {
  return this_ptr->depthFunction();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QDepthTest_metaObject(const Qt3DRender::QDepthTest* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QDepthTest* qt_3d_render_c_Qt3DRender_QDepthTest_new_no_args() {
  return new Qt3DRender::QDepthTest();
}

Qt3DRender::QDepthTest* qt_3d_render_c_Qt3DRender_QDepthTest_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QDepthTest(parent);
}

int qt_3d_render_c_Qt3DRender_QDepthTest_qt_metacall(Qt3DRender::QDepthTest* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QDepthTest_qt_metacast(Qt3DRender::QDepthTest* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QDepthTest_setDepthFunction(Qt3DRender::QDepthTest* this_ptr, Qt3DRender::QDepthTest::DepthFunction depthFunction) {
  this_ptr->setDepthFunction(depthFunction);
}

void qt_3d_render_c_Qt3DRender_QDepthTest_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QDepthTest::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QDepthTest_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QDepthTest::tr(s, c, n));
}

