#ifndef QT_CORE_C_QTNUMERIC_H
#define QT_CORE_C_QTNUMERIC_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT quint64 qt_core_c_QtNumeric_G_qFloatDistance_double_double(double a, double b);
QT_CORE_C_EXPORT quint32 qt_core_c_QtNumeric_G_qFloatDistance_float_float(float a, float b);
QT_CORE_C_EXPORT double qt_core_c_QtNumeric_G_qInf();
QT_CORE_C_EXPORT bool qt_core_c_QtNumeric_G_qIsFinite_d(double d);
QT_CORE_C_EXPORT bool qt_core_c_QtNumeric_G_qIsFinite_f(float f);
QT_CORE_C_EXPORT bool qt_core_c_QtNumeric_G_qIsInf_d(double d);
QT_CORE_C_EXPORT bool qt_core_c_QtNumeric_G_qIsInf_f(float f);
QT_CORE_C_EXPORT bool qt_core_c_QtNumeric_G_qIsNaN_d(double d);
QT_CORE_C_EXPORT bool qt_core_c_QtNumeric_G_qIsNaN_f(float f);
QT_CORE_C_EXPORT double qt_core_c_QtNumeric_G_qQNaN();
QT_CORE_C_EXPORT double qt_core_c_QtNumeric_G_qSNaN();

} // extern "C"

#endif // QT_CORE_C_QTNUMERIC_H
