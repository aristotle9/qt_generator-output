#ifndef QT_WIDGETS_C_QSTYLEOPTIONTITLEBAR_H
#define QT_WIDGETS_C_QSTYLEOPTIONTITLEBAR_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QStyleOptionComplex* qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOptionComplex_ptr(QStyleOptionTitleBar* ptr);
QT_WIDGETS_C_EXPORT QStyleOptionTitleBar* qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOptionTitleBar_ptr_QStyleOption(QStyleOption* ptr);
QT_WIDGETS_C_EXPORT QStyleOptionTitleBar* qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOptionTitleBar_ptr_QStyleOptionComplex(QStyleOptionComplex* ptr);
QT_WIDGETS_C_EXPORT QStyleOption* qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOption_ptr(QStyleOptionTitleBar* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionTitleBar_constructor_no_args(QStyleOptionTitleBar* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionTitleBar_constructor_other(const QStyleOptionTitleBar* other, QStyleOptionTitleBar* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionTitleBar_destructor(QStyleOptionTitleBar* this_ptr);
QT_WIDGETS_C_EXPORT const QIcon* qt_widgets_c_QStyleOptionTitleBar_icon(const QStyleOptionTitleBar* this_ptr);
QT_WIDGETS_C_EXPORT QIcon* qt_widgets_c_QStyleOptionTitleBar_icon_mut(QStyleOptionTitleBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionTitleBar_set_icon(QStyleOptionTitleBar* this_ptr, const QIcon* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionTitleBar_set_text(QStyleOptionTitleBar* this_ptr, const QString* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionTitleBar_set_titleBarState(QStyleOptionTitleBar* this_ptr, int value);
QT_WIDGETS_C_EXPORT const QString* qt_widgets_c_QStyleOptionTitleBar_text(const QStyleOptionTitleBar* this_ptr);
QT_WIDGETS_C_EXPORT QString* qt_widgets_c_QStyleOptionTitleBar_text_mut(QStyleOptionTitleBar* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleOptionTitleBar_titleBarState(const QStyleOptionTitleBar* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QSTYLEOPTIONTITLEBAR_H
