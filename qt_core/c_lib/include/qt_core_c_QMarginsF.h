#ifndef QT_CORE_C_QMARGINSF_H
#define QT_CORE_C_QMARGINSF_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT double qt_core_c_QMarginsF_bottom(const QMarginsF* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMarginsF_constructor_left_top_right_bottom(double left, double top, double right, double bottom, QMarginsF* output);
QT_CORE_C_EXPORT void qt_core_c_QMarginsF_constructor_margins(const QMargins* margins, QMarginsF* output);
QT_CORE_C_EXPORT void qt_core_c_QMarginsF_constructor_no_args(QMarginsF* output);
QT_CORE_C_EXPORT void qt_core_c_QMarginsF_destructor(QMarginsF* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMarginsF_isNull(const QMarginsF* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QMarginsF_left(const QMarginsF* this_ptr);
QT_CORE_C_EXPORT QMarginsF* qt_core_c_QMarginsF_operator_add_assign_addend(QMarginsF* this_ptr, double addend);
QT_CORE_C_EXPORT QMarginsF* qt_core_c_QMarginsF_operator_add_assign_margins(QMarginsF* this_ptr, const QMarginsF* margins);
QT_CORE_C_EXPORT QMarginsF* qt_core_c_QMarginsF_operator_div_assign(QMarginsF* this_ptr, double divisor);
QT_CORE_C_EXPORT QMarginsF* qt_core_c_QMarginsF_operator_mul_assign(QMarginsF* this_ptr, double factor);
QT_CORE_C_EXPORT QMarginsF* qt_core_c_QMarginsF_operator_sub_assign_margins(QMarginsF* this_ptr, const QMarginsF* margins);
QT_CORE_C_EXPORT QMarginsF* qt_core_c_QMarginsF_operator_sub_assign_subtrahend(QMarginsF* this_ptr, double subtrahend);
QT_CORE_C_EXPORT double qt_core_c_QMarginsF_right(const QMarginsF* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMarginsF_setBottom(QMarginsF* this_ptr, double bottom);
QT_CORE_C_EXPORT void qt_core_c_QMarginsF_setLeft(QMarginsF* this_ptr, double left);
QT_CORE_C_EXPORT void qt_core_c_QMarginsF_setRight(QMarginsF* this_ptr, double right);
QT_CORE_C_EXPORT void qt_core_c_QMarginsF_setTop(QMarginsF* this_ptr, double top);
QT_CORE_C_EXPORT void qt_core_c_QMarginsF_toMargins_to_output(const QMarginsF* this_ptr, QMargins* output);
QT_CORE_C_EXPORT double qt_core_c_QMarginsF_top(const QMarginsF* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QMARGINSF_H
