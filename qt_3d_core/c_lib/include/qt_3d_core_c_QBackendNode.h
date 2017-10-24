#ifndef QT_3D_CORE_C_QBACKENDNODE_H
#define QT_3D_CORE_C_QBACKENDNODE_H

#include "qt_3d_core_c_global.h"

extern "C" {

QT_3D_CORE_C_EXPORT Qt3DCore::QBackendNode* qt_3d_core_c_Qt3DCore_QBackendNodeMapper_create(const Qt3DCore::QBackendNodeMapper* this_ptr, const QSharedPointer< Qt3DCore::QNodeCreatedChangeBase >* change);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QBackendNodeMapper_delete(Qt3DCore::QBackendNodeMapper* this_ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QBackendNodeMapper_destroy(const Qt3DCore::QBackendNodeMapper* this_ptr, const Qt3DCore::QNodeId* id);
QT_3D_CORE_C_EXPORT Qt3DCore::QBackendNode* qt_3d_core_c_Qt3DCore_QBackendNodeMapper_get(const Qt3DCore::QBackendNodeMapper* this_ptr, const Qt3DCore::QNodeId* id);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QBackendNode_delete(Qt3DCore::QBackendNode* this_ptr);
QT_3D_CORE_C_EXPORT bool qt_3d_core_c_Qt3DCore_QBackendNode_isEnabled(const Qt3DCore::QBackendNode* this_ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QBackendNode::Mode qt_3d_core_c_Qt3DCore_QBackendNode_mode(const Qt3DCore::QBackendNode* this_ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QBackendNode* qt_3d_core_c_Qt3DCore_QBackendNode_new_mode(Qt3DCore::QBackendNode::Mode mode);
QT_3D_CORE_C_EXPORT Qt3DCore::QBackendNode* qt_3d_core_c_Qt3DCore_QBackendNode_new_no_args();
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QBackendNode_peerId_to_output(const Qt3DCore::QBackendNode* this_ptr, Qt3DCore::QNodeId* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QBackendNode_setEnabled(Qt3DCore::QBackendNode* this_ptr, bool enabled);

} // extern "C"

#endif // QT_3D_CORE_C_QBACKENDNODE_H
