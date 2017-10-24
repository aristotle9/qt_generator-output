#include "qt_3d_render_c_QLayer.h"

QObject* qt_3d_render_c_QLayer_G_static_cast_QObject_ptr(Qt3DRender::QLayer* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_QLayer_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QLayer* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QLayer_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QLayer* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QLayer* qt_3d_render_c_QLayer_G_static_cast_Qt3DRender_QLayer_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QLayer*>(ptr);
}

Qt3DRender::QLayer* qt_3d_render_c_QLayer_G_static_cast_Qt3DRender_QLayer_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DRender::QLayer*>(ptr);
}

Qt3DRender::QLayer* qt_3d_render_c_QLayer_G_static_cast_Qt3DRender_QLayer_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QLayer*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QLayer_delete(Qt3DRender::QLayer* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QLayer_metaObject(const Qt3DRender::QLayer* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QLayer* qt_3d_render_c_Qt3DRender_QLayer_new_no_args() {
  return new Qt3DRender::QLayer();
}

Qt3DRender::QLayer* qt_3d_render_c_Qt3DRender_QLayer_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QLayer(parent);
}

int qt_3d_render_c_Qt3DRender_QLayer_qt_metacall(Qt3DRender::QLayer* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QLayer_qt_metacast(Qt3DRender::QLayer* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QLayer_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QLayer::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QLayer_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QLayer::tr(s, c, n));
}

