#include "qt_3d_extras_c_QSkyboxEntity.h"

QObject* qt_3d_extras_c_QSkyboxEntity_G_static_cast_QObject_ptr(Qt3DExtras::QSkyboxEntity* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QEntity* qt_3d_extras_c_QSkyboxEntity_G_static_cast_Qt3DCore_QEntity_ptr(Qt3DExtras::QSkyboxEntity* ptr) {
  return static_cast<Qt3DCore::QEntity*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QSkyboxEntity_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QSkyboxEntity* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QSkyboxEntity* qt_3d_extras_c_QSkyboxEntity_G_static_cast_Qt3DExtras_QSkyboxEntity_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QSkyboxEntity*>(ptr);
}

Qt3DExtras::QSkyboxEntity* qt_3d_extras_c_QSkyboxEntity_G_static_cast_Qt3DExtras_QSkyboxEntity_ptr_Qt3DCore_QEntity(Qt3DCore::QEntity* ptr) {
  return static_cast<Qt3DExtras::QSkyboxEntity*>(ptr);
}

Qt3DExtras::QSkyboxEntity* qt_3d_extras_c_QSkyboxEntity_G_static_cast_Qt3DExtras_QSkyboxEntity_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QSkyboxEntity*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_baseName_to_output(const Qt3DExtras::QSkyboxEntity* this_ptr, QString* output) {
  new(output) QString(this_ptr->baseName());
}

void qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_delete(Qt3DExtras::QSkyboxEntity* this_ptr) {
  delete this_ptr;
}

void qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_extension_to_output(const Qt3DExtras::QSkyboxEntity* this_ptr, QString* output) {
  new(output) QString(this_ptr->extension());
}

bool qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_isGammaCorrectEnabled(const Qt3DExtras::QSkyboxEntity* this_ptr) {
  return this_ptr->isGammaCorrectEnabled();
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_metaObject(const Qt3DExtras::QSkyboxEntity* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QSkyboxEntity* qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_new_no_args() {
  return new Qt3DExtras::QSkyboxEntity();
}

Qt3DExtras::QSkyboxEntity* qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QSkyboxEntity(parent);
}

int qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_qt_metacall(Qt3DExtras::QSkyboxEntity* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_qt_metacast(Qt3DExtras::QSkyboxEntity* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_setBaseName(Qt3DExtras::QSkyboxEntity* this_ptr, const QString* path) {
  this_ptr->setBaseName(*path);
}

void qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_setExtension(Qt3DExtras::QSkyboxEntity* this_ptr, const QString* extension) {
  this_ptr->setExtension(*extension);
}

void qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_setGammaCorrectEnabled(Qt3DExtras::QSkyboxEntity* this_ptr, bool enabled) {
  this_ptr->setGammaCorrectEnabled(enabled);
}

void qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QSkyboxEntity::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QSkyboxEntity::tr(s, c, n));
}

