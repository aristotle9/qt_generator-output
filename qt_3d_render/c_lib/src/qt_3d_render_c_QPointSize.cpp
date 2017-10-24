#include "qt_3d_render_c_QPointSize.h"

Qt3DRender::QPointSize* qt_3d_render_c_QPointSize_G_dynamic_cast_Qt3DRender_QPointSize_ptr(Qt3DRender::QRenderState* ptr) {
  return dynamic_cast<Qt3DRender::QPointSize*>(ptr);
}

QObject* qt_3d_render_c_QPointSize_G_static_cast_QObject_ptr(Qt3DRender::QPointSize* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QPointSize_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QPointSize* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QPointSize* qt_3d_render_c_QPointSize_G_static_cast_Qt3DRender_QPointSize_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QPointSize*>(ptr);
}

Qt3DRender::QPointSize* qt_3d_render_c_QPointSize_G_static_cast_Qt3DRender_QPointSize_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QPointSize*>(ptr);
}

Qt3DRender::QPointSize* qt_3d_render_c_QPointSize_G_static_cast_Qt3DRender_QPointSize_ptr_Qt3DRender_QRenderState(Qt3DRender::QRenderState* ptr) {
  return static_cast<Qt3DRender::QPointSize*>(ptr);
}

Qt3DRender::QRenderState* qt_3d_render_c_QPointSize_G_static_cast_Qt3DRender_QRenderState_ptr(Qt3DRender::QPointSize* ptr) {
  return static_cast<Qt3DRender::QRenderState*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QPointSize_delete(Qt3DRender::QPointSize* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QPointSize_metaObject(const Qt3DRender::QPointSize* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QPointSize* qt_3d_render_c_Qt3DRender_QPointSize_new_no_args() {
  return new Qt3DRender::QPointSize();
}

Qt3DRender::QPointSize* qt_3d_render_c_Qt3DRender_QPointSize_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QPointSize(parent);
}

int qt_3d_render_c_Qt3DRender_QPointSize_qt_metacall(Qt3DRender::QPointSize* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QPointSize_qt_metacast(Qt3DRender::QPointSize* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QPointSize_setSizeMode(Qt3DRender::QPointSize* this_ptr, Qt3DRender::QPointSize::SizeMode sizeMode) {
  this_ptr->setSizeMode(sizeMode);
}

void qt_3d_render_c_Qt3DRender_QPointSize_setValue(Qt3DRender::QPointSize* this_ptr, float value) {
  this_ptr->setValue(value);
}

Qt3DRender::QPointSize::SizeMode qt_3d_render_c_Qt3DRender_QPointSize_sizeMode(const Qt3DRender::QPointSize* this_ptr) {
  return this_ptr->sizeMode();
}

void qt_3d_render_c_Qt3DRender_QPointSize_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QPointSize::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QPointSize_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QPointSize::tr(s, c, n));
}

float qt_3d_render_c_Qt3DRender_QPointSize_value(const Qt3DRender::QPointSize* this_ptr) {
  return this_ptr->value();
}

