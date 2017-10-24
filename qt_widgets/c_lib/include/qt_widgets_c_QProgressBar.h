#ifndef QT_WIDGETS_C_QPROGRESSBAR_H
#define QT_WIDGETS_C_QPROGRESSBAR_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QProgressBar* qt_widgets_c_QProgressBar_G_dynamic_cast_QProgressBar_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QProgressBar_G_static_cast_QObject_ptr(QProgressBar* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QProgressBar_G_static_cast_QPaintDevice_ptr(QProgressBar* ptr);
QT_WIDGETS_C_EXPORT QProgressBar* qt_widgets_c_QProgressBar_G_static_cast_QProgressBar_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QProgressBar* qt_widgets_c_QProgressBar_G_static_cast_QProgressBar_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QProgressBar* qt_widgets_c_QProgressBar_G_static_cast_QProgressBar_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QProgressBar_G_static_cast_QWidget_ptr(QProgressBar* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_delete(QProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_format_to_output(const QProgressBar* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QProgressBar_invertedAppearance(const QProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QProgressBar_isTextVisible(const QProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProgressBar_maximum(const QProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QProgressBar_metaObject(const QProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProgressBar_minimum(const QProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_minimumSizeHint_to_output(const QProgressBar* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QProgressBar* qt_widgets_c_QProgressBar_new_no_args();
QT_WIDGETS_C_EXPORT QProgressBar* qt_widgets_c_QProgressBar_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProgressBar_qt_metacall(QProgressBar* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QProgressBar_qt_metacast(QProgressBar* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_reset(QProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_resetFormat(QProgressBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_setFormat(QProgressBar* this_ptr, const QString* format);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_setInvertedAppearance(QProgressBar* this_ptr, bool invert);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_setMaximum(QProgressBar* this_ptr, int maximum);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_setMinimum(QProgressBar* this_ptr, int minimum);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_setOrientation(QProgressBar* this_ptr, const Qt::Orientation* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_setRange(QProgressBar* this_ptr, int minimum, int maximum);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_setTextDirection(QProgressBar* this_ptr, const QProgressBar::Direction* textDirection);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_setTextVisible(QProgressBar* this_ptr, bool visible);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_setValue(QProgressBar* this_ptr, int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_sizeHint_to_output(const QProgressBar* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_text_to_output(const QProgressBar* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressBar_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProgressBar_value(const QProgressBar* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QPROGRESSBAR_H
