#include "qt_3d_extras_c_QDiffuseSpecularMapMaterial.h"

QObject* qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_QObject_ptr(Qt3DExtras::QDiffuseSpecularMapMaterial* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DExtras::QDiffuseSpecularMapMaterial* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QDiffuseSpecularMapMaterial* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QDiffuseSpecularMapMaterial* qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DExtras_QDiffuseSpecularMapMaterial_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QDiffuseSpecularMapMaterial*>(ptr);
}

Qt3DExtras::QDiffuseSpecularMapMaterial* qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DExtras_QDiffuseSpecularMapMaterial_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DExtras::QDiffuseSpecularMapMaterial*>(ptr);
}

Qt3DExtras::QDiffuseSpecularMapMaterial* qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DExtras_QDiffuseSpecularMapMaterial_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QDiffuseSpecularMapMaterial*>(ptr);
}

Qt3DExtras::QDiffuseSpecularMapMaterial* qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DExtras_QDiffuseSpecularMapMaterial_ptr_Qt3DRender_QMaterial(Qt3DRender::QMaterial* ptr) {
  return static_cast<Qt3DExtras::QDiffuseSpecularMapMaterial*>(ptr);
}

Qt3DRender::QMaterial* qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(Qt3DExtras::QDiffuseSpecularMapMaterial* ptr) {
  return static_cast<Qt3DRender::QMaterial*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_ambient_to_output(const Qt3DExtras::QDiffuseSpecularMapMaterial* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->ambient());
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_delete(Qt3DExtras::QDiffuseSpecularMapMaterial* this_ptr) {
  delete this_ptr;
}

Qt3DRender::QAbstractTexture* qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_diffuse(const Qt3DExtras::QDiffuseSpecularMapMaterial* this_ptr) {
  return this_ptr->diffuse();
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_metaObject(const Qt3DExtras::QDiffuseSpecularMapMaterial* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QDiffuseSpecularMapMaterial* qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_new_no_args() {
  return new Qt3DExtras::QDiffuseSpecularMapMaterial();
}

Qt3DExtras::QDiffuseSpecularMapMaterial* qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QDiffuseSpecularMapMaterial(parent);
}

int qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_qt_metacall(Qt3DExtras::QDiffuseSpecularMapMaterial* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_qt_metacast(Qt3DExtras::QDiffuseSpecularMapMaterial* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_setAmbient(Qt3DExtras::QDiffuseSpecularMapMaterial* this_ptr, const QColor* ambient) {
  this_ptr->setAmbient(*ambient);
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_setDiffuse(Qt3DExtras::QDiffuseSpecularMapMaterial* this_ptr, Qt3DRender::QAbstractTexture* diffuse) {
  this_ptr->setDiffuse(diffuse);
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_setShininess(Qt3DExtras::QDiffuseSpecularMapMaterial* this_ptr, float shininess) {
  this_ptr->setShininess(shininess);
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_setSpecular(Qt3DExtras::QDiffuseSpecularMapMaterial* this_ptr, Qt3DRender::QAbstractTexture* specular) {
  this_ptr->setSpecular(specular);
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_setTextureScale(Qt3DExtras::QDiffuseSpecularMapMaterial* this_ptr, float textureScale) {
  this_ptr->setTextureScale(textureScale);
}

float qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_shininess(const Qt3DExtras::QDiffuseSpecularMapMaterial* this_ptr) {
  return this_ptr->shininess();
}

Qt3DRender::QAbstractTexture* qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_specular(const Qt3DExtras::QDiffuseSpecularMapMaterial* this_ptr) {
  return this_ptr->specular();
}

float qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_textureScale(const Qt3DExtras::QDiffuseSpecularMapMaterial* this_ptr) {
  return this_ptr->textureScale();
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QDiffuseSpecularMapMaterial::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QDiffuseSpecularMapMaterial::tr(s, c, n));
}

