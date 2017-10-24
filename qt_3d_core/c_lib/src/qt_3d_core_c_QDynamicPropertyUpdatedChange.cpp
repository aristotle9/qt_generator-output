#include "qt_3d_core_c_QDynamicPropertyUpdatedChange.h"

Qt3DCore::QDynamicPropertyUpdatedChange* qt_3d_core_c_QDynamicPropertyUpdatedChange_G_dynamic_cast_Qt3DCore_QDynamicPropertyUpdatedChange_ptr_Qt3DCore_QPropertyUpdatedChangeBase(Qt3DCore::QPropertyUpdatedChangeBase* ptr) {
  return dynamic_cast<Qt3DCore::QDynamicPropertyUpdatedChange*>(ptr);
}

Qt3DCore::QDynamicPropertyUpdatedChange* qt_3d_core_c_QDynamicPropertyUpdatedChange_G_dynamic_cast_Qt3DCore_QDynamicPropertyUpdatedChange_ptr_Qt3DCore_QSceneChange(Qt3DCore::QSceneChange* ptr) {
  return dynamic_cast<Qt3DCore::QDynamicPropertyUpdatedChange*>(ptr);
}

Qt3DCore::QDynamicPropertyUpdatedChange* qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QDynamicPropertyUpdatedChange_ptr_Qt3DCore_QPropertyUpdatedChangeBase(Qt3DCore::QPropertyUpdatedChangeBase* ptr) {
  return static_cast<Qt3DCore::QDynamicPropertyUpdatedChange*>(ptr);
}

Qt3DCore::QDynamicPropertyUpdatedChange* qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QDynamicPropertyUpdatedChange_ptr_Qt3DCore_QSceneChange(Qt3DCore::QSceneChange* ptr) {
  return static_cast<Qt3DCore::QDynamicPropertyUpdatedChange*>(ptr);
}

Qt3DCore::QPropertyUpdatedChangeBase* qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChangeBase_ptr(Qt3DCore::QDynamicPropertyUpdatedChange* ptr) {
  return static_cast<Qt3DCore::QPropertyUpdatedChangeBase*>(ptr);
}

Qt3DCore::QSceneChange* qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(Qt3DCore::QDynamicPropertyUpdatedChange* ptr) {
  return static_cast<Qt3DCore::QSceneChange*>(ptr);
}

void qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_delete(Qt3DCore::QDynamicPropertyUpdatedChange* this_ptr) {
  delete this_ptr;
}

Qt3DCore::QDynamicPropertyUpdatedChange* qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_new(const Qt3DCore::QNodeId* subjectId) {
  return new Qt3DCore::QDynamicPropertyUpdatedChange(*subjectId);
}

void qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_propertyName_to_output(const Qt3DCore::QDynamicPropertyUpdatedChange* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->propertyName());
}

void qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_setPropertyName(Qt3DCore::QDynamicPropertyUpdatedChange* this_ptr, const QByteArray* name) {
  this_ptr->setPropertyName(*name);
}

void qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_setValue(Qt3DCore::QDynamicPropertyUpdatedChange* this_ptr, const QVariant* value) {
  this_ptr->setValue(*value);
}

void qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_value_to_output(const Qt3DCore::QDynamicPropertyUpdatedChange* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->value());
}

