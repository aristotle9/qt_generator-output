#ifndef QT_GUI_C_QTRANSFORM_H
#define QT_GUI_C_QTRANSFORM_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QTransform_G_operator_add_to_output(const QTransform* a, double n, QTransform* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_G_operator_div_to_output(const QTransform* a, double n, QTransform* output);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QTransform_G_operator_mul_as_ptr(const QRegion* r, const QTransform* m);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_G_operator_mul_to_output_QLineF_QTransform(const QLineF* l, const QTransform* m, QLineF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_G_operator_mul_to_output_QLine_QTransform(const QLine* l, const QTransform* m, QLine* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_G_operator_mul_to_output_QPainterPath_QTransform(const QPainterPath* p, const QTransform* m, QPainterPath* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_G_operator_mul_to_output_QPointF_QTransform(const QPointF* p, const QTransform* m, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_G_operator_mul_to_output_QPoint_QTransform(const QPoint* p, const QTransform* m, QPoint* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_G_operator_mul_to_output_QPolygonF_QTransform(const QPolygonF* a, const QTransform* m, QPolygonF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_G_operator_mul_to_output_QPolygon_QTransform(const QPolygon* a, const QTransform* m, QPolygon* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_G_operator_mul_to_output_QTransform_double(const QTransform* a, double n, QTransform* output);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QTransform_G_operator_shl(QDataStream* arg1, const QTransform* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_G_operator_shl_to_output(const QDebug* arg1, const QTransform* arg2, QDebug* output);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QTransform_G_operator_shr(QDataStream* arg1, QTransform* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_G_operator_sub_to_output(const QTransform* a, double n, QTransform* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTransform_G_qFuzzyCompare(const QTransform* t1, const QTransform* t2);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QTransform_G_qHash_key(const QTransform* key);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QTransform_G_qHash_key_seed(const QTransform* key, unsigned int seed);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_adjoint_to_output(const QTransform* this_ptr, QTransform* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_constructor_h11_h12_h13_h21_h22_h23_h31_h32(double h11, double h12, double h13, double h21, double h22, double h23, double h31, double h32, QTransform* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_constructor_h11_h12_h13_h21_h22_h23_h31_h32_h33(double h11, double h12, double h13, double h21, double h22, double h23, double h31, double h32, double h33, QTransform* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_constructor_h11_h12_h21_h22_dx_dy(double h11, double h12, double h21, double h22, double dx, double dy, QTransform* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_constructor_mtx(const QMatrix* mtx, QTransform* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_constructor_no_args(QTransform* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_constructor_other(const QTransform* other, QTransform* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_convert_to_QVariant_to_output(const QTransform* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_destructor(QTransform* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTransform_det(const QTransform* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTransform_determinant(const QTransform* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTransform_dx(const QTransform* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTransform_dy(const QTransform* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_fromScale_to_output(double dx, double dy, QTransform* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_fromTranslate_to_output(double dx, double dy, QTransform* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_inverted_to_output_invertible(const QTransform* this_ptr, bool* invertible, QTransform* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_inverted_to_output_no_args(const QTransform* this_ptr, QTransform* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTransform_isAffine(const QTransform* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTransform_isIdentity(const QTransform* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTransform_isInvertible(const QTransform* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTransform_isRotating(const QTransform* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTransform_isScaling(const QTransform* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTransform_isTranslating(const QTransform* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTransform_m11(const QTransform* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTransform_m12(const QTransform* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTransform_m13(const QTransform* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTransform_m21(const QTransform* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTransform_m22(const QTransform* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTransform_m23(const QTransform* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTransform_m31(const QTransform* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTransform_m32(const QTransform* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTransform_m33(const QTransform* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_mapRect_to_output_QRect(const QTransform* this_ptr, const QRect* arg1, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_mapRect_to_output_QRectF(const QTransform* this_ptr, const QRectF* arg1, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_mapToPolygon_to_output(const QTransform* this_ptr, const QRect* r, QPolygon* output);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QTransform_map_as_ptr(const QTransform* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_map_double_double_double_double(const QTransform* this_ptr, double x, double y, double* tx, double* ty);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_map_int_int_int_int(const QTransform* this_ptr, int x, int y, int* tx, int* ty);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_map_to_output_QLine(const QTransform* this_ptr, const QLine* l, QLine* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_map_to_output_QLineF(const QTransform* this_ptr, const QLineF* l, QLineF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_map_to_output_QPainterPath(const QTransform* this_ptr, const QPainterPath* p, QPainterPath* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_map_to_output_QPoint(const QTransform* this_ptr, const QPoint* p, QPoint* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_map_to_output_QPointF(const QTransform* this_ptr, const QPointF* p, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_map_to_output_QPolygon(const QTransform* this_ptr, const QPolygon* a, QPolygon* output);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_map_to_output_QPolygonF(const QTransform* this_ptr, const QPolygonF* a, QPolygonF* output);
QT_GUI_C_EXPORT QTransform* qt_gui_c_QTransform_operator_add_assign(QTransform* this_ptr, double div);
QT_GUI_C_EXPORT QTransform* qt_gui_c_QTransform_operator_assign(QTransform* this_ptr, const QTransform* arg1);
QT_GUI_C_EXPORT QTransform* qt_gui_c_QTransform_operator_div_assign(QTransform* this_ptr, double div);
QT_GUI_C_EXPORT bool qt_gui_c_QTransform_operator_eq(const QTransform* this_ptr, const QTransform* arg1);
QT_GUI_C_EXPORT QTransform* qt_gui_c_QTransform_operator_mul_assign_arg1(QTransform* this_ptr, const QTransform* arg1);
QT_GUI_C_EXPORT QTransform* qt_gui_c_QTransform_operator_mul_assign_div(QTransform* this_ptr, double div);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_operator_mul_to_output(const QTransform* this_ptr, const QTransform* o, QTransform* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTransform_operator_neq(const QTransform* this_ptr, const QTransform* arg1);
QT_GUI_C_EXPORT QTransform* qt_gui_c_QTransform_operator_sub_assign(QTransform* this_ptr, double div);
QT_GUI_C_EXPORT bool qt_gui_c_QTransform_quadToQuad(const QPolygonF* one, const QPolygonF* two, QTransform* result);
QT_GUI_C_EXPORT bool qt_gui_c_QTransform_quadToSquare(const QPolygonF* quad, QTransform* result);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_reset(QTransform* this_ptr);
QT_GUI_C_EXPORT QTransform* qt_gui_c_QTransform_rotateRadians_a(QTransform* this_ptr, double a);
QT_GUI_C_EXPORT QTransform* qt_gui_c_QTransform_rotateRadians_a_axis(QTransform* this_ptr, double a, const Qt::Axis* axis);
QT_GUI_C_EXPORT QTransform* qt_gui_c_QTransform_rotate_a(QTransform* this_ptr, double a);
QT_GUI_C_EXPORT QTransform* qt_gui_c_QTransform_rotate_a_axis(QTransform* this_ptr, double a, const Qt::Axis* axis);
QT_GUI_C_EXPORT QTransform* qt_gui_c_QTransform_scale(QTransform* this_ptr, double sx, double sy);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_setMatrix(QTransform* this_ptr, double m11, double m12, double m13, double m21, double m22, double m23, double m31, double m32, double m33);
QT_GUI_C_EXPORT QTransform* qt_gui_c_QTransform_shear(QTransform* this_ptr, double sh, double sv);
QT_GUI_C_EXPORT bool qt_gui_c_QTransform_squareToQuad(const QPolygonF* square, QTransform* result);
QT_GUI_C_EXPORT const QMatrix* qt_gui_c_QTransform_toAffine(const QTransform* this_ptr);
QT_GUI_C_EXPORT QTransform* qt_gui_c_QTransform_translate(QTransform* this_ptr, double dx, double dy);
QT_GUI_C_EXPORT void qt_gui_c_QTransform_transposed_to_output(const QTransform* this_ptr, QTransform* output);
QT_GUI_C_EXPORT QTransform::TransformationType qt_gui_c_QTransform_type(const QTransform* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTRANSFORM_H
