#ifndef QT_WIDGETS_C_QTREEVIEW_H
#define QT_WIDGETS_C_QTREEVIEW_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QTreeView* qt_widgets_c_QTreeView_G_dynamic_cast_QTreeView_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QTreeView* qt_widgets_c_QTreeView_G_dynamic_cast_QTreeView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QTreeView* qt_widgets_c_QTreeView_G_dynamic_cast_QTreeView_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QTreeView* qt_widgets_c_QTreeView_G_dynamic_cast_QTreeView_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractItemView* qt_widgets_c_QTreeView_G_static_cast_QAbstractItemView_ptr(QTreeView* ptr);
QT_WIDGETS_C_EXPORT QAbstractScrollArea* qt_widgets_c_QTreeView_G_static_cast_QAbstractScrollArea_ptr(QTreeView* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QTreeView_G_static_cast_QFrame_ptr(QTreeView* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QTreeView_G_static_cast_QObject_ptr(QTreeView* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QTreeView_G_static_cast_QPaintDevice_ptr(QTreeView* ptr);
QT_WIDGETS_C_EXPORT QTreeView* qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QTreeView* qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QTreeView* qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QTreeView* qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QTreeView* qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QTreeView* qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QTreeView_G_static_cast_QWidget_ptr(QTreeView* ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeView_allColumnsShowFocus(const QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeView_autoExpandDelay(const QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_collapse(QTreeView* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_collapseAll(QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeView_columnAt(const QTreeView* this_ptr, int x);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeView_columnViewportPosition(const QTreeView* this_ptr, int column);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeView_columnWidth(const QTreeView* this_ptr, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_dataChanged_topLeft_bottomRight(QTreeView* this_ptr, const QModelIndex* topLeft, const QModelIndex* bottomRight);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_dataChanged_topLeft_bottomRight_roles(QTreeView* this_ptr, const QModelIndex* topLeft, const QModelIndex* bottomRight, const QVector< int >* roles);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_delete(QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_doItemsLayout(QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_expand(QTreeView* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_expandAll(QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_expandToDepth(QTreeView* this_ptr, int depth);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeView_expandsOnDoubleClick(const QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT QHeaderView* qt_widgets_c_QTreeView_header(const QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_hideColumn(QTreeView* this_ptr, int column);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeView_indentation(const QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_indexAbove_to_output(const QTreeView* this_ptr, const QModelIndex* index, QModelIndex* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_indexAt_to_output(const QTreeView* this_ptr, const QPoint* p, QModelIndex* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_indexBelow_to_output(const QTreeView* this_ptr, const QModelIndex* index, QModelIndex* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeView_isAnimated(const QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeView_isColumnHidden(const QTreeView* this_ptr, int column);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeView_isExpanded(const QTreeView* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeView_isFirstColumnSpanned(const QTreeView* this_ptr, int row, const QModelIndex* parent);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeView_isHeaderHidden(const QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeView_isRowHidden(const QTreeView* this_ptr, int row, const QModelIndex* parent);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeView_isSortingEnabled(const QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeView_itemsExpandable(const QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_keyboardSearch(QTreeView* this_ptr, const QString* search);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QTreeView_metaObject(const QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT QTreeView* qt_widgets_c_QTreeView_new_no_args();
QT_WIDGETS_C_EXPORT QTreeView* qt_widgets_c_QTreeView_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeView_qt_metacall(QTreeView* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QTreeView_qt_metacast(QTreeView* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_reset(QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_resetIndentation(QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_resizeColumnToContents(QTreeView* this_ptr, int column);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeView_rootIsDecorated(const QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_scrollTo_index(QTreeView* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_scrollTo_index_hint(QTreeView* this_ptr, const QModelIndex* index, QAbstractItemView::ScrollHint hint);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_selectAll(QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setAllColumnsShowFocus(QTreeView* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setAnimated(QTreeView* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setAutoExpandDelay(QTreeView* this_ptr, int delay);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setColumnHidden(QTreeView* this_ptr, int column, bool hide);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setColumnWidth(QTreeView* this_ptr, int column, int width);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setExpanded(QTreeView* this_ptr, const QModelIndex* index, bool expand);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setExpandsOnDoubleClick(QTreeView* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setFirstColumnSpanned(QTreeView* this_ptr, int row, const QModelIndex* parent, bool span);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setHeader(QTreeView* this_ptr, QHeaderView* header);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setHeaderHidden(QTreeView* this_ptr, bool hide);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setIndentation(QTreeView* this_ptr, int i);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setItemsExpandable(QTreeView* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setModel(QTreeView* this_ptr, QAbstractItemModel* model);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setRootIndex(QTreeView* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setRootIsDecorated(QTreeView* this_ptr, bool show);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setRowHidden(QTreeView* this_ptr, int row, const QModelIndex* parent, bool hide);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setSelectionModel(QTreeView* this_ptr, QItemSelectionModel* selectionModel);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setSortingEnabled(QTreeView* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setTreePosition(QTreeView* this_ptr, int logicalIndex);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setUniformRowHeights(QTreeView* this_ptr, bool uniform);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_setWordWrap(QTreeView* this_ptr, bool on);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_showColumn(QTreeView* this_ptr, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_sortByColumn_column(QTreeView* this_ptr, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_sortByColumn_column_order(QTreeView* this_ptr, int column, const Qt::SortOrder* order);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeView_treePosition(const QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeView_uniformRowHeights(const QTreeView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeView_visualRect_to_output(const QTreeView* this_ptr, const QModelIndex* index, QRect* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeView_wordWrap(const QTreeView* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QTREEVIEW_H
