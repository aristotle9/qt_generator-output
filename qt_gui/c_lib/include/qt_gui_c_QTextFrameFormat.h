#ifndef QT_GUI_C_QTEXTFRAMEFORMAT_H
#define QT_GUI_C_QTEXTFRAMEFORMAT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QTextFormat* qt_gui_c_QTextFrameFormat_G_static_cast_QTextFormat_ptr(QTextFrameFormat* ptr);
QT_GUI_C_EXPORT QTextFrameFormat* qt_gui_c_QTextFrameFormat_G_static_cast_QTextFrameFormat_ptr(QTextFormat* ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextFrameFormat_border(const QTextFrameFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_borderBrush_to_output(const QTextFrameFormat* this_ptr, QBrush* output);
QT_GUI_C_EXPORT QTextFrameFormat::BorderStyle qt_gui_c_QTextFrameFormat_borderStyle(const QTextFrameFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextFrameFormat_bottomMargin(const QTextFrameFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_constructor(QTextFrameFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_destructor(QTextFrameFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_height_to_output(const QTextFrameFormat* this_ptr, QTextLength* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFrameFormat_isValid(const QTextFrameFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextFrameFormat_leftMargin(const QTextFrameFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextFrameFormat_margin(const QTextFrameFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextFrameFormat_padding(const QTextFrameFormat* this_ptr);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QTextFrameFormat_pageBreakPolicy(const QTextFrameFormat* this_ptr);
QT_GUI_C_EXPORT QTextFrameFormat::Position qt_gui_c_QTextFrameFormat_position(const QTextFrameFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextFrameFormat_rightMargin(const QTextFrameFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_setBorder(QTextFrameFormat* this_ptr, double border);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_setBorderBrush(QTextFrameFormat* this_ptr, const QBrush* brush);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_setBorderStyle(QTextFrameFormat* this_ptr, QTextFrameFormat::BorderStyle style);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_setBottomMargin(QTextFrameFormat* this_ptr, double margin);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_setHeight_QTextLength(QTextFrameFormat* this_ptr, const QTextLength* height);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_setHeight_double(QTextFrameFormat* this_ptr, double height);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_setLeftMargin(QTextFrameFormat* this_ptr, double margin);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_setMargin(QTextFrameFormat* this_ptr, double margin);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_setPadding(QTextFrameFormat* this_ptr, double padding);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_setPageBreakPolicy(QTextFrameFormat* this_ptr, unsigned int flags);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_setPosition(QTextFrameFormat* this_ptr, QTextFrameFormat::Position f);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_setRightMargin(QTextFrameFormat* this_ptr, double margin);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_setTopMargin(QTextFrameFormat* this_ptr, double margin);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_setWidth_length(QTextFrameFormat* this_ptr, const QTextLength* length);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_setWidth_width(QTextFrameFormat* this_ptr, double width);
QT_GUI_C_EXPORT double qt_gui_c_QTextFrameFormat_topMargin(const QTextFrameFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrameFormat_width_to_output(const QTextFrameFormat* this_ptr, QTextLength* output);

} // extern "C"

#endif // QT_GUI_C_QTEXTFRAMEFORMAT_H
