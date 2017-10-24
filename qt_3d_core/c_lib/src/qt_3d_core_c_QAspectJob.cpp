#include "qt_3d_core_c_QAspectJob.h"

void qt_3d_core_c_Qt3DCore_QAspectJob_addDependency(Qt3DCore::QAspectJob* this_ptr, const QWeakPointer< Qt3DCore::QAspectJob >* dependency) {
  this_ptr->addDependency(*dependency);
}

void qt_3d_core_c_Qt3DCore_QAspectJob_delete(Qt3DCore::QAspectJob* this_ptr) {
  delete this_ptr;
}

void qt_3d_core_c_Qt3DCore_QAspectJob_dependencies_to_output(const Qt3DCore::QAspectJob* this_ptr, QVector< QWeakPointer< Qt3DCore::QAspectJob > >* output) {
  new(output) QVector< QWeakPointer< Qt3DCore::QAspectJob > >(this_ptr->dependencies());
}

void qt_3d_core_c_Qt3DCore_QAspectJob_removeDependency(Qt3DCore::QAspectJob* this_ptr, const QWeakPointer< Qt3DCore::QAspectJob >* dependency) {
  this_ptr->removeDependency(*dependency);
}

void qt_3d_core_c_Qt3DCore_QAspectJob_run(Qt3DCore::QAspectJob* this_ptr) {
  this_ptr->run();
}

