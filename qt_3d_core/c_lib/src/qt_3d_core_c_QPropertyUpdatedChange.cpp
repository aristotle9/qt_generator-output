#include "qt_3d_core_c_QPropertyUpdatedChange.h"

Qt3DCore::QPropertyUpdatedChange* qt_3d_core_c_QPropertyUpdatedChange_G_dynamic_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QPropertyUpdatedChangeBase(Qt3DCore::QPropertyUpdatedChangeBase* ptr) {
  return dynamic_cast<Qt3DCore::QPropertyUpdatedChange*>(ptr);
}

Qt3DCore::QPropertyUpdatedChange* qt_3d_core_c_QPropertyUpdatedChange_G_dynamic_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QSceneChange(Qt3DCore::QSceneChange* ptr) {
  return dynamic_cast<Qt3DCore::QPropertyUpdatedChange*>(ptr);
}

Qt3DCore::QPropertyUpdatedChange* qt_3d_core_c_QPropertyUpdatedChange_G_dynamic_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QStaticPropertyUpdatedChangeBase(Qt3DCore::QStaticPropertyUpdatedChangeBase* ptr) {
  return dynamic_cast<Qt3DCore::QPropertyUpdatedChange*>(ptr);
}

Qt3DCore::QPropertyUpdatedChangeBase* qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChangeBase_ptr(Qt3DCore::QPropertyUpdatedChange* ptr) {
  return static_cast<Qt3DCore::QPropertyUpdatedChangeBase*>(ptr);
}

Qt3DCore::QPropertyUpdatedChange* qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QPropertyUpdatedChangeBase(Qt3DCore::QPropertyUpdatedChangeBase* ptr) {
  return static_cast<Qt3DCore::QPropertyUpdatedChange*>(ptr);
}

Qt3DCore::QPropertyUpdatedChange* qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QSceneChange(Qt3DCore::QSceneChange* ptr) {
  return static_cast<Qt3DCore::QPropertyUpdatedChange*>(ptr);
}

Qt3DCore::QPropertyUpdatedChange* qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QStaticPropertyUpdatedChangeBase(Qt3DCore::QStaticPropertyUpdatedChangeBase* ptr) {
  return static_cast<Qt3DCore::QPropertyUpdatedChange*>(ptr);
}

Qt3DCore::QSceneChange* qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(Qt3DCore::QPropertyUpdatedChange* ptr) {
  return static_cast<Qt3DCore::QSceneChange*>(ptr);
}

Qt3DCore::QStaticPropertyUpdatedChangeBase* qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QStaticPropertyUpdatedChangeBase_ptr(Qt3DCore::QPropertyUpdatedChange* ptr) {
  return static_cast<Qt3DCore::QStaticPropertyUpdatedChangeBase*>(ptr);
}

void qt_3d_core_c_Qt3DCore_QPropertyUpdatedChange_delete(Qt3DCore::QPropertyUpdatedChange* this_ptr) {
  delete this_ptr;
}

Qt3DCore::QPropertyUpdatedChange* qt_3d_core_c_Qt3DCore_QPropertyUpdatedChange_new(const Qt3DCore::QNodeId* subjectId) {
  return new Qt3DCore::QPropertyUpdatedChange(*subjectId);
}

void qt_3d_core_c_Qt3DCore_QPropertyUpdatedChange_setValue(Qt3DCore::QPropertyUpdatedChange* this_ptr, const QVariant* value) {
  this_ptr->setValue(*value);
}

void qt_3d_core_c_Qt3DCore_QPropertyUpdatedChange_value_to_output(const Qt3DCore::QPropertyUpdatedChange* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->value());
}

