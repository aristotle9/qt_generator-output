#ifndef QT_CORE_C_QSIZEF_H
#define QT_CORE_C_QSIZEF_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QSizeF_boundedTo_to_output(const QSizeF* this_ptr, const QSizeF* arg1, QSizeF* output);
QT_CORE_C_EXPORT void qt_core_c_QSizeF_constructor_no_args(QSizeF* output);
QT_CORE_C_EXPORT void qt_core_c_QSizeF_constructor_sz(const QSize* sz, QSizeF* output);
QT_CORE_C_EXPORT void qt_core_c_QSizeF_constructor_w_h(double w, double h, QSizeF* output);
QT_CORE_C_EXPORT void qt_core_c_QSizeF_destructor(QSizeF* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSizeF_expandedTo_to_output(const QSizeF* this_ptr, const QSizeF* arg1, QSizeF* output);
QT_CORE_C_EXPORT double qt_core_c_QSizeF_height(const QSizeF* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSizeF_isEmpty(const QSizeF* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSizeF_isNull(const QSizeF* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSizeF_isValid(const QSizeF* this_ptr);
QT_CORE_C_EXPORT QSizeF* qt_core_c_QSizeF_operator_add_assign(QSizeF* this_ptr, const QSizeF* arg1);
QT_CORE_C_EXPORT QSizeF* qt_core_c_QSizeF_operator_div_assign(QSizeF* this_ptr, double c);
QT_CORE_C_EXPORT QSizeF* qt_core_c_QSizeF_operator_mul_assign(QSizeF* this_ptr, double c);
QT_CORE_C_EXPORT QSizeF* qt_core_c_QSizeF_operator_sub_assign(QSizeF* this_ptr, const QSizeF* arg1);
QT_CORE_C_EXPORT double* qt_core_c_QSizeF_rheight(QSizeF* this_ptr);
QT_CORE_C_EXPORT double* qt_core_c_QSizeF_rwidth(QSizeF* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSizeF_scale_s_mode(QSizeF* this_ptr, const QSizeF* s, const Qt::AspectRatioMode* mode);
QT_CORE_C_EXPORT void qt_core_c_QSizeF_scale_w_h_mode(QSizeF* this_ptr, double w, double h, const Qt::AspectRatioMode* mode);
QT_CORE_C_EXPORT void qt_core_c_QSizeF_scaled_to_output_s_mode(const QSizeF* this_ptr, const QSizeF* s, const Qt::AspectRatioMode* mode, QSizeF* output);
QT_CORE_C_EXPORT void qt_core_c_QSizeF_scaled_to_output_w_h_mode(const QSizeF* this_ptr, double w, double h, const Qt::AspectRatioMode* mode, QSizeF* output);
QT_CORE_C_EXPORT void qt_core_c_QSizeF_setHeight(QSizeF* this_ptr, double h);
QT_CORE_C_EXPORT void qt_core_c_QSizeF_setWidth(QSizeF* this_ptr, double w);
QT_CORE_C_EXPORT void qt_core_c_QSizeF_toSize_to_output(const QSizeF* this_ptr, QSize* output);
QT_CORE_C_EXPORT void qt_core_c_QSizeF_transpose(QSizeF* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSizeF_transposed_to_output(const QSizeF* this_ptr, QSizeF* output);
QT_CORE_C_EXPORT double qt_core_c_QSizeF_width(const QSizeF* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QSIZEF_H
