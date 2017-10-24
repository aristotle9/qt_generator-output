#ifndef QT_CORE_C_QEASINGCURVE_H
#define QT_CORE_C_QEASINGCURVE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QEasingCurve_G_swap(QEasingCurve* value1, QEasingCurve* value2);
QT_CORE_C_EXPORT void qt_core_c_QEasingCurve_addCubicBezierSegment(QEasingCurve* this_ptr, const QPointF* c1, const QPointF* c2, const QPointF* endPoint);
QT_CORE_C_EXPORT void qt_core_c_QEasingCurve_addTCBSegment(QEasingCurve* this_ptr, const QPointF* nextPoint, double t, double c, double b);
QT_CORE_C_EXPORT double qt_core_c_QEasingCurve_amplitude(const QEasingCurve* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QEasingCurve_constructor_no_args(QEasingCurve* output);
QT_CORE_C_EXPORT void qt_core_c_QEasingCurve_constructor_other(const QEasingCurve* other, QEasingCurve* output);
QT_CORE_C_EXPORT void qt_core_c_QEasingCurve_constructor_type(QEasingCurve::Type type, QEasingCurve* output);
QT_CORE_C_EXPORT double (*qt_core_c_QEasingCurve_customType(const QEasingCurve* this_ptr))(double);
QT_CORE_C_EXPORT void qt_core_c_QEasingCurve_destructor(QEasingCurve* this_ptr);
QT_CORE_C_EXPORT QEasingCurve* qt_core_c_QEasingCurve_operator_assign(QEasingCurve* this_ptr, const QEasingCurve* other);
QT_CORE_C_EXPORT bool qt_core_c_QEasingCurve_operator_eq(const QEasingCurve* this_ptr, const QEasingCurve* other);
QT_CORE_C_EXPORT bool qt_core_c_QEasingCurve_operator_neq(const QEasingCurve* this_ptr, const QEasingCurve* other);
QT_CORE_C_EXPORT double qt_core_c_QEasingCurve_overshoot(const QEasingCurve* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QEasingCurve_period(const QEasingCurve* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QEasingCurve_setAmplitude(QEasingCurve* this_ptr, double amplitude);
QT_CORE_C_EXPORT void qt_core_c_QEasingCurve_setCustomType(QEasingCurve* this_ptr, double (*func)(double));
QT_CORE_C_EXPORT void qt_core_c_QEasingCurve_setOvershoot(QEasingCurve* this_ptr, double overshoot);
QT_CORE_C_EXPORT void qt_core_c_QEasingCurve_setPeriod(QEasingCurve* this_ptr, double period);
QT_CORE_C_EXPORT void qt_core_c_QEasingCurve_setType(QEasingCurve* this_ptr, QEasingCurve::Type type);
QT_CORE_C_EXPORT void qt_core_c_QEasingCurve_swap(QEasingCurve* this_ptr, QEasingCurve* other);
QT_CORE_C_EXPORT void qt_core_c_QEasingCurve_toCubicSpline_to_output(const QEasingCurve* this_ptr, QVector< QPointF >* output);
QT_CORE_C_EXPORT QEasingCurve::Type qt_core_c_QEasingCurve_type(const QEasingCurve* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QEasingCurve_valueForProgress(const QEasingCurve* this_ptr, double progress);

} // extern "C"

#endif // QT_CORE_C_QEASINGCURVE_H
