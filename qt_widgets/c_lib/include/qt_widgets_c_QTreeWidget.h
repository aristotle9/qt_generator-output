#ifndef QT_WIDGETS_C_QTREEWIDGET_H
#define QT_WIDGETS_C_QTREEWIDGET_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QTreeWidget* qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QTreeWidget* qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QTreeWidget* qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QTreeWidget* qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QTreeView(QTreeView* ptr);
QT_WIDGETS_C_EXPORT QTreeWidget* qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QDataStream* qt_widgets_c_QTreeWidget_G_operator_shl(QDataStream* out, const QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT QDataStream* qt_widgets_c_QTreeWidget_G_operator_shr(QDataStream* in, QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT QAbstractItemView* qt_widgets_c_QTreeWidget_G_static_cast_QAbstractItemView_ptr(QTreeWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractScrollArea* qt_widgets_c_QTreeWidget_G_static_cast_QAbstractScrollArea_ptr(QTreeWidget* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QTreeWidget_G_static_cast_QFrame_ptr(QTreeWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QTreeWidget_G_static_cast_QObject_ptr(QTreeWidget* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QTreeWidget_G_static_cast_QPaintDevice_ptr(QTreeWidget* ptr);
QT_WIDGETS_C_EXPORT QTreeView* qt_widgets_c_QTreeWidget_G_static_cast_QTreeView_ptr(QTreeWidget* ptr);
QT_WIDGETS_C_EXPORT QTreeWidget* qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QTreeWidget* qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QTreeWidget* qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QTreeWidget* qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QTreeWidget* qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QTreeWidget* qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QTreeView(QTreeView* ptr);
QT_WIDGETS_C_EXPORT QTreeWidget* qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QTreeWidget_G_static_cast_QWidget_ptr(QTreeWidget* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_addTopLevelItem(QTreeWidget* this_ptr, QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_addTopLevelItems(QTreeWidget* this_ptr, const QList< QTreeWidgetItem* >* items);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_clear(QTreeWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_closePersistentEditor_item(QTreeWidget* this_ptr, QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_closePersistentEditor_item_column(QTreeWidget* this_ptr, QTreeWidgetItem* item, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_collapseItem(QTreeWidget* this_ptr, const QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeWidget_columnCount(const QTreeWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeWidget_currentColumn(const QTreeWidget* this_ptr);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidget_currentItem(const QTreeWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_delete(QTreeWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_editItem_item(QTreeWidget* this_ptr, QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_editItem_item_column(QTreeWidget* this_ptr, QTreeWidgetItem* item, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_expandItem(QTreeWidget* this_ptr, const QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidget_headerItem(const QTreeWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeWidget_indexOfTopLevelItem(const QTreeWidget* this_ptr, QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_insertTopLevelItem(QTreeWidget* this_ptr, int index, QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_insertTopLevelItems(QTreeWidget* this_ptr, int index, const QList< QTreeWidgetItem* >* items);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidget_invisibleRootItem(const QTreeWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeWidget_isFirstItemColumnSpanned(const QTreeWidget* this_ptr, const QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeWidget_isItemExpanded(const QTreeWidget* this_ptr, const QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeWidget_isItemHidden(const QTreeWidget* this_ptr, const QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeWidget_isItemSelected(const QTreeWidget* this_ptr, const QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidget_itemAbove(const QTreeWidget* this_ptr, const QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidget_itemAt_p(const QTreeWidget* this_ptr, const QPoint* p);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidget_itemAt_x_y(const QTreeWidget* this_ptr, int x, int y);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidget_itemBelow(const QTreeWidget* this_ptr, const QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QTreeWidget_itemWidget(const QTreeWidget* this_ptr, QTreeWidgetItem* item, int column);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QTreeWidget_metaObject(const QTreeWidget* this_ptr);
QT_WIDGETS_C_EXPORT QTreeWidget* qt_widgets_c_QTreeWidget_new_no_args();
QT_WIDGETS_C_EXPORT QTreeWidget* qt_widgets_c_QTreeWidget_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_openPersistentEditor_item(QTreeWidget* this_ptr, QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_openPersistentEditor_item_column(QTreeWidget* this_ptr, QTreeWidgetItem* item, int column);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeWidget_qt_metacall(QTreeWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QTreeWidget_qt_metacast(QTreeWidget* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_removeItemWidget(QTreeWidget* this_ptr, QTreeWidgetItem* item, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_scrollToItem_item(QTreeWidget* this_ptr, const QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_scrollToItem_item_hint(QTreeWidget* this_ptr, const QTreeWidgetItem* item, const QAbstractItemView::ScrollHint* hint);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_selectedItems_to_output(const QTreeWidget* this_ptr, QList< QTreeWidgetItem* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_setColumnCount(QTreeWidget* this_ptr, int columns);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_setCurrentItem_item(QTreeWidget* this_ptr, QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_setCurrentItem_item_column(QTreeWidget* this_ptr, QTreeWidgetItem* item, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_setFirstItemColumnSpanned(QTreeWidget* this_ptr, const QTreeWidgetItem* item, bool span);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_setHeaderItem(QTreeWidget* this_ptr, QTreeWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_setHeaderLabel(QTreeWidget* this_ptr, const QString* label);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_setHeaderLabels(QTreeWidget* this_ptr, const QStringList* labels);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_setItemExpanded(QTreeWidget* this_ptr, const QTreeWidgetItem* item, bool expand);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_setItemHidden(QTreeWidget* this_ptr, const QTreeWidgetItem* item, bool hide);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_setItemSelected(QTreeWidget* this_ptr, const QTreeWidgetItem* item, bool select);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_setItemWidget(QTreeWidget* this_ptr, QTreeWidgetItem* item, int column, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_setSelectionModel(QTreeWidget* this_ptr, QItemSelectionModel* selectionModel);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeWidget_sortColumn(const QTreeWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_sortItems(QTreeWidget* this_ptr, int column, const Qt::SortOrder* order);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidget_takeTopLevelItem(QTreeWidget* this_ptr, int index);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidget_topLevelItem(const QTreeWidget* this_ptr, int index);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeWidget_topLevelItemCount(const QTreeWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidget_visualItemRect_to_output(const QTreeWidget* this_ptr, const QTreeWidgetItem* item, QRect* output);

} // extern "C"

#endif // QT_WIDGETS_C_QTREEWIDGET_H
