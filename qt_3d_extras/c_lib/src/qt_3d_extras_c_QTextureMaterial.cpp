#include "qt_3d_extras_c_QTextureMaterial.h"

QObject* qt_3d_extras_c_QTextureMaterial_G_static_cast_QObject_ptr(Qt3DExtras::QTextureMaterial* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DExtras::QTextureMaterial* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QTextureMaterial* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QTextureMaterial* qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DExtras_QTextureMaterial_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QTextureMaterial*>(ptr);
}

Qt3DExtras::QTextureMaterial* qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DExtras_QTextureMaterial_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DExtras::QTextureMaterial*>(ptr);
}

Qt3DExtras::QTextureMaterial* qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DExtras_QTextureMaterial_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QTextureMaterial*>(ptr);
}

Qt3DExtras::QTextureMaterial* qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DExtras_QTextureMaterial_ptr_Qt3DRender_QMaterial(Qt3DRender::QMaterial* ptr) {
  return static_cast<Qt3DExtras::QTextureMaterial*>(ptr);
}

Qt3DRender::QMaterial* qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(Qt3DExtras::QTextureMaterial* ptr) {
  return static_cast<Qt3DRender::QMaterial*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QTextureMaterial_delete(Qt3DExtras::QTextureMaterial* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QTextureMaterial_metaObject(const Qt3DExtras::QTextureMaterial* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QTextureMaterial* qt_3d_extras_c_Qt3DExtras_QTextureMaterial_new_no_args() {
  return new Qt3DExtras::QTextureMaterial();
}

Qt3DExtras::QTextureMaterial* qt_3d_extras_c_Qt3DExtras_QTextureMaterial_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QTextureMaterial(parent);
}

int qt_3d_extras_c_Qt3DExtras_QTextureMaterial_qt_metacall(Qt3DExtras::QTextureMaterial* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QTextureMaterial_qt_metacast(Qt3DExtras::QTextureMaterial* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QTextureMaterial_setTexture(Qt3DExtras::QTextureMaterial* this_ptr, Qt3DRender::QAbstractTexture* texture) {
  this_ptr->setTexture(texture);
}

void qt_3d_extras_c_Qt3DExtras_QTextureMaterial_setTextureOffset(Qt3DExtras::QTextureMaterial* this_ptr, const QVector2D* textureOffset) {
  this_ptr->setTextureOffset(*textureOffset);
}

Qt3DRender::QAbstractTexture* qt_3d_extras_c_Qt3DExtras_QTextureMaterial_texture(const Qt3DExtras::QTextureMaterial* this_ptr) {
  return this_ptr->texture();
}

QVector2D* qt_3d_extras_c_Qt3DExtras_QTextureMaterial_textureOffset_as_ptr(const Qt3DExtras::QTextureMaterial* this_ptr) {
  return new QVector2D(this_ptr->textureOffset());
}

void qt_3d_extras_c_Qt3DExtras_QTextureMaterial_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QTextureMaterial::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QTextureMaterial_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QTextureMaterial::tr(s, c, n));
}

