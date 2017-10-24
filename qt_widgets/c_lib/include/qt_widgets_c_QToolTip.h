#ifndef QT_WIDGETS_C_QTOOLTIP_H
#define QT_WIDGETS_C_QTOOLTIP_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolTip_delete(QToolTip* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolTip_font_to_output(QFont* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolTip_hideText();
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QToolTip_isVisible();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolTip_palette_to_output(QPalette* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolTip_setFont(const QFont* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolTip_setPalette(const QPalette* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolTip_showText_pos_text(const QPoint* pos, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolTip_showText_pos_text_w(const QPoint* pos, const QString* text, QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolTip_showText_pos_text_w_rect(const QPoint* pos, const QString* text, QWidget* w, const QRect* rect);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolTip_showText_pos_text_w_rect_msecShowTime(const QPoint* pos, const QString* text, QWidget* w, const QRect* rect, int msecShowTime);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolTip_text_to_output(QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QTOOLTIP_H
