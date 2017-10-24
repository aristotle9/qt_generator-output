#ifndef QT_WIDGETS_C_QSTYLEOPTIONBUTTON_H
#define QT_WIDGETS_C_QSTYLEOPTIONBUTTON_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QStyleOptionButton* qt_widgets_c_QStyleOptionButton_G_static_cast_QStyleOptionButton_ptr(QStyleOption* ptr);
QT_WIDGETS_C_EXPORT QStyleOption* qt_widgets_c_QStyleOptionButton_G_static_cast_QStyleOption_ptr(QStyleOptionButton* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionButton_delete(QStyleOptionButton* this_ptr);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QStyleOptionButton_features(const QStyleOptionButton* this_ptr);
QT_WIDGETS_C_EXPORT const QIcon* qt_widgets_c_QStyleOptionButton_icon(const QStyleOptionButton* this_ptr);
QT_WIDGETS_C_EXPORT const QSize* qt_widgets_c_QStyleOptionButton_iconSize(const QStyleOptionButton* this_ptr);
QT_WIDGETS_C_EXPORT QSize* qt_widgets_c_QStyleOptionButton_iconSize_mut(QStyleOptionButton* this_ptr);
QT_WIDGETS_C_EXPORT QIcon* qt_widgets_c_QStyleOptionButton_icon_mut(QStyleOptionButton* this_ptr);
QT_WIDGETS_C_EXPORT QStyleOptionButton* qt_widgets_c_QStyleOptionButton_new_no_args();
QT_WIDGETS_C_EXPORT QStyleOptionButton* qt_widgets_c_QStyleOptionButton_new_other(const QStyleOptionButton* other);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionButton_set_features(QStyleOptionButton* this_ptr, unsigned int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionButton_set_icon(QStyleOptionButton* this_ptr, const QIcon* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionButton_set_iconSize(QStyleOptionButton* this_ptr, const QSize* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionButton_set_text(QStyleOptionButton* this_ptr, const QString* value);
QT_WIDGETS_C_EXPORT const QString* qt_widgets_c_QStyleOptionButton_text(const QStyleOptionButton* this_ptr);
QT_WIDGETS_C_EXPORT QString* qt_widgets_c_QStyleOptionButton_text_mut(QStyleOptionButton* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QSTYLEOPTIONBUTTON_H
