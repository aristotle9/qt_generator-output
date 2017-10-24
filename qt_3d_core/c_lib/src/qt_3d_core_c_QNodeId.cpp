#include "qt_3d_core_c_QNodeId.h"

unsigned int qt_3d_core_c_QNodeId_G_Qt3DCore_qHash_id(const Qt3DCore::QNodeId* id) {
  return Qt3DCore::qHash(*id);
}

unsigned int qt_3d_core_c_QNodeId_G_Qt3DCore_qHash_id_seed(const Qt3DCore::QNodeId* id, unsigned int seed) {
  return Qt3DCore::qHash(*id, seed);
}

void qt_3d_core_c_QNodeId_G_operator_shl_to_output(const QDebug* d, const Qt3DCore::QNodeId* id, QDebug* output) {
  new(output) QDebug(Qt3DCore::operator<<(*d, *id));
}

void qt_3d_core_c_Qt3DCore_QNodeId_constructor(Qt3DCore::QNodeId* output) {
  new(output) Qt3DCore::QNodeId();
}

bool qt_3d_core_c_Qt3DCore_QNodeId_convert_to_bool(const Qt3DCore::QNodeId* this_ptr) {
  return this_ptr->operator bool();
}

void qt_3d_core_c_Qt3DCore_QNodeId_createId_to_output(Qt3DCore::QNodeId* output) {
  new(output) Qt3DCore::QNodeId(Qt3DCore::QNodeId::createId());
}

void qt_3d_core_c_Qt3DCore_QNodeId_destructor(Qt3DCore::QNodeId* this_ptr) {
  qt_3d_core_c_call_destructor(this_ptr);
}

quint64 qt_3d_core_c_Qt3DCore_QNodeId_id(const Qt3DCore::QNodeId* this_ptr) {
  return this_ptr->id();
}

bool qt_3d_core_c_Qt3DCore_QNodeId_isNull(const Qt3DCore::QNodeId* this_ptr) {
  return this_ptr->isNull();
}

bool qt_3d_core_c_Qt3DCore_QNodeId_operator_eq(const Qt3DCore::QNodeId* this_ptr, const Qt3DCore::QNodeId* other) {
  return this_ptr->operator==(*other);
}

bool qt_3d_core_c_Qt3DCore_QNodeId_operator_gt(const Qt3DCore::QNodeId* this_ptr, const Qt3DCore::QNodeId* other) {
  return this_ptr->operator>(*other);
}

bool qt_3d_core_c_Qt3DCore_QNodeId_operator_lt(const Qt3DCore::QNodeId* this_ptr, const Qt3DCore::QNodeId* other) {
  return this_ptr->operator<(*other);
}

bool qt_3d_core_c_Qt3DCore_QNodeId_operator_neq(const Qt3DCore::QNodeId* this_ptr, const Qt3DCore::QNodeId* other) {
  return this_ptr->operator!=(*other);
}

