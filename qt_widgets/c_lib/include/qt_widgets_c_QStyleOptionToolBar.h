#ifndef QT_WIDGETS_C_QSTYLEOPTIONTOOLBAR_H
#define QT_WIDGETS_C_QSTYLEOPTIONTOOLBAR_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QStyleOptionToolBar* qt_widgets_c_QStyleOptionToolBar_G_static_cast_QStyleOptionToolBar_ptr(QStyleOption* ptr);
QT_WIDGETS_C_EXPORT QStyleOption* qt_widgets_c_QStyleOptionToolBar_G_static_cast_QStyleOption_ptr(QStyleOptionToolBar* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionToolBar_delete(QStyleOptionToolBar* this_ptr);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QStyleOptionToolBar_features(const QStyleOptionToolBar* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleOptionToolBar_lineWidth(const QStyleOptionToolBar* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleOptionToolBar_midLineWidth(const QStyleOptionToolBar* this_ptr);
QT_WIDGETS_C_EXPORT QStyleOptionToolBar* qt_widgets_c_QStyleOptionToolBar_new_no_args();
QT_WIDGETS_C_EXPORT QStyleOptionToolBar* qt_widgets_c_QStyleOptionToolBar_new_other(const QStyleOptionToolBar* other);
QT_WIDGETS_C_EXPORT QStyleOptionToolBar::ToolBarPosition qt_widgets_c_QStyleOptionToolBar_positionOfLine(const QStyleOptionToolBar* this_ptr);
QT_WIDGETS_C_EXPORT QStyleOptionToolBar::ToolBarPosition qt_widgets_c_QStyleOptionToolBar_positionWithinLine(const QStyleOptionToolBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionToolBar_set_features(QStyleOptionToolBar* this_ptr, unsigned int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionToolBar_set_lineWidth(QStyleOptionToolBar* this_ptr, int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionToolBar_set_midLineWidth(QStyleOptionToolBar* this_ptr, int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionToolBar_set_positionOfLine(QStyleOptionToolBar* this_ptr, QStyleOptionToolBar::ToolBarPosition value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionToolBar_set_positionWithinLine(QStyleOptionToolBar* this_ptr, QStyleOptionToolBar::ToolBarPosition value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionToolBar_set_toolBarArea(QStyleOptionToolBar* this_ptr, const Qt::ToolBarArea* value);
QT_WIDGETS_C_EXPORT const Qt::ToolBarArea* qt_widgets_c_QStyleOptionToolBar_toolBarArea(const QStyleOptionToolBar* this_ptr);
QT_WIDGETS_C_EXPORT Qt::ToolBarArea* qt_widgets_c_QStyleOptionToolBar_toolBarArea_mut(QStyleOptionToolBar* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QSTYLEOPTIONTOOLBAR_H
