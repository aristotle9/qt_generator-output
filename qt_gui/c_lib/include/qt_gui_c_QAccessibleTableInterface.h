#ifndef QT_GUI_C_QACCESSIBLETABLEINTERFACE_H
#define QT_GUI_C_QACCESSIBLETABLEINTERFACE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QAccessibleInterface* qt_gui_c_QAccessibleTableInterface_caption(const QAccessibleTableInterface* this_ptr);
QT_GUI_C_EXPORT QAccessibleInterface* qt_gui_c_QAccessibleTableInterface_cellAt(const QAccessibleTableInterface* this_ptr, int row, int column);
QT_GUI_C_EXPORT int qt_gui_c_QAccessibleTableInterface_columnCount(const QAccessibleTableInterface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTableInterface_columnDescription_to_output(const QAccessibleTableInterface* this_ptr, int column, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTableInterface_delete(QAccessibleTableInterface* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QAccessibleTableInterface_isColumnSelected(const QAccessibleTableInterface* this_ptr, int column);
QT_GUI_C_EXPORT bool qt_gui_c_QAccessibleTableInterface_isRowSelected(const QAccessibleTableInterface* this_ptr, int row);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTableInterface_modelChange(QAccessibleTableInterface* this_ptr, QAccessibleTableModelChangeEvent* event);
QT_GUI_C_EXPORT int qt_gui_c_QAccessibleTableInterface_rowCount(const QAccessibleTableInterface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTableInterface_rowDescription_to_output(const QAccessibleTableInterface* this_ptr, int row, QString* output);
QT_GUI_C_EXPORT bool qt_gui_c_QAccessibleTableInterface_selectColumn(QAccessibleTableInterface* this_ptr, int column);
QT_GUI_C_EXPORT bool qt_gui_c_QAccessibleTableInterface_selectRow(QAccessibleTableInterface* this_ptr, int row);
QT_GUI_C_EXPORT int qt_gui_c_QAccessibleTableInterface_selectedCellCount(const QAccessibleTableInterface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTableInterface_selectedCells_to_output(const QAccessibleTableInterface* this_ptr, QList< QAccessibleInterface* >* output);
QT_GUI_C_EXPORT int qt_gui_c_QAccessibleTableInterface_selectedColumnCount(const QAccessibleTableInterface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTableInterface_selectedColumns_to_output(const QAccessibleTableInterface* this_ptr, QList< int >* output);
QT_GUI_C_EXPORT int qt_gui_c_QAccessibleTableInterface_selectedRowCount(const QAccessibleTableInterface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTableInterface_selectedRows_to_output(const QAccessibleTableInterface* this_ptr, QList< int >* output);
QT_GUI_C_EXPORT QAccessibleInterface* qt_gui_c_QAccessibleTableInterface_summary(const QAccessibleTableInterface* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QAccessibleTableInterface_unselectColumn(QAccessibleTableInterface* this_ptr, int column);
QT_GUI_C_EXPORT bool qt_gui_c_QAccessibleTableInterface_unselectRow(QAccessibleTableInterface* this_ptr, int row);

} // extern "C"

#endif // QT_GUI_C_QACCESSIBLETABLEINTERFACE_H
