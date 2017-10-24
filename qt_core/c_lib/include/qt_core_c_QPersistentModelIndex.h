#ifndef QT_CORE_C_QPERSISTENTMODELINDEX_H
#define QT_CORE_C_QPERSISTENTMODELINDEX_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QPersistentModelIndex_child_to_output(const QPersistentModelIndex* this_ptr, int row, int column, QModelIndex* output);
QT_CORE_C_EXPORT int qt_core_c_QPersistentModelIndex_column(const QPersistentModelIndex* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QPersistentModelIndex_constructor_index(const QModelIndex* index, QPersistentModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QPersistentModelIndex_constructor_no_args(QPersistentModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QPersistentModelIndex_constructor_other(const QPersistentModelIndex* other, QPersistentModelIndex* output);
QT_CORE_C_EXPORT const QModelIndex* qt_core_c_QPersistentModelIndex_convert_to_QModelIndex_ref(const QPersistentModelIndex* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QPersistentModelIndex_data_to_output_no_args(const QPersistentModelIndex* this_ptr, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QPersistentModelIndex_data_to_output_role(const QPersistentModelIndex* this_ptr, int role, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QPersistentModelIndex_destructor(QPersistentModelIndex* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QPersistentModelIndex_isValid(const QPersistentModelIndex* this_ptr);
QT_CORE_C_EXPORT const QAbstractItemModel* qt_core_c_QPersistentModelIndex_model(const QPersistentModelIndex* this_ptr);
QT_CORE_C_EXPORT QPersistentModelIndex* qt_core_c_QPersistentModelIndex_operator_assign_QModelIndex(QPersistentModelIndex* this_ptr, const QModelIndex* other);
QT_CORE_C_EXPORT QPersistentModelIndex* qt_core_c_QPersistentModelIndex_operator_assign_QPersistentModelIndex(QPersistentModelIndex* this_ptr, const QPersistentModelIndex* other);
QT_CORE_C_EXPORT bool qt_core_c_QPersistentModelIndex_operator_eq_QModelIndex(const QPersistentModelIndex* this_ptr, const QModelIndex* other);
QT_CORE_C_EXPORT bool qt_core_c_QPersistentModelIndex_operator_eq_QPersistentModelIndex(const QPersistentModelIndex* this_ptr, const QPersistentModelIndex* other);
QT_CORE_C_EXPORT bool qt_core_c_QPersistentModelIndex_operator_lt(const QPersistentModelIndex* this_ptr, const QPersistentModelIndex* other);
QT_CORE_C_EXPORT bool qt_core_c_QPersistentModelIndex_operator_neq_QModelIndex(const QPersistentModelIndex* this_ptr, const QModelIndex* other);
QT_CORE_C_EXPORT bool qt_core_c_QPersistentModelIndex_operator_neq_QPersistentModelIndex(const QPersistentModelIndex* this_ptr, const QPersistentModelIndex* other);
QT_CORE_C_EXPORT void qt_core_c_QPersistentModelIndex_parent_to_output(const QPersistentModelIndex* this_ptr, QModelIndex* output);
QT_CORE_C_EXPORT int qt_core_c_QPersistentModelIndex_row(const QPersistentModelIndex* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QPersistentModelIndex_sibling_to_output(const QPersistentModelIndex* this_ptr, int row, int column, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QPersistentModelIndex_swap(QPersistentModelIndex* this_ptr, QPersistentModelIndex* other);

} // extern "C"

#endif // QT_CORE_C_QPERSISTENTMODELINDEX_H
