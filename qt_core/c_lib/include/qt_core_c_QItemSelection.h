#ifndef QT_CORE_C_QITEMSELECTION_H
#define QT_CORE_C_QITEMSELECTION_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QItemSelection* qt_core_c_QItemSelection_G_static_cast_QItemSelection_ptr(QList< QItemSelectionRange >* ptr);
QT_CORE_C_EXPORT QList< QItemSelectionRange >* qt_core_c_QItemSelection_G_static_cast_QList_QItemSelectionRange_ptr(QItemSelection* ptr);
QT_CORE_C_EXPORT void qt_core_c_QItemSelection_constructor_no_args(QItemSelection* output);
QT_CORE_C_EXPORT void qt_core_c_QItemSelection_constructor_topLeft_bottomRight(const QModelIndex* topLeft, const QModelIndex* bottomRight, QItemSelection* output);
QT_CORE_C_EXPORT bool qt_core_c_QItemSelection_contains(const QItemSelection* this_ptr, const QModelIndex* index);
QT_CORE_C_EXPORT void qt_core_c_QItemSelection_destructor(QItemSelection* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QItemSelection_indexes_to_output(const QItemSelection* this_ptr, QList< QModelIndex >* output);
QT_CORE_C_EXPORT void qt_core_c_QItemSelection_select(QItemSelection* this_ptr, const QModelIndex* topLeft, const QModelIndex* bottomRight);
QT_CORE_C_EXPORT void qt_core_c_QItemSelection_split(const QItemSelectionRange* range, const QItemSelectionRange* other, QItemSelection* result);

} // extern "C"

#endif // QT_CORE_C_QITEMSELECTION_H
