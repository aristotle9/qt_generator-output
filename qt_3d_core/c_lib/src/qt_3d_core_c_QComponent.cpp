#include "qt_3d_core_c_QComponent.h"

Qt3DCore::QComponent* qt_3d_core_c_QComponent_G_dynamic_cast_Qt3DCore_QComponent_ptr(Qt3DCore::QNode* ptr) {
  return dynamic_cast<Qt3DCore::QComponent*>(ptr);
}

QObject* qt_3d_core_c_QComponent_G_static_cast_QObject_ptr(Qt3DCore::QComponent* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_core_c_QComponent_G_static_cast_Qt3DCore_QComponent_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QComponent* qt_3d_core_c_QComponent_G_static_cast_Qt3DCore_QComponent_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_core_c_QComponent_G_static_cast_Qt3DCore_QNode_ptr(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

void qt_3d_core_c_Qt3DCore_QComponent_delete(Qt3DCore::QComponent* this_ptr) {
  delete this_ptr;
}

void qt_3d_core_c_Qt3DCore_QComponent_entities_to_output(const Qt3DCore::QComponent* this_ptr, QVector< Qt3DCore::QEntity* >* output) {
  new(output) QVector< Qt3DCore::QEntity* >(this_ptr->entities());
}

bool qt_3d_core_c_Qt3DCore_QComponent_isShareable(const Qt3DCore::QComponent* this_ptr) {
  return this_ptr->isShareable();
}

const QMetaObject* qt_3d_core_c_Qt3DCore_QComponent_metaObject(const Qt3DCore::QComponent* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DCore::QComponent* qt_3d_core_c_Qt3DCore_QComponent_new_no_args() {
  return new Qt3DCore::QComponent();
}

Qt3DCore::QComponent* qt_3d_core_c_Qt3DCore_QComponent_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DCore::QComponent(parent);
}

int qt_3d_core_c_Qt3DCore_QComponent_qt_metacall(Qt3DCore::QComponent* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_core_c_Qt3DCore_QComponent_qt_metacast(Qt3DCore::QComponent* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_core_c_Qt3DCore_QComponent_setShareable(Qt3DCore::QComponent* this_ptr, bool isShareable) {
  this_ptr->setShareable(isShareable);
}

void qt_3d_core_c_Qt3DCore_QComponent_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DCore::QComponent::trUtf8(s, c, n));
}

void qt_3d_core_c_Qt3DCore_QComponent_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DCore::QComponent::tr(s, c, n));
}

