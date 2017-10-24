#ifndef QT_GUI_C_QFONTMETRICSF_H
#define QT_GUI_C_QFONTMETRICSF_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_ascent(const QFontMetricsF* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_averageCharWidth(const QFontMetricsF* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetricsF_boundingRect_to_output_arg1(const QFontMetricsF* this_ptr, const QChar* arg1, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetricsF_boundingRect_to_output_r_flags_string(const QFontMetricsF* this_ptr, const QRectF* r, int flags, const QString* string, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetricsF_boundingRect_to_output_r_flags_string_tabstops(const QFontMetricsF* this_ptr, const QRectF* r, int flags, const QString* string, int tabstops, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetricsF_boundingRect_to_output_r_flags_string_tabstops_tabarray(const QFontMetricsF* this_ptr, const QRectF* r, int flags, const QString* string, int tabstops, int* tabarray, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetricsF_boundingRect_to_output_string(const QFontMetricsF* this_ptr, const QString* string, QRectF* output);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_capHeight(const QFontMetricsF* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetricsF_constructor_QFont(const QFont* arg1, QFontMetricsF* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetricsF_constructor_QFontMetrics(const QFontMetrics* arg1, QFontMetricsF* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetricsF_constructor_QFontMetricsF(const QFontMetricsF* arg1, QFontMetricsF* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetricsF_constructor_QFont_QPaintDevice(const QFont* arg1, QPaintDevice* pd, QFontMetricsF* output);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_descent(const QFontMetricsF* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetricsF_destructor(QFontMetricsF* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetricsF_elidedText_to_output_text_mode_width(const QFontMetricsF* this_ptr, const QString* text, const Qt::TextElideMode* mode, double width, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetricsF_elidedText_to_output_text_mode_width_flags(const QFontMetricsF* this_ptr, const QString* text, const Qt::TextElideMode* mode, double width, int flags, QString* output);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_height(const QFontMetricsF* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QFontMetricsF_inFont(const QFontMetricsF* this_ptr, const QChar* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QFontMetricsF_inFontUcs4(const QFontMetricsF* this_ptr, unsigned int ucs4);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_leading(const QFontMetricsF* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_leftBearing(const QFontMetricsF* this_ptr, const QChar* arg1);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_lineSpacing(const QFontMetricsF* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_lineWidth(const QFontMetricsF* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_maxWidth(const QFontMetricsF* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_minLeftBearing(const QFontMetricsF* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_minRightBearing(const QFontMetricsF* this_ptr);
QT_GUI_C_EXPORT QFontMetricsF* qt_gui_c_QFontMetricsF_operator_assign_QFontMetrics(QFontMetricsF* this_ptr, const QFontMetrics* arg1);
QT_GUI_C_EXPORT QFontMetricsF* qt_gui_c_QFontMetricsF_operator_assign_QFontMetricsF(QFontMetricsF* this_ptr, const QFontMetricsF* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QFontMetricsF_operator_eq(const QFontMetricsF* this_ptr, const QFontMetricsF* other);
QT_GUI_C_EXPORT bool qt_gui_c_QFontMetricsF_operator_neq(const QFontMetricsF* this_ptr, const QFontMetricsF* other);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_overlinePos(const QFontMetricsF* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_rightBearing(const QFontMetricsF* this_ptr, const QChar* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetricsF_size_to_output_flags_str(const QFontMetricsF* this_ptr, int flags, const QString* str, QSizeF* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetricsF_size_to_output_flags_str_tabstops(const QFontMetricsF* this_ptr, int flags, const QString* str, int tabstops, QSizeF* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetricsF_size_to_output_flags_str_tabstops_tabarray(const QFontMetricsF* this_ptr, int flags, const QString* str, int tabstops, int* tabarray, QSizeF* output);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_strikeOutPos(const QFontMetricsF* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetricsF_swap(QFontMetricsF* this_ptr, QFontMetricsF* other);
QT_GUI_C_EXPORT void qt_gui_c_QFontMetricsF_tightBoundingRect_to_output(const QFontMetricsF* this_ptr, const QString* text, QRectF* output);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_underlinePos(const QFontMetricsF* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_width_arg1(const QFontMetricsF* this_ptr, const QChar* arg1);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_width_string(const QFontMetricsF* this_ptr, const QString* string);
QT_GUI_C_EXPORT double qt_gui_c_QFontMetricsF_xHeight(const QFontMetricsF* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QFONTMETRICSF_H
