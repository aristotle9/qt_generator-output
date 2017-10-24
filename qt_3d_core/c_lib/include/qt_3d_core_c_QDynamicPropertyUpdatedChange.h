#ifndef QT_3D_CORE_C_QDYNAMICPROPERTYUPDATEDCHANGE_H
#define QT_3D_CORE_C_QDYNAMICPROPERTYUPDATEDCHANGE_H

#include "qt_3d_core_c_global.h"

extern "C" {

QT_3D_CORE_C_EXPORT Qt3DCore::QDynamicPropertyUpdatedChange* qt_3d_core_c_QDynamicPropertyUpdatedChange_G_dynamic_cast_Qt3DCore_QDynamicPropertyUpdatedChange_ptr_Qt3DCore_QPropertyUpdatedChangeBase(Qt3DCore::QPropertyUpdatedChangeBase* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QDynamicPropertyUpdatedChange* qt_3d_core_c_QDynamicPropertyUpdatedChange_G_dynamic_cast_Qt3DCore_QDynamicPropertyUpdatedChange_ptr_Qt3DCore_QSceneChange(Qt3DCore::QSceneChange* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QDynamicPropertyUpdatedChange* qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QDynamicPropertyUpdatedChange_ptr_Qt3DCore_QPropertyUpdatedChangeBase(Qt3DCore::QPropertyUpdatedChangeBase* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QDynamicPropertyUpdatedChange* qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QDynamicPropertyUpdatedChange_ptr_Qt3DCore_QSceneChange(Qt3DCore::QSceneChange* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QPropertyUpdatedChangeBase* qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChangeBase_ptr(Qt3DCore::QDynamicPropertyUpdatedChange* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QSceneChange* qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(Qt3DCore::QDynamicPropertyUpdatedChange* ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_delete(Qt3DCore::QDynamicPropertyUpdatedChange* this_ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QDynamicPropertyUpdatedChange* qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_new(const Qt3DCore::QNodeId* subjectId);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_propertyName_to_output(const Qt3DCore::QDynamicPropertyUpdatedChange* this_ptr, QByteArray* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_setPropertyName(Qt3DCore::QDynamicPropertyUpdatedChange* this_ptr, const QByteArray* name);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_setValue(Qt3DCore::QDynamicPropertyUpdatedChange* this_ptr, const QVariant* value);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_value_to_output(const Qt3DCore::QDynamicPropertyUpdatedChange* this_ptr, QVariant* output);

} // extern "C"

#endif // QT_3D_CORE_C_QDYNAMICPROPERTYUPDATEDCHANGE_H
