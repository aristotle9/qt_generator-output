#ifndef QT_GUI_C_QTEXTTABLE_H
#define QT_GUI_C_QTEXTTABLE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QTextTable* qt_gui_c_QTextTable_G_dynamic_cast_QTextTable_ptr_QTextFrame(QTextFrame* ptr);
QT_GUI_C_EXPORT QTextTable* qt_gui_c_QTextTable_G_dynamic_cast_QTextTable_ptr_QTextObject(QTextObject* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QTextTable_G_static_cast_QObject_ptr(QTextTable* ptr);
QT_GUI_C_EXPORT QTextFrame* qt_gui_c_QTextTable_G_static_cast_QTextFrame_ptr(QTextTable* ptr);
QT_GUI_C_EXPORT QTextObject* qt_gui_c_QTextTable_G_static_cast_QTextObject_ptr(QTextTable* ptr);
QT_GUI_C_EXPORT QTextTable* qt_gui_c_QTextTable_G_static_cast_QTextTable_ptr_QObject(QObject* ptr);
QT_GUI_C_EXPORT QTextTable* qt_gui_c_QTextTable_G_static_cast_QTextTable_ptr_QTextFrame(QTextFrame* ptr);
QT_GUI_C_EXPORT QTextTable* qt_gui_c_QTextTable_G_static_cast_QTextTable_ptr_QTextObject(QTextObject* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_appendColumns(QTextTable* this_ptr, int count);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_appendRows(QTextTable* this_ptr, int count);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_cellAt_to_output_c(const QTextTable* this_ptr, const QTextCursor* c, QTextTableCell* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_cellAt_to_output_position(const QTextTable* this_ptr, int position, QTextTableCell* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_cellAt_to_output_row_col(const QTextTable* this_ptr, int row, int col, QTextTableCell* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextTable_columns(const QTextTable* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_delete(QTextTable* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_format_to_output(const QTextTable* this_ptr, QTextTableFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_insertColumns(QTextTable* this_ptr, int pos, int num);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_insertRows(QTextTable* this_ptr, int pos, int num);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_mergeCells_cursor(QTextTable* this_ptr, const QTextCursor* cursor);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_mergeCells_row_col_numRows_numCols(QTextTable* this_ptr, int row, int col, int numRows, int numCols);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QTextTable_metaObject(const QTextTable* this_ptr);
QT_GUI_C_EXPORT QTextTable* qt_gui_c_QTextTable_new(QTextDocument* doc);
QT_GUI_C_EXPORT int qt_gui_c_QTextTable_qt_metacall(QTextTable* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QTextTable_qt_metacast(QTextTable* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_removeColumns(QTextTable* this_ptr, int pos, int num);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_removeRows(QTextTable* this_ptr, int pos, int num);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_resize(QTextTable* this_ptr, int rows, int cols);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextTable_rowEnd_as_ptr(const QTextTable* this_ptr, const QTextCursor* c);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextTable_rowStart_as_ptr(const QTextTable* this_ptr, const QTextCursor* c);
QT_GUI_C_EXPORT int qt_gui_c_QTextTable_rows(const QTextTable* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_setFormat(QTextTable* this_ptr, const QTextTableFormat* format);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_splitCell(QTextTable* this_ptr, int row, int col, int numRows, int numCols);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextTable_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QTEXTTABLE_H
