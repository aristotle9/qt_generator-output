#include "qt_3d_render_c_QLayerFilter.h"

Qt3DRender::QLayerFilter* qt_3d_render_c_QLayerFilter_G_dynamic_cast_Qt3DRender_QLayerFilter_ptr(Qt3DRender::QFrameGraphNode* ptr) {
  return dynamic_cast<Qt3DRender::QLayerFilter*>(ptr);
}

QObject* qt_3d_render_c_QLayerFilter_G_static_cast_QObject_ptr(Qt3DRender::QLayerFilter* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QLayerFilter* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QLayerFilter* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNode*>(ptr);
}

Qt3DRender::QLayerFilter* qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QLayerFilter_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QLayerFilter*>(ptr);
}

Qt3DRender::QLayerFilter* qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QLayerFilter_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QLayerFilter*>(ptr);
}

Qt3DRender::QLayerFilter* qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QLayerFilter_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr) {
  return static_cast<Qt3DRender::QLayerFilter*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QLayerFilter_addLayer(Qt3DRender::QLayerFilter* this_ptr, Qt3DRender::QLayer* layer) {
  this_ptr->addLayer(layer);
}

void qt_3d_render_c_Qt3DRender_QLayerFilter_delete(Qt3DRender::QLayerFilter* this_ptr) {
  delete this_ptr;
}

void qt_3d_render_c_Qt3DRender_QLayerFilter_layers_to_output(const Qt3DRender::QLayerFilter* this_ptr, QVector< Qt3DRender::QLayer* >* output) {
  new(output) QVector< Qt3DRender::QLayer* >(this_ptr->layers());
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QLayerFilter_metaObject(const Qt3DRender::QLayerFilter* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QLayerFilter* qt_3d_render_c_Qt3DRender_QLayerFilter_new_no_args() {
  return new Qt3DRender::QLayerFilter();
}

Qt3DRender::QLayerFilter* qt_3d_render_c_Qt3DRender_QLayerFilter_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QLayerFilter(parent);
}

int qt_3d_render_c_Qt3DRender_QLayerFilter_qt_metacall(Qt3DRender::QLayerFilter* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QLayerFilter_qt_metacast(Qt3DRender::QLayerFilter* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QLayerFilter_removeLayer(Qt3DRender::QLayerFilter* this_ptr, Qt3DRender::QLayer* layer) {
  this_ptr->removeLayer(layer);
}

void qt_3d_render_c_Qt3DRender_QLayerFilter_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QLayerFilter::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QLayerFilter_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QLayerFilter::tr(s, c, n));
}

