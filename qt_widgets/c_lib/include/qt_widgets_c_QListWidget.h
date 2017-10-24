#ifndef QT_WIDGETS_C_QLISTWIDGET_H
#define QT_WIDGETS_C_QLISTWIDGET_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QListWidget* qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QListWidget* qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QListWidget* qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QListWidget* qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QListView(QListView* ptr);
QT_WIDGETS_C_EXPORT QListWidget* qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QDataStream* qt_widgets_c_QListWidget_G_operator_shl(QDataStream* out, const QListWidgetItem* item);
QT_WIDGETS_C_EXPORT QDataStream* qt_widgets_c_QListWidget_G_operator_shr(QDataStream* in, QListWidgetItem* item);
QT_WIDGETS_C_EXPORT QAbstractItemView* qt_widgets_c_QListWidget_G_static_cast_QAbstractItemView_ptr(QListWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractScrollArea* qt_widgets_c_QListWidget_G_static_cast_QAbstractScrollArea_ptr(QListWidget* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QListWidget_G_static_cast_QFrame_ptr(QListWidget* ptr);
QT_WIDGETS_C_EXPORT QListView* qt_widgets_c_QListWidget_G_static_cast_QListView_ptr(QListWidget* ptr);
QT_WIDGETS_C_EXPORT QListWidget* qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QListWidget* qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QListWidget* qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QListWidget* qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QListView(QListView* ptr);
QT_WIDGETS_C_EXPORT QListWidget* qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QListWidget* qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QListWidget* qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QListWidget_G_static_cast_QObject_ptr(QListWidget* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QListWidget_G_static_cast_QPaintDevice_ptr(QListWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QListWidget_G_static_cast_QWidget_ptr(QListWidget* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_addItem_item(QListWidget* this_ptr, QListWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_addItem_label(QListWidget* this_ptr, const QString* label);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_addItems(QListWidget* this_ptr, const QStringList* labels);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_clear(QListWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_closePersistentEditor(QListWidget* this_ptr, QListWidgetItem* item);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QListWidget_count(const QListWidget* this_ptr);
QT_WIDGETS_C_EXPORT QListWidgetItem* qt_widgets_c_QListWidget_currentItem(const QListWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QListWidget_currentRow(const QListWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_delete(QListWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_dropEvent(QListWidget* this_ptr, QDropEvent* event);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_editItem(QListWidget* this_ptr, QListWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_insertItem_row_item(QListWidget* this_ptr, int row, QListWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_insertItem_row_label(QListWidget* this_ptr, int row, const QString* label);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_insertItems(QListWidget* this_ptr, int row, const QStringList* labels);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QListWidget_isItemHidden(const QListWidget* this_ptr, const QListWidgetItem* item);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QListWidget_isItemSelected(const QListWidget* this_ptr, const QListWidgetItem* item);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QListWidget_isSortingEnabled(const QListWidget* this_ptr);
QT_WIDGETS_C_EXPORT QListWidgetItem* qt_widgets_c_QListWidget_item(const QListWidget* this_ptr, int row);
QT_WIDGETS_C_EXPORT QListWidgetItem* qt_widgets_c_QListWidget_itemAt_p(const QListWidget* this_ptr, const QPoint* p);
QT_WIDGETS_C_EXPORT QListWidgetItem* qt_widgets_c_QListWidget_itemAt_x_y(const QListWidget* this_ptr, int x, int y);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QListWidget_itemWidget(const QListWidget* this_ptr, QListWidgetItem* item);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QListWidget_metaObject(const QListWidget* this_ptr);
QT_WIDGETS_C_EXPORT QListWidget* qt_widgets_c_QListWidget_new_no_args();
QT_WIDGETS_C_EXPORT QListWidget* qt_widgets_c_QListWidget_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_openPersistentEditor(QListWidget* this_ptr, QListWidgetItem* item);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QListWidget_qt_metacall(QListWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QListWidget_qt_metacast(QListWidget* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_removeItemWidget(QListWidget* this_ptr, QListWidgetItem* item);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QListWidget_row(const QListWidget* this_ptr, const QListWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_scrollToItem_item(QListWidget* this_ptr, const QListWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_scrollToItem_item_hint(QListWidget* this_ptr, const QListWidgetItem* item, const QAbstractItemView::ScrollHint* hint);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_selectedItems_to_output(const QListWidget* this_ptr, QList< QListWidgetItem* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_setCurrentItem(QListWidget* this_ptr, QListWidgetItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_setCurrentRow(QListWidget* this_ptr, int row);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_setItemHidden(QListWidget* this_ptr, const QListWidgetItem* item, bool hide);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_setItemSelected(QListWidget* this_ptr, const QListWidgetItem* item, bool select);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_setItemWidget(QListWidget* this_ptr, QListWidgetItem* item, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_setSelectionModel(QListWidget* this_ptr, QItemSelectionModel* selectionModel);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_setSortingEnabled(QListWidget* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_sortItems_no_args(QListWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_sortItems_order(QListWidget* this_ptr, const Qt::SortOrder* order);
QT_WIDGETS_C_EXPORT QListWidgetItem* qt_widgets_c_QListWidget_takeItem(QListWidget* this_ptr, int row);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidget_visualItemRect_to_output(const QListWidget* this_ptr, const QListWidgetItem* item, QRect* output);

} // extern "C"

#endif // QT_WIDGETS_C_QLISTWIDGET_H
