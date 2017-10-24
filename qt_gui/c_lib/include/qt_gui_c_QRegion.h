#ifndef QT_GUI_C_QREGION_H
#define QT_GUI_C_QREGION_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QRegion_G_operator_shl_to_output(const QDebug* arg1, const QRegion* arg2, QDebug* output);
QT_GUI_C_EXPORT void qt_gui_c_QRegion_G_swap(QRegion* value1, QRegion* value2);
QT_GUI_C_EXPORT const QRect* qt_gui_c_QRegion_begin(const QRegion* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRegion_boundingRect_to_output(const QRegion* this_ptr, QRect* output);
QT_GUI_C_EXPORT const QRect* qt_gui_c_QRegion_cbegin(const QRegion* this_ptr);
QT_GUI_C_EXPORT const QRect* qt_gui_c_QRegion_cend(const QRegion* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QRegion_contains_p(const QRegion* this_ptr, const QPoint* p);
QT_GUI_C_EXPORT bool qt_gui_c_QRegion_contains_r(const QRegion* this_ptr, const QRect* r);
QT_GUI_C_EXPORT void qt_gui_c_QRegion_convert_to_QVariant_to_output(const QRegion* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QRegion_delete(QRegion* this_ptr);
QT_GUI_C_EXPORT const QRect* qt_gui_c_QRegion_end(const QRegion* this_ptr);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_intersected_as_ptr_QRect(const QRegion* this_ptr, const QRect* r);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_intersected_as_ptr_QRegion(const QRegion* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT bool qt_gui_c_QRegion_intersects_QRect(const QRegion* this_ptr, const QRect* r);
QT_GUI_C_EXPORT bool qt_gui_c_QRegion_intersects_QRegion(const QRegion* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT bool qt_gui_c_QRegion_isEmpty(const QRegion* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QRegion_isNull(const QRegion* this_ptr);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_new_bitmap(const QBitmap* bitmap);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_new_no_args();
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_new_pa(const QPolygon* pa);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_new_pa_fillRule(const QPolygon* pa, const Qt::FillRule* fillRule);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_new_r(const QRect* r);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_new_r_t(const QRect* r, QRegion::RegionType t);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_new_region(const QRegion* region);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_new_x_y_w_h(int x, int y, int w, int h);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_new_x_y_w_h_t(int x, int y, int w, int h, QRegion::RegionType t);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_operator_add_as_ptr_QRect(const QRegion* this_ptr, const QRect* r);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_operator_add_as_ptr_QRegion(const QRegion* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_operator_add_assign_QRect(QRegion* this_ptr, const QRect* r);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_operator_add_assign_QRegion(QRegion* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_operator_assign(QRegion* this_ptr, const QRegion* arg1);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_operator_bit_and_as_ptr_QRect(const QRegion* this_ptr, const QRect* r);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_operator_bit_and_as_ptr_QRegion(const QRegion* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_operator_bit_and_assign_QRect(QRegion* this_ptr, const QRect* r);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_operator_bit_and_assign_QRegion(QRegion* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_operator_bit_or_as_ptr(const QRegion* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_operator_bit_or_assign(QRegion* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_operator_bit_xor_as_ptr(const QRegion* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_operator_bit_xor_assign(QRegion* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT bool qt_gui_c_QRegion_operator_eq(const QRegion* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT bool qt_gui_c_QRegion_operator_neq(const QRegion* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_operator_sub_as_ptr(const QRegion* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_operator_sub_assign(QRegion* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT int qt_gui_c_QRegion_rectCount(const QRegion* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRegion_rects_to_output(const QRegion* this_ptr, QVector< QRect >* output);
QT_GUI_C_EXPORT void qt_gui_c_QRegion_setRects(QRegion* this_ptr, const QRect* rect, int num);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_subtracted_as_ptr(const QRegion* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT void qt_gui_c_QRegion_swap(QRegion* this_ptr, QRegion* other);
QT_GUI_C_EXPORT void qt_gui_c_QRegion_translate_dx_dy(QRegion* this_ptr, int dx, int dy);
QT_GUI_C_EXPORT void qt_gui_c_QRegion_translate_p(QRegion* this_ptr, const QPoint* p);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_translated_as_ptr_dx_dy(const QRegion* this_ptr, int dx, int dy);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_translated_as_ptr_p(const QRegion* this_ptr, const QPoint* p);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_united_as_ptr_QRect(const QRegion* this_ptr, const QRect* r);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_united_as_ptr_QRegion(const QRegion* this_ptr, const QRegion* r);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QRegion_xored_as_ptr(const QRegion* this_ptr, const QRegion* r);

} // extern "C"

#endif // QT_GUI_C_QREGION_H
