#include "qt_3d_extras_c_QMetalRoughMaterial.h"

QObject* qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_QObject_ptr(Qt3DExtras::QMetalRoughMaterial* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DExtras::QMetalRoughMaterial* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QMetalRoughMaterial* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QMetalRoughMaterial* qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DExtras_QMetalRoughMaterial_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QMetalRoughMaterial*>(ptr);
}

Qt3DExtras::QMetalRoughMaterial* qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DExtras_QMetalRoughMaterial_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DExtras::QMetalRoughMaterial*>(ptr);
}

Qt3DExtras::QMetalRoughMaterial* qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DExtras_QMetalRoughMaterial_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QMetalRoughMaterial*>(ptr);
}

Qt3DExtras::QMetalRoughMaterial* qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DExtras_QMetalRoughMaterial_ptr_Qt3DRender_QMaterial(Qt3DRender::QMaterial* ptr) {
  return static_cast<Qt3DExtras::QMetalRoughMaterial*>(ptr);
}

Qt3DRender::QMaterial* qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(Qt3DExtras::QMetalRoughMaterial* ptr) {
  return static_cast<Qt3DRender::QMaterial*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_baseColor_to_output(const Qt3DExtras::QMetalRoughMaterial* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->baseColor());
}

void qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_delete(Qt3DExtras::QMetalRoughMaterial* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_metaObject(const Qt3DExtras::QMetalRoughMaterial* this_ptr) {
  return this_ptr->metaObject();
}

float qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_metalness(const Qt3DExtras::QMetalRoughMaterial* this_ptr) {
  return this_ptr->metalness();
}

Qt3DExtras::QMetalRoughMaterial* qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_new_no_args() {
  return new Qt3DExtras::QMetalRoughMaterial();
}

Qt3DExtras::QMetalRoughMaterial* qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QMetalRoughMaterial(parent);
}

int qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_qt_metacall(Qt3DExtras::QMetalRoughMaterial* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_qt_metacast(Qt3DExtras::QMetalRoughMaterial* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

float qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_roughness(const Qt3DExtras::QMetalRoughMaterial* this_ptr) {
  return this_ptr->roughness();
}

void qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_setBaseColor(Qt3DExtras::QMetalRoughMaterial* this_ptr, const QColor* baseColor) {
  this_ptr->setBaseColor(*baseColor);
}

void qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_setMetalness(Qt3DExtras::QMetalRoughMaterial* this_ptr, float metalness) {
  this_ptr->setMetalness(metalness);
}

void qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_setRoughness(Qt3DExtras::QMetalRoughMaterial* this_ptr, float roughness) {
  this_ptr->setRoughness(roughness);
}

void qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QMetalRoughMaterial::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QMetalRoughMaterial::tr(s, c, n));
}

