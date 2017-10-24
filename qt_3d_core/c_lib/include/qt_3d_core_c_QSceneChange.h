#ifndef QT_3D_CORE_C_QSCENECHANGE_H
#define QT_3D_CORE_C_QSCENECHANGE_H

#include "qt_3d_core_c_global.h"

extern "C" {

QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QSceneChange_delete(Qt3DCore::QSceneChange* this_ptr);
QT_3D_CORE_C_EXPORT unsigned int qt_3d_core_c_Qt3DCore_QSceneChange_deliveryFlags(const Qt3DCore::QSceneChange* this_ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QSceneChange_setDeliveryFlags(Qt3DCore::QSceneChange* this_ptr, unsigned int flags);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QSceneChange_subjectId_to_output(const Qt3DCore::QSceneChange* this_ptr, Qt3DCore::QNodeId* output);
QT_3D_CORE_C_EXPORT Qt3DCore::ChangeFlag qt_3d_core_c_Qt3DCore_QSceneChange_type(const Qt3DCore::QSceneChange* this_ptr);

} // extern "C"

#endif // QT_3D_CORE_C_QSCENECHANGE_H
