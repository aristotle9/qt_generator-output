#include "qt_3d_core_c_QWeakPointer.h"

void qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_clear(QWeakPointer< Qt3DCore::QAspectJob >* this_ptr) {
  this_ptr->clear();
}

void qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_constructor_no_args(QWeakPointer< Qt3DCore::QAspectJob >* output) {
  new(output) QWeakPointer< Qt3DCore::QAspectJob >();
}

void qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_constructor_o(const QSharedPointer< Qt3DCore::QAspectJob >* o, QWeakPointer< Qt3DCore::QAspectJob >* output) {
  new(output) QWeakPointer< Qt3DCore::QAspectJob >(*o);
}

void qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_constructor_other(const QWeakPointer< Qt3DCore::QAspectJob >* other, QWeakPointer< Qt3DCore::QAspectJob >* output) {
  new(output) QWeakPointer< Qt3DCore::QAspectJob >(*other);
}

Qt3DCore::QAspectJob* qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_data(const QWeakPointer< Qt3DCore::QAspectJob >* this_ptr) {
  return this_ptr->data();
}

void qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_destructor(QWeakPointer< Qt3DCore::QAspectJob >* this_ptr) {
  qt_3d_core_c_call_destructor(this_ptr);
}

bool qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_isNull(const QWeakPointer< Qt3DCore::QAspectJob >* this_ptr) {
  return this_ptr->isNull();
}

void qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_lock_to_output(const QWeakPointer< Qt3DCore::QAspectJob >* this_ptr, QSharedPointer< Qt3DCore::QAspectJob >* output) {
  new(output) QSharedPointer< Qt3DCore::QAspectJob >(this_ptr->lock());
}

QWeakPointer< Qt3DCore::QAspectJob >* qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_operator_assign_o(QWeakPointer< Qt3DCore::QAspectJob >* this_ptr, const QSharedPointer< Qt3DCore::QAspectJob >* o) {
  return &this_ptr->operator=(*o);
}

QWeakPointer< Qt3DCore::QAspectJob >* qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_operator_assign_other(QWeakPointer< Qt3DCore::QAspectJob >* this_ptr, const QWeakPointer< Qt3DCore::QAspectJob >* other) {
  return &this_ptr->operator=(*other);
}

bool qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_operator_not(const QWeakPointer< Qt3DCore::QAspectJob >* this_ptr) {
  return this_ptr->operator!();
}

void qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_swap(QWeakPointer< Qt3DCore::QAspectJob >* this_ptr, QWeakPointer< Qt3DCore::QAspectJob >* other) {
  this_ptr->swap(*other);
}

void qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_toStrongRef_to_output(const QWeakPointer< Qt3DCore::QAspectJob >* this_ptr, QSharedPointer< Qt3DCore::QAspectJob >* output) {
  new(output) QSharedPointer< Qt3DCore::QAspectJob >(this_ptr->toStrongRef());
}

