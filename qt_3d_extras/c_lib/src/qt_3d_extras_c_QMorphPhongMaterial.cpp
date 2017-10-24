#include "qt_3d_extras_c_QMorphPhongMaterial.h"

QObject* qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_QObject_ptr(Qt3DExtras::QMorphPhongMaterial* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DExtras::QMorphPhongMaterial* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QMorphPhongMaterial* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QMorphPhongMaterial* qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DExtras_QMorphPhongMaterial_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QMorphPhongMaterial*>(ptr);
}

Qt3DExtras::QMorphPhongMaterial* qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DExtras_QMorphPhongMaterial_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DExtras::QMorphPhongMaterial*>(ptr);
}

Qt3DExtras::QMorphPhongMaterial* qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DExtras_QMorphPhongMaterial_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QMorphPhongMaterial*>(ptr);
}

Qt3DExtras::QMorphPhongMaterial* qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DExtras_QMorphPhongMaterial_ptr_Qt3DRender_QMaterial(Qt3DRender::QMaterial* ptr) {
  return static_cast<Qt3DExtras::QMorphPhongMaterial*>(ptr);
}

Qt3DRender::QMaterial* qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(Qt3DExtras::QMorphPhongMaterial* ptr) {
  return static_cast<Qt3DRender::QMaterial*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_ambient_to_output(const Qt3DExtras::QMorphPhongMaterial* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->ambient());
}

void qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_delete(Qt3DExtras::QMorphPhongMaterial* this_ptr) {
  delete this_ptr;
}

void qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_diffuse_to_output(const Qt3DExtras::QMorphPhongMaterial* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->diffuse());
}

float qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_interpolator(const Qt3DExtras::QMorphPhongMaterial* this_ptr) {
  return this_ptr->interpolator();
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_metaObject(const Qt3DExtras::QMorphPhongMaterial* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QMorphPhongMaterial* qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_new_no_args() {
  return new Qt3DExtras::QMorphPhongMaterial();
}

Qt3DExtras::QMorphPhongMaterial* qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QMorphPhongMaterial(parent);
}

int qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_qt_metacall(Qt3DExtras::QMorphPhongMaterial* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_qt_metacast(Qt3DExtras::QMorphPhongMaterial* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_setAmbient(Qt3DExtras::QMorphPhongMaterial* this_ptr, const QColor* ambient) {
  this_ptr->setAmbient(*ambient);
}

void qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_setDiffuse(Qt3DExtras::QMorphPhongMaterial* this_ptr, const QColor* diffuse) {
  this_ptr->setDiffuse(*diffuse);
}

void qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_setInterpolator(Qt3DExtras::QMorphPhongMaterial* this_ptr, float interpolator) {
  this_ptr->setInterpolator(interpolator);
}

void qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_setShininess(Qt3DExtras::QMorphPhongMaterial* this_ptr, float shininess) {
  this_ptr->setShininess(shininess);
}

void qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_setSpecular(Qt3DExtras::QMorphPhongMaterial* this_ptr, const QColor* specular) {
  this_ptr->setSpecular(*specular);
}

float qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_shininess(const Qt3DExtras::QMorphPhongMaterial* this_ptr) {
  return this_ptr->shininess();
}

void qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_specular_to_output(const Qt3DExtras::QMorphPhongMaterial* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->specular());
}

void qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QMorphPhongMaterial::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QMorphPhongMaterial::tr(s, c, n));
}

