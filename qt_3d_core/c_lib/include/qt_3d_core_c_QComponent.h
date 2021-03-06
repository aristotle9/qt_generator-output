#ifndef QT_3D_CORE_C_QCOMPONENT_H
#define QT_3D_CORE_C_QCOMPONENT_H

#include "qt_3d_core_c_global.h"

extern "C" {

QT_3D_CORE_C_EXPORT Qt3DCore::QComponent* qt_3d_core_c_QComponent_G_dynamic_cast_Qt3DCore_QComponent_ptr(Qt3DCore::QNode* ptr);
QT_3D_CORE_C_EXPORT QObject* qt_3d_core_c_QComponent_G_static_cast_QObject_ptr(Qt3DCore::QComponent* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QComponent* qt_3d_core_c_QComponent_G_static_cast_Qt3DCore_QComponent_ptr_QObject(QObject* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QComponent* qt_3d_core_c_QComponent_G_static_cast_Qt3DCore_QComponent_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QNode* qt_3d_core_c_QComponent_G_static_cast_Qt3DCore_QNode_ptr(Qt3DCore::QComponent* ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QComponent_delete(Qt3DCore::QComponent* this_ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QComponent_entities_to_output(const Qt3DCore::QComponent* this_ptr, QVector< Qt3DCore::QEntity* >* output);
QT_3D_CORE_C_EXPORT bool qt_3d_core_c_Qt3DCore_QComponent_isShareable(const Qt3DCore::QComponent* this_ptr);
QT_3D_CORE_C_EXPORT const QMetaObject* qt_3d_core_c_Qt3DCore_QComponent_metaObject(const Qt3DCore::QComponent* this_ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QComponent* qt_3d_core_c_Qt3DCore_QComponent_new_no_args();
QT_3D_CORE_C_EXPORT Qt3DCore::QComponent* qt_3d_core_c_Qt3DCore_QComponent_new_parent(Qt3DCore::QNode* parent);
QT_3D_CORE_C_EXPORT int qt_3d_core_c_Qt3DCore_QComponent_qt_metacall(Qt3DCore::QComponent* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_CORE_C_EXPORT void* qt_3d_core_c_Qt3DCore_QComponent_qt_metacast(Qt3DCore::QComponent* this_ptr, const char* arg1);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QComponent_setShareable(Qt3DCore::QComponent* this_ptr, bool isShareable);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QComponent_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QComponent_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_CORE_C_QCOMPONENT_H
