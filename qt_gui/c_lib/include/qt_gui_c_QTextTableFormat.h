#ifndef QT_GUI_C_QTEXTTABLEFORMAT_H
#define QT_GUI_C_QTEXTTABLEFORMAT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QTextFormat* qt_gui_c_QTextTableFormat_G_static_cast_QTextFormat_ptr(QTextTableFormat* ptr);
QT_GUI_C_EXPORT QTextFrameFormat* qt_gui_c_QTextTableFormat_G_static_cast_QTextFrameFormat_ptr(QTextTableFormat* ptr);
QT_GUI_C_EXPORT QTextTableFormat* qt_gui_c_QTextTableFormat_G_static_cast_QTextTableFormat_ptr_QTextFormat(QTextFormat* ptr);
QT_GUI_C_EXPORT QTextTableFormat* qt_gui_c_QTextTableFormat_G_static_cast_QTextTableFormat_ptr_QTextFrameFormat(QTextFrameFormat* ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextTableFormat_cellPadding(const QTextTableFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextTableFormat_cellSpacing(const QTextTableFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableFormat_clearColumnWidthConstraints(QTextTableFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableFormat_columnWidthConstraints_to_output(const QTextTableFormat* this_ptr, QVector< QTextLength >* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextTableFormat_columns(const QTextTableFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableFormat_constructor(QTextTableFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableFormat_destructor(QTextTableFormat* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextTableFormat_headerRowCount(const QTextTableFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextTableFormat_isValid(const QTextTableFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableFormat_setCellPadding(QTextTableFormat* this_ptr, double padding);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableFormat_setCellSpacing(QTextTableFormat* this_ptr, double spacing);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableFormat_setColumnWidthConstraints(QTextTableFormat* this_ptr, const QVector< QTextLength >* constraints);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableFormat_setColumns(QTextTableFormat* this_ptr, int columns);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableFormat_setHeaderRowCount(QTextTableFormat* this_ptr, int count);

} // extern "C"

#endif // QT_GUI_C_QTEXTTABLEFORMAT_H
