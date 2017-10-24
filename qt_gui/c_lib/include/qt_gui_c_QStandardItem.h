#ifndef QT_GUI_C_QSTANDARDITEM_H
#define QT_GUI_C_QSTANDARDITEM_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_accessibleDescription_to_output(const QStandardItem* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_accessibleText_to_output(const QStandardItem* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_appendColumn(QStandardItem* this_ptr, const QList< QStandardItem* >* items);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_appendRow_item(QStandardItem* this_ptr, QStandardItem* item);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_appendRow_items(QStandardItem* this_ptr, const QList< QStandardItem* >* items);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_appendRows(QStandardItem* this_ptr, const QList< QStandardItem* >* items);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_background_to_output(const QStandardItem* this_ptr, QBrush* output);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItem_child_row(const QStandardItem* this_ptr, int row);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItem_child_row_column(const QStandardItem* this_ptr, int row, int column);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItem_clone(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QStandardItem_column(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QStandardItem_columnCount(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_data_to_output_no_args(const QStandardItem* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_data_to_output_role(const QStandardItem* this_ptr, int role, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_delete(QStandardItem* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_font_to_output(const QStandardItem* this_ptr, QFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_foreground_to_output(const QStandardItem* this_ptr, QBrush* output);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItem_hasChildren(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_icon_to_output(const QStandardItem* this_ptr, QIcon* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_index_to_output(const QStandardItem* this_ptr, QModelIndex* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_insertColumn(QStandardItem* this_ptr, int column, const QList< QStandardItem* >* items);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_insertColumns(QStandardItem* this_ptr, int column, int count);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_insertRow_row_item(QStandardItem* this_ptr, int row, QStandardItem* item);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_insertRow_row_items(QStandardItem* this_ptr, int row, const QList< QStandardItem* >* items);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_insertRows_row_count(QStandardItem* this_ptr, int row, int count);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_insertRows_row_items(QStandardItem* this_ptr, int row, const QList< QStandardItem* >* items);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItem_isAutoTristate(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItem_isCheckable(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItem_isDragEnabled(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItem_isDropEnabled(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItem_isEditable(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItem_isEnabled(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItem_isSelectable(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItem_isTristate(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItem_isUserTristate(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT QStandardItemModel* qt_gui_c_QStandardItem_model(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItem_new_icon_text(const QIcon* icon, const QString* text);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItem_new_no_args();
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItem_new_rows(int rows);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItem_new_rows_columns(int rows, int columns);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItem_new_text(const QString* text);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItem_operator_lt(const QStandardItem* this_ptr, const QStandardItem* other);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItem_parent(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_read(QStandardItem* this_ptr, QDataStream* in);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_removeColumn(QStandardItem* this_ptr, int column);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_removeColumns(QStandardItem* this_ptr, int column, int count);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_removeRow(QStandardItem* this_ptr, int row);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_removeRows(QStandardItem* this_ptr, int row, int count);
QT_GUI_C_EXPORT int qt_gui_c_QStandardItem_row(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QStandardItem_rowCount(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setAccessibleDescription(QStandardItem* this_ptr, const QString* accessibleDescription);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setAccessibleText(QStandardItem* this_ptr, const QString* accessibleText);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setAutoTristate(QStandardItem* this_ptr, bool tristate);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setBackground(QStandardItem* this_ptr, const QBrush* brush);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setCheckState(QStandardItem* this_ptr, const Qt::CheckState* checkState);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setCheckable(QStandardItem* this_ptr, bool checkable);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setChild_row_column_item(QStandardItem* this_ptr, int row, int column, QStandardItem* item);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setChild_row_item(QStandardItem* this_ptr, int row, QStandardItem* item);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setColumnCount(QStandardItem* this_ptr, int columns);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setData_value(QStandardItem* this_ptr, const QVariant* value);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setData_value_role(QStandardItem* this_ptr, const QVariant* value, int role);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setDragEnabled(QStandardItem* this_ptr, bool dragEnabled);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setDropEnabled(QStandardItem* this_ptr, bool dropEnabled);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setEditable(QStandardItem* this_ptr, bool editable);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setEnabled(QStandardItem* this_ptr, bool enabled);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setFont(QStandardItem* this_ptr, const QFont* font);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setForeground(QStandardItem* this_ptr, const QBrush* brush);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setIcon(QStandardItem* this_ptr, const QIcon* icon);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setRowCount(QStandardItem* this_ptr, int rows);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setSelectable(QStandardItem* this_ptr, bool selectable);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setSizeHint(QStandardItem* this_ptr, const QSize* sizeHint);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setStatusTip(QStandardItem* this_ptr, const QString* statusTip);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setText(QStandardItem* this_ptr, const QString* text);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setToolTip(QStandardItem* this_ptr, const QString* toolTip);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setTristate(QStandardItem* this_ptr, bool tristate);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setUserTristate(QStandardItem* this_ptr, bool tristate);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_setWhatsThis(QStandardItem* this_ptr, const QString* whatsThis);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_sizeHint_to_output(const QStandardItem* this_ptr, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_sortChildren_column(QStandardItem* this_ptr, int column);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_sortChildren_column_order(QStandardItem* this_ptr, int column, const Qt::SortOrder* order);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_statusTip_to_output(const QStandardItem* this_ptr, QString* output);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItem_takeChild_row(QStandardItem* this_ptr, int row);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItem_takeChild_row_column(QStandardItem* this_ptr, int row, int column);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_takeColumn_to_output(QStandardItem* this_ptr, int column, QList< QStandardItem* >* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_takeRow_to_output(QStandardItem* this_ptr, int row, QList< QStandardItem* >* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_text_to_output(const QStandardItem* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_toolTip_to_output(const QStandardItem* this_ptr, QString* output);
QT_GUI_C_EXPORT int qt_gui_c_QStandardItem_type(const QStandardItem* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_whatsThis_to_output(const QStandardItem* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItem_write(const QStandardItem* this_ptr, QDataStream* out);

} // extern "C"

#endif // QT_GUI_C_QSTANDARDITEM_H
