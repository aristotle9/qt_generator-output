#ifndef QT_CORE_C_QPOINT_H
#define QT_CORE_C_QPOINT_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QDataStream* qt_core_c_QPoint_G_operator_shl_QDataStream_QPoint(QDataStream* arg1, const QPoint* arg2);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QPoint_G_operator_shl_QDataStream_QPointF(QDataStream* arg1, const QPointF* arg2);
QT_CORE_C_EXPORT void qt_core_c_QPoint_G_operator_shl_to_output_arg1_arg2(const QDebug* arg1, const QPoint* arg2, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QPoint_G_operator_shl_to_output_d_p(const QDebug* d, const QPointF* p, QDebug* output);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QPoint_G_operator_shr_QDataStream_QPoint(QDataStream* arg1, QPoint* arg2);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QPoint_G_operator_shr_QDataStream_QPointF(QDataStream* arg1, QPointF* arg2);
QT_CORE_C_EXPORT void qt_core_c_QPoint_constructor_no_args(QPoint* output);
QT_CORE_C_EXPORT void qt_core_c_QPoint_constructor_xpos_ypos(int xpos, int ypos, QPoint* output);
QT_CORE_C_EXPORT void qt_core_c_QPoint_destructor(QPoint* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QPoint_dotProduct(const QPoint* p1, const QPoint* p2);
QT_CORE_C_EXPORT bool qt_core_c_QPoint_isNull(const QPoint* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QPoint_manhattanLength(const QPoint* this_ptr);
QT_CORE_C_EXPORT QPoint* qt_core_c_QPoint_operator_add_assign(QPoint* this_ptr, const QPoint* p);
QT_CORE_C_EXPORT QPoint* qt_core_c_QPoint_operator_div_assign(QPoint* this_ptr, double divisor);
QT_CORE_C_EXPORT QPoint* qt_core_c_QPoint_operator_mul_assign_double(QPoint* this_ptr, double factor);
QT_CORE_C_EXPORT QPoint* qt_core_c_QPoint_operator_mul_assign_float(QPoint* this_ptr, float factor);
QT_CORE_C_EXPORT QPoint* qt_core_c_QPoint_operator_mul_assign_int(QPoint* this_ptr, int factor);
QT_CORE_C_EXPORT QPoint* qt_core_c_QPoint_operator_sub_assign(QPoint* this_ptr, const QPoint* p);
QT_CORE_C_EXPORT int* qt_core_c_QPoint_rx(QPoint* this_ptr);
QT_CORE_C_EXPORT int* qt_core_c_QPoint_ry(QPoint* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QPoint_setX(QPoint* this_ptr, int x);
QT_CORE_C_EXPORT void qt_core_c_QPoint_setY(QPoint* this_ptr, int y);
QT_CORE_C_EXPORT int qt_core_c_QPoint_x(const QPoint* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QPoint_y(const QPoint* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QPOINT_H
