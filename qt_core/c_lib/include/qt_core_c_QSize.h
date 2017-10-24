#ifndef QT_CORE_C_QSIZE_H
#define QT_CORE_C_QSIZE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QDataStream* qt_core_c_QSize_G_operator_shl_QDataStream_QSize(QDataStream* arg1, const QSize* arg2);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QSize_G_operator_shl_QDataStream_QSizeF(QDataStream* arg1, const QSizeF* arg2);
QT_CORE_C_EXPORT void qt_core_c_QSize_G_operator_shl_to_output_QDebug_QSize(const QDebug* arg1, const QSize* arg2, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QSize_G_operator_shl_to_output_QDebug_QSizeF(const QDebug* arg1, const QSizeF* arg2, QDebug* output);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QSize_G_operator_shr_QDataStream_QSize(QDataStream* arg1, QSize* arg2);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QSize_G_operator_shr_QDataStream_QSizeF(QDataStream* arg1, QSizeF* arg2);
QT_CORE_C_EXPORT void qt_core_c_QSize_boundedTo_to_output(const QSize* this_ptr, const QSize* arg1, QSize* output);
QT_CORE_C_EXPORT void qt_core_c_QSize_constructor_no_args(QSize* output);
QT_CORE_C_EXPORT void qt_core_c_QSize_constructor_w_h(int w, int h, QSize* output);
QT_CORE_C_EXPORT void qt_core_c_QSize_destructor(QSize* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSize_expandedTo_to_output(const QSize* this_ptr, const QSize* arg1, QSize* output);
QT_CORE_C_EXPORT int qt_core_c_QSize_height(const QSize* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSize_isEmpty(const QSize* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSize_isNull(const QSize* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSize_isValid(const QSize* this_ptr);
QT_CORE_C_EXPORT QSize* qt_core_c_QSize_operator_add_assign(QSize* this_ptr, const QSize* arg1);
QT_CORE_C_EXPORT QSize* qt_core_c_QSize_operator_div_assign(QSize* this_ptr, double c);
QT_CORE_C_EXPORT QSize* qt_core_c_QSize_operator_mul_assign(QSize* this_ptr, double c);
QT_CORE_C_EXPORT QSize* qt_core_c_QSize_operator_sub_assign(QSize* this_ptr, const QSize* arg1);
QT_CORE_C_EXPORT int* qt_core_c_QSize_rheight(QSize* this_ptr);
QT_CORE_C_EXPORT int* qt_core_c_QSize_rwidth(QSize* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSize_scale_s_mode(QSize* this_ptr, const QSize* s, const Qt::AspectRatioMode* mode);
QT_CORE_C_EXPORT void qt_core_c_QSize_scale_w_h_mode(QSize* this_ptr, int w, int h, const Qt::AspectRatioMode* mode);
QT_CORE_C_EXPORT void qt_core_c_QSize_scaled_to_output_s_mode(const QSize* this_ptr, const QSize* s, const Qt::AspectRatioMode* mode, QSize* output);
QT_CORE_C_EXPORT void qt_core_c_QSize_scaled_to_output_w_h_mode(const QSize* this_ptr, int w, int h, const Qt::AspectRatioMode* mode, QSize* output);
QT_CORE_C_EXPORT void qt_core_c_QSize_setHeight(QSize* this_ptr, int h);
QT_CORE_C_EXPORT void qt_core_c_QSize_setWidth(QSize* this_ptr, int w);
QT_CORE_C_EXPORT void qt_core_c_QSize_transpose(QSize* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSize_transposed_to_output(const QSize* this_ptr, QSize* output);
QT_CORE_C_EXPORT int qt_core_c_QSize_width(const QSize* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QSIZE_H
