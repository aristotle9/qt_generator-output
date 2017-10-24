#ifndef QT_WIDGETS_C_QSTYLEOPTIONFRAME_H
#define QT_WIDGETS_C_QSTYLEOPTIONFRAME_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QStyleOptionFrame* qt_widgets_c_QStyleOptionFrame_G_static_cast_QStyleOptionFrame_ptr(QStyleOption* ptr);
QT_WIDGETS_C_EXPORT QStyleOption* qt_widgets_c_QStyleOptionFrame_G_static_cast_QStyleOption_ptr(QStyleOptionFrame* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionFrame_delete(QStyleOptionFrame* this_ptr);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QStyleOptionFrame_features(const QStyleOptionFrame* this_ptr);
QT_WIDGETS_C_EXPORT const QFrame::Shape* qt_widgets_c_QStyleOptionFrame_frameShape(const QStyleOptionFrame* this_ptr);
QT_WIDGETS_C_EXPORT QFrame::Shape* qt_widgets_c_QStyleOptionFrame_frameShape_mut(QStyleOptionFrame* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleOptionFrame_lineWidth(const QStyleOptionFrame* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleOptionFrame_midLineWidth(const QStyleOptionFrame* this_ptr);
QT_WIDGETS_C_EXPORT QStyleOptionFrame* qt_widgets_c_QStyleOptionFrame_new_no_args();
QT_WIDGETS_C_EXPORT QStyleOptionFrame* qt_widgets_c_QStyleOptionFrame_new_other(const QStyleOptionFrame* other);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionFrame_set_features(QStyleOptionFrame* this_ptr, unsigned int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionFrame_set_frameShape(QStyleOptionFrame* this_ptr, const QFrame::Shape* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionFrame_set_lineWidth(QStyleOptionFrame* this_ptr, int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionFrame_set_midLineWidth(QStyleOptionFrame* this_ptr, int value);

} // extern "C"

#endif // QT_WIDGETS_C_QSTYLEOPTIONFRAME_H
