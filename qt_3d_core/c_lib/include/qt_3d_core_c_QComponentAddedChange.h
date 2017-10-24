#ifndef QT_3D_CORE_C_QCOMPONENTADDEDCHANGE_H
#define QT_3D_CORE_C_QCOMPONENTADDEDCHANGE_H

#include "qt_3d_core_c_global.h"

extern "C" {

QT_3D_CORE_C_EXPORT Qt3DCore::QComponentAddedChange* qt_3d_core_c_QComponentAddedChange_G_dynamic_cast_Qt3DCore_QComponentAddedChange_ptr(Qt3DCore::QSceneChange* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QComponentAddedChange* qt_3d_core_c_QComponentAddedChange_G_static_cast_Qt3DCore_QComponentAddedChange_ptr(Qt3DCore::QSceneChange* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QSceneChange* qt_3d_core_c_QComponentAddedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(Qt3DCore::QComponentAddedChange* ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QComponentAddedChange_componentId_to_output(const Qt3DCore::QComponentAddedChange* this_ptr, Qt3DCore::QNodeId* output);
QT_3D_CORE_C_EXPORT const QMetaObject* qt_3d_core_c_Qt3DCore_QComponentAddedChange_componentMetaObject(const Qt3DCore::QComponentAddedChange* this_ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QComponentAddedChange_delete(Qt3DCore::QComponentAddedChange* this_ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QComponentAddedChange_entityId_to_output(const Qt3DCore::QComponentAddedChange* this_ptr, Qt3DCore::QNodeId* output);
QT_3D_CORE_C_EXPORT Qt3DCore::QComponentAddedChange* qt_3d_core_c_Qt3DCore_QComponentAddedChange_new(const Qt3DCore::QEntity* entity, const Qt3DCore::QComponent* component);

} // extern "C"

#endif // QT_3D_CORE_C_QCOMPONENTADDEDCHANGE_H
