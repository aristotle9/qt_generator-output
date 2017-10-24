#ifndef QT_WIDGETS_C_QGRAPHICSLINEITEM_H
#define QT_WIDGETS_C_QGRAPHICSLINEITEM_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsLineItem* qt_widgets_c_QGraphicsLineItem_G_dynamic_cast_QGraphicsLineItem_ptr(QGraphicsItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsItem* qt_widgets_c_QGraphicsLineItem_G_static_cast_QGraphicsItem_ptr(QGraphicsLineItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsLineItem* qt_widgets_c_QGraphicsLineItem_G_static_cast_QGraphicsLineItem_ptr(QGraphicsItem* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLineItem_boundingRect_to_output(const QGraphicsLineItem* this_ptr, QRectF* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsLineItem_contains(const QGraphicsLineItem* this_ptr, const QPointF* point);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLineItem_delete(QGraphicsLineItem* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsLineItem_isObscuredBy(const QGraphicsLineItem* this_ptr, const QGraphicsItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLineItem_line_to_output(const QGraphicsLineItem* this_ptr, QLineF* output);
QT_WIDGETS_C_EXPORT QGraphicsLineItem* qt_widgets_c_QGraphicsLineItem_new_line(const QLineF* line);
QT_WIDGETS_C_EXPORT QGraphicsLineItem* qt_widgets_c_QGraphicsLineItem_new_line_parent(const QLineF* line, QGraphicsItem* parent);
QT_WIDGETS_C_EXPORT QGraphicsLineItem* qt_widgets_c_QGraphicsLineItem_new_no_args();
QT_WIDGETS_C_EXPORT QGraphicsLineItem* qt_widgets_c_QGraphicsLineItem_new_parent(QGraphicsItem* parent);
QT_WIDGETS_C_EXPORT QGraphicsLineItem* qt_widgets_c_QGraphicsLineItem_new_x1_y1_x2_y2(double x1, double y1, double x2, double y2);
QT_WIDGETS_C_EXPORT QGraphicsLineItem* qt_widgets_c_QGraphicsLineItem_new_x1_y1_x2_y2_parent(double x1, double y1, double x2, double y2, QGraphicsItem* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLineItem_opaqueArea_to_output(const QGraphicsLineItem* this_ptr, QPainterPath* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLineItem_paint_painter_option(QGraphicsLineItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLineItem_paint_painter_option_widget(QGraphicsLineItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLineItem_pen_to_output(const QGraphicsLineItem* this_ptr, QPen* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLineItem_setLine_line(QGraphicsLineItem* this_ptr, const QLineF* line);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLineItem_setLine_x1_y1_x2_y2(QGraphicsLineItem* this_ptr, double x1, double y1, double x2, double y2);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLineItem_setPen(QGraphicsLineItem* this_ptr, const QPen* pen);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLineItem_shape_to_output(const QGraphicsLineItem* this_ptr, QPainterPath* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsLineItem_type(const QGraphicsLineItem* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSLINEITEM_H
