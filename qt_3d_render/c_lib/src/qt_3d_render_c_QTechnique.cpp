#include "qt_3d_render_c_QTechnique.h"

QObject* qt_3d_render_c_QTechnique_G_static_cast_QObject_ptr(Qt3DRender::QTechnique* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QTechnique_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QTechnique* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QTechnique* qt_3d_render_c_QTechnique_G_static_cast_Qt3DRender_QTechnique_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QTechnique*>(ptr);
}

Qt3DRender::QTechnique* qt_3d_render_c_QTechnique_G_static_cast_Qt3DRender_QTechnique_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QTechnique*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QTechnique_addFilterKey(Qt3DRender::QTechnique* this_ptr, Qt3DRender::QFilterKey* filterKey) {
  this_ptr->addFilterKey(filterKey);
}

void qt_3d_render_c_Qt3DRender_QTechnique_addParameter(Qt3DRender::QTechnique* this_ptr, Qt3DRender::QParameter* p) {
  this_ptr->addParameter(p);
}

void qt_3d_render_c_Qt3DRender_QTechnique_addRenderPass(Qt3DRender::QTechnique* this_ptr, Qt3DRender::QRenderPass* pass) {
  this_ptr->addRenderPass(pass);
}

void qt_3d_render_c_Qt3DRender_QTechnique_delete(Qt3DRender::QTechnique* this_ptr) {
  delete this_ptr;
}

void qt_3d_render_c_Qt3DRender_QTechnique_filterKeys_to_output(const Qt3DRender::QTechnique* this_ptr, QVector< Qt3DRender::QFilterKey* >* output) {
  new(output) QVector< Qt3DRender::QFilterKey* >(this_ptr->filterKeys());
}

Qt3DRender::QGraphicsApiFilter* qt_3d_render_c_Qt3DRender_QTechnique_graphicsApiFilter(Qt3DRender::QTechnique* this_ptr) {
  return this_ptr->graphicsApiFilter();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QTechnique_metaObject(const Qt3DRender::QTechnique* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QTechnique* qt_3d_render_c_Qt3DRender_QTechnique_new_no_args() {
  return new Qt3DRender::QTechnique();
}

Qt3DRender::QTechnique* qt_3d_render_c_Qt3DRender_QTechnique_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QTechnique(parent);
}

void qt_3d_render_c_Qt3DRender_QTechnique_parameters_to_output(const Qt3DRender::QTechnique* this_ptr, QVector< Qt3DRender::QParameter* >* output) {
  new(output) QVector< Qt3DRender::QParameter* >(this_ptr->parameters());
}

int qt_3d_render_c_Qt3DRender_QTechnique_qt_metacall(Qt3DRender::QTechnique* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QTechnique_qt_metacast(Qt3DRender::QTechnique* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QTechnique_removeFilterKey(Qt3DRender::QTechnique* this_ptr, Qt3DRender::QFilterKey* filterKey) {
  this_ptr->removeFilterKey(filterKey);
}

void qt_3d_render_c_Qt3DRender_QTechnique_removeParameter(Qt3DRender::QTechnique* this_ptr, Qt3DRender::QParameter* p) {
  this_ptr->removeParameter(p);
}

void qt_3d_render_c_Qt3DRender_QTechnique_removeRenderPass(Qt3DRender::QTechnique* this_ptr, Qt3DRender::QRenderPass* pass) {
  this_ptr->removeRenderPass(pass);
}

void qt_3d_render_c_Qt3DRender_QTechnique_renderPasses_to_output(const Qt3DRender::QTechnique* this_ptr, QVector< Qt3DRender::QRenderPass* >* output) {
  new(output) QVector< Qt3DRender::QRenderPass* >(this_ptr->renderPasses());
}

void qt_3d_render_c_Qt3DRender_QTechnique_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QTechnique::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QTechnique_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QTechnique::tr(s, c, n));
}

