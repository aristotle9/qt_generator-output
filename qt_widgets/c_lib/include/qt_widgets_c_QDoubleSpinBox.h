#ifndef QT_WIDGETS_C_QDOUBLESPINBOX_H
#define QT_WIDGETS_C_QDOUBLESPINBOX_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QDoubleSpinBox* qt_widgets_c_QDoubleSpinBox_G_dynamic_cast_QDoubleSpinBox_ptr_QAbstractSpinBox(QAbstractSpinBox* ptr);
QT_WIDGETS_C_EXPORT QDoubleSpinBox* qt_widgets_c_QDoubleSpinBox_G_dynamic_cast_QDoubleSpinBox_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractSpinBox* qt_widgets_c_QDoubleSpinBox_G_static_cast_QAbstractSpinBox_ptr(QDoubleSpinBox* ptr);
QT_WIDGETS_C_EXPORT QDoubleSpinBox* qt_widgets_c_QDoubleSpinBox_G_static_cast_QDoubleSpinBox_ptr_QAbstractSpinBox(QAbstractSpinBox* ptr);
QT_WIDGETS_C_EXPORT QDoubleSpinBox* qt_widgets_c_QDoubleSpinBox_G_static_cast_QDoubleSpinBox_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QDoubleSpinBox* qt_widgets_c_QDoubleSpinBox_G_static_cast_QDoubleSpinBox_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QDoubleSpinBox* qt_widgets_c_QDoubleSpinBox_G_static_cast_QDoubleSpinBox_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QDoubleSpinBox_G_static_cast_QObject_ptr(QDoubleSpinBox* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QDoubleSpinBox_G_static_cast_QPaintDevice_ptr(QDoubleSpinBox* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QDoubleSpinBox_G_static_cast_QWidget_ptr(QDoubleSpinBox* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDoubleSpinBox_cleanText_to_output(const QDoubleSpinBox* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDoubleSpinBox_decimals(const QDoubleSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDoubleSpinBox_delete(QDoubleSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDoubleSpinBox_fixup(const QDoubleSpinBox* this_ptr, QString* str);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QDoubleSpinBox_maximum(const QDoubleSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QDoubleSpinBox_metaObject(const QDoubleSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QDoubleSpinBox_minimum(const QDoubleSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT QDoubleSpinBox* qt_widgets_c_QDoubleSpinBox_new_no_args();
QT_WIDGETS_C_EXPORT QDoubleSpinBox* qt_widgets_c_QDoubleSpinBox_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDoubleSpinBox_prefix_to_output(const QDoubleSpinBox* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDoubleSpinBox_qt_metacall(QDoubleSpinBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QDoubleSpinBox_qt_metacast(QDoubleSpinBox* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDoubleSpinBox_setDecimals(QDoubleSpinBox* this_ptr, int prec);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDoubleSpinBox_setMaximum(QDoubleSpinBox* this_ptr, double max);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDoubleSpinBox_setMinimum(QDoubleSpinBox* this_ptr, double min);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDoubleSpinBox_setPrefix(QDoubleSpinBox* this_ptr, const QString* prefix);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDoubleSpinBox_setRange(QDoubleSpinBox* this_ptr, double min, double max);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDoubleSpinBox_setSingleStep(QDoubleSpinBox* this_ptr, double val);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDoubleSpinBox_setSuffix(QDoubleSpinBox* this_ptr, const QString* suffix);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDoubleSpinBox_setValue(QDoubleSpinBox* this_ptr, double val);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QDoubleSpinBox_singleStep(const QDoubleSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDoubleSpinBox_suffix_to_output(const QDoubleSpinBox* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDoubleSpinBox_textFromValue_to_output(const QDoubleSpinBox* this_ptr, double val, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDoubleSpinBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDoubleSpinBox_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QDoubleSpinBox_value(const QDoubleSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QDoubleSpinBox_valueFromText(const QDoubleSpinBox* this_ptr, const QString* text);

} // extern "C"

#endif // QT_WIDGETS_C_QDOUBLESPINBOX_H
