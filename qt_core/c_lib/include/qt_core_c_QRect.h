#ifndef QT_CORE_C_QRECT_H
#define QT_CORE_C_QRECT_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QRect_G_operator_add_to_output_QMarginsF_QRectF(const QMarginsF* lhs, const QRectF* rhs, QRectF* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_G_operator_add_to_output_QMargins_QRect(const QMargins* margins, const QRect* rectangle, QRect* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_G_operator_add_to_output_QRectF_QMarginsF(const QRectF* lhs, const QMarginsF* rhs, QRectF* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_G_operator_add_to_output_QRect_QMargins(const QRect* rectangle, const QMargins* margins, QRect* output);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QRect_G_operator_shl_QDataStream_QRect(QDataStream* arg1, const QRect* arg2);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QRect_G_operator_shl_QDataStream_QRectF(QDataStream* arg1, const QRectF* arg2);
QT_CORE_C_EXPORT void qt_core_c_QRect_G_operator_shl_to_output_QDebug_QRect(const QDebug* arg1, const QRect* arg2, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_G_operator_shl_to_output_QDebug_QRectF(const QDebug* arg1, const QRectF* arg2, QDebug* output);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QRect_G_operator_shr_QDataStream_QRect(QDataStream* arg1, QRect* arg2);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QRect_G_operator_shr_QDataStream_QRectF(QDataStream* arg1, QRectF* arg2);
QT_CORE_C_EXPORT void qt_core_c_QRect_G_operator_sub_to_output_QRectF_QMarginsF(const QRectF* lhs, const QMarginsF* rhs, QRectF* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_G_operator_sub_to_output_QRect_QMargins(const QRect* lhs, const QMargins* rhs, QRect* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_adjust(QRect* this_ptr, int x1, int y1, int x2, int y2);
QT_CORE_C_EXPORT void qt_core_c_QRect_adjusted_to_output(const QRect* this_ptr, int x1, int y1, int x2, int y2, QRect* output);
QT_CORE_C_EXPORT int qt_core_c_QRect_bottom(const QRect* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRect_bottomLeft_to_output(const QRect* this_ptr, QPoint* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_bottomRight_to_output(const QRect* this_ptr, QPoint* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_center_to_output(const QRect* this_ptr, QPoint* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_constructor_left_top_width_height(int left, int top, int width, int height, QRect* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_constructor_no_args(QRect* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_constructor_topleft_bottomright(const QPoint* topleft, const QPoint* bottomright, QRect* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_constructor_topleft_size(const QPoint* topleft, const QSize* size, QRect* output);
QT_CORE_C_EXPORT bool qt_core_c_QRect_contains_p(const QRect* this_ptr, const QPoint* p);
QT_CORE_C_EXPORT bool qt_core_c_QRect_contains_p_proper(const QRect* this_ptr, const QPoint* p, bool proper);
QT_CORE_C_EXPORT bool qt_core_c_QRect_contains_r(const QRect* this_ptr, const QRect* r);
QT_CORE_C_EXPORT bool qt_core_c_QRect_contains_r_proper(const QRect* this_ptr, const QRect* r, bool proper);
QT_CORE_C_EXPORT bool qt_core_c_QRect_contains_x_y(const QRect* this_ptr, int x, int y);
QT_CORE_C_EXPORT bool qt_core_c_QRect_contains_x_y_proper(const QRect* this_ptr, int x, int y, bool proper);
QT_CORE_C_EXPORT void qt_core_c_QRect_destructor(QRect* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRect_getCoords(const QRect* this_ptr, int* x1, int* y1, int* x2, int* y2);
QT_CORE_C_EXPORT void qt_core_c_QRect_getRect(const QRect* this_ptr, int* x, int* y, int* w, int* h);
QT_CORE_C_EXPORT int qt_core_c_QRect_height(const QRect* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRect_intersected_to_output(const QRect* this_ptr, const QRect* other, QRect* output);
QT_CORE_C_EXPORT bool qt_core_c_QRect_intersects(const QRect* this_ptr, const QRect* r);
QT_CORE_C_EXPORT bool qt_core_c_QRect_isEmpty(const QRect* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QRect_isNull(const QRect* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QRect_isValid(const QRect* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QRect_left(const QRect* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRect_marginsAdded_to_output(const QRect* this_ptr, const QMargins* margins, QRect* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_marginsRemoved_to_output(const QRect* this_ptr, const QMargins* margins, QRect* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_moveBottom(QRect* this_ptr, int pos);
QT_CORE_C_EXPORT void qt_core_c_QRect_moveBottomLeft(QRect* this_ptr, const QPoint* p);
QT_CORE_C_EXPORT void qt_core_c_QRect_moveBottomRight(QRect* this_ptr, const QPoint* p);
QT_CORE_C_EXPORT void qt_core_c_QRect_moveCenter(QRect* this_ptr, const QPoint* p);
QT_CORE_C_EXPORT void qt_core_c_QRect_moveLeft(QRect* this_ptr, int pos);
QT_CORE_C_EXPORT void qt_core_c_QRect_moveRight(QRect* this_ptr, int pos);
QT_CORE_C_EXPORT void qt_core_c_QRect_moveTo_p(QRect* this_ptr, const QPoint* p);
QT_CORE_C_EXPORT void qt_core_c_QRect_moveTo_x_t(QRect* this_ptr, int x, int t);
QT_CORE_C_EXPORT void qt_core_c_QRect_moveTop(QRect* this_ptr, int pos);
QT_CORE_C_EXPORT void qt_core_c_QRect_moveTopLeft(QRect* this_ptr, const QPoint* p);
QT_CORE_C_EXPORT void qt_core_c_QRect_moveTopRight(QRect* this_ptr, const QPoint* p);
QT_CORE_C_EXPORT void qt_core_c_QRect_normalized_to_output(const QRect* this_ptr, QRect* output);
QT_CORE_C_EXPORT QRect* qt_core_c_QRect_operator_add_assign(QRect* this_ptr, const QMargins* margins);
QT_CORE_C_EXPORT QRect* qt_core_c_QRect_operator_bit_and_assign(QRect* this_ptr, const QRect* r);
QT_CORE_C_EXPORT void qt_core_c_QRect_operator_bit_and_to_output(const QRect* this_ptr, const QRect* r, QRect* output);
QT_CORE_C_EXPORT QRect* qt_core_c_QRect_operator_bit_or_assign(QRect* this_ptr, const QRect* r);
QT_CORE_C_EXPORT void qt_core_c_QRect_operator_bit_or_to_output(const QRect* this_ptr, const QRect* r, QRect* output);
QT_CORE_C_EXPORT QRect* qt_core_c_QRect_operator_sub_assign(QRect* this_ptr, const QMargins* margins);
QT_CORE_C_EXPORT int qt_core_c_QRect_right(const QRect* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRect_setBottom(QRect* this_ptr, int pos);
QT_CORE_C_EXPORT void qt_core_c_QRect_setBottomLeft(QRect* this_ptr, const QPoint* p);
QT_CORE_C_EXPORT void qt_core_c_QRect_setBottomRight(QRect* this_ptr, const QPoint* p);
QT_CORE_C_EXPORT void qt_core_c_QRect_setCoords(QRect* this_ptr, int x1, int y1, int x2, int y2);
QT_CORE_C_EXPORT void qt_core_c_QRect_setHeight(QRect* this_ptr, int h);
QT_CORE_C_EXPORT void qt_core_c_QRect_setLeft(QRect* this_ptr, int pos);
QT_CORE_C_EXPORT void qt_core_c_QRect_setRect(QRect* this_ptr, int x, int y, int w, int h);
QT_CORE_C_EXPORT void qt_core_c_QRect_setRight(QRect* this_ptr, int pos);
QT_CORE_C_EXPORT void qt_core_c_QRect_setSize(QRect* this_ptr, const QSize* s);
QT_CORE_C_EXPORT void qt_core_c_QRect_setTop(QRect* this_ptr, int pos);
QT_CORE_C_EXPORT void qt_core_c_QRect_setTopLeft(QRect* this_ptr, const QPoint* p);
QT_CORE_C_EXPORT void qt_core_c_QRect_setTopRight(QRect* this_ptr, const QPoint* p);
QT_CORE_C_EXPORT void qt_core_c_QRect_setWidth(QRect* this_ptr, int w);
QT_CORE_C_EXPORT void qt_core_c_QRect_setX(QRect* this_ptr, int x);
QT_CORE_C_EXPORT void qt_core_c_QRect_setY(QRect* this_ptr, int y);
QT_CORE_C_EXPORT void qt_core_c_QRect_size_to_output(const QRect* this_ptr, QSize* output);
QT_CORE_C_EXPORT int qt_core_c_QRect_top(const QRect* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRect_topLeft_to_output(const QRect* this_ptr, QPoint* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_topRight_to_output(const QRect* this_ptr, QPoint* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_translate_dx_dy(QRect* this_ptr, int dx, int dy);
QT_CORE_C_EXPORT void qt_core_c_QRect_translate_p(QRect* this_ptr, const QPoint* p);
QT_CORE_C_EXPORT void qt_core_c_QRect_translated_to_output_dx_dy(const QRect* this_ptr, int dx, int dy, QRect* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_translated_to_output_p(const QRect* this_ptr, const QPoint* p, QRect* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_transposed_to_output(const QRect* this_ptr, QRect* output);
QT_CORE_C_EXPORT void qt_core_c_QRect_united_to_output(const QRect* this_ptr, const QRect* other, QRect* output);
QT_CORE_C_EXPORT int qt_core_c_QRect_width(const QRect* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QRect_x(const QRect* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QRect_y(const QRect* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QRECT_H
