#ifndef QT_3D_CORE_C_QCOMPONENTREMOVEDCHANGE_H
#define QT_3D_CORE_C_QCOMPONENTREMOVEDCHANGE_H

#include "qt_3d_core_c_global.h"

extern "C" {

QT_3D_CORE_C_EXPORT Qt3DCore::QComponentRemovedChange* qt_3d_core_c_QComponentRemovedChange_G_dynamic_cast_Qt3DCore_QComponentRemovedChange_ptr(Qt3DCore::QSceneChange* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QComponentRemovedChange* qt_3d_core_c_QComponentRemovedChange_G_static_cast_Qt3DCore_QComponentRemovedChange_ptr(Qt3DCore::QSceneChange* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QSceneChange* qt_3d_core_c_QComponentRemovedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(Qt3DCore::QComponentRemovedChange* ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QComponentRemovedChange_componentId_to_output(const Qt3DCore::QComponentRemovedChange* this_ptr, Qt3DCore::QNodeId* output);
QT_3D_CORE_C_EXPORT const QMetaObject* qt_3d_core_c_Qt3DCore_QComponentRemovedChange_componentMetaObject(const Qt3DCore::QComponentRemovedChange* this_ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QComponentRemovedChange_delete(Qt3DCore::QComponentRemovedChange* this_ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QComponentRemovedChange_entityId_to_output(const Qt3DCore::QComponentRemovedChange* this_ptr, Qt3DCore::QNodeId* output);
QT_3D_CORE_C_EXPORT Qt3DCore::QComponentRemovedChange* qt_3d_core_c_Qt3DCore_QComponentRemovedChange_new(const Qt3DCore::QEntity* entity, const Qt3DCore::QComponent* component);

} // extern "C"

#endif // QT_3D_CORE_C_QCOMPONENTREMOVEDCHANGE_H
