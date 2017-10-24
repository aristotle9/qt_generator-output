#ifndef QT_CORE_C_QMARGINS_H
#define QT_CORE_C_QMARGINS_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_add_to_output_QMarginsF_QMarginsF(const QMarginsF* lhs, const QMarginsF* rhs, QMarginsF* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_add_to_output_QMarginsF_double(const QMarginsF* lhs, double rhs, QMarginsF* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_add_to_output_QMargins_QMargins(const QMargins* m1, const QMargins* m2, QMargins* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_add_to_output_QMargins_int(const QMargins* lhs, int rhs, QMargins* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_add_to_output_double_QMarginsF(double lhs, const QMarginsF* rhs, QMarginsF* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_add_to_output_int_QMargins(int lhs, const QMargins* rhs, QMargins* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_div_to_output_QMarginsF_double(const QMarginsF* lhs, double divisor, QMarginsF* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_div_to_output_QMargins_double(const QMargins* margins, double divisor, QMargins* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_div_to_output_QMargins_int(const QMargins* margins, int divisor, QMargins* output);
QT_CORE_C_EXPORT bool qt_core_c_QMargins_G_operator_eq(const QMarginsF* lhs, const QMarginsF* rhs);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_mul_to_output_QMarginsF_double(const QMarginsF* lhs, double rhs, QMarginsF* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_mul_to_output_QMargins_double(const QMargins* margins, double factor, QMargins* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_mul_to_output_QMargins_int(const QMargins* margins, int factor, QMargins* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_mul_to_output_double_QMargins(double factor, const QMargins* margins, QMargins* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_mul_to_output_double_QMarginsF(double lhs, const QMarginsF* rhs, QMarginsF* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_mul_to_output_int_QMargins(int factor, const QMargins* margins, QMargins* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_neg_to_output_QMargins(const QMargins* margins, QMargins* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_neg_to_output_QMarginsF(const QMarginsF* margins, QMarginsF* output);
QT_CORE_C_EXPORT bool qt_core_c_QMargins_G_operator_neq(const QMarginsF* lhs, const QMarginsF* rhs);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QMargins_G_operator_shl_QDataStream_QMargins(QDataStream* arg1, const QMargins* arg2);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QMargins_G_operator_shl_QDataStream_QMarginsF(QDataStream* arg1, const QMarginsF* arg2);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_shl_to_output_QDebug_QMargins(const QDebug* arg1, const QMargins* arg2, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_shl_to_output_QDebug_QMarginsF(const QDebug* arg1, const QMarginsF* arg2, QDebug* output);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QMargins_G_operator_shr_QDataStream_QMargins(QDataStream* arg1, QMargins* arg2);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QMargins_G_operator_shr_QDataStream_QMarginsF(QDataStream* arg1, QMarginsF* arg2);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_sub_to_output_QMarginsF_QMarginsF(const QMarginsF* lhs, const QMarginsF* rhs, QMarginsF* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_sub_to_output_QMarginsF_double(const QMarginsF* lhs, double rhs, QMarginsF* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_sub_to_output_QMargins_QMargins(const QMargins* m1, const QMargins* m2, QMargins* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_sub_to_output_QMargins_int(const QMargins* lhs, int rhs, QMargins* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_unary_plus_to_output_QMargins(const QMargins* margins, QMargins* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_G_operator_unary_plus_to_output_QMarginsF(const QMarginsF* margins, QMarginsF* output);
QT_CORE_C_EXPORT int qt_core_c_QMargins_bottom(const QMargins* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMargins_constructor_left_top_right_bottom(int left, int top, int right, int bottom, QMargins* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_constructor_no_args(QMargins* output);
QT_CORE_C_EXPORT void qt_core_c_QMargins_destructor(QMargins* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMargins_isNull(const QMargins* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QMargins_left(const QMargins* this_ptr);
QT_CORE_C_EXPORT QMargins* qt_core_c_QMargins_operator_add_assign_arg1(QMargins* this_ptr, int arg1);
QT_CORE_C_EXPORT QMargins* qt_core_c_QMargins_operator_add_assign_margins(QMargins* this_ptr, const QMargins* margins);
QT_CORE_C_EXPORT QMargins* qt_core_c_QMargins_operator_div_assign_double(QMargins* this_ptr, double arg1);
QT_CORE_C_EXPORT QMargins* qt_core_c_QMargins_operator_div_assign_int(QMargins* this_ptr, int arg1);
QT_CORE_C_EXPORT QMargins* qt_core_c_QMargins_operator_mul_assign_double(QMargins* this_ptr, double arg1);
QT_CORE_C_EXPORT QMargins* qt_core_c_QMargins_operator_mul_assign_int(QMargins* this_ptr, int arg1);
QT_CORE_C_EXPORT QMargins* qt_core_c_QMargins_operator_sub_assign_arg1(QMargins* this_ptr, int arg1);
QT_CORE_C_EXPORT QMargins* qt_core_c_QMargins_operator_sub_assign_margins(QMargins* this_ptr, const QMargins* margins);
QT_CORE_C_EXPORT int qt_core_c_QMargins_right(const QMargins* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMargins_setBottom(QMargins* this_ptr, int bottom);
QT_CORE_C_EXPORT void qt_core_c_QMargins_setLeft(QMargins* this_ptr, int left);
QT_CORE_C_EXPORT void qt_core_c_QMargins_setRight(QMargins* this_ptr, int right);
QT_CORE_C_EXPORT void qt_core_c_QMargins_setTop(QMargins* this_ptr, int top);
QT_CORE_C_EXPORT int qt_core_c_QMargins_top(const QMargins* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QMARGINS_H
