#include "qt_3d_extras_c_QDiffuseMapMaterial.h"

QObject* qt_3d_extras_c_QDiffuseMapMaterial_G_static_cast_QObject_ptr(Qt3DExtras::QDiffuseMapMaterial* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_extras_c_QDiffuseMapMaterial_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DExtras::QDiffuseMapMaterial* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QDiffuseMapMaterial_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QDiffuseMapMaterial* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QDiffuseMapMaterial* qt_3d_extras_c_QDiffuseMapMaterial_G_static_cast_Qt3DExtras_QDiffuseMapMaterial_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QDiffuseMapMaterial*>(ptr);
}

Qt3DExtras::QDiffuseMapMaterial* qt_3d_extras_c_QDiffuseMapMaterial_G_static_cast_Qt3DExtras_QDiffuseMapMaterial_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DExtras::QDiffuseMapMaterial*>(ptr);
}

Qt3DExtras::QDiffuseMapMaterial* qt_3d_extras_c_QDiffuseMapMaterial_G_static_cast_Qt3DExtras_QDiffuseMapMaterial_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QDiffuseMapMaterial*>(ptr);
}

Qt3DExtras::QDiffuseMapMaterial* qt_3d_extras_c_QDiffuseMapMaterial_G_static_cast_Qt3DExtras_QDiffuseMapMaterial_ptr_Qt3DRender_QMaterial(Qt3DRender::QMaterial* ptr) {
  return static_cast<Qt3DExtras::QDiffuseMapMaterial*>(ptr);
}

Qt3DRender::QMaterial* qt_3d_extras_c_QDiffuseMapMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(Qt3DExtras::QDiffuseMapMaterial* ptr) {
  return static_cast<Qt3DRender::QMaterial*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_ambient_to_output(const Qt3DExtras::QDiffuseMapMaterial* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->ambient());
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_delete(Qt3DExtras::QDiffuseMapMaterial* this_ptr) {
  delete this_ptr;
}

Qt3DRender::QAbstractTexture* qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_diffuse(const Qt3DExtras::QDiffuseMapMaterial* this_ptr) {
  return this_ptr->diffuse();
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_metaObject(const Qt3DExtras::QDiffuseMapMaterial* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QDiffuseMapMaterial* qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_new_no_args() {
  return new Qt3DExtras::QDiffuseMapMaterial();
}

Qt3DExtras::QDiffuseMapMaterial* qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QDiffuseMapMaterial(parent);
}

int qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_qt_metacall(Qt3DExtras::QDiffuseMapMaterial* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_qt_metacast(Qt3DExtras::QDiffuseMapMaterial* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_setAmbient(Qt3DExtras::QDiffuseMapMaterial* this_ptr, const QColor* color) {
  this_ptr->setAmbient(*color);
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_setDiffuse(Qt3DExtras::QDiffuseMapMaterial* this_ptr, Qt3DRender::QAbstractTexture* diffuse) {
  this_ptr->setDiffuse(diffuse);
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_setShininess(Qt3DExtras::QDiffuseMapMaterial* this_ptr, float shininess) {
  this_ptr->setShininess(shininess);
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_setSpecular(Qt3DExtras::QDiffuseMapMaterial* this_ptr, const QColor* specular) {
  this_ptr->setSpecular(*specular);
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_setTextureScale(Qt3DExtras::QDiffuseMapMaterial* this_ptr, float textureScale) {
  this_ptr->setTextureScale(textureScale);
}

float qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_shininess(const Qt3DExtras::QDiffuseMapMaterial* this_ptr) {
  return this_ptr->shininess();
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_specular_to_output(const Qt3DExtras::QDiffuseMapMaterial* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->specular());
}

float qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_textureScale(const Qt3DExtras::QDiffuseMapMaterial* this_ptr) {
  return this_ptr->textureScale();
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QDiffuseMapMaterial::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QDiffuseMapMaterial_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QDiffuseMapMaterial::tr(s, c, n));
}

