#ifndef QT_GUI_C_QMATRIX_H
#define QT_GUI_C_QMATRIX_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QRegion* qt_gui_c_QMatrix_G_operator_mul_as_ptr(const QRegion* r, const QMatrix* m);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_G_operator_mul_to_output_QLineF_QMatrix(const QLineF* l, const QMatrix* m, QLineF* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_G_operator_mul_to_output_QLine_QMatrix(const QLine* l, const QMatrix* m, QLine* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_G_operator_mul_to_output_QPainterPath_QMatrix(const QPainterPath* p, const QMatrix* m, QPainterPath* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_G_operator_mul_to_output_QPointF_QMatrix(const QPointF* p, const QMatrix* m, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_G_operator_mul_to_output_QPoint_QMatrix(const QPoint* p, const QMatrix* m, QPoint* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_G_operator_mul_to_output_QPolygonF_QMatrix(const QPolygonF* a, const QMatrix* m, QPolygonF* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_G_operator_mul_to_output_QPolygon_QMatrix(const QPolygon* a, const QMatrix* m, QPolygon* output);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QMatrix_G_operator_shl(QDataStream* arg1, const QMatrix* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_G_operator_shl_to_output(const QDebug* arg1, const QMatrix* arg2, QDebug* output);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QMatrix_G_operator_shr(QDataStream* arg1, QMatrix* arg2);
QT_GUI_C_EXPORT bool qt_gui_c_QMatrix_G_qFuzzyCompare(const QMatrix* m1, const QMatrix* m2);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QMatrix_G_qHash_key(const QMatrix* key);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QMatrix_G_qHash_key_seed(const QMatrix* key, unsigned int seed);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_constructor_m11_m12_m21_m22_dx_dy(double m11, double m12, double m21, double m22, double dx, double dy, QMatrix* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_constructor_no_args(QMatrix* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_constructor_other(const QMatrix* other, QMatrix* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_convert_to_QVariant_to_output(const QMatrix* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_destructor(QMatrix* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QMatrix_determinant(const QMatrix* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QMatrix_dx(const QMatrix* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QMatrix_dy(const QMatrix* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_inverted_to_output_invertible(const QMatrix* this_ptr, bool* invertible, QMatrix* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_inverted_to_output_no_args(const QMatrix* this_ptr, QMatrix* output);
QT_GUI_C_EXPORT bool qt_gui_c_QMatrix_isIdentity(const QMatrix* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QMatrix_isInvertible(const QMatrix* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QMatrix_m11(const QMatrix* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QMatrix_m12(const QMatrix* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QMatrix_m21(const QMatrix* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QMatrix_m22(const QMatrix* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_mapRect_to_output_QRect(const QMatrix* this_ptr, const QRect* arg1, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_mapRect_to_output_QRectF(const QMatrix* this_ptr, const QRectF* arg1, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_mapToPolygon_to_output(const QMatrix* this_ptr, const QRect* r, QPolygon* output);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QMatrix_map_as_ptr(const QMatrix* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_map_double_double_double_double(const QMatrix* this_ptr, double x, double y, double* tx, double* ty);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_map_int_int_int_int(const QMatrix* this_ptr, int x, int y, int* tx, int* ty);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_map_to_output_QLine(const QMatrix* this_ptr, const QLine* l, QLine* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_map_to_output_QLineF(const QMatrix* this_ptr, const QLineF* l, QLineF* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_map_to_output_QPainterPath(const QMatrix* this_ptr, const QPainterPath* p, QPainterPath* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_map_to_output_QPoint(const QMatrix* this_ptr, const QPoint* p, QPoint* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_map_to_output_QPointF(const QMatrix* this_ptr, const QPointF* p, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_map_to_output_QPolygon(const QMatrix* this_ptr, const QPolygon* a, QPolygon* output);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_map_to_output_QPolygonF(const QMatrix* this_ptr, const QPolygonF* a, QPolygonF* output);
QT_GUI_C_EXPORT QMatrix* qt_gui_c_QMatrix_operator_assign(QMatrix* this_ptr, const QMatrix* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QMatrix_operator_eq(const QMatrix* this_ptr, const QMatrix* arg1);
QT_GUI_C_EXPORT QMatrix* qt_gui_c_QMatrix_operator_mul_assign(QMatrix* this_ptr, const QMatrix* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_operator_mul_to_output(const QMatrix* this_ptr, const QMatrix* o, QMatrix* output);
QT_GUI_C_EXPORT bool qt_gui_c_QMatrix_operator_neq(const QMatrix* this_ptr, const QMatrix* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_reset(QMatrix* this_ptr);
QT_GUI_C_EXPORT QMatrix* qt_gui_c_QMatrix_rotate(QMatrix* this_ptr, double a);
QT_GUI_C_EXPORT QMatrix* qt_gui_c_QMatrix_scale(QMatrix* this_ptr, double sx, double sy);
QT_GUI_C_EXPORT void qt_gui_c_QMatrix_setMatrix(QMatrix* this_ptr, double m11, double m12, double m21, double m22, double dx, double dy);
QT_GUI_C_EXPORT QMatrix* qt_gui_c_QMatrix_shear(QMatrix* this_ptr, double sh, double sv);
QT_GUI_C_EXPORT QMatrix* qt_gui_c_QMatrix_translate(QMatrix* this_ptr, double dx, double dy);

} // extern "C"

#endif // QT_GUI_C_QMATRIX_H
