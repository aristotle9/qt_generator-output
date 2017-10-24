#include "qt_3d_core_c_QPropertyNodeAddedChange.h"

Qt3DCore::QPropertyNodeAddedChange* qt_3d_core_c_QPropertyNodeAddedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QPropertyValueAddedChangeBase(Qt3DCore::QPropertyValueAddedChangeBase* ptr) {
  return dynamic_cast<Qt3DCore::QPropertyNodeAddedChange*>(ptr);
}

Qt3DCore::QPropertyNodeAddedChange* qt_3d_core_c_QPropertyNodeAddedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QSceneChange(Qt3DCore::QSceneChange* ptr) {
  return dynamic_cast<Qt3DCore::QPropertyNodeAddedChange*>(ptr);
}

Qt3DCore::QPropertyNodeAddedChange* qt_3d_core_c_QPropertyNodeAddedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QStaticPropertyValueAddedChangeBase(Qt3DCore::QStaticPropertyValueAddedChangeBase* ptr) {
  return dynamic_cast<Qt3DCore::QPropertyNodeAddedChange*>(ptr);
}

Qt3DCore::QPropertyNodeAddedChange* qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QPropertyValueAddedChangeBase(Qt3DCore::QPropertyValueAddedChangeBase* ptr) {
  return static_cast<Qt3DCore::QPropertyNodeAddedChange*>(ptr);
}

Qt3DCore::QPropertyNodeAddedChange* qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QSceneChange(Qt3DCore::QSceneChange* ptr) {
  return static_cast<Qt3DCore::QPropertyNodeAddedChange*>(ptr);
}

Qt3DCore::QPropertyNodeAddedChange* qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QStaticPropertyValueAddedChangeBase(Qt3DCore::QStaticPropertyValueAddedChangeBase* ptr) {
  return static_cast<Qt3DCore::QPropertyNodeAddedChange*>(ptr);
}

Qt3DCore::QPropertyValueAddedChangeBase* qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QPropertyValueAddedChangeBase_ptr(Qt3DCore::QPropertyNodeAddedChange* ptr) {
  return static_cast<Qt3DCore::QPropertyValueAddedChangeBase*>(ptr);
}

Qt3DCore::QSceneChange* qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(Qt3DCore::QPropertyNodeAddedChange* ptr) {
  return static_cast<Qt3DCore::QSceneChange*>(ptr);
}

Qt3DCore::QStaticPropertyValueAddedChangeBase* qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr(Qt3DCore::QPropertyNodeAddedChange* ptr) {
  return static_cast<Qt3DCore::QStaticPropertyValueAddedChangeBase*>(ptr);
}

void qt_3d_core_c_Qt3DCore_QPropertyNodeAddedChange_addedNodeId_to_output(const Qt3DCore::QPropertyNodeAddedChange* this_ptr, Qt3DCore::QNodeId* output) {
  new(output) Qt3DCore::QNodeId(this_ptr->addedNodeId());
}

void qt_3d_core_c_Qt3DCore_QPropertyNodeAddedChange_delete(Qt3DCore::QPropertyNodeAddedChange* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_core_c_Qt3DCore_QPropertyNodeAddedChange_metaObject(const Qt3DCore::QPropertyNodeAddedChange* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DCore::QPropertyNodeAddedChange* qt_3d_core_c_Qt3DCore_QPropertyNodeAddedChange_new(const Qt3DCore::QNodeId* subjectId, Qt3DCore::QNode* node) {
  return new Qt3DCore::QPropertyNodeAddedChange(*subjectId, node);
}

