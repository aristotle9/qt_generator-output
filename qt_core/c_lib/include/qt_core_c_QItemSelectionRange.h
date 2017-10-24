#ifndef QT_CORE_C_QITEMSELECTIONRANGE_H
#define QT_CORE_C_QITEMSELECTIONRANGE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT int qt_core_c_QItemSelectionRange_bottom(const QItemSelectionRange* this_ptr);
QT_CORE_C_EXPORT const QPersistentModelIndex* qt_core_c_QItemSelectionRange_bottomRight(const QItemSelectionRange* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionRange_constructor_index(const QModelIndex* index, QItemSelectionRange* output);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionRange_constructor_no_args(QItemSelectionRange* output);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionRange_constructor_other(const QItemSelectionRange* other, QItemSelectionRange* output);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionRange_constructor_topL_bottomR(const QModelIndex* topL, const QModelIndex* bottomR, QItemSelectionRange* output);
QT_CORE_C_EXPORT bool qt_core_c_QItemSelectionRange_contains_index(const QItemSelectionRange* this_ptr, const QModelIndex* index);
QT_CORE_C_EXPORT bool qt_core_c_QItemSelectionRange_contains_row_column_parentIndex(const QItemSelectionRange* this_ptr, int row, int column, const QModelIndex* parentIndex);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionRange_destructor(QItemSelectionRange* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QItemSelectionRange_height(const QItemSelectionRange* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionRange_indexes_to_output(const QItemSelectionRange* this_ptr, QList< QModelIndex >* output);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionRange_intersected_to_output(const QItemSelectionRange* this_ptr, const QItemSelectionRange* other, QItemSelectionRange* output);
QT_CORE_C_EXPORT bool qt_core_c_QItemSelectionRange_intersects(const QItemSelectionRange* this_ptr, const QItemSelectionRange* other);
QT_CORE_C_EXPORT bool qt_core_c_QItemSelectionRange_isEmpty(const QItemSelectionRange* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QItemSelectionRange_isValid(const QItemSelectionRange* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QItemSelectionRange_left(const QItemSelectionRange* this_ptr);
QT_CORE_C_EXPORT const QAbstractItemModel* qt_core_c_QItemSelectionRange_model(const QItemSelectionRange* this_ptr);
QT_CORE_C_EXPORT QItemSelectionRange* qt_core_c_QItemSelectionRange_operator_assign(QItemSelectionRange* this_ptr, const QItemSelectionRange* other);
QT_CORE_C_EXPORT bool qt_core_c_QItemSelectionRange_operator_eq(const QItemSelectionRange* this_ptr, const QItemSelectionRange* other);
QT_CORE_C_EXPORT bool qt_core_c_QItemSelectionRange_operator_lt(const QItemSelectionRange* this_ptr, const QItemSelectionRange* other);
QT_CORE_C_EXPORT bool qt_core_c_QItemSelectionRange_operator_neq(const QItemSelectionRange* this_ptr, const QItemSelectionRange* other);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionRange_parent_to_output(const QItemSelectionRange* this_ptr, QModelIndex* output);
QT_CORE_C_EXPORT int qt_core_c_QItemSelectionRange_right(const QItemSelectionRange* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionRange_swap(QItemSelectionRange* this_ptr, QItemSelectionRange* other);
QT_CORE_C_EXPORT int qt_core_c_QItemSelectionRange_top(const QItemSelectionRange* this_ptr);
QT_CORE_C_EXPORT const QPersistentModelIndex* qt_core_c_QItemSelectionRange_topLeft(const QItemSelectionRange* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QItemSelectionRange_width(const QItemSelectionRange* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QITEMSELECTIONRANGE_H
