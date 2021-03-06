#ifndef QT_WIDGETS_C_QSTYLEOPTIONSLIDER_H
#define QT_WIDGETS_C_QSTYLEOPTIONSLIDER_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QStyleOptionComplex* qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOptionComplex_ptr(QStyleOptionSlider* ptr);
QT_WIDGETS_C_EXPORT QStyleOptionSlider* qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOptionSlider_ptr_QStyleOption(QStyleOption* ptr);
QT_WIDGETS_C_EXPORT QStyleOptionSlider* qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOptionSlider_ptr_QStyleOptionComplex(QStyleOptionComplex* ptr);
QT_WIDGETS_C_EXPORT QStyleOption* qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOption_ptr(QStyleOptionSlider* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionSlider_delete(QStyleOptionSlider* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QStyleOptionSlider_dialWrapping(const QStyleOptionSlider* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleOptionSlider_maximum(const QStyleOptionSlider* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleOptionSlider_minimum(const QStyleOptionSlider* this_ptr);
QT_WIDGETS_C_EXPORT QStyleOptionSlider* qt_widgets_c_QStyleOptionSlider_new_no_args();
QT_WIDGETS_C_EXPORT QStyleOptionSlider* qt_widgets_c_QStyleOptionSlider_new_other(const QStyleOptionSlider* other);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QStyleOptionSlider_notchTarget(const QStyleOptionSlider* this_ptr);
QT_WIDGETS_C_EXPORT const Qt::Orientation* qt_widgets_c_QStyleOptionSlider_orientation(const QStyleOptionSlider* this_ptr);
QT_WIDGETS_C_EXPORT Qt::Orientation* qt_widgets_c_QStyleOptionSlider_orientation_mut(QStyleOptionSlider* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleOptionSlider_pageStep(const QStyleOptionSlider* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionSlider_set_dialWrapping(QStyleOptionSlider* this_ptr, bool value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionSlider_set_maximum(QStyleOptionSlider* this_ptr, int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionSlider_set_minimum(QStyleOptionSlider* this_ptr, int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionSlider_set_notchTarget(QStyleOptionSlider* this_ptr, double value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionSlider_set_orientation(QStyleOptionSlider* this_ptr, const Qt::Orientation* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionSlider_set_pageStep(QStyleOptionSlider* this_ptr, int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionSlider_set_singleStep(QStyleOptionSlider* this_ptr, int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionSlider_set_sliderPosition(QStyleOptionSlider* this_ptr, int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionSlider_set_sliderValue(QStyleOptionSlider* this_ptr, int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionSlider_set_tickInterval(QStyleOptionSlider* this_ptr, int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionSlider_set_tickPosition(QStyleOptionSlider* this_ptr, const QSlider::TickPosition* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionSlider_set_upsideDown(QStyleOptionSlider* this_ptr, bool value);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleOptionSlider_singleStep(const QStyleOptionSlider* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleOptionSlider_sliderPosition(const QStyleOptionSlider* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleOptionSlider_sliderValue(const QStyleOptionSlider* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleOptionSlider_tickInterval(const QStyleOptionSlider* this_ptr);
QT_WIDGETS_C_EXPORT const QSlider::TickPosition* qt_widgets_c_QStyleOptionSlider_tickPosition(const QStyleOptionSlider* this_ptr);
QT_WIDGETS_C_EXPORT QSlider::TickPosition* qt_widgets_c_QStyleOptionSlider_tickPosition_mut(QStyleOptionSlider* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QStyleOptionSlider_upsideDown(const QStyleOptionSlider* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QSTYLEOPTIONSLIDER_H
