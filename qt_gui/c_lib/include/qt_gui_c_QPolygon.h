#ifndef QT_GUI_C_QPOLYGON_H
#define QT_GUI_C_QPOLYGON_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QDataStream* qt_gui_c_QPolygon_G_operator_shl_stream_array(QDataStream* stream, const QPolygonF* array);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QPolygon_G_operator_shl_stream_polygon(QDataStream* stream, const QPolygon* polygon);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_G_operator_shl_to_output_QDebug_QPolygon(const QDebug* arg1, const QPolygon* arg2, QDebug* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_G_operator_shl_to_output_QDebug_QPolygonF(const QDebug* arg1, const QPolygonF* arg2, QDebug* output);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QPolygon_G_operator_shr_stream_array(QDataStream* stream, QPolygonF* array);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QPolygon_G_operator_shr_stream_polygon(QDataStream* stream, QPolygon* polygon);
QT_GUI_C_EXPORT QPolygon* qt_gui_c_QPolygon_G_static_cast_QPolygon_ptr(QVector< QPoint >* ptr);
QT_GUI_C_EXPORT QVector< QPoint >* qt_gui_c_QPolygon_G_static_cast_QVector_QPoint_ptr(QPolygon* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_G_swap_QPolygonF_QPolygonF(QPolygonF* value1, QPolygonF* value2);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_G_swap_QPolygon_QPolygon(QPolygon* value1, QPolygon* value2);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_boundingRect_to_output(const QPolygon* this_ptr, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_constructor_nPoints_points(int nPoints, const int* points, QPolygon* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_constructor_no_args(QPolygon* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_constructor_other(const QPolygon* other, QPolygon* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_constructor_r(const QRect* r, QPolygon* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_constructor_r_closed(const QRect* r, bool closed, QPolygon* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_constructor_size(int size, QPolygon* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_constructor_v(const QVector< QPoint >* v, QPolygon* output);
QT_GUI_C_EXPORT bool qt_gui_c_QPolygon_containsPoint(const QPolygon* this_ptr, const QPoint* pt, const Qt::FillRule* fillRule);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_convert_to_QVariant_to_output(const QPolygon* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_destructor(QPolygon* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_intersected_to_output(const QPolygon* this_ptr, const QPolygon* r, QPolygon* output);
QT_GUI_C_EXPORT QPolygon* qt_gui_c_QPolygon_operator_assign(QPolygon* this_ptr, const QPolygon* other);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_point(const QPolygon* this_ptr, int i, int* x, int* y);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_point_to_output(const QPolygon* this_ptr, int i, QPoint* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_putPoints_index_nPoints_from(QPolygon* this_ptr, int index, int nPoints, const QPolygon* from);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_putPoints_index_nPoints_from_fromIndex(QPolygon* this_ptr, int index, int nPoints, const QPolygon* from, int fromIndex);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_putPoints_index_nPoints_points(QPolygon* this_ptr, int index, int nPoints, const int* points);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_setPoint_index_p(QPolygon* this_ptr, int index, const QPoint* p);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_setPoint_index_x_y(QPolygon* this_ptr, int index, int x, int y);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_setPoints(QPolygon* this_ptr, int nPoints, const int* points);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_subtracted_to_output(const QPolygon* this_ptr, const QPolygon* r, QPolygon* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_swap(QPolygon* this_ptr, QPolygon* other);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_translate_dx_dy(QPolygon* this_ptr, int dx, int dy);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_translate_offset(QPolygon* this_ptr, const QPoint* offset);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_translated_to_output_dx_dy(const QPolygon* this_ptr, int dx, int dy, QPolygon* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_translated_to_output_offset(const QPolygon* this_ptr, const QPoint* offset, QPolygon* output);
QT_GUI_C_EXPORT void qt_gui_c_QPolygon_united_to_output(const QPolygon* this_ptr, const QPolygon* r, QPolygon* output);

} // extern "C"

#endif // QT_GUI_C_QPOLYGON_H
