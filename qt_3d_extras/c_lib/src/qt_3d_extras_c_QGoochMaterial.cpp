#include "qt_3d_extras_c_QGoochMaterial.h"

QObject* qt_3d_extras_c_QGoochMaterial_G_static_cast_QObject_ptr(Qt3DExtras::QGoochMaterial* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DExtras::QGoochMaterial* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QGoochMaterial* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QGoochMaterial* qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DExtras_QGoochMaterial_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QGoochMaterial*>(ptr);
}

Qt3DExtras::QGoochMaterial* qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DExtras_QGoochMaterial_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DExtras::QGoochMaterial*>(ptr);
}

Qt3DExtras::QGoochMaterial* qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DExtras_QGoochMaterial_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QGoochMaterial*>(ptr);
}

Qt3DExtras::QGoochMaterial* qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DExtras_QGoochMaterial_ptr_Qt3DRender_QMaterial(Qt3DRender::QMaterial* ptr) {
  return static_cast<Qt3DExtras::QGoochMaterial*>(ptr);
}

Qt3DRender::QMaterial* qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(Qt3DExtras::QGoochMaterial* ptr) {
  return static_cast<Qt3DRender::QMaterial*>(ptr);
}

float qt_3d_extras_c_Qt3DExtras_QGoochMaterial_alpha(const Qt3DExtras::QGoochMaterial* this_ptr) {
  return this_ptr->alpha();
}

float qt_3d_extras_c_Qt3DExtras_QGoochMaterial_beta(const Qt3DExtras::QGoochMaterial* this_ptr) {
  return this_ptr->beta();
}

void qt_3d_extras_c_Qt3DExtras_QGoochMaterial_cool_to_output(const Qt3DExtras::QGoochMaterial* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->cool());
}

void qt_3d_extras_c_Qt3DExtras_QGoochMaterial_delete(Qt3DExtras::QGoochMaterial* this_ptr) {
  delete this_ptr;
}

void qt_3d_extras_c_Qt3DExtras_QGoochMaterial_diffuse_to_output(const Qt3DExtras::QGoochMaterial* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->diffuse());
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QGoochMaterial_metaObject(const Qt3DExtras::QGoochMaterial* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QGoochMaterial* qt_3d_extras_c_Qt3DExtras_QGoochMaterial_new_no_args() {
  return new Qt3DExtras::QGoochMaterial();
}

Qt3DExtras::QGoochMaterial* qt_3d_extras_c_Qt3DExtras_QGoochMaterial_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QGoochMaterial(parent);
}

int qt_3d_extras_c_Qt3DExtras_QGoochMaterial_qt_metacall(Qt3DExtras::QGoochMaterial* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QGoochMaterial_qt_metacast(Qt3DExtras::QGoochMaterial* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QGoochMaterial_setAlpha(Qt3DExtras::QGoochMaterial* this_ptr, float alpha) {
  this_ptr->setAlpha(alpha);
}

void qt_3d_extras_c_Qt3DExtras_QGoochMaterial_setBeta(Qt3DExtras::QGoochMaterial* this_ptr, float beta) {
  this_ptr->setBeta(beta);
}

void qt_3d_extras_c_Qt3DExtras_QGoochMaterial_setCool(Qt3DExtras::QGoochMaterial* this_ptr, const QColor* cool) {
  this_ptr->setCool(*cool);
}

void qt_3d_extras_c_Qt3DExtras_QGoochMaterial_setDiffuse(Qt3DExtras::QGoochMaterial* this_ptr, const QColor* diffuse) {
  this_ptr->setDiffuse(*diffuse);
}

void qt_3d_extras_c_Qt3DExtras_QGoochMaterial_setShininess(Qt3DExtras::QGoochMaterial* this_ptr, float shininess) {
  this_ptr->setShininess(shininess);
}

void qt_3d_extras_c_Qt3DExtras_QGoochMaterial_setSpecular(Qt3DExtras::QGoochMaterial* this_ptr, const QColor* specular) {
  this_ptr->setSpecular(*specular);
}

void qt_3d_extras_c_Qt3DExtras_QGoochMaterial_setWarm(Qt3DExtras::QGoochMaterial* this_ptr, const QColor* warm) {
  this_ptr->setWarm(*warm);
}

float qt_3d_extras_c_Qt3DExtras_QGoochMaterial_shininess(const Qt3DExtras::QGoochMaterial* this_ptr) {
  return this_ptr->shininess();
}

void qt_3d_extras_c_Qt3DExtras_QGoochMaterial_specular_to_output(const Qt3DExtras::QGoochMaterial* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->specular());
}

void qt_3d_extras_c_Qt3DExtras_QGoochMaterial_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QGoochMaterial::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QGoochMaterial_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QGoochMaterial::tr(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QGoochMaterial_warm_to_output(const Qt3DExtras::QGoochMaterial* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->warm());
}

