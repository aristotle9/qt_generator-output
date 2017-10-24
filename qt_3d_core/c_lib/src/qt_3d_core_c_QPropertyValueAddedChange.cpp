#include "qt_3d_core_c_QPropertyValueAddedChange.h"

Qt3DCore::QPropertyValueAddedChange* qt_3d_core_c_QPropertyValueAddedChange_G_dynamic_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QPropertyValueAddedChangeBase(Qt3DCore::QPropertyValueAddedChangeBase* ptr) {
  return dynamic_cast<Qt3DCore::QPropertyValueAddedChange*>(ptr);
}

Qt3DCore::QPropertyValueAddedChange* qt_3d_core_c_QPropertyValueAddedChange_G_dynamic_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QSceneChange(Qt3DCore::QSceneChange* ptr) {
  return dynamic_cast<Qt3DCore::QPropertyValueAddedChange*>(ptr);
}

Qt3DCore::QPropertyValueAddedChange* qt_3d_core_c_QPropertyValueAddedChange_G_dynamic_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QStaticPropertyValueAddedChangeBase(Qt3DCore::QStaticPropertyValueAddedChangeBase* ptr) {
  return dynamic_cast<Qt3DCore::QPropertyValueAddedChange*>(ptr);
}

Qt3DCore::QPropertyValueAddedChangeBase* qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QPropertyValueAddedChangeBase_ptr(Qt3DCore::QPropertyValueAddedChange* ptr) {
  return static_cast<Qt3DCore::QPropertyValueAddedChangeBase*>(ptr);
}

Qt3DCore::QPropertyValueAddedChange* qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QPropertyValueAddedChangeBase(Qt3DCore::QPropertyValueAddedChangeBase* ptr) {
  return static_cast<Qt3DCore::QPropertyValueAddedChange*>(ptr);
}

Qt3DCore::QPropertyValueAddedChange* qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QSceneChange(Qt3DCore::QSceneChange* ptr) {
  return static_cast<Qt3DCore::QPropertyValueAddedChange*>(ptr);
}

Qt3DCore::QPropertyValueAddedChange* qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QStaticPropertyValueAddedChangeBase(Qt3DCore::QStaticPropertyValueAddedChangeBase* ptr) {
  return static_cast<Qt3DCore::QPropertyValueAddedChange*>(ptr);
}

Qt3DCore::QSceneChange* qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(Qt3DCore::QPropertyValueAddedChange* ptr) {
  return static_cast<Qt3DCore::QSceneChange*>(ptr);
}

Qt3DCore::QStaticPropertyValueAddedChangeBase* qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr(Qt3DCore::QPropertyValueAddedChange* ptr) {
  return static_cast<Qt3DCore::QStaticPropertyValueAddedChangeBase*>(ptr);
}

void qt_3d_core_c_Qt3DCore_QPropertyValueAddedChange_addedValue_to_output(const Qt3DCore::QPropertyValueAddedChange* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->addedValue());
}

void qt_3d_core_c_Qt3DCore_QPropertyValueAddedChange_delete(Qt3DCore::QPropertyValueAddedChange* this_ptr) {
  delete this_ptr;
}

Qt3DCore::QPropertyValueAddedChange* qt_3d_core_c_Qt3DCore_QPropertyValueAddedChange_new(const Qt3DCore::QNodeId* subjectId) {
  return new Qt3DCore::QPropertyValueAddedChange(*subjectId);
}

void qt_3d_core_c_Qt3DCore_QPropertyValueAddedChange_setAddedValue(Qt3DCore::QPropertyValueAddedChange* this_ptr, const QVariant* value) {
  this_ptr->setAddedValue(*value);
}

