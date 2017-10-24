#include "qt_3d_extras_c_QNormalDiffuseSpecularMapMaterial.h"

QObject* qt_3d_extras_c_QNormalDiffuseSpecularMapMaterial_G_static_cast_QObject_ptr(Qt3DExtras::QNormalDiffuseSpecularMapMaterial* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_extras_c_QNormalDiffuseSpecularMapMaterial_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DExtras::QNormalDiffuseSpecularMapMaterial* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QNormalDiffuseSpecularMapMaterial_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QNormalDiffuseSpecularMapMaterial* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QNormalDiffuseSpecularMapMaterial* qt_3d_extras_c_QNormalDiffuseSpecularMapMaterial_G_static_cast_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QNormalDiffuseSpecularMapMaterial*>(ptr);
}

Qt3DExtras::QNormalDiffuseSpecularMapMaterial* qt_3d_extras_c_QNormalDiffuseSpecularMapMaterial_G_static_cast_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DExtras::QNormalDiffuseSpecularMapMaterial*>(ptr);
}

Qt3DExtras::QNormalDiffuseSpecularMapMaterial* qt_3d_extras_c_QNormalDiffuseSpecularMapMaterial_G_static_cast_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QNormalDiffuseSpecularMapMaterial*>(ptr);
}

Qt3DExtras::QNormalDiffuseSpecularMapMaterial* qt_3d_extras_c_QNormalDiffuseSpecularMapMaterial_G_static_cast_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_ptr_Qt3DRender_QMaterial(Qt3DRender::QMaterial* ptr) {
  return static_cast<Qt3DExtras::QNormalDiffuseSpecularMapMaterial*>(ptr);
}

Qt3DRender::QMaterial* qt_3d_extras_c_QNormalDiffuseSpecularMapMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(Qt3DExtras::QNormalDiffuseSpecularMapMaterial* ptr) {
  return static_cast<Qt3DRender::QMaterial*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_ambient_to_output(const Qt3DExtras::QNormalDiffuseSpecularMapMaterial* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->ambient());
}

void qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_delete(Qt3DExtras::QNormalDiffuseSpecularMapMaterial* this_ptr) {
  delete this_ptr;
}

Qt3DRender::QAbstractTexture* qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_diffuse(const Qt3DExtras::QNormalDiffuseSpecularMapMaterial* this_ptr) {
  return this_ptr->diffuse();
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_metaObject(const Qt3DExtras::QNormalDiffuseSpecularMapMaterial* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QNormalDiffuseSpecularMapMaterial* qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_new_no_args() {
  return new Qt3DExtras::QNormalDiffuseSpecularMapMaterial();
}

Qt3DExtras::QNormalDiffuseSpecularMapMaterial* qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QNormalDiffuseSpecularMapMaterial(parent);
}

Qt3DRender::QAbstractTexture* qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_normal(const Qt3DExtras::QNormalDiffuseSpecularMapMaterial* this_ptr) {
  return this_ptr->normal();
}

int qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_qt_metacall(Qt3DExtras::QNormalDiffuseSpecularMapMaterial* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_qt_metacast(Qt3DExtras::QNormalDiffuseSpecularMapMaterial* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_setAmbient(Qt3DExtras::QNormalDiffuseSpecularMapMaterial* this_ptr, const QColor* ambient) {
  this_ptr->setAmbient(*ambient);
}

void qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_setDiffuse(Qt3DExtras::QNormalDiffuseSpecularMapMaterial* this_ptr, Qt3DRender::QAbstractTexture* diffuse) {
  this_ptr->setDiffuse(diffuse);
}

void qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_setNormal(Qt3DExtras::QNormalDiffuseSpecularMapMaterial* this_ptr, Qt3DRender::QAbstractTexture* normal) {
  this_ptr->setNormal(normal);
}

void qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_setShininess(Qt3DExtras::QNormalDiffuseSpecularMapMaterial* this_ptr, float shininess) {
  this_ptr->setShininess(shininess);
}

void qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_setSpecular(Qt3DExtras::QNormalDiffuseSpecularMapMaterial* this_ptr, Qt3DRender::QAbstractTexture* specular) {
  this_ptr->setSpecular(specular);
}

void qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_setTextureScale(Qt3DExtras::QNormalDiffuseSpecularMapMaterial* this_ptr, float textureScale) {
  this_ptr->setTextureScale(textureScale);
}

float qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_shininess(const Qt3DExtras::QNormalDiffuseSpecularMapMaterial* this_ptr) {
  return this_ptr->shininess();
}

Qt3DRender::QAbstractTexture* qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_specular(const Qt3DExtras::QNormalDiffuseSpecularMapMaterial* this_ptr) {
  return this_ptr->specular();
}

float qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_textureScale(const Qt3DExtras::QNormalDiffuseSpecularMapMaterial* this_ptr) {
  return this_ptr->textureScale();
}

void qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QNormalDiffuseSpecularMapMaterial::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QNormalDiffuseSpecularMapMaterial_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QNormalDiffuseSpecularMapMaterial::tr(s, c, n));
}

