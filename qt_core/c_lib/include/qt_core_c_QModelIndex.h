#ifndef QT_CORE_C_QMODELINDEX_H
#define QT_CORE_C_QMODELINDEX_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QModelIndex_child_to_output(const QModelIndex* this_ptr, int row, int column, QModelIndex* output);
QT_CORE_C_EXPORT int qt_core_c_QModelIndex_column(const QModelIndex* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QModelIndex_constructor(QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QModelIndex_data_to_output_no_args(const QModelIndex* this_ptr, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QModelIndex_data_to_output_role(const QModelIndex* this_ptr, int role, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QModelIndex_destructor(QModelIndex* this_ptr);
QT_CORE_C_EXPORT quintptr qt_core_c_QModelIndex_internalId(const QModelIndex* this_ptr);
QT_CORE_C_EXPORT void* qt_core_c_QModelIndex_internalPointer(const QModelIndex* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QModelIndex_isValid(const QModelIndex* this_ptr);
QT_CORE_C_EXPORT const QAbstractItemModel* qt_core_c_QModelIndex_model(const QModelIndex* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QModelIndex_operator_eq(const QModelIndex* this_ptr, const QModelIndex* other);
QT_CORE_C_EXPORT bool qt_core_c_QModelIndex_operator_lt(const QModelIndex* this_ptr, const QModelIndex* other);
QT_CORE_C_EXPORT bool qt_core_c_QModelIndex_operator_neq(const QModelIndex* this_ptr, const QModelIndex* other);
QT_CORE_C_EXPORT void qt_core_c_QModelIndex_parent_to_output(const QModelIndex* this_ptr, QModelIndex* output);
QT_CORE_C_EXPORT int qt_core_c_QModelIndex_row(const QModelIndex* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QModelIndex_sibling_to_output(const QModelIndex* this_ptr, int row, int column, QModelIndex* output);

} // extern "C"

#endif // QT_CORE_C_QMODELINDEX_H
