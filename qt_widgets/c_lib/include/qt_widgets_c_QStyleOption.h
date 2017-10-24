#ifndef QT_WIDGETS_C_QSTYLEOPTION_H
#define QT_WIDGETS_C_QSTYLEOPTION_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOption_G_operator_shl_to_output_debug_option(const QDebug* debug, const QStyleOption* option, QDebug* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOption_G_operator_shl_to_output_debug_optionType(const QDebug* debug, const QStyleOption::OptionType* optionType, QDebug* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOption_delete(QStyleOption* this_ptr);
QT_WIDGETS_C_EXPORT const Qt::LayoutDirection* qt_widgets_c_QStyleOption_direction(const QStyleOption* this_ptr);
QT_WIDGETS_C_EXPORT Qt::LayoutDirection* qt_widgets_c_QStyleOption_direction_mut(QStyleOption* this_ptr);
QT_WIDGETS_C_EXPORT const QFontMetrics* qt_widgets_c_QStyleOption_fontMetrics(const QStyleOption* this_ptr);
QT_WIDGETS_C_EXPORT QFontMetrics* qt_widgets_c_QStyleOption_fontMetrics_mut(QStyleOption* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOption_init(QStyleOption* this_ptr, const QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOption_initFrom(QStyleOption* this_ptr, const QWidget* w);
QT_WIDGETS_C_EXPORT QStyleOption* qt_widgets_c_QStyleOption_new_no_args();
QT_WIDGETS_C_EXPORT QStyleOption* qt_widgets_c_QStyleOption_new_other(const QStyleOption* other);
QT_WIDGETS_C_EXPORT QStyleOption* qt_widgets_c_QStyleOption_new_version(int version);
QT_WIDGETS_C_EXPORT QStyleOption* qt_widgets_c_QStyleOption_new_version_type(int version, int type);
QT_WIDGETS_C_EXPORT QStyleOption* qt_widgets_c_QStyleOption_operator_assign(QStyleOption* this_ptr, const QStyleOption* other);
QT_WIDGETS_C_EXPORT const QPalette* qt_widgets_c_QStyleOption_palette(const QStyleOption* this_ptr);
QT_WIDGETS_C_EXPORT QPalette* qt_widgets_c_QStyleOption_palette_mut(QStyleOption* this_ptr);
QT_WIDGETS_C_EXPORT const QRect* qt_widgets_c_QStyleOption_rect(const QStyleOption* this_ptr);
QT_WIDGETS_C_EXPORT QRect* qt_widgets_c_QStyleOption_rect_mut(QStyleOption* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOption_set_direction(QStyleOption* this_ptr, const Qt::LayoutDirection* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOption_set_fontMetrics(QStyleOption* this_ptr, const QFontMetrics* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOption_set_palette(QStyleOption* this_ptr, const QPalette* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOption_set_rect(QStyleOption* this_ptr, const QRect* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOption_set_styleObject(QStyleOption* this_ptr, QObject* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOption_set_type(QStyleOption* this_ptr, int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOption_set_version(QStyleOption* this_ptr, int value);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QStyleOption_styleObject(const QStyleOption* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleOption_type(const QStyleOption* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleOption_version(const QStyleOption* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QSTYLEOPTION_H
