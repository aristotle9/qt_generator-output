#ifndef QT_CORE_C_QITEMSELECTIONMODEL_H
#define QT_CORE_C_QITEMSELECTIONMODEL_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QItemSelectionModel* qt_core_c_QItemSelectionModel_G_dynamic_cast_QItemSelectionModel_ptr(QObject* ptr);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionModel_G_operator_shl_to_output(const QDebug* arg1, const QItemSelectionRange* arg2, QDebug* output);
QT_CORE_C_EXPORT unsigned int qt_core_c_QItemSelectionModel_G_qHash(const QItemSelectionRange* arg1);
QT_CORE_C_EXPORT QItemSelectionModel* qt_core_c_QItemSelectionModel_G_static_cast_QItemSelectionModel_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QItemSelectionModel_G_static_cast_QObject_ptr(QItemSelectionModel* ptr);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionModel_G_swap(QItemSelection* value1, QItemSelection* value2);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionModel_clear(QItemSelectionModel* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionModel_clearCurrentIndex(QItemSelectionModel* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionModel_clearSelection(QItemSelectionModel* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QItemSelectionModel_columnIntersectsSelection(const QItemSelectionModel* this_ptr, int column, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionModel_currentIndex_to_output(const QItemSelectionModel* this_ptr, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionModel_delete(QItemSelectionModel* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QItemSelectionModel_hasSelection(const QItemSelectionModel* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QItemSelectionModel_isColumnSelected(const QItemSelectionModel* this_ptr, int column, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QItemSelectionModel_isRowSelected(const QItemSelectionModel* this_ptr, int row, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QItemSelectionModel_isSelected(const QItemSelectionModel* this_ptr, const QModelIndex* index);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QItemSelectionModel_metaObject(const QItemSelectionModel* this_ptr);
QT_CORE_C_EXPORT QAbstractItemModel* qt_core_c_QItemSelectionModel_model(QItemSelectionModel* this_ptr);
QT_CORE_C_EXPORT const QAbstractItemModel* qt_core_c_QItemSelectionModel_model_const(const QItemSelectionModel* this_ptr);
QT_CORE_C_EXPORT QItemSelectionModel* qt_core_c_QItemSelectionModel_new_model(QAbstractItemModel* model);
QT_CORE_C_EXPORT QItemSelectionModel* qt_core_c_QItemSelectionModel_new_model_parent(QAbstractItemModel* model, QObject* parent);
QT_CORE_C_EXPORT QItemSelectionModel* qt_core_c_QItemSelectionModel_new_no_args();
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionModel_reset(QItemSelectionModel* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QItemSelectionModel_rowIntersectsSelection(const QItemSelectionModel* this_ptr, int row, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionModel_selectedColumns_to_output_no_args(const QItemSelectionModel* this_ptr, QList< QModelIndex >* output);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionModel_selectedColumns_to_output_row(const QItemSelectionModel* this_ptr, int row, QList< QModelIndex >* output);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionModel_selectedIndexes_to_output(const QItemSelectionModel* this_ptr, QList< QModelIndex >* output);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionModel_selectedRows_to_output_column(const QItemSelectionModel* this_ptr, int column, QList< QModelIndex >* output);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionModel_selectedRows_to_output_no_args(const QItemSelectionModel* this_ptr, QList< QModelIndex >* output);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionModel_selection_to_output(const QItemSelectionModel* this_ptr, QItemSelection* output);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionModel_setModel(QItemSelectionModel* this_ptr, QAbstractItemModel* model);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QItemSelectionModel_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QITEMSELECTIONMODEL_H
