#ifndef QT_CORE_C_QRECTF_H
#define QT_CORE_C_QRECTF_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QRectF_adjust(QRectF* this_ptr, double x1, double y1, double x2, double y2);
QT_CORE_C_EXPORT void qt_core_c_QRectF_adjusted_to_output(const QRectF* this_ptr, double x1, double y1, double x2, double y2, QRectF* output);
QT_CORE_C_EXPORT double qt_core_c_QRectF_bottom(const QRectF* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRectF_bottomLeft_to_output(const QRectF* this_ptr, QPointF* output);
QT_CORE_C_EXPORT void qt_core_c_QRectF_bottomRight_to_output(const QRectF* this_ptr, QPointF* output);
QT_CORE_C_EXPORT void qt_core_c_QRectF_center_to_output(const QRectF* this_ptr, QPointF* output);
QT_CORE_C_EXPORT void qt_core_c_QRectF_constructor_left_top_width_height(double left, double top, double width, double height, QRectF* output);
QT_CORE_C_EXPORT void qt_core_c_QRectF_constructor_no_args(QRectF* output);
QT_CORE_C_EXPORT void qt_core_c_QRectF_constructor_rect(const QRect* rect, QRectF* output);
QT_CORE_C_EXPORT void qt_core_c_QRectF_constructor_topleft_bottomRight(const QPointF* topleft, const QPointF* bottomRight, QRectF* output);
QT_CORE_C_EXPORT void qt_core_c_QRectF_constructor_topleft_size(const QPointF* topleft, const QSizeF* size, QRectF* output);
QT_CORE_C_EXPORT bool qt_core_c_QRectF_contains_p(const QRectF* this_ptr, const QPointF* p);
QT_CORE_C_EXPORT bool qt_core_c_QRectF_contains_r(const QRectF* this_ptr, const QRectF* r);
QT_CORE_C_EXPORT bool qt_core_c_QRectF_contains_x_y(const QRectF* this_ptr, double x, double y);
QT_CORE_C_EXPORT void qt_core_c_QRectF_destructor(QRectF* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRectF_getCoords(const QRectF* this_ptr, double* x1, double* y1, double* x2, double* y2);
QT_CORE_C_EXPORT void qt_core_c_QRectF_getRect(const QRectF* this_ptr, double* x, double* y, double* w, double* h);
QT_CORE_C_EXPORT double qt_core_c_QRectF_height(const QRectF* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRectF_intersected_to_output(const QRectF* this_ptr, const QRectF* other, QRectF* output);
QT_CORE_C_EXPORT bool qt_core_c_QRectF_intersects(const QRectF* this_ptr, const QRectF* r);
QT_CORE_C_EXPORT bool qt_core_c_QRectF_isEmpty(const QRectF* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QRectF_isNull(const QRectF* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QRectF_isValid(const QRectF* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QRectF_left(const QRectF* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRectF_marginsAdded_to_output(const QRectF* this_ptr, const QMarginsF* margins, QRectF* output);
QT_CORE_C_EXPORT void qt_core_c_QRectF_marginsRemoved_to_output(const QRectF* this_ptr, const QMarginsF* margins, QRectF* output);
QT_CORE_C_EXPORT void qt_core_c_QRectF_moveBottom(QRectF* this_ptr, double pos);
QT_CORE_C_EXPORT void qt_core_c_QRectF_moveBottomLeft(QRectF* this_ptr, const QPointF* p);
QT_CORE_C_EXPORT void qt_core_c_QRectF_moveBottomRight(QRectF* this_ptr, const QPointF* p);
QT_CORE_C_EXPORT void qt_core_c_QRectF_moveCenter(QRectF* this_ptr, const QPointF* p);
QT_CORE_C_EXPORT void qt_core_c_QRectF_moveLeft(QRectF* this_ptr, double pos);
QT_CORE_C_EXPORT void qt_core_c_QRectF_moveRight(QRectF* this_ptr, double pos);
QT_CORE_C_EXPORT void qt_core_c_QRectF_moveTo_p(QRectF* this_ptr, const QPointF* p);
QT_CORE_C_EXPORT void qt_core_c_QRectF_moveTo_x_y(QRectF* this_ptr, double x, double y);
QT_CORE_C_EXPORT void qt_core_c_QRectF_moveTop(QRectF* this_ptr, double pos);
QT_CORE_C_EXPORT void qt_core_c_QRectF_moveTopLeft(QRectF* this_ptr, const QPointF* p);
QT_CORE_C_EXPORT void qt_core_c_QRectF_moveTopRight(QRectF* this_ptr, const QPointF* p);
QT_CORE_C_EXPORT void qt_core_c_QRectF_normalized_to_output(const QRectF* this_ptr, QRectF* output);
QT_CORE_C_EXPORT QRectF* qt_core_c_QRectF_operator_add_assign(QRectF* this_ptr, const QMarginsF* margins);
QT_CORE_C_EXPORT QRectF* qt_core_c_QRectF_operator_bit_and_assign(QRectF* this_ptr, const QRectF* r);
QT_CORE_C_EXPORT void qt_core_c_QRectF_operator_bit_and_to_output(const QRectF* this_ptr, const QRectF* r, QRectF* output);
QT_CORE_C_EXPORT QRectF* qt_core_c_QRectF_operator_bit_or_assign(QRectF* this_ptr, const QRectF* r);
QT_CORE_C_EXPORT void qt_core_c_QRectF_operator_bit_or_to_output(const QRectF* this_ptr, const QRectF* r, QRectF* output);
QT_CORE_C_EXPORT QRectF* qt_core_c_QRectF_operator_sub_assign(QRectF* this_ptr, const QMarginsF* margins);
QT_CORE_C_EXPORT double qt_core_c_QRectF_right(const QRectF* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRectF_setBottom(QRectF* this_ptr, double pos);
QT_CORE_C_EXPORT void qt_core_c_QRectF_setBottomLeft(QRectF* this_ptr, const QPointF* p);
QT_CORE_C_EXPORT void qt_core_c_QRectF_setBottomRight(QRectF* this_ptr, const QPointF* p);
QT_CORE_C_EXPORT void qt_core_c_QRectF_setCoords(QRectF* this_ptr, double x1, double y1, double x2, double y2);
QT_CORE_C_EXPORT void qt_core_c_QRectF_setHeight(QRectF* this_ptr, double h);
QT_CORE_C_EXPORT void qt_core_c_QRectF_setLeft(QRectF* this_ptr, double pos);
QT_CORE_C_EXPORT void qt_core_c_QRectF_setRect(QRectF* this_ptr, double x, double y, double w, double h);
QT_CORE_C_EXPORT void qt_core_c_QRectF_setRight(QRectF* this_ptr, double pos);
QT_CORE_C_EXPORT void qt_core_c_QRectF_setSize(QRectF* this_ptr, const QSizeF* s);
QT_CORE_C_EXPORT void qt_core_c_QRectF_setTop(QRectF* this_ptr, double pos);
QT_CORE_C_EXPORT void qt_core_c_QRectF_setTopLeft(QRectF* this_ptr, const QPointF* p);
QT_CORE_C_EXPORT void qt_core_c_QRectF_setTopRight(QRectF* this_ptr, const QPointF* p);
QT_CORE_C_EXPORT void qt_core_c_QRectF_setWidth(QRectF* this_ptr, double w);
QT_CORE_C_EXPORT void qt_core_c_QRectF_setX(QRectF* this_ptr, double pos);
QT_CORE_C_EXPORT void qt_core_c_QRectF_setY(QRectF* this_ptr, double pos);
QT_CORE_C_EXPORT void qt_core_c_QRectF_size_to_output(const QRectF* this_ptr, QSizeF* output);
QT_CORE_C_EXPORT void qt_core_c_QRectF_toAlignedRect_to_output(const QRectF* this_ptr, QRect* output);
QT_CORE_C_EXPORT void qt_core_c_QRectF_toRect_to_output(const QRectF* this_ptr, QRect* output);
QT_CORE_C_EXPORT double qt_core_c_QRectF_top(const QRectF* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRectF_topLeft_to_output(const QRectF* this_ptr, QPointF* output);
QT_CORE_C_EXPORT void qt_core_c_QRectF_topRight_to_output(const QRectF* this_ptr, QPointF* output);
QT_CORE_C_EXPORT void qt_core_c_QRectF_translate_dx_dy(QRectF* this_ptr, double dx, double dy);
QT_CORE_C_EXPORT void qt_core_c_QRectF_translate_p(QRectF* this_ptr, const QPointF* p);
QT_CORE_C_EXPORT void qt_core_c_QRectF_translated_to_output_dx_dy(const QRectF* this_ptr, double dx, double dy, QRectF* output);
QT_CORE_C_EXPORT void qt_core_c_QRectF_translated_to_output_p(const QRectF* this_ptr, const QPointF* p, QRectF* output);
QT_CORE_C_EXPORT void qt_core_c_QRectF_transposed_to_output(const QRectF* this_ptr, QRectF* output);
QT_CORE_C_EXPORT void qt_core_c_QRectF_united_to_output(const QRectF* this_ptr, const QRectF* other, QRectF* output);
QT_CORE_C_EXPORT double qt_core_c_QRectF_width(const QRectF* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QRectF_x(const QRectF* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QRectF_y(const QRectF* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QRECTF_H
