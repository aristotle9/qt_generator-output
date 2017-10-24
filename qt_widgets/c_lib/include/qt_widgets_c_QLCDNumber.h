#ifndef QT_WIDGETS_C_QLCDNUMBER_H
#define QT_WIDGETS_C_QLCDNUMBER_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QLCDNumber* qt_widgets_c_QLCDNumber_G_dynamic_cast_QLCDNumber_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QLCDNumber* qt_widgets_c_QLCDNumber_G_dynamic_cast_QLCDNumber_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QLCDNumber_G_static_cast_QFrame_ptr(QLCDNumber* ptr);
QT_WIDGETS_C_EXPORT QLCDNumber* qt_widgets_c_QLCDNumber_G_static_cast_QLCDNumber_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QLCDNumber* qt_widgets_c_QLCDNumber_G_static_cast_QLCDNumber_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QLCDNumber* qt_widgets_c_QLCDNumber_G_static_cast_QLCDNumber_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QLCDNumber* qt_widgets_c_QLCDNumber_G_static_cast_QLCDNumber_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QLCDNumber_G_static_cast_QObject_ptr(QLCDNumber* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QLCDNumber_G_static_cast_QPaintDevice_ptr(QLCDNumber* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QLCDNumber_G_static_cast_QWidget_ptr(QLCDNumber* ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLCDNumber_checkOverflow_double(const QLCDNumber* this_ptr, double num);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLCDNumber_checkOverflow_int(const QLCDNumber* this_ptr, int num);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLCDNumber_delete(QLCDNumber* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLCDNumber_digitCount(const QLCDNumber* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLCDNumber_display_QString(QLCDNumber* this_ptr, const QString* str);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLCDNumber_display_double(QLCDNumber* this_ptr, double num);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLCDNumber_display_int(QLCDNumber* this_ptr, int num);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLCDNumber_intValue(const QLCDNumber* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QLCDNumber_metaObject(const QLCDNumber* this_ptr);
QT_WIDGETS_C_EXPORT QLCDNumber::Mode qt_widgets_c_QLCDNumber_mode(const QLCDNumber* this_ptr);
QT_WIDGETS_C_EXPORT QLCDNumber* qt_widgets_c_QLCDNumber_new_no_args();
QT_WIDGETS_C_EXPORT QLCDNumber* qt_widgets_c_QLCDNumber_new_numDigits(unsigned int numDigits);
QT_WIDGETS_C_EXPORT QLCDNumber* qt_widgets_c_QLCDNumber_new_numDigits_parent(unsigned int numDigits, QWidget* parent);
QT_WIDGETS_C_EXPORT QLCDNumber* qt_widgets_c_QLCDNumber_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLCDNumber_qt_metacall(QLCDNumber* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QLCDNumber_qt_metacast(QLCDNumber* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT QLCDNumber::SegmentStyle qt_widgets_c_QLCDNumber_segmentStyle(const QLCDNumber* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLCDNumber_setBinMode(QLCDNumber* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLCDNumber_setDecMode(QLCDNumber* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLCDNumber_setDigitCount(QLCDNumber* this_ptr, int nDigits);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLCDNumber_setHexMode(QLCDNumber* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLCDNumber_setMode(QLCDNumber* this_ptr, QLCDNumber::Mode arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLCDNumber_setOctMode(QLCDNumber* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLCDNumber_setSegmentStyle(QLCDNumber* this_ptr, QLCDNumber::SegmentStyle arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLCDNumber_setSmallDecimalPoint(QLCDNumber* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLCDNumber_sizeHint_to_output(const QLCDNumber* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLCDNumber_smallDecimalPoint(const QLCDNumber* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLCDNumber_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLCDNumber_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QLCDNumber_value(const QLCDNumber* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QLCDNUMBER_H
