#include "qt_3d_render_c_QRenderPass.h"

QObject* qt_3d_render_c_QRenderPass_G_static_cast_QObject_ptr(Qt3DRender::QRenderPass* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QRenderPass_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QRenderPass* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QRenderPass* qt_3d_render_c_QRenderPass_G_static_cast_Qt3DRender_QRenderPass_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QRenderPass*>(ptr);
}

Qt3DRender::QRenderPass* qt_3d_render_c_QRenderPass_G_static_cast_Qt3DRender_QRenderPass_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QRenderPass*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QRenderPass_addFilterKey(Qt3DRender::QRenderPass* this_ptr, Qt3DRender::QFilterKey* filterKey) {
  this_ptr->addFilterKey(filterKey);
}

void qt_3d_render_c_Qt3DRender_QRenderPass_addParameter(Qt3DRender::QRenderPass* this_ptr, Qt3DRender::QParameter* p) {
  this_ptr->addParameter(p);
}

void qt_3d_render_c_Qt3DRender_QRenderPass_addRenderState(Qt3DRender::QRenderPass* this_ptr, Qt3DRender::QRenderState* state) {
  this_ptr->addRenderState(state);
}

void qt_3d_render_c_Qt3DRender_QRenderPass_delete(Qt3DRender::QRenderPass* this_ptr) {
  delete this_ptr;
}

void qt_3d_render_c_Qt3DRender_QRenderPass_filterKeys_to_output(const Qt3DRender::QRenderPass* this_ptr, QVector< Qt3DRender::QFilterKey* >* output) {
  new(output) QVector< Qt3DRender::QFilterKey* >(this_ptr->filterKeys());
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QRenderPass_metaObject(const Qt3DRender::QRenderPass* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QRenderPass* qt_3d_render_c_Qt3DRender_QRenderPass_new_no_args() {
  return new Qt3DRender::QRenderPass();
}

Qt3DRender::QRenderPass* qt_3d_render_c_Qt3DRender_QRenderPass_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QRenderPass(parent);
}

void qt_3d_render_c_Qt3DRender_QRenderPass_parameters_to_output(const Qt3DRender::QRenderPass* this_ptr, QVector< Qt3DRender::QParameter* >* output) {
  new(output) QVector< Qt3DRender::QParameter* >(this_ptr->parameters());
}

int qt_3d_render_c_Qt3DRender_QRenderPass_qt_metacall(Qt3DRender::QRenderPass* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QRenderPass_qt_metacast(Qt3DRender::QRenderPass* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QRenderPass_removeFilterKey(Qt3DRender::QRenderPass* this_ptr, Qt3DRender::QFilterKey* filterKey) {
  this_ptr->removeFilterKey(filterKey);
}

void qt_3d_render_c_Qt3DRender_QRenderPass_removeParameter(Qt3DRender::QRenderPass* this_ptr, Qt3DRender::QParameter* p) {
  this_ptr->removeParameter(p);
}

void qt_3d_render_c_Qt3DRender_QRenderPass_removeRenderState(Qt3DRender::QRenderPass* this_ptr, Qt3DRender::QRenderState* state) {
  this_ptr->removeRenderState(state);
}

void qt_3d_render_c_Qt3DRender_QRenderPass_renderStates_to_output(const Qt3DRender::QRenderPass* this_ptr, QVector< Qt3DRender::QRenderState* >* output) {
  new(output) QVector< Qt3DRender::QRenderState* >(this_ptr->renderStates());
}

void qt_3d_render_c_Qt3DRender_QRenderPass_setShaderProgram(Qt3DRender::QRenderPass* this_ptr, Qt3DRender::QShaderProgram* shaderProgram) {
  this_ptr->setShaderProgram(shaderProgram);
}

Qt3DRender::QShaderProgram* qt_3d_render_c_Qt3DRender_QRenderPass_shaderProgram(const Qt3DRender::QRenderPass* this_ptr) {
  return this_ptr->shaderProgram();
}

void qt_3d_render_c_Qt3DRender_QRenderPass_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderPass::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QRenderPass_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderPass::tr(s, c, n));
}

