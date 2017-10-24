#include "qt_3d_core_c_QComponentRemovedChange.h"

Qt3DCore::QComponentRemovedChange* qt_3d_core_c_QComponentRemovedChange_G_dynamic_cast_Qt3DCore_QComponentRemovedChange_ptr(Qt3DCore::QSceneChange* ptr) {
  return dynamic_cast<Qt3DCore::QComponentRemovedChange*>(ptr);
}

Qt3DCore::QComponentRemovedChange* qt_3d_core_c_QComponentRemovedChange_G_static_cast_Qt3DCore_QComponentRemovedChange_ptr(Qt3DCore::QSceneChange* ptr) {
  return static_cast<Qt3DCore::QComponentRemovedChange*>(ptr);
}

Qt3DCore::QSceneChange* qt_3d_core_c_QComponentRemovedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(Qt3DCore::QComponentRemovedChange* ptr) {
  return static_cast<Qt3DCore::QSceneChange*>(ptr);
}

void qt_3d_core_c_Qt3DCore_QComponentRemovedChange_componentId_to_output(const Qt3DCore::QComponentRemovedChange* this_ptr, Qt3DCore::QNodeId* output) {
  new(output) Qt3DCore::QNodeId(this_ptr->componentId());
}

const QMetaObject* qt_3d_core_c_Qt3DCore_QComponentRemovedChange_componentMetaObject(const Qt3DCore::QComponentRemovedChange* this_ptr) {
  return this_ptr->componentMetaObject();
}

void qt_3d_core_c_Qt3DCore_QComponentRemovedChange_delete(Qt3DCore::QComponentRemovedChange* this_ptr) {
  delete this_ptr;
}

void qt_3d_core_c_Qt3DCore_QComponentRemovedChange_entityId_to_output(const Qt3DCore::QComponentRemovedChange* this_ptr, Qt3DCore::QNodeId* output) {
  new(output) Qt3DCore::QNodeId(this_ptr->entityId());
}

Qt3DCore::QComponentRemovedChange* qt_3d_core_c_Qt3DCore_QComponentRemovedChange_new(const Qt3DCore::QEntity* entity, const Qt3DCore::QComponent* component) {
  return new Qt3DCore::QComponentRemovedChange(entity, component);
}

