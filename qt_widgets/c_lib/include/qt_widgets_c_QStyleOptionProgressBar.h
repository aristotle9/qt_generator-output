#ifndef QT_WIDGETS_C_QSTYLEOPTIONPROGRESSBAR_H
#define QT_WIDGETS_C_QSTYLEOPTIONPROGRESSBAR_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QStyleOptionProgressBar* qt_widgets_c_QStyleOptionProgressBar_G_static_cast_QStyleOptionProgressBar_ptr(QStyleOption* ptr);
QT_WIDGETS_C_EXPORT QStyleOption* qt_widgets_c_QStyleOptionProgressBar_G_static_cast_QStyleOption_ptr(QStyleOptionProgressBar* ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QStyleOptionProgressBar_bottomToTop(const QStyleOptionProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionProgressBar_delete(QStyleOptionProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QStyleOptionProgressBar_invertedAppearance(const QStyleOptionProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleOptionProgressBar_maximum(const QStyleOptionProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleOptionProgressBar_minimum(const QStyleOptionProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT QStyleOptionProgressBar* qt_widgets_c_QStyleOptionProgressBar_new_no_args();
QT_WIDGETS_C_EXPORT QStyleOptionProgressBar* qt_widgets_c_QStyleOptionProgressBar_new_other(const QStyleOptionProgressBar* other);
QT_WIDGETS_C_EXPORT const Qt::Orientation* qt_widgets_c_QStyleOptionProgressBar_orientation(const QStyleOptionProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT Qt::Orientation* qt_widgets_c_QStyleOptionProgressBar_orientation_mut(QStyleOptionProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleOptionProgressBar_progress(const QStyleOptionProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionProgressBar_set_bottomToTop(QStyleOptionProgressBar* this_ptr, bool value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionProgressBar_set_invertedAppearance(QStyleOptionProgressBar* this_ptr, bool value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionProgressBar_set_maximum(QStyleOptionProgressBar* this_ptr, int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionProgressBar_set_minimum(QStyleOptionProgressBar* this_ptr, int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionProgressBar_set_orientation(QStyleOptionProgressBar* this_ptr, const Qt::Orientation* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionProgressBar_set_progress(QStyleOptionProgressBar* this_ptr, int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionProgressBar_set_text(QStyleOptionProgressBar* this_ptr, const QString* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionProgressBar_set_textVisible(QStyleOptionProgressBar* this_ptr, bool value);
QT_WIDGETS_C_EXPORT const QString* qt_widgets_c_QStyleOptionProgressBar_text(const QStyleOptionProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QStyleOptionProgressBar_textVisible(const QStyleOptionProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT QString* qt_widgets_c_QStyleOptionProgressBar_text_mut(QStyleOptionProgressBar* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QSTYLEOPTIONPROGRESSBAR_H