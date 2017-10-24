#include "qt_3d_render_c_QEffect.h"

QObject* qt_3d_render_c_QEffect_G_static_cast_QObject_ptr(Qt3DRender::QEffect* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QEffect_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QEffect* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QEffect* qt_3d_render_c_QEffect_G_static_cast_Qt3DRender_QEffect_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QEffect*>(ptr);
}

Qt3DRender::QEffect* qt_3d_render_c_QEffect_G_static_cast_Qt3DRender_QEffect_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QEffect*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QEffect_addParameter(Qt3DRender::QEffect* this_ptr, Qt3DRender::QParameter* parameter) {
  this_ptr->addParameter(parameter);
}

void qt_3d_render_c_Qt3DRender_QEffect_addTechnique(Qt3DRender::QEffect* this_ptr, Qt3DRender::QTechnique* t) {
  this_ptr->addTechnique(t);
}

void qt_3d_render_c_Qt3DRender_QEffect_delete(Qt3DRender::QEffect* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QEffect_metaObject(const Qt3DRender::QEffect* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QEffect* qt_3d_render_c_Qt3DRender_QEffect_new_no_args() {
  return new Qt3DRender::QEffect();
}

Qt3DRender::QEffect* qt_3d_render_c_Qt3DRender_QEffect_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QEffect(parent);
}

void qt_3d_render_c_Qt3DRender_QEffect_parameters_to_output(const Qt3DRender::QEffect* this_ptr, QVector< Qt3DRender::QParameter* >* output) {
  new(output) QVector< Qt3DRender::QParameter* >(this_ptr->parameters());
}

int qt_3d_render_c_Qt3DRender_QEffect_qt_metacall(Qt3DRender::QEffect* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QEffect_qt_metacast(Qt3DRender::QEffect* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QEffect_removeParameter(Qt3DRender::QEffect* this_ptr, Qt3DRender::QParameter* parameter) {
  this_ptr->removeParameter(parameter);
}

void qt_3d_render_c_Qt3DRender_QEffect_removeTechnique(Qt3DRender::QEffect* this_ptr, Qt3DRender::QTechnique* t) {
  this_ptr->removeTechnique(t);
}

void qt_3d_render_c_Qt3DRender_QEffect_techniques_to_output(const Qt3DRender::QEffect* this_ptr, QVector< Qt3DRender::QTechnique* >* output) {
  new(output) QVector< Qt3DRender::QTechnique* >(this_ptr->techniques());
}

void qt_3d_render_c_Qt3DRender_QEffect_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QEffect::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QEffect_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QEffect::tr(s, c, n));
}

