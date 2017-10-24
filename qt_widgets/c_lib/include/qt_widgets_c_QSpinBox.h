#ifndef QT_WIDGETS_C_QSPINBOX_H
#define QT_WIDGETS_C_QSPINBOX_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QSpinBox* qt_widgets_c_QSpinBox_G_dynamic_cast_QSpinBox_ptr_QAbstractSpinBox(QAbstractSpinBox* ptr);
QT_WIDGETS_C_EXPORT QSpinBox* qt_widgets_c_QSpinBox_G_dynamic_cast_QSpinBox_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractSpinBox* qt_widgets_c_QSpinBox_G_static_cast_QAbstractSpinBox_ptr(QSpinBox* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QSpinBox_G_static_cast_QObject_ptr(QSpinBox* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QSpinBox_G_static_cast_QPaintDevice_ptr(QSpinBox* ptr);
QT_WIDGETS_C_EXPORT QSpinBox* qt_widgets_c_QSpinBox_G_static_cast_QSpinBox_ptr_QAbstractSpinBox(QAbstractSpinBox* ptr);
QT_WIDGETS_C_EXPORT QSpinBox* qt_widgets_c_QSpinBox_G_static_cast_QSpinBox_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QSpinBox* qt_widgets_c_QSpinBox_G_static_cast_QSpinBox_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QSpinBox* qt_widgets_c_QSpinBox_G_static_cast_QSpinBox_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QSpinBox_G_static_cast_QWidget_ptr(QSpinBox* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpinBox_cleanText_to_output(const QSpinBox* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpinBox_delete(QSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSpinBox_displayIntegerBase(const QSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSpinBox_maximum(const QSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QSpinBox_metaObject(const QSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSpinBox_minimum(const QSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT QSpinBox* qt_widgets_c_QSpinBox_new_no_args();
QT_WIDGETS_C_EXPORT QSpinBox* qt_widgets_c_QSpinBox_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpinBox_prefix_to_output(const QSpinBox* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSpinBox_qt_metacall(QSpinBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QSpinBox_qt_metacast(QSpinBox* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpinBox_setDisplayIntegerBase(QSpinBox* this_ptr, int base);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpinBox_setMaximum(QSpinBox* this_ptr, int max);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpinBox_setMinimum(QSpinBox* this_ptr, int min);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpinBox_setPrefix(QSpinBox* this_ptr, const QString* prefix);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpinBox_setRange(QSpinBox* this_ptr, int min, int max);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpinBox_setSingleStep(QSpinBox* this_ptr, int val);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpinBox_setSuffix(QSpinBox* this_ptr, const QString* suffix);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpinBox_setValue(QSpinBox* this_ptr, int val);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSpinBox_singleStep(const QSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpinBox_suffix_to_output(const QSpinBox* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpinBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpinBox_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSpinBox_value(const QSpinBox* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QSPINBOX_H
