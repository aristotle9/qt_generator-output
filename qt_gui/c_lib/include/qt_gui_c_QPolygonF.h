#ifndef QT_GUI_C_QPOLYGONF_H
#define QT_GUI_C_QPOLYGONF_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QPolygonF* qt_gui_c_QPolygonF_G_static_cast_QPolygonF_ptr(QVector< QPointF >* ptr);
QT_GUI_C_EXPORT QVector< QPointF >* qt_gui_c_QPolygonF_G_static_cast_QVector_QPointF_ptr(QPolygonF* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_boundingRect_to_output(const QPolygonF* this_ptr, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_constructor_QPolygon(const QPolygon* a, QPolygonF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_constructor_QPolygonF(const QPolygonF* a, QPolygonF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_constructor_QRectF(const QRectF* r, QPolygonF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_constructor_QVector_QPointF(const QVector< QPointF >* v, QPolygonF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_constructor_int(int size, QPolygonF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_constructor_no_args(QPolygonF* output);
QT_GUI_C_EXPORT bool qt_gui_c_QPolygonF_containsPoint(const QPolygonF* this_ptr, const QPointF* pt, const Qt::FillRule* fillRule);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_convert_to_QVariant_to_output(const QPolygonF* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_destructor(QPolygonF* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_intersected_to_output(const QPolygonF* this_ptr, const QPolygonF* r, QPolygonF* output);
QT_GUI_C_EXPORT bool qt_gui_c_QPolygonF_isClosed(const QPolygonF* this_ptr);
QT_GUI_C_EXPORT QPolygonF* qt_gui_c_QPolygonF_operator_assign(QPolygonF* this_ptr, const QPolygonF* other);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_subtracted_to_output(const QPolygonF* this_ptr, const QPolygonF* r, QPolygonF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_swap(QPolygonF* this_ptr, QPolygonF* other);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_toPolygon_to_output(const QPolygonF* this_ptr, QPolygon* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_translate_dx_dy(QPolygonF* this_ptr, double dx, double dy);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_translate_offset(QPolygonF* this_ptr, const QPointF* offset);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_translated_to_output_dx_dy(const QPolygonF* this_ptr, double dx, double dy, QPolygonF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_translated_to_output_offset(const QPolygonF* this_ptr, const QPointF* offset, QPolygonF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygonF_united_to_output(const QPolygonF* this_ptr, const QPolygonF* r, QPolygonF* output);

} // extern "C"

#endif // QT_GUI_C_QPOLYGONF_H
