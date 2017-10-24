#ifndef QT_3D_CORE_C_QNODECREATEDCHANGE_H
#define QT_3D_CORE_C_QNODECREATEDCHANGE_H

#include "qt_3d_core_c_global.h"

extern "C" {

QT_3D_CORE_C_EXPORT Qt3DCore::QNodeCreatedChangeBase* qt_3d_core_c_QNodeCreatedChange_G_dynamic_cast_Qt3DCore_QNodeCreatedChangeBase_ptr(Qt3DCore::QSceneChange* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QNodeCreatedChangeBase* qt_3d_core_c_QNodeCreatedChange_G_static_cast_Qt3DCore_QNodeCreatedChangeBase_ptr(Qt3DCore::QSceneChange* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QSceneChange* qt_3d_core_c_QNodeCreatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(Qt3DCore::QNodeCreatedChangeBase* ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNodeCreatedChangeBase_delete(Qt3DCore::QNodeCreatedChangeBase* this_ptr);
QT_3D_CORE_C_EXPORT bool qt_3d_core_c_Qt3DCore_QNodeCreatedChangeBase_isNodeEnabled(const Qt3DCore::QNodeCreatedChangeBase* this_ptr);
QT_3D_CORE_C_EXPORT const QMetaObject* qt_3d_core_c_Qt3DCore_QNodeCreatedChangeBase_metaObject(const Qt3DCore::QNodeCreatedChangeBase* this_ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QNodeCreatedChangeBase* qt_3d_core_c_Qt3DCore_QNodeCreatedChangeBase_new(const Qt3DCore::QNode* node);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNodeCreatedChangeBase_parentId_to_output(const Qt3DCore::QNodeCreatedChangeBase* this_ptr, Qt3DCore::QNodeId* output);

} // extern "C"

#endif // QT_3D_CORE_C_QNODECREATEDCHANGE_H
