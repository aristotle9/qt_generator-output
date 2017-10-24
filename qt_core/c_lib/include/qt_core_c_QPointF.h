#ifndef QT_CORE_C_QPOINTF_H
#define QT_CORE_C_QPOINTF_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QPointF_constructor_no_args(QPointF* output);
QT_CORE_C_EXPORT void qt_core_c_QPointF_constructor_p(const QPoint* p, QPointF* output);
QT_CORE_C_EXPORT void qt_core_c_QPointF_constructor_xpos_ypos(double xpos, double ypos, QPointF* output);
QT_CORE_C_EXPORT void qt_core_c_QPointF_destructor(QPointF* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QPointF_dotProduct(const QPointF* p1, const QPointF* p2);
QT_CORE_C_EXPORT bool qt_core_c_QPointF_isNull(const QPointF* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QPointF_manhattanLength(const QPointF* this_ptr);
QT_CORE_C_EXPORT QPointF* qt_core_c_QPointF_operator_add_assign(QPointF* this_ptr, const QPointF* p);
QT_CORE_C_EXPORT QPointF* qt_core_c_QPointF_operator_div_assign(QPointF* this_ptr, double c);
QT_CORE_C_EXPORT QPointF* qt_core_c_QPointF_operator_mul_assign(QPointF* this_ptr, double c);
QT_CORE_C_EXPORT QPointF* qt_core_c_QPointF_operator_sub_assign(QPointF* this_ptr, const QPointF* p);
QT_CORE_C_EXPORT double* qt_core_c_QPointF_rx(QPointF* this_ptr);
QT_CORE_C_EXPORT double* qt_core_c_QPointF_ry(QPointF* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QPointF_setX(QPointF* this_ptr, double x);
QT_CORE_C_EXPORT void qt_core_c_QPointF_setY(QPointF* this_ptr, double y);
QT_CORE_C_EXPORT void qt_core_c_QPointF_toPoint_to_output(const QPointF* this_ptr, QPoint* output);
QT_CORE_C_EXPORT double qt_core_c_QPointF_x(const QPointF* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QPointF_y(const QPointF* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QPOINTF_H
