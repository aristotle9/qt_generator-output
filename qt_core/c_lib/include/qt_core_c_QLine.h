#ifndef QT_CORE_C_QLINE_H
#define QT_CORE_C_QLINE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QDataStream* qt_core_c_QLine_G_operator_shl_QDataStream_QLine(QDataStream* arg1, const QLine* arg2);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QLine_G_operator_shl_QDataStream_QLineF(QDataStream* arg1, const QLineF* arg2);
QT_CORE_C_EXPORT void qt_core_c_QLine_G_operator_shl_to_output_QDebug_QLine(const QDebug* d, const QLine* p, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QLine_G_operator_shl_to_output_QDebug_QLineF(const QDebug* d, const QLineF* p, QDebug* output);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QLine_G_operator_shr_QDataStream_QLine(QDataStream* arg1, QLine* arg2);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QLine_G_operator_shr_QDataStream_QLineF(QDataStream* arg1, QLineF* arg2);
QT_CORE_C_EXPORT void qt_core_c_QLine_center_to_output(const QLine* this_ptr, QPoint* output);
QT_CORE_C_EXPORT void qt_core_c_QLine_constructor_no_args(QLine* output);
QT_CORE_C_EXPORT void qt_core_c_QLine_constructor_pt1_pt2(const QPoint* pt1, const QPoint* pt2, QLine* output);
QT_CORE_C_EXPORT void qt_core_c_QLine_constructor_x1_y1_x2_y2(int x1, int y1, int x2, int y2, QLine* output);
QT_CORE_C_EXPORT void qt_core_c_QLine_destructor(QLine* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QLine_dx(const QLine* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QLine_dy(const QLine* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QLine_isNull(const QLine* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QLine_operator_eq(const QLine* this_ptr, const QLine* d);
QT_CORE_C_EXPORT bool qt_core_c_QLine_operator_neq(const QLine* this_ptr, const QLine* d);
QT_CORE_C_EXPORT void qt_core_c_QLine_p1_to_output(const QLine* this_ptr, QPoint* output);
QT_CORE_C_EXPORT void qt_core_c_QLine_p2_to_output(const QLine* this_ptr, QPoint* output);
QT_CORE_C_EXPORT void qt_core_c_QLine_setLine(QLine* this_ptr, int x1, int y1, int x2, int y2);
QT_CORE_C_EXPORT void qt_core_c_QLine_setP1(QLine* this_ptr, const QPoint* p1);
QT_CORE_C_EXPORT void qt_core_c_QLine_setP2(QLine* this_ptr, const QPoint* p2);
QT_CORE_C_EXPORT void qt_core_c_QLine_setPoints(QLine* this_ptr, const QPoint* p1, const QPoint* p2);
QT_CORE_C_EXPORT void qt_core_c_QLine_translate_dx_dy(QLine* this_ptr, int dx, int dy);
QT_CORE_C_EXPORT void qt_core_c_QLine_translate_p(QLine* this_ptr, const QPoint* p);
QT_CORE_C_EXPORT void qt_core_c_QLine_translated_to_output_dx_dy(const QLine* this_ptr, int dx, int dy, QLine* output);
QT_CORE_C_EXPORT void qt_core_c_QLine_translated_to_output_p(const QLine* this_ptr, const QPoint* p, QLine* output);
QT_CORE_C_EXPORT int qt_core_c_QLine_x1(const QLine* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QLine_x2(const QLine* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QLine_y1(const QLine* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QLine_y2(const QLine* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QLINE_H
