#include "qt_3d_render_c_QTechniqueFilter.h"

Qt3DRender::QTechniqueFilter* qt_3d_render_c_QTechniqueFilter_G_dynamic_cast_Qt3DRender_QTechniqueFilter_ptr(Qt3DRender::QFrameGraphNode* ptr) {
  return dynamic_cast<Qt3DRender::QTechniqueFilter*>(ptr);
}

QObject* qt_3d_render_c_QTechniqueFilter_G_static_cast_QObject_ptr(Qt3DRender::QTechniqueFilter* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QTechniqueFilter_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QTechniqueFilter* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_QTechniqueFilter_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QTechniqueFilter* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNode*>(ptr);
}

Qt3DRender::QTechniqueFilter* qt_3d_render_c_QTechniqueFilter_G_static_cast_Qt3DRender_QTechniqueFilter_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QTechniqueFilter*>(ptr);
}

Qt3DRender::QTechniqueFilter* qt_3d_render_c_QTechniqueFilter_G_static_cast_Qt3DRender_QTechniqueFilter_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QTechniqueFilter*>(ptr);
}

Qt3DRender::QTechniqueFilter* qt_3d_render_c_QTechniqueFilter_G_static_cast_Qt3DRender_QTechniqueFilter_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr) {
  return static_cast<Qt3DRender::QTechniqueFilter*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QTechniqueFilter_addMatch(Qt3DRender::QTechniqueFilter* this_ptr, Qt3DRender::QFilterKey* filterKey) {
  this_ptr->addMatch(filterKey);
}

void qt_3d_render_c_Qt3DRender_QTechniqueFilter_addParameter(Qt3DRender::QTechniqueFilter* this_ptr, Qt3DRender::QParameter* p) {
  this_ptr->addParameter(p);
}

void qt_3d_render_c_Qt3DRender_QTechniqueFilter_delete(Qt3DRender::QTechniqueFilter* this_ptr) {
  delete this_ptr;
}

void qt_3d_render_c_Qt3DRender_QTechniqueFilter_matchAll_to_output(const Qt3DRender::QTechniqueFilter* this_ptr, QVector< Qt3DRender::QFilterKey* >* output) {
  new(output) QVector< Qt3DRender::QFilterKey* >(this_ptr->matchAll());
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QTechniqueFilter_metaObject(const Qt3DRender::QTechniqueFilter* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QTechniqueFilter* qt_3d_render_c_Qt3DRender_QTechniqueFilter_new_no_args() {
  return new Qt3DRender::QTechniqueFilter();
}

Qt3DRender::QTechniqueFilter* qt_3d_render_c_Qt3DRender_QTechniqueFilter_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QTechniqueFilter(parent);
}

void qt_3d_render_c_Qt3DRender_QTechniqueFilter_parameters_to_output(const Qt3DRender::QTechniqueFilter* this_ptr, QVector< Qt3DRender::QParameter* >* output) {
  new(output) QVector< Qt3DRender::QParameter* >(this_ptr->parameters());
}

int qt_3d_render_c_Qt3DRender_QTechniqueFilter_qt_metacall(Qt3DRender::QTechniqueFilter* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QTechniqueFilter_qt_metacast(Qt3DRender::QTechniqueFilter* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QTechniqueFilter_removeMatch(Qt3DRender::QTechniqueFilter* this_ptr, Qt3DRender::QFilterKey* filterKey) {
  this_ptr->removeMatch(filterKey);
}

void qt_3d_render_c_Qt3DRender_QTechniqueFilter_removeParameter(Qt3DRender::QTechniqueFilter* this_ptr, Qt3DRender::QParameter* p) {
  this_ptr->removeParameter(p);
}

void qt_3d_render_c_Qt3DRender_QTechniqueFilter_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QTechniqueFilter::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QTechniqueFilter_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QTechniqueFilter::tr(s, c, n));
}

