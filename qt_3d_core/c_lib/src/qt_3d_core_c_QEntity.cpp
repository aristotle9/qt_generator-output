#include "qt_3d_core_c_QEntity.h"

Qt3DCore::QEntity* qt_3d_core_c_QEntity_G_dynamic_cast_Qt3DCore_QEntity_ptr(Qt3DCore::QNode* ptr) {
  return dynamic_cast<Qt3DCore::QEntity*>(ptr);
}

QObject* qt_3d_core_c_QEntity_G_static_cast_QObject_ptr(Qt3DCore::QEntity* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QEntity* qt_3d_core_c_QEntity_G_static_cast_Qt3DCore_QEntity_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DCore::QEntity*>(ptr);
}

Qt3DCore::QEntity* qt_3d_core_c_QEntity_G_static_cast_Qt3DCore_QEntity_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DCore::QEntity*>(ptr);
}

Qt3DCore::QNode* qt_3d_core_c_QEntity_G_static_cast_Qt3DCore_QNode_ptr(Qt3DCore::QEntity* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

void qt_3d_core_c_Qt3DCore_QEntity_addComponent(Qt3DCore::QEntity* this_ptr, Qt3DCore::QComponent* comp) {
  this_ptr->addComponent(comp);
}

void qt_3d_core_c_Qt3DCore_QEntity_components_to_output(const Qt3DCore::QEntity* this_ptr, QVector< Qt3DCore::QComponent* >* output) {
  new(output) QVector< Qt3DCore::QComponent* >(this_ptr->components());
}

void qt_3d_core_c_Qt3DCore_QEntity_delete(Qt3DCore::QEntity* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_core_c_Qt3DCore_QEntity_metaObject(const Qt3DCore::QEntity* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DCore::QEntity* qt_3d_core_c_Qt3DCore_QEntity_new_no_args() {
  return new Qt3DCore::QEntity();
}

Qt3DCore::QEntity* qt_3d_core_c_Qt3DCore_QEntity_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DCore::QEntity(parent);
}

Qt3DCore::QEntity* qt_3d_core_c_Qt3DCore_QEntity_parentEntity(const Qt3DCore::QEntity* this_ptr) {
  return this_ptr->parentEntity();
}

int qt_3d_core_c_Qt3DCore_QEntity_qt_metacall(Qt3DCore::QEntity* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_core_c_Qt3DCore_QEntity_qt_metacast(Qt3DCore::QEntity* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_core_c_Qt3DCore_QEntity_removeComponent(Qt3DCore::QEntity* this_ptr, Qt3DCore::QComponent* comp) {
  this_ptr->removeComponent(comp);
}

void qt_3d_core_c_Qt3DCore_QEntity_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DCore::QEntity::trUtf8(s, c, n));
}

void qt_3d_core_c_Qt3DCore_QEntity_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DCore::QEntity::tr(s, c, n));
}

