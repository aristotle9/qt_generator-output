#ifndef QT_3D_CORE_C_QENTITY_H
#define QT_3D_CORE_C_QENTITY_H

#include "qt_3d_core_c_global.h"

extern "C" {

QT_3D_CORE_C_EXPORT Qt3DCore::QEntity* qt_3d_core_c_QEntity_G_dynamic_cast_Qt3DCore_QEntity_ptr(Qt3DCore::QNode* ptr);
QT_3D_CORE_C_EXPORT QObject* qt_3d_core_c_QEntity_G_static_cast_QObject_ptr(Qt3DCore::QEntity* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QEntity* qt_3d_core_c_QEntity_G_static_cast_Qt3DCore_QEntity_ptr_QObject(QObject* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QEntity* qt_3d_core_c_QEntity_G_static_cast_Qt3DCore_QEntity_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QNode* qt_3d_core_c_QEntity_G_static_cast_Qt3DCore_QNode_ptr(Qt3DCore::QEntity* ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QEntity_addComponent(Qt3DCore::QEntity* this_ptr, Qt3DCore::QComponent* comp);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QEntity_components_to_output(const Qt3DCore::QEntity* this_ptr, QVector< Qt3DCore::QComponent* >* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QEntity_delete(Qt3DCore::QEntity* this_ptr);
QT_3D_CORE_C_EXPORT const QMetaObject* qt_3d_core_c_Qt3DCore_QEntity_metaObject(const Qt3DCore::QEntity* this_ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QEntity* qt_3d_core_c_Qt3DCore_QEntity_new_no_args();
QT_3D_CORE_C_EXPORT Qt3DCore::QEntity* qt_3d_core_c_Qt3DCore_QEntity_new_parent(Qt3DCore::QNode* parent);
QT_3D_CORE_C_EXPORT Qt3DCore::QEntity* qt_3d_core_c_Qt3DCore_QEntity_parentEntity(const Qt3DCore::QEntity* this_ptr);
QT_3D_CORE_C_EXPORT int qt_3d_core_c_Qt3DCore_QEntity_qt_metacall(Qt3DCore::QEntity* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_CORE_C_EXPORT void* qt_3d_core_c_Qt3DCore_QEntity_qt_metacast(Qt3DCore::QEntity* this_ptr, const char* arg1);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QEntity_removeComponent(Qt3DCore::QEntity* this_ptr, Qt3DCore::QComponent* comp);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QEntity_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QEntity_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_CORE_C_QENTITY_H
