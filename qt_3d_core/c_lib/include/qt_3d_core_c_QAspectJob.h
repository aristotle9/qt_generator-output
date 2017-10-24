#ifndef QT_3D_CORE_C_QASPECTJOB_H
#define QT_3D_CORE_C_QASPECTJOB_H

#include "qt_3d_core_c_global.h"

extern "C" {

QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QAspectJob_addDependency(Qt3DCore::QAspectJob* this_ptr, const QWeakPointer< Qt3DCore::QAspectJob >* dependency);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QAspectJob_delete(Qt3DCore::QAspectJob* this_ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QAspectJob_dependencies_to_output(const Qt3DCore::QAspectJob* this_ptr, QVector< QWeakPointer< Qt3DCore::QAspectJob > >* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QAspectJob_removeDependency(Qt3DCore::QAspectJob* this_ptr, const QWeakPointer< Qt3DCore::QAspectJob >* dependency);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QAspectJob_run(Qt3DCore::QAspectJob* this_ptr);

} // extern "C"

#endif // QT_3D_CORE_C_QASPECTJOB_H
