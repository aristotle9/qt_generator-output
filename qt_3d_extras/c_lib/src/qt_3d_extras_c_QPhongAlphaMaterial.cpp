#include "qt_3d_extras_c_QPhongAlphaMaterial.h"

QObject* qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_QObject_ptr(Qt3DExtras::QPhongAlphaMaterial* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DExtras::QPhongAlphaMaterial* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QPhongAlphaMaterial* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QPhongAlphaMaterial* qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DExtras_QPhongAlphaMaterial_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QPhongAlphaMaterial*>(ptr);
}

Qt3DExtras::QPhongAlphaMaterial* qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DExtras_QPhongAlphaMaterial_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DExtras::QPhongAlphaMaterial*>(ptr);
}

Qt3DExtras::QPhongAlphaMaterial* qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DExtras_QPhongAlphaMaterial_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QPhongAlphaMaterial*>(ptr);
}

Qt3DExtras::QPhongAlphaMaterial* qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DExtras_QPhongAlphaMaterial_ptr_Qt3DRender_QMaterial(Qt3DRender::QMaterial* ptr) {
  return static_cast<Qt3DExtras::QPhongAlphaMaterial*>(ptr);
}

Qt3DRender::QMaterial* qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(Qt3DExtras::QPhongAlphaMaterial* ptr) {
  return static_cast<Qt3DRender::QMaterial*>(ptr);
}

float qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_alpha(const Qt3DExtras::QPhongAlphaMaterial* this_ptr) {
  return this_ptr->alpha();
}

void qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_ambient_to_output(const Qt3DExtras::QPhongAlphaMaterial* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->ambient());
}

void qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_delete(Qt3DExtras::QPhongAlphaMaterial* this_ptr) {
  delete this_ptr;
}

void qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_diffuse_to_output(const Qt3DExtras::QPhongAlphaMaterial* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->diffuse());
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_metaObject(const Qt3DExtras::QPhongAlphaMaterial* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QPhongAlphaMaterial* qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_new_no_args() {
  return new Qt3DExtras::QPhongAlphaMaterial();
}

Qt3DExtras::QPhongAlphaMaterial* qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QPhongAlphaMaterial(parent);
}

int qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_qt_metacall(Qt3DExtras::QPhongAlphaMaterial* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_qt_metacast(Qt3DExtras::QPhongAlphaMaterial* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setAlpha(Qt3DExtras::QPhongAlphaMaterial* this_ptr, float alpha) {
  this_ptr->setAlpha(alpha);
}

void qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setAmbient(Qt3DExtras::QPhongAlphaMaterial* this_ptr, const QColor* ambient) {
  this_ptr->setAmbient(*ambient);
}

void qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setBlendFunctionArg(Qt3DExtras::QPhongAlphaMaterial* this_ptr, const Qt3DRender::QBlendEquation::BlendFunction* blendFunctionArg) {
  this_ptr->setBlendFunctionArg(*blendFunctionArg);
}

void qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setDestinationAlphaArg(Qt3DExtras::QPhongAlphaMaterial* this_ptr, const Qt3DRender::QBlendEquationArguments::Blending* destinationAlphaArg) {
  this_ptr->setDestinationAlphaArg(*destinationAlphaArg);
}

void qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setDestinationRgbArg(Qt3DExtras::QPhongAlphaMaterial* this_ptr, const Qt3DRender::QBlendEquationArguments::Blending* destinationRgbArg) {
  this_ptr->setDestinationRgbArg(*destinationRgbArg);
}

void qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setDiffuse(Qt3DExtras::QPhongAlphaMaterial* this_ptr, const QColor* diffuse) {
  this_ptr->setDiffuse(*diffuse);
}

void qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setShininess(Qt3DExtras::QPhongAlphaMaterial* this_ptr, float shininess) {
  this_ptr->setShininess(shininess);
}

void qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setSourceAlphaArg(Qt3DExtras::QPhongAlphaMaterial* this_ptr, const Qt3DRender::QBlendEquationArguments::Blending* sourceAlphaArg) {
  this_ptr->setSourceAlphaArg(*sourceAlphaArg);
}

void qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setSourceRgbArg(Qt3DExtras::QPhongAlphaMaterial* this_ptr, const Qt3DRender::QBlendEquationArguments::Blending* sourceRgbArg) {
  this_ptr->setSourceRgbArg(*sourceRgbArg);
}

void qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setSpecular(Qt3DExtras::QPhongAlphaMaterial* this_ptr, const QColor* specular) {
  this_ptr->setSpecular(*specular);
}

float qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_shininess(const Qt3DExtras::QPhongAlphaMaterial* this_ptr) {
  return this_ptr->shininess();
}

void qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_specular_to_output(const Qt3DExtras::QPhongAlphaMaterial* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->specular());
}

void qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QPhongAlphaMaterial::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QPhongAlphaMaterial::tr(s, c, n));
}

