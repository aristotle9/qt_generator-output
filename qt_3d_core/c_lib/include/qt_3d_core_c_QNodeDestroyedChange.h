#ifndef QT_3D_CORE_C_QNODEDESTROYEDCHANGE_H
#define QT_3D_CORE_C_QNODEDESTROYEDCHANGE_H

#include "qt_3d_core_c_global.h"

extern "C" {

QT_3D_CORE_C_EXPORT Qt3DCore::QNodeDestroyedChange* qt_3d_core_c_QNodeDestroyedChange_G_dynamic_cast_Qt3DCore_QNodeDestroyedChange_ptr(Qt3DCore::QSceneChange* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QNodeDestroyedChange* qt_3d_core_c_QNodeDestroyedChange_G_static_cast_Qt3DCore_QNodeDestroyedChange_ptr(Qt3DCore::QSceneChange* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QSceneChange* qt_3d_core_c_QNodeDestroyedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(Qt3DCore::QNodeDestroyedChange* ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNodeDestroyedChange_delete(Qt3DCore::QNodeDestroyedChange* this_ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QNodeDestroyedChange* qt_3d_core_c_Qt3DCore_QNodeDestroyedChange_new(const Qt3DCore::QNode* node, const QVector< Qt3DCore::QNodeIdTypePair >* subtreeIdsAndTypes);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNodeDestroyedChange_subtreeIdsAndTypes_to_output(const Qt3DCore::QNodeDestroyedChange* this_ptr, QVector< Qt3DCore::QNodeIdTypePair >* output);

} // extern "C"

#endif // QT_3D_CORE_C_QNODEDESTROYEDCHANGE_H
