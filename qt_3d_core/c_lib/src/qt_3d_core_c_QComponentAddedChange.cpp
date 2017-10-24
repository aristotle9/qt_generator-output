#include "qt_3d_core_c_QComponentAddedChange.h"

Qt3DCore::QComponentAddedChange* qt_3d_core_c_QComponentAddedChange_G_dynamic_cast_Qt3DCore_QComponentAddedChange_ptr(Qt3DCore::QSceneChange* ptr) {
  return dynamic_cast<Qt3DCore::QComponentAddedChange*>(ptr);
}

Qt3DCore::QComponentAddedChange* qt_3d_core_c_QComponentAddedChange_G_static_cast_Qt3DCore_QComponentAddedChange_ptr(Qt3DCore::QSceneChange* ptr) {
  return static_cast<Qt3DCore::QComponentAddedChange*>(ptr);
}

Qt3DCore::QSceneChange* qt_3d_core_c_QComponentAddedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(Qt3DCore::QComponentAddedChange* ptr) {
  return static_cast<Qt3DCore::QSceneChange*>(ptr);
}

void qt_3d_core_c_Qt3DCore_QComponentAddedChange_componentId_to_output(const Qt3DCore::QComponentAddedChange* this_ptr, Qt3DCore::QNodeId* output) {
  new(output) Qt3DCore::QNodeId(this_ptr->componentId());
}

const QMetaObject* qt_3d_core_c_Qt3DCore_QComponentAddedChange_componentMetaObject(const Qt3DCore::QComponentAddedChange* this_ptr) {
  return this_ptr->componentMetaObject();
}

void qt_3d_core_c_Qt3DCore_QComponentAddedChange_delete(Qt3DCore::QComponentAddedChange* this_ptr) {
  delete this_ptr;
}

void qt_3d_core_c_Qt3DCore_QComponentAddedChange_entityId_to_output(const Qt3DCore::QComponentAddedChange* this_ptr, Qt3DCore::QNodeId* output) {
  new(output) Qt3DCore::QNodeId(this_ptr->entityId());
}

Qt3DCore::QComponentAddedChange* qt_3d_core_c_Qt3DCore_QComponentAddedChange_new(const Qt3DCore::QEntity* entity, const Qt3DCore::QComponent* component) {
  return new Qt3DCore::QComponentAddedChange(entity, component);
}

