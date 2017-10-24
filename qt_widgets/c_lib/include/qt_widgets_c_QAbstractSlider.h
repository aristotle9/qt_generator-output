#ifndef QT_WIDGETS_C_QABSTRACTSLIDER_H
#define QT_WIDGETS_C_QABSTRACTSLIDER_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QAbstractSlider* qt_widgets_c_QAbstractSlider_G_dynamic_cast_QAbstractSlider_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractSlider* qt_widgets_c_QAbstractSlider_G_static_cast_QAbstractSlider_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QAbstractSlider* qt_widgets_c_QAbstractSlider_G_static_cast_QAbstractSlider_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QAbstractSlider* qt_widgets_c_QAbstractSlider_G_static_cast_QAbstractSlider_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QAbstractSlider_G_static_cast_QObject_ptr(QAbstractSlider* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QAbstractSlider_G_static_cast_QPaintDevice_ptr(QAbstractSlider* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QAbstractSlider_G_static_cast_QWidget_ptr(QAbstractSlider* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSlider_delete(QAbstractSlider* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractSlider_hasTracking(const QAbstractSlider* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractSlider_invertedAppearance(const QAbstractSlider* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractSlider_invertedControls(const QAbstractSlider* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractSlider_isSliderDown(const QAbstractSlider* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QAbstractSlider_maximum(const QAbstractSlider* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QAbstractSlider_metaObject(const QAbstractSlider* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QAbstractSlider_minimum(const QAbstractSlider* this_ptr);
QT_WIDGETS_C_EXPORT QAbstractSlider* qt_widgets_c_QAbstractSlider_new_no_args();
QT_WIDGETS_C_EXPORT QAbstractSlider* qt_widgets_c_QAbstractSlider_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QAbstractSlider_pageStep(const QAbstractSlider* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QAbstractSlider_qt_metacall(QAbstractSlider* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QAbstractSlider_qt_metacast(QAbstractSlider* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSlider_setInvertedAppearance(QAbstractSlider* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSlider_setInvertedControls(QAbstractSlider* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSlider_setMaximum(QAbstractSlider* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSlider_setMinimum(QAbstractSlider* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSlider_setOrientation(QAbstractSlider* this_ptr, const Qt::Orientation* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSlider_setPageStep(QAbstractSlider* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSlider_setRange(QAbstractSlider* this_ptr, int min, int max);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSlider_setSingleStep(QAbstractSlider* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSlider_setSliderDown(QAbstractSlider* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSlider_setSliderPosition(QAbstractSlider* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSlider_setTracking(QAbstractSlider* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSlider_setValue(QAbstractSlider* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QAbstractSlider_singleStep(const QAbstractSlider* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QAbstractSlider_sliderPosition(const QAbstractSlider* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSlider_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSlider_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSlider_triggerAction(QAbstractSlider* this_ptr, QAbstractSlider::SliderAction action);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QAbstractSlider_value(const QAbstractSlider* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QABSTRACTSLIDER_H
