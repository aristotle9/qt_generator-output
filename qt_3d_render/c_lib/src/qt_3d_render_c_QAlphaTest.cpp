#include "qt_3d_render_c_QAlphaTest.h"

Qt3DRender::QAlphaTest* qt_3d_render_c_QAlphaTest_G_dynamic_cast_Qt3DRender_QAlphaTest_ptr(Qt3DRender::QRenderState* ptr) {
  return dynamic_cast<Qt3DRender::QAlphaTest*>(ptr);
}

QObject* qt_3d_render_c_QAlphaTest_G_static_cast_QObject_ptr(Qt3DRender::QAlphaTest* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QAlphaTest_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QAlphaTest* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QAlphaTest* qt_3d_render_c_QAlphaTest_G_static_cast_Qt3DRender_QAlphaTest_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QAlphaTest*>(ptr);
}

Qt3DRender::QAlphaTest* qt_3d_render_c_QAlphaTest_G_static_cast_Qt3DRender_QAlphaTest_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QAlphaTest*>(ptr);
}

Qt3DRender::QAlphaTest* qt_3d_render_c_QAlphaTest_G_static_cast_Qt3DRender_QAlphaTest_ptr_Qt3DRender_QRenderState(Qt3DRender::QRenderState* ptr) {
  return static_cast<Qt3DRender::QAlphaTest*>(ptr);
}

Qt3DRender::QRenderState* qt_3d_render_c_QAlphaTest_G_static_cast_Qt3DRender_QRenderState_ptr(Qt3DRender::QAlphaTest* ptr) {
  return static_cast<Qt3DRender::QRenderState*>(ptr);
}

Qt3DRender::QAlphaTest::AlphaFunction qt_3d_render_c_Qt3DRender_QAlphaTest_alphaFunction(const Qt3DRender::QAlphaTest* this_ptr) {
  return this_ptr->alphaFunction();
}

void qt_3d_render_c_Qt3DRender_QAlphaTest_delete(Qt3DRender::QAlphaTest* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QAlphaTest_metaObject(const Qt3DRender::QAlphaTest* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QAlphaTest* qt_3d_render_c_Qt3DRender_QAlphaTest_new_no_args() {
  return new Qt3DRender::QAlphaTest();
}

Qt3DRender::QAlphaTest* qt_3d_render_c_Qt3DRender_QAlphaTest_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QAlphaTest(parent);
}

int qt_3d_render_c_Qt3DRender_QAlphaTest_qt_metacall(Qt3DRender::QAlphaTest* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QAlphaTest_qt_metacast(Qt3DRender::QAlphaTest* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

float qt_3d_render_c_Qt3DRender_QAlphaTest_referenceValue(const Qt3DRender::QAlphaTest* this_ptr) {
  return this_ptr->referenceValue();
}

void qt_3d_render_c_Qt3DRender_QAlphaTest_setAlphaFunction(Qt3DRender::QAlphaTest* this_ptr, Qt3DRender::QAlphaTest::AlphaFunction alphaFunction) {
  this_ptr->setAlphaFunction(alphaFunction);
}

void qt_3d_render_c_Qt3DRender_QAlphaTest_setReferenceValue(Qt3DRender::QAlphaTest* this_ptr, float referenceValue) {
  this_ptr->setReferenceValue(referenceValue);
}

void qt_3d_render_c_Qt3DRender_QAlphaTest_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QAlphaTest::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QAlphaTest_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QAlphaTest::tr(s, c, n));
}

