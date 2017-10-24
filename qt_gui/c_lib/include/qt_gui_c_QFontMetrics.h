#ifndef QT_GUI_C_QFONTMETRICS_H
#define QT_GUI_C_QFONTMETRICS_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_G_swap_QFontMetricsF_QFontMetricsF(QFontMetricsF* value1, QFontMetricsF* value2);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_G_swap_QFontMetrics_QFontMetrics(QFontMetrics* value1, QFontMetrics* value2);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_ascent(const QFontMetrics* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_averageCharWidth(const QFontMetrics* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_boundingRect_to_output_arg1(const QFontMetrics* this_ptr, const QChar* arg1, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_boundingRect_to_output_r_flags_text(const QFontMetrics* this_ptr, const QRect* r, int flags, const QString* text, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_boundingRect_to_output_r_flags_text_tabstops(const QFontMetrics* this_ptr, const QRect* r, int flags, const QString* text, int tabstops, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_boundingRect_to_output_r_flags_text_tabstops_tabarray(const QFontMetrics* this_ptr, const QRect* r, int flags, const QString* text, int tabstops, int* tabarray, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_boundingRect_to_output_text(const QFontMetrics* this_ptr, const QString* text, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_boundingRect_to_output_x_y_w_h_flags_text(const QFontMetrics* this_ptr, int x, int y, int w, int h, int flags, const QString* text, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_boundingRect_to_output_x_y_w_h_flags_text_tabstops(const QFontMetrics* this_ptr, int x, int y, int w, int h, int flags, const QString* text, int tabstops, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_boundingRect_to_output_x_y_w_h_flags_text_tabstops_tabarray(const QFontMetrics* this_ptr, int x, int y, int w, int h, int flags, const QString* text, int tabstops, int* tabarray, QRect* output);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_capHeight(const QFontMetrics* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_charWidth(const QFontMetrics* this_ptr, const QString* str, int pos);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_constructor_QFont(const QFont* arg1, QFontMetrics* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_constructor_QFontMetrics(const QFontMetrics* arg1, QFontMetrics* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_constructor_QFont_QPaintDevice(const QFont* arg1, QPaintDevice* pd, QFontMetrics* output);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_descent(const QFontMetrics* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_destructor(QFontMetrics* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_elidedText_to_output_text_mode_width(const QFontMetrics* this_ptr, const QString* text, const Qt::TextElideMode* mode, int width, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_elidedText_to_output_text_mode_width_flags(const QFontMetrics* this_ptr, const QString* text, const Qt::TextElideMode* mode, int width, int flags, QString* output);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_height(const QFontMetrics* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QFontMetrics_inFont(const QFontMetrics* this_ptr, const QChar* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QFontMetrics_inFontUcs4(const QFontMetrics* this_ptr, unsigned int ucs4);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_leading(const QFontMetrics* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_leftBearing(const QFontMetrics* this_ptr, const QChar* arg1);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_lineSpacing(const QFontMetrics* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_lineWidth(const QFontMetrics* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_maxWidth(const QFontMetrics* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_minLeftBearing(const QFontMetrics* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_minRightBearing(const QFontMetrics* this_ptr);
QT_GUI_C_EXPORT QFontMetrics* qt_gui_c_QFontMetrics_operator_assign(QFontMetrics* this_ptr, const QFontMetrics* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QFontMetrics_operator_eq(const QFontMetrics* this_ptr, const QFontMetrics* other);
QT_GUI_C_EXPORT bool qt_gui_c_QFontMetrics_operator_neq(const QFontMetrics* this_ptr, const QFontMetrics* other);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_overlinePos(const QFontMetrics* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_rightBearing(const QFontMetrics* this_ptr, const QChar* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_size_to_output_flags_str(const QFontMetrics* this_ptr, int flags, const QString* str, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_size_to_output_flags_str_tabstops(const QFontMetrics* this_ptr, int flags, const QString* str, int tabstops, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_size_to_output_flags_str_tabstops_tabarray(const QFontMetrics* this_ptr, int flags, const QString* str, int tabstops, int* tabarray, QSize* output);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_strikeOutPos(const QFontMetrics* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_swap(QFontMetrics* this_ptr, QFontMetrics* other);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetrics_tightBoundingRect_to_output(const QFontMetrics* this_ptr, const QString* text, QRect* output);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_underlinePos(const QFontMetrics* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_width_QChar(const QFontMetrics* this_ptr, const QChar* arg1);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_width_QString(const QFontMetrics* this_ptr, const QString* arg1);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_width_QString_int(const QFontMetrics* this_ptr, const QString* arg1, int len);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_width_QString_int_int(const QFontMetrics* this_ptr, const QString* arg1, int len, int flags);
QT_GUI_C_EXPORT int qt_gui_c_QFontMetrics_xHeight(const QFontMetrics* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QFONTMETRICS_H
