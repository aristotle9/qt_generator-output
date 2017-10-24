#include "qt_3d_core_c_QPropertyNodeRemovedChange.h"

Qt3DCore::QPropertyNodeRemovedChange* qt_3d_core_c_QPropertyNodeRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QPropertyValueRemovedChangeBase(Qt3DCore::QPropertyValueRemovedChangeBase* ptr) {
  return dynamic_cast<Qt3DCore::QPropertyNodeRemovedChange*>(ptr);
}

Qt3DCore::QPropertyNodeRemovedChange* qt_3d_core_c_QPropertyNodeRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QSceneChange(Qt3DCore::QSceneChange* ptr) {
  return dynamic_cast<Qt3DCore::QPropertyNodeRemovedChange*>(ptr);
}

Qt3DCore::QPropertyNodeRemovedChange* qt_3d_core_c_QPropertyNodeRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QStaticPropertyValueRemovedChangeBase(Qt3DCore::QStaticPropertyValueRemovedChangeBase* ptr) {
  return dynamic_cast<Qt3DCore::QPropertyNodeRemovedChange*>(ptr);
}

Qt3DCore::QPropertyNodeRemovedChange* qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QPropertyValueRemovedChangeBase(Qt3DCore::QPropertyValueRemovedChangeBase* ptr) {
  return static_cast<Qt3DCore::QPropertyNodeRemovedChange*>(ptr);
}

Qt3DCore::QPropertyNodeRemovedChange* qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QSceneChange(Qt3DCore::QSceneChange* ptr) {
  return static_cast<Qt3DCore::QPropertyNodeRemovedChange*>(ptr);
}

Qt3DCore::QPropertyNodeRemovedChange* qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QStaticPropertyValueRemovedChangeBase(Qt3DCore::QStaticPropertyValueRemovedChangeBase* ptr) {
  return static_cast<Qt3DCore::QPropertyNodeRemovedChange*>(ptr);
}

Qt3DCore::QPropertyValueRemovedChangeBase* qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QPropertyValueRemovedChangeBase_ptr(Qt3DCore::QPropertyNodeRemovedChange* ptr) {
  return static_cast<Qt3DCore::QPropertyValueRemovedChangeBase*>(ptr);
}

Qt3DCore::QSceneChange* qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(Qt3DCore::QPropertyNodeRemovedChange* ptr) {
  return static_cast<Qt3DCore::QSceneChange*>(ptr);
}

Qt3DCore::QStaticPropertyValueRemovedChangeBase* qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr(Qt3DCore::QPropertyNodeRemovedChange* ptr) {
  return static_cast<Qt3DCore::QStaticPropertyValueRemovedChangeBase*>(ptr);
}

void qt_3d_core_c_Qt3DCore_QPropertyNodeRemovedChange_delete(Qt3DCore::QPropertyNodeRemovedChange* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_core_c_Qt3DCore_QPropertyNodeRemovedChange_metaObject(const Qt3DCore::QPropertyNodeRemovedChange* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DCore::QPropertyNodeRemovedChange* qt_3d_core_c_Qt3DCore_QPropertyNodeRemovedChange_new(const Qt3DCore::QNodeId* subjectId, Qt3DCore::QNode* node) {
  return new Qt3DCore::QPropertyNodeRemovedChange(*subjectId, node);
}

void qt_3d_core_c_Qt3DCore_QPropertyNodeRemovedChange_removedNodeId_to_output(const Qt3DCore::QPropertyNodeRemovedChange* this_ptr, Qt3DCore::QNodeId* output) {
  new(output) Qt3DCore::QNodeId(this_ptr->removedNodeId());
}

