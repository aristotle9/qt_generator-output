#include "qt_3d_core_c_QSceneChange.h"

void qt_3d_core_c_Qt3DCore_QSceneChange_delete(Qt3DCore::QSceneChange* this_ptr) {
  delete this_ptr;
}

unsigned int qt_3d_core_c_Qt3DCore_QSceneChange_deliveryFlags(const Qt3DCore::QSceneChange* this_ptr) {
  return uint(this_ptr->deliveryFlags());
}

void qt_3d_core_c_Qt3DCore_QSceneChange_setDeliveryFlags(Qt3DCore::QSceneChange* this_ptr, unsigned int flags) {
  this_ptr->setDeliveryFlags(QFlags< Qt3DCore::QSceneChange::DeliveryFlag >(flags));
}

void qt_3d_core_c_Qt3DCore_QSceneChange_subjectId_to_output(const Qt3DCore::QSceneChange* this_ptr, Qt3DCore::QNodeId* output) {
  new(output) Qt3DCore::QNodeId(this_ptr->subjectId());
}

Qt3DCore::ChangeFlag qt_3d_core_c_Qt3DCore_QSceneChange_type(const Qt3DCore::QSceneChange* this_ptr) {
  return this_ptr->type();
}

