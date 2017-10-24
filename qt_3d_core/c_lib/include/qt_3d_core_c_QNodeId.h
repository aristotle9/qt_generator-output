#ifndef QT_3D_CORE_C_QNODEID_H
#define QT_3D_CORE_C_QNODEID_H

#include "qt_3d_core_c_global.h"

extern "C" {

QT_3D_CORE_C_EXPORT unsigned int qt_3d_core_c_QNodeId_G_Qt3DCore_qHash_id(const Qt3DCore::QNodeId* id);
QT_3D_CORE_C_EXPORT unsigned int qt_3d_core_c_QNodeId_G_Qt3DCore_qHash_id_seed(const Qt3DCore::QNodeId* id, unsigned int seed);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_QNodeId_G_operator_shl_to_output(const QDebug* d, const Qt3DCore::QNodeId* id, QDebug* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNodeId_constructor(Qt3DCore::QNodeId* output);
QT_3D_CORE_C_EXPORT bool qt_3d_core_c_Qt3DCore_QNodeId_convert_to_bool(const Qt3DCore::QNodeId* this_ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNodeId_createId_to_output(Qt3DCore::QNodeId* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QNodeId_destructor(Qt3DCore::QNodeId* this_ptr);
QT_3D_CORE_C_EXPORT quint64 qt_3d_core_c_Qt3DCore_QNodeId_id(const Qt3DCore::QNodeId* this_ptr);
QT_3D_CORE_C_EXPORT bool qt_3d_core_c_Qt3DCore_QNodeId_isNull(const Qt3DCore::QNodeId* this_ptr);
QT_3D_CORE_C_EXPORT bool qt_3d_core_c_Qt3DCore_QNodeId_operator_eq(const Qt3DCore::QNodeId* this_ptr, const Qt3DCore::QNodeId* other);
QT_3D_CORE_C_EXPORT bool qt_3d_core_c_Qt3DCore_QNodeId_operator_gt(const Qt3DCore::QNodeId* this_ptr, const Qt3DCore::QNodeId* other);
QT_3D_CORE_C_EXPORT bool qt_3d_core_c_Qt3DCore_QNodeId_operator_lt(const Qt3DCore::QNodeId* this_ptr, const Qt3DCore::QNodeId* other);
QT_3D_CORE_C_EXPORT bool qt_3d_core_c_Qt3DCore_QNodeId_operator_neq(const Qt3DCore::QNodeId* this_ptr, const Qt3DCore::QNodeId* other);

} // extern "C"

#endif // QT_3D_CORE_C_QNODEID_H
