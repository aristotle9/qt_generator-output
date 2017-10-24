#include "qt_3d_render_c_QRenderPassFilter.h"

Qt3DRender::QRenderPassFilter* qt_3d_render_c_QRenderPassFilter_G_dynamic_cast_Qt3DRender_QRenderPassFilter_ptr(Qt3DRender::QFrameGraphNode* ptr) {
  return dynamic_cast<Qt3DRender::QRenderPassFilter*>(ptr);
}

QObject* qt_3d_render_c_QRenderPassFilter_G_static_cast_QObject_ptr(Qt3DRender::QRenderPassFilter* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QRenderPassFilter* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QRenderPassFilter* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNode*>(ptr);
}

Qt3DRender::QRenderPassFilter* qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QRenderPassFilter_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QRenderPassFilter*>(ptr);
}

Qt3DRender::QRenderPassFilter* qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QRenderPassFilter_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QRenderPassFilter*>(ptr);
}

Qt3DRender::QRenderPassFilter* qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QRenderPassFilter_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr) {
  return static_cast<Qt3DRender::QRenderPassFilter*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QRenderPassFilter_addMatch(Qt3DRender::QRenderPassFilter* this_ptr, Qt3DRender::QFilterKey* filterKey) {
  this_ptr->addMatch(filterKey);
}

void qt_3d_render_c_Qt3DRender_QRenderPassFilter_addParameter(Qt3DRender::QRenderPassFilter* this_ptr, Qt3DRender::QParameter* parameter) {
  this_ptr->addParameter(parameter);
}

void qt_3d_render_c_Qt3DRender_QRenderPassFilter_delete(Qt3DRender::QRenderPassFilter* this_ptr) {
  delete this_ptr;
}

void qt_3d_render_c_Qt3DRender_QRenderPassFilter_matchAny_to_output(const Qt3DRender::QRenderPassFilter* this_ptr, QVector< Qt3DRender::QFilterKey* >* output) {
  new(output) QVector< Qt3DRender::QFilterKey* >(this_ptr->matchAny());
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QRenderPassFilter_metaObject(const Qt3DRender::QRenderPassFilter* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QRenderPassFilter* qt_3d_render_c_Qt3DRender_QRenderPassFilter_new_no_args() {
  return new Qt3DRender::QRenderPassFilter();
}

Qt3DRender::QRenderPassFilter* qt_3d_render_c_Qt3DRender_QRenderPassFilter_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QRenderPassFilter(parent);
}

void qt_3d_render_c_Qt3DRender_QRenderPassFilter_parameters_to_output(const Qt3DRender::QRenderPassFilter* this_ptr, QVector< Qt3DRender::QParameter* >* output) {
  new(output) QVector< Qt3DRender::QParameter* >(this_ptr->parameters());
}

int qt_3d_render_c_Qt3DRender_QRenderPassFilter_qt_metacall(Qt3DRender::QRenderPassFilter* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QRenderPassFilter_qt_metacast(Qt3DRender::QRenderPassFilter* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QRenderPassFilter_removeMatch(Qt3DRender::QRenderPassFilter* this_ptr, Qt3DRender::QFilterKey* filterKey) {
  this_ptr->removeMatch(filterKey);
}

void qt_3d_render_c_Qt3DRender_QRenderPassFilter_removeParameter(Qt3DRender::QRenderPassFilter* this_ptr, Qt3DRender::QParameter* parameter) {
  this_ptr->removeParameter(parameter);
}

void qt_3d_render_c_Qt3DRender_QRenderPassFilter_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderPassFilter::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QRenderPassFilter_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderPassFilter::tr(s, c, n));
}

