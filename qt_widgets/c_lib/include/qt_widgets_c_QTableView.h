#ifndef QT_WIDGETS_C_QTABLEVIEW_H
#define QT_WIDGETS_C_QTABLEVIEW_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QTableView* qt_widgets_c_QTableView_G_dynamic_cast_QTableView_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QTableView* qt_widgets_c_QTableView_G_dynamic_cast_QTableView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QTableView* qt_widgets_c_QTableView_G_dynamic_cast_QTableView_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QTableView* qt_widgets_c_QTableView_G_dynamic_cast_QTableView_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractItemView* qt_widgets_c_QTableView_G_static_cast_QAbstractItemView_ptr(QTableView* ptr);
QT_WIDGETS_C_EXPORT QAbstractScrollArea* qt_widgets_c_QTableView_G_static_cast_QAbstractScrollArea_ptr(QTableView* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QTableView_G_static_cast_QFrame_ptr(QTableView* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QTableView_G_static_cast_QObject_ptr(QTableView* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QTableView_G_static_cast_QPaintDevice_ptr(QTableView* ptr);
QT_WIDGETS_C_EXPORT QTableView* qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QTableView* qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QTableView* qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QTableView* qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QTableView* qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QTableView* qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QTableView_G_static_cast_QWidget_ptr(QTableView* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_clearSpans(QTableView* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableView_columnAt(const QTableView* this_ptr, int x);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableView_columnSpan(const QTableView* this_ptr, int row, int column);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableView_columnViewportPosition(const QTableView* this_ptr, int column);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableView_columnWidth(const QTableView* this_ptr, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_delete(QTableView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_doItemsLayout(QTableView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_hideColumn(QTableView* this_ptr, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_hideRow(QTableView* this_ptr, int row);
QT_WIDGETS_C_EXPORT QHeaderView* qt_widgets_c_QTableView_horizontalHeader(const QTableView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_indexAt_to_output(const QTableView* this_ptr, const QPoint* p, QModelIndex* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTableView_isColumnHidden(const QTableView* this_ptr, int column);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTableView_isCornerButtonEnabled(const QTableView* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTableView_isRowHidden(const QTableView* this_ptr, int row);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTableView_isSortingEnabled(const QTableView* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QTableView_metaObject(const QTableView* this_ptr);
QT_WIDGETS_C_EXPORT QTableView* qt_widgets_c_QTableView_new_no_args();
QT_WIDGETS_C_EXPORT QTableView* qt_widgets_c_QTableView_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableView_qt_metacall(QTableView* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QTableView_qt_metacast(QTableView* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_resizeColumnToContents(QTableView* this_ptr, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_resizeColumnsToContents(QTableView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_resizeRowToContents(QTableView* this_ptr, int row);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_resizeRowsToContents(QTableView* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableView_rowAt(const QTableView* this_ptr, int y);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableView_rowHeight(const QTableView* this_ptr, int row);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableView_rowSpan(const QTableView* this_ptr, int row, int column);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableView_rowViewportPosition(const QTableView* this_ptr, int row);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_scrollTo_index(QTableView* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_scrollTo_index_hint(QTableView* this_ptr, const QModelIndex* index, QAbstractItemView::ScrollHint hint);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_selectColumn(QTableView* this_ptr, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_selectRow(QTableView* this_ptr, int row);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_setColumnHidden(QTableView* this_ptr, int column, bool hide);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_setColumnWidth(QTableView* this_ptr, int column, int width);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_setCornerButtonEnabled(QTableView* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_setGridStyle(QTableView* this_ptr, const Qt::PenStyle* style);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_setHorizontalHeader(QTableView* this_ptr, QHeaderView* header);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_setModel(QTableView* this_ptr, QAbstractItemModel* model);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_setRootIndex(QTableView* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_setRowHeight(QTableView* this_ptr, int row, int height);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_setRowHidden(QTableView* this_ptr, int row, bool hide);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_setSelectionModel(QTableView* this_ptr, QItemSelectionModel* selectionModel);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_setShowGrid(QTableView* this_ptr, bool show);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_setSortingEnabled(QTableView* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_setSpan(QTableView* this_ptr, int row, int column, int rowSpan, int columnSpan);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_setVerticalHeader(QTableView* this_ptr, QHeaderView* header);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_setWordWrap(QTableView* this_ptr, bool on);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_showColumn(QTableView* this_ptr, int column);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTableView_showGrid(const QTableView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_showRow(QTableView* this_ptr, int row);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_sortByColumn_column(QTableView* this_ptr, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_sortByColumn_column_order(QTableView* this_ptr, int column, const Qt::SortOrder* order);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT QHeaderView* qt_widgets_c_QTableView_verticalHeader(const QTableView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableView_visualRect_to_output(const QTableView* this_ptr, const QModelIndex* index, QRect* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTableView_wordWrap(const QTableView* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QTABLEVIEW_H
