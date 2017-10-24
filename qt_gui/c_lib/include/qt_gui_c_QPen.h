#ifndef QT_GUI_C_QPEN_H
#define QT_GUI_C_QPEN_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QDataStream* qt_gui_c_QPen_G_operator_shl(QDataStream* arg1, const QPen* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QPen_G_operator_shl_to_output(const QDebug* arg1, const QPen* arg2, QDebug* output);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QPen_G_operator_shr(QDataStream* arg1, QPen* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QPen_G_swap(QPen* value1, QPen* value2);
QT_GUI_C_EXPORT void qt_gui_c_QPen_brush_to_output(const QPen* this_ptr, QBrush* output);
QT_GUI_C_EXPORT void qt_gui_c_QPen_color_to_output(const QPen* this_ptr, QColor* output);
QT_GUI_C_EXPORT void qt_gui_c_QPen_constructor_arg1(const Qt::PenStyle* arg1, QPen* output);
QT_GUI_C_EXPORT void qt_gui_c_QPen_constructor_brush_width(const QBrush* brush, double width, QPen* output);
QT_GUI_C_EXPORT void qt_gui_c_QPen_constructor_brush_width_s(const QBrush* brush, double width, const Qt::PenStyle* s, QPen* output);
QT_GUI_C_EXPORT void qt_gui_c_QPen_constructor_brush_width_s_c(const QBrush* brush, double width, const Qt::PenStyle* s, const Qt::PenCapStyle* c, QPen* output);
QT_GUI_C_EXPORT void qt_gui_c_QPen_constructor_brush_width_s_c_j(const QBrush* brush, double width, const Qt::PenStyle* s, const Qt::PenCapStyle* c, const Qt::PenJoinStyle* j, QPen* output);
QT_GUI_C_EXPORT void qt_gui_c_QPen_constructor_color(const QColor* color, QPen* output);
QT_GUI_C_EXPORT void qt_gui_c_QPen_constructor_no_args(QPen* output);
QT_GUI_C_EXPORT void qt_gui_c_QPen_constructor_pen(const QPen* pen, QPen* output);
QT_GUI_C_EXPORT void qt_gui_c_QPen_convert_to_QVariant_to_output(const QPen* this_ptr, QVariant* output);
QT_GUI_C_EXPORT double qt_gui_c_QPen_dashOffset(const QPen* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPen_dashPattern_to_output(const QPen* this_ptr, QVector< double >* output);
QT_GUI_C_EXPORT void qt_gui_c_QPen_destructor(QPen* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPen_isCosmetic(const QPen* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPen_isDetached(QPen* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPen_isSolid(const QPen* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPen_miterLimit(const QPen* this_ptr);
QT_GUI_C_EXPORT QPen* qt_gui_c_QPen_operator_assign(QPen* this_ptr, const QPen* pen);
QT_GUI_C_EXPORT bool qt_gui_c_QPen_operator_eq(const QPen* this_ptr, const QPen* p);
QT_GUI_C_EXPORT bool qt_gui_c_QPen_operator_neq(const QPen* this_ptr, const QPen* p);
QT_GUI_C_EXPORT void qt_gui_c_QPen_setBrush(QPen* this_ptr, const QBrush* brush);
QT_GUI_C_EXPORT void qt_gui_c_QPen_setCapStyle(QPen* this_ptr, const Qt::PenCapStyle* pcs);
QT_GUI_C_EXPORT void qt_gui_c_QPen_setColor(QPen* this_ptr, const QColor* color);
QT_GUI_C_EXPORT void qt_gui_c_QPen_setCosmetic(QPen* this_ptr, bool cosmetic);
QT_GUI_C_EXPORT void qt_gui_c_QPen_setDashOffset(QPen* this_ptr, double doffset);
QT_GUI_C_EXPORT void qt_gui_c_QPen_setDashPattern(QPen* this_ptr, const QVector< double >* pattern);
QT_GUI_C_EXPORT void qt_gui_c_QPen_setJoinStyle(QPen* this_ptr, const Qt::PenJoinStyle* pcs);
QT_GUI_C_EXPORT void qt_gui_c_QPen_setMiterLimit(QPen* this_ptr, double limit);
QT_GUI_C_EXPORT void qt_gui_c_QPen_setStyle(QPen* this_ptr, const Qt::PenStyle* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPen_setWidth(QPen* this_ptr, int width);
QT_GUI_C_EXPORT void qt_gui_c_QPen_setWidthF(QPen* this_ptr, double width);
QT_GUI_C_EXPORT void qt_gui_c_QPen_swap(QPen* this_ptr, QPen* other);
QT_GUI_C_EXPORT int qt_gui_c_QPen_width(const QPen* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPen_widthF(const QPen* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QPEN_H
