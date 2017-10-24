#include "qt_3d_core_c_QNodeCreatedChange.h"

Qt3DCore::QNodeCreatedChangeBase* qt_3d_core_c_QNodeCreatedChange_G_dynamic_cast_Qt3DCore_QNodeCreatedChangeBase_ptr(Qt3DCore::QSceneChange* ptr) {
  return dynamic_cast<Qt3DCore::QNodeCreatedChangeBase*>(ptr);
}

Qt3DCore::QNodeCreatedChangeBase* qt_3d_core_c_QNodeCreatedChange_G_static_cast_Qt3DCore_QNodeCreatedChangeBase_ptr(Qt3DCore::QSceneChange* ptr) {
  return static_cast<Qt3DCore::QNodeCreatedChangeBase*>(ptr);
}

Qt3DCore::QSceneChange* qt_3d_core_c_QNodeCreatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(Qt3DCore::QNodeCreatedChangeBase* ptr) {
  return static_cast<Qt3DCore::QSceneChange*>(ptr);
}

void qt_3d_core_c_Qt3DCore_QNodeCreatedChangeBase_delete(Qt3DCore::QNodeCreatedChangeBase* this_ptr) {
  delete this_ptr;
}

bool qt_3d_core_c_Qt3DCore_QNodeCreatedChangeBase_isNodeEnabled(const Qt3DCore::QNodeCreatedChangeBase* this_ptr) {
  return this_ptr->isNodeEnabled();
}

const QMetaObject* qt_3d_core_c_Qt3DCore_QNodeCreatedChangeBase_metaObject(const Qt3DCore::QNodeCreatedChangeBase* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DCore::QNodeCreatedChangeBase* qt_3d_core_c_Qt3DCore_QNodeCreatedChangeBase_new(const Qt3DCore::QNode* node) {
  return new Qt3DCore::QNodeCreatedChangeBase(node);
}

void qt_3d_core_c_Qt3DCore_QNodeCreatedChangeBase_parentId_to_output(const Qt3DCore::QNodeCreatedChangeBase* this_ptr, Qt3DCore::QNodeId* output) {
  new(output) Qt3DCore::QNodeId(this_ptr->parentId());
}

