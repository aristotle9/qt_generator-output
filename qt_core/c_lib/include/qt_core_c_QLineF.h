#ifndef QT_CORE_C_QLINEF_H
#define QT_CORE_C_QLINEF_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT double qt_core_c_QLineF_angleTo(const QLineF* this_ptr, const QLineF* l);
QT_CORE_C_EXPORT double qt_core_c_QLineF_angle_l(const QLineF* this_ptr, const QLineF* l);
QT_CORE_C_EXPORT double qt_core_c_QLineF_angle_no_args(const QLineF* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QLineF_center_to_output(const QLineF* this_ptr, QPointF* output);
QT_CORE_C_EXPORT void qt_core_c_QLineF_constructor_line(const QLine* line, QLineF* output);
QT_CORE_C_EXPORT void qt_core_c_QLineF_constructor_no_args(QLineF* output);
QT_CORE_C_EXPORT void qt_core_c_QLineF_constructor_pt1_pt2(const QPointF* pt1, const QPointF* pt2, QLineF* output);
QT_CORE_C_EXPORT void qt_core_c_QLineF_constructor_x1_y1_x2_y2(double x1, double y1, double x2, double y2, QLineF* output);
QT_CORE_C_EXPORT void qt_core_c_QLineF_destructor(QLineF* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QLineF_dx(const QLineF* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QLineF_dy(const QLineF* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QLineF_fromPolar_to_output(double length, double angle, QLineF* output);
QT_CORE_C_EXPORT QLineF::IntersectType qt_core_c_QLineF_intersect(const QLineF* this_ptr, const QLineF* l, QPointF* intersectionPoint);
QT_CORE_C_EXPORT bool qt_core_c_QLineF_isNull(const QLineF* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QLineF_length(const QLineF* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QLineF_normalVector_to_output(const QLineF* this_ptr, QLineF* output);
QT_CORE_C_EXPORT bool qt_core_c_QLineF_operator_eq(const QLineF* this_ptr, const QLineF* d);
QT_CORE_C_EXPORT bool qt_core_c_QLineF_operator_neq(const QLineF* this_ptr, const QLineF* d);
QT_CORE_C_EXPORT void qt_core_c_QLineF_p1_to_output(const QLineF* this_ptr, QPointF* output);
QT_CORE_C_EXPORT void qt_core_c_QLineF_p2_to_output(const QLineF* this_ptr, QPointF* output);
QT_CORE_C_EXPORT void qt_core_c_QLineF_pointAt_to_output(const QLineF* this_ptr, double t, QPointF* output);
QT_CORE_C_EXPORT void qt_core_c_QLineF_setAngle(QLineF* this_ptr, double angle);
QT_CORE_C_EXPORT void qt_core_c_QLineF_setLength(QLineF* this_ptr, double len);
QT_CORE_C_EXPORT void qt_core_c_QLineF_setLine(QLineF* this_ptr, double x1, double y1, double x2, double y2);
QT_CORE_C_EXPORT void qt_core_c_QLineF_setP1(QLineF* this_ptr, const QPointF* p1);
QT_CORE_C_EXPORT void qt_core_c_QLineF_setP2(QLineF* this_ptr, const QPointF* p2);
QT_CORE_C_EXPORT void qt_core_c_QLineF_setPoints(QLineF* this_ptr, const QPointF* p1, const QPointF* p2);
QT_CORE_C_EXPORT void qt_core_c_QLineF_toLine_to_output(const QLineF* this_ptr, QLine* output);
QT_CORE_C_EXPORT void qt_core_c_QLineF_translate_dx_dy(QLineF* this_ptr, double dx, double dy);
QT_CORE_C_EXPORT void qt_core_c_QLineF_translate_p(QLineF* this_ptr, const QPointF* p);
QT_CORE_C_EXPORT void qt_core_c_QLineF_translated_to_output_dx_dy(const QLineF* this_ptr, double dx, double dy, QLineF* output);
QT_CORE_C_EXPORT void qt_core_c_QLineF_translated_to_output_p(const QLineF* this_ptr, const QPointF* p, QLineF* output);
QT_CORE_C_EXPORT void qt_core_c_QLineF_unitVector_to_output(const QLineF* this_ptr, QLineF* output);
QT_CORE_C_EXPORT double qt_core_c_QLineF_x1(const QLineF* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QLineF_x2(const QLineF* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QLineF_y1(const QLineF* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QLineF_y2(const QLineF* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QLINEF_H
