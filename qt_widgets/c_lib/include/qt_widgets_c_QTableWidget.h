#ifndef QT_WIDGETS_C_QTABLEWIDGET_H
#define QT_WIDGETS_C_QTABLEWIDGET_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QTableWidget* qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QTableWidget* qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QTableWidget* qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QTableWidget* qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QTableView(QTableView* ptr);
QT_WIDGETS_C_EXPORT QTableWidget* qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QDataStream* qt_widgets_c_QTableWidget_G_operator_shl(QDataStream* out, const QTableWidgetItem* item);
QT_WIDGETS_C_EXPORT QDataStream* qt_widgets_c_QTableWidget_G_operator_shr(QDataStream* in, QTableWidgetItem* item);
QT_WIDGETS_C_EXPORT QAbstractItemView* qt_widgets_c_QTableWidget_G_static_cast_QAbstractItemView_ptr(QTableWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractScrollArea* qt_widgets_c_QTableWidget_G_static_cast_QAbstractScrollArea_ptr(QTableWidget* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QTableWidget_G_static_cast_QFrame_ptr(QTableWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QTableWidget_G_static_cast_QObject_ptr(QTableWidget* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QTableWidget_G_static_cast_QPaintDevice_ptr(QTableWidget* ptr);
QT_WIDGETS_C_EXPORT QTableView* qt_widgets_c_QTableWidget_G_static_cast_QTableView_ptr(QTableWidget* ptr);
QT_WIDGETS_C_EXPORT QTableWidget* qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QTableWidget* qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QTableWidget* qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QTableWidget* qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QTableWidget* qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QTableWidget* qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QTableView(QTableView* ptr);
QT_WIDGETS_C_EXPORT QTableWidget* qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QTableWidget_G_static_cast_QWidget_ptr(QTableWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QTableWidget_cellWidget(const QTableWidget* this_ptr, int row, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_clear(QTableWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_clearContents(QTableWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_closePersistentEditor(QTableWidget* this_ptr, QTableWidgetItem* item);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidget_column(const QTableWidget* this_ptr, const QTableWidgetItem* item);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidget_columnCount(const QTableWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidget_currentColumn(const QTableWidget* this_ptr);
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidget_currentItem(const QTableWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidget_currentRow(const QTableWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_delete(QTableWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_editItem(QTableWidget* this_ptr, QTableWidgetItem* item);
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidget_horizontalHeaderItem(const QTableWidget* this_ptr, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_insertColumn(QTableWidget* this_ptr, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_insertRow(QTableWidget* this_ptr, int row);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTableWidget_isItemSelected(const QTableWidget* this_ptr, const QTableWidgetItem* item);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTableWidget_isSortingEnabled(const QTableWidget* this_ptr);
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidget_item(const QTableWidget* this_ptr, int row, int column);
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidget_itemAt_p(const QTableWidget* this_ptr, const QPoint* p);
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidget_itemAt_x_y(const QTableWidget* this_ptr, int x, int y);
QT_WIDGETS_C_EXPORT const QTableWidgetItem* qt_widgets_c_QTableWidget_itemPrototype(const QTableWidget* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QTableWidget_metaObject(const QTableWidget* this_ptr);
QT_WIDGETS_C_EXPORT QTableWidget* qt_widgets_c_QTableWidget_new_no_args();
QT_WIDGETS_C_EXPORT QTableWidget* qt_widgets_c_QTableWidget_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT QTableWidget* qt_widgets_c_QTableWidget_new_rows_columns(int rows, int columns);
QT_WIDGETS_C_EXPORT QTableWidget* qt_widgets_c_QTableWidget_new_rows_columns_parent(int rows, int columns, QWidget* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_openPersistentEditor(QTableWidget* this_ptr, QTableWidgetItem* item);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidget_qt_metacall(QTableWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QTableWidget_qt_metacast(QTableWidget* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_removeCellWidget(QTableWidget* this_ptr, int row, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_removeColumn(QTableWidget* this_ptr, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_removeRow(QTableWidget* this_ptr, int row);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidget_row(const QTableWidget* this_ptr, const QTableWidgetItem* item);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidget_rowCount(const QTableWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_scrollToItem_item(QTableWidget* this_ptr, const QTableWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_scrollToItem_item_hint(QTableWidget* this_ptr, const QTableWidgetItem* item, const QAbstractItemView::ScrollHint* hint);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_selectedItems_to_output(const QTableWidget* this_ptr, QList< QTableWidgetItem* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_selectedRanges_to_output(const QTableWidget* this_ptr, QList< QTableWidgetSelectionRange >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_setCellWidget(QTableWidget* this_ptr, int row, int column, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_setColumnCount(QTableWidget* this_ptr, int columns);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_setCurrentCell(QTableWidget* this_ptr, int row, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_setCurrentItem(QTableWidget* this_ptr, QTableWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_setHorizontalHeaderItem(QTableWidget* this_ptr, int column, QTableWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_setHorizontalHeaderLabels(QTableWidget* this_ptr, const QStringList* labels);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_setItem(QTableWidget* this_ptr, int row, int column, QTableWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_setItemPrototype(QTableWidget* this_ptr, const QTableWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_setItemSelected(QTableWidget* this_ptr, const QTableWidgetItem* item, bool select);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_setRangeSelected(QTableWidget* this_ptr, const QTableWidgetSelectionRange* range, bool select);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_setRowCount(QTableWidget* this_ptr, int rows);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_setSortingEnabled(QTableWidget* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_setVerticalHeaderItem(QTableWidget* this_ptr, int row, QTableWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_setVerticalHeaderLabels(QTableWidget* this_ptr, const QStringList* labels);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_sortItems_column(QTableWidget* this_ptr, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_sortItems_column_order(QTableWidget* this_ptr, int column, const Qt::SortOrder* order);
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidget_takeHorizontalHeaderItem(QTableWidget* this_ptr, int column);
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidget_takeItem(QTableWidget* this_ptr, int row, int column);
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidget_takeVerticalHeaderItem(QTableWidget* this_ptr, int row);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidget_verticalHeaderItem(const QTableWidget* this_ptr, int row);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidget_visualColumn(const QTableWidget* this_ptr, int logicalColumn);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidget_visualItemRect_to_output(const QTableWidget* this_ptr, const QTableWidgetItem* item, QRect* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidget_visualRow(const QTableWidget* this_ptr, int logicalRow);

} // extern "C"

#endif // QT_WIDGETS_C_QTABLEWIDGET_H
