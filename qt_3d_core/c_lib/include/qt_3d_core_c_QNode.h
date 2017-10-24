#ifndef QT_3D_CORE_C_QNODE_H
#define QT_3D_CORE_C_QNODE_H

#include "qt_3d_core_c_global.h"

extern "C" {

QT_3D_CORE_C_EXPORT void qt_3d_core_c_QNode_G_Qt3DCore_qIdForNode_to_output(Qt3DCore::QNode* node, Qt3DCore::QNodeId* output);
QT_3D_CORE_C_EXPORT QObject* qt_3d_core_c_QNode_G_static_cast_QObject_ptr(Qt3DCore::QNode* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QNode* qt_3d_core_c_QNode_G_static_cast_Qt3DCore_QNode_ptr(QObject* ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNodeIdTypePair_constructor__id__type(const Qt3DCore::QNodeId* _id, const QMetaObject* _type, Qt3DCore::QNodeIdTypePair* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNodeIdTypePair_constructor_no_args(Qt3DCore::QNodeIdTypePair* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNodeIdTypePair_destructor(Qt3DCore::QNodeIdTypePair* this_ptr);
QT_3D_CORE_C_EXPORT const Qt3DCore::QNodeId* qt_3d_core_c_Qt3DCore_QNodeIdTypePair_id(const Qt3DCore::QNodeIdTypePair* this_ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QNodeId* qt_3d_core_c_Qt3DCore_QNodeIdTypePair_id_mut(Qt3DCore::QNodeIdTypePair* this_ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNodeIdTypePair_set_id(Qt3DCore::QNodeIdTypePair* this_ptr, const Qt3DCore::QNodeId* value);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNodeIdTypePair_set_type(Qt3DCore::QNodeIdTypePair* this_ptr, const QMetaObject* value);
QT_3D_CORE_C_EXPORT const QMetaObject* qt_3d_core_c_Qt3DCore_QNodeIdTypePair_type(const Qt3DCore::QNodeIdTypePair* this_ptr);
QT_3D_CORE_C_EXPORT bool qt_3d_core_c_Qt3DCore_QNode_blockNotifications(Qt3DCore::QNode* this_ptr, bool block);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNode_childNodes_to_output(const Qt3DCore::QNode* this_ptr, QVector< Qt3DCore::QNode* >* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNode_clearPropertyTracking(Qt3DCore::QNode* this_ptr, const QString* propertyName);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNode_clearPropertyTrackings(Qt3DCore::QNode* this_ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QNode::PropertyTrackingMode qt_3d_core_c_Qt3DCore_QNode_defaultPropertyTrackingMode(const Qt3DCore::QNode* this_ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNode_delete(Qt3DCore::QNode* this_ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNode_id_to_output(const Qt3DCore::QNode* this_ptr, Qt3DCore::QNodeId* output);
QT_3D_CORE_C_EXPORT bool qt_3d_core_c_Qt3DCore_QNode_isEnabled(const Qt3DCore::QNode* this_ptr);
QT_3D_CORE_C_EXPORT const QMetaObject* qt_3d_core_c_Qt3DCore_QNode_metaObject(const Qt3DCore::QNode* this_ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QNode* qt_3d_core_c_Qt3DCore_QNode_new_no_args();
QT_3D_CORE_C_EXPORT Qt3DCore::QNode* qt_3d_core_c_Qt3DCore_QNode_new_parent(Qt3DCore::QNode* parent);
QT_3D_CORE_C_EXPORT bool qt_3d_core_c_Qt3DCore_QNode_notificationsBlocked(const Qt3DCore::QNode* this_ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QNode* qt_3d_core_c_Qt3DCore_QNode_parentNode(const Qt3DCore::QNode* this_ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QNode::PropertyTrackingMode qt_3d_core_c_Qt3DCore_QNode_propertyTracking(const Qt3DCore::QNode* this_ptr, const QString* propertyName);
QT_3D_CORE_C_EXPORT int qt_3d_core_c_Qt3DCore_QNode_qt_metacall(Qt3DCore::QNode* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_CORE_C_EXPORT void* qt_3d_core_c_Qt3DCore_QNode_qt_metacast(Qt3DCore::QNode* this_ptr, const char* arg1);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNode_setDefaultPropertyTrackingMode(Qt3DCore::QNode* this_ptr, Qt3DCore::QNode::PropertyTrackingMode mode);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNode_setEnabled(Qt3DCore::QNode* this_ptr, bool isEnabled);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNode_setParent(Qt3DCore::QNode* this_ptr, Qt3DCore::QNode* parent);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNode_setPropertyTracking(Qt3DCore::QNode* this_ptr, const QString* propertyName, Qt3DCore::QNode::PropertyTrackingMode trackMode);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNode_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNode_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_CORE_C_QNODE_H
