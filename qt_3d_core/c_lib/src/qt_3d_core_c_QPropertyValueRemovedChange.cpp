#include "qt_3d_core_c_QPropertyValueRemovedChange.h"

Qt3DCore::QPropertyValueRemovedChange* qt_3d_core_c_QPropertyValueRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QPropertyValueRemovedChangeBase(Qt3DCore::QPropertyValueRemovedChangeBase* ptr) {
  return dynamic_cast<Qt3DCore::QPropertyValueRemovedChange*>(ptr);
}

Qt3DCore::QPropertyValueRemovedChange* qt_3d_core_c_QPropertyValueRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QSceneChange(Qt3DCore::QSceneChange* ptr) {
  return dynamic_cast<Qt3DCore::QPropertyValueRemovedChange*>(ptr);
}

Qt3DCore::QPropertyValueRemovedChange* qt_3d_core_c_QPropertyValueRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QStaticPropertyValueRemovedChangeBase(Qt3DCore::QStaticPropertyValueRemovedChangeBase* ptr) {
  return dynamic_cast<Qt3DCore::QPropertyValueRemovedChange*>(ptr);
}

Qt3DCore::QPropertyValueRemovedChangeBase* qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QPropertyValueRemovedChangeBase_ptr(Qt3DCore::QPropertyValueRemovedChange* ptr) {
  return static_cast<Qt3DCore::QPropertyValueRemovedChangeBase*>(ptr);
}

Qt3DCore::QPropertyValueRemovedChange* qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QPropertyValueRemovedChangeBase(Qt3DCore::QPropertyValueRemovedChangeBase* ptr) {
  return static_cast<Qt3DCore::QPropertyValueRemovedChange*>(ptr);
}

Qt3DCore::QPropertyValueRemovedChange* qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QSceneChange(Qt3DCore::QSceneChange* ptr) {
  return static_cast<Qt3DCore::QPropertyValueRemovedChange*>(ptr);
}

Qt3DCore::QPropertyValueRemovedChange* qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QStaticPropertyValueRemovedChangeBase(Qt3DCore::QStaticPropertyValueRemovedChangeBase* ptr) {
  return static_cast<Qt3DCore::QPropertyValueRemovedChange*>(ptr);
}

Qt3DCore::QSceneChange* qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(Qt3DCore::QPropertyValueRemovedChange* ptr) {
  return static_cast<Qt3DCore::QSceneChange*>(ptr);
}

Qt3DCore::QStaticPropertyValueRemovedChangeBase* qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr(Qt3DCore::QPropertyValueRemovedChange* ptr) {
  return static_cast<Qt3DCore::QStaticPropertyValueRemovedChangeBase*>(ptr);
}

void qt_3d_core_c_Qt3DCore_QPropertyValueRemovedChange_delete(Qt3DCore::QPropertyValueRemovedChange* this_ptr) {
  delete this_ptr;
}

Qt3DCore::QPropertyValueRemovedChange* qt_3d_core_c_Qt3DCore_QPropertyValueRemovedChange_new(const Qt3DCore::QNodeId* subjectId) {
  return new Qt3DCore::QPropertyValueRemovedChange(*subjectId);
}

void qt_3d_core_c_Qt3DCore_QPropertyValueRemovedChange_removedValue_to_output(const Qt3DCore::QPropertyValueRemovedChange* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->removedValue());
}

void qt_3d_core_c_Qt3DCore_QPropertyValueRemovedChange_setRemovedValue(Qt3DCore::QPropertyValueRemovedChange* this_ptr, const QVariant* value) {
  this_ptr->setRemovedValue(*value);
}

