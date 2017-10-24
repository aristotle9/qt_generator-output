#include "qt_3d_render_c_QMaterial.h"

QObject* qt_3d_render_c_QMaterial_G_static_cast_QObject_ptr(Qt3DRender::QMaterial* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_QMaterial_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QMaterial* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QMaterial_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QMaterial* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QMaterial* qt_3d_render_c_QMaterial_G_static_cast_Qt3DRender_QMaterial_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QMaterial*>(ptr);
}

Qt3DRender::QMaterial* qt_3d_render_c_QMaterial_G_static_cast_Qt3DRender_QMaterial_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DRender::QMaterial*>(ptr);
}

Qt3DRender::QMaterial* qt_3d_render_c_QMaterial_G_static_cast_Qt3DRender_QMaterial_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QMaterial*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QMaterial_addParameter(Qt3DRender::QMaterial* this_ptr, Qt3DRender::QParameter* parameter) {
  this_ptr->addParameter(parameter);
}

void qt_3d_render_c_Qt3DRender_QMaterial_delete(Qt3DRender::QMaterial* this_ptr) {
  delete this_ptr;
}

Qt3DRender::QEffect* qt_3d_render_c_Qt3DRender_QMaterial_effect(const Qt3DRender::QMaterial* this_ptr) {
  return this_ptr->effect();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QMaterial_metaObject(const Qt3DRender::QMaterial* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QMaterial* qt_3d_render_c_Qt3DRender_QMaterial_new_no_args() {
  return new Qt3DRender::QMaterial();
}

Qt3DRender::QMaterial* qt_3d_render_c_Qt3DRender_QMaterial_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QMaterial(parent);
}

void qt_3d_render_c_Qt3DRender_QMaterial_parameters_to_output(const Qt3DRender::QMaterial* this_ptr, QVector< Qt3DRender::QParameter* >* output) {
  new(output) QVector< Qt3DRender::QParameter* >(this_ptr->parameters());
}

int qt_3d_render_c_Qt3DRender_QMaterial_qt_metacall(Qt3DRender::QMaterial* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QMaterial_qt_metacast(Qt3DRender::QMaterial* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QMaterial_removeParameter(Qt3DRender::QMaterial* this_ptr, Qt3DRender::QParameter* parameter) {
  this_ptr->removeParameter(parameter);
}

void qt_3d_render_c_Qt3DRender_QMaterial_setEffect(Qt3DRender::QMaterial* this_ptr, Qt3DRender::QEffect* effect) {
  this_ptr->setEffect(effect);
}

void qt_3d_render_c_Qt3DRender_QMaterial_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QMaterial::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QMaterial_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QMaterial::tr(s, c, n));
}

