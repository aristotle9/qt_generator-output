#ifndef QT_GUI_C_QTEXTTABLECELLFORMAT_H
#define QT_GUI_C_QTEXTTABLECELLFORMAT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QTextCharFormat* qt_gui_c_QTextTableCellFormat_G_static_cast_QTextCharFormat_ptr(QTextTableCellFormat* ptr);
QT_GUI_C_EXPORT QTextFormat* qt_gui_c_QTextTableCellFormat_G_static_cast_QTextFormat_ptr(QTextTableCellFormat* ptr);
QT_GUI_C_EXPORT QTextTableCellFormat* qt_gui_c_QTextTableCellFormat_G_static_cast_QTextTableCellFormat_ptr_QTextCharFormat(QTextCharFormat* ptr);
QT_GUI_C_EXPORT QTextTableCellFormat* qt_gui_c_QTextTableCellFormat_G_static_cast_QTextTableCellFormat_ptr_QTextFormat(QTextFormat* ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextTableCellFormat_bottomPadding(const QTextTableCellFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableCellFormat_constructor(QTextTableCellFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableCellFormat_destructor(QTextTableCellFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextTableCellFormat_isValid(const QTextTableCellFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextTableCellFormat_leftPadding(const QTextTableCellFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextTableCellFormat_rightPadding(const QTextTableCellFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableCellFormat_setBottomPadding(QTextTableCellFormat* this_ptr, double padding);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableCellFormat_setLeftPadding(QTextTableCellFormat* this_ptr, double padding);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableCellFormat_setPadding(QTextTableCellFormat* this_ptr, double padding);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableCellFormat_setRightPadding(QTextTableCellFormat* this_ptr, double padding);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableCellFormat_setTopPadding(QTextTableCellFormat* this_ptr, double padding);
QT_GUI_C_EXPORT double qt_gui_c_QTextTableCellFormat_topPadding(const QTextTableCellFormat* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTEXTTABLECELLFORMAT_H
