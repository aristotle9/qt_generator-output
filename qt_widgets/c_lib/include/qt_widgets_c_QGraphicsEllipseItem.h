#ifndef QT_WIDGETS_C_QGRAPHICSELLIPSEITEM_H
#define QT_WIDGETS_C_QGRAPHICSELLIPSEITEM_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_G_dynamic_cast_QGraphicsEllipseItem_ptr_QAbstractGraphicsShapeItem(QAbstractGraphicsShapeItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_G_dynamic_cast_QGraphicsEllipseItem_ptr_QGraphicsItem(QGraphicsItem* ptr);
QT_WIDGETS_C_EXPORT QAbstractGraphicsShapeItem* qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(QGraphicsEllipseItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QGraphicsEllipseItem_ptr_QAbstractGraphicsShapeItem(QAbstractGraphicsShapeItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QGraphicsEllipseItem_ptr_QGraphicsItem(QGraphicsItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsItem* qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QGraphicsItem_ptr(QGraphicsEllipseItem* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEllipseItem_boundingRect_to_output(const QGraphicsEllipseItem* this_ptr, QRectF* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsEllipseItem_contains(const QGraphicsEllipseItem* this_ptr, const QPointF* point);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEllipseItem_delete(QGraphicsEllipseItem* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsEllipseItem_isObscuredBy(const QGraphicsEllipseItem* this_ptr, const QGraphicsItem* item);
QT_WIDGETS_C_EXPORT QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_new_no_args();
QT_WIDGETS_C_EXPORT QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_new_parent(QGraphicsItem* parent);
QT_WIDGETS_C_EXPORT QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_new_rect(const QRectF* rect);
QT_WIDGETS_C_EXPORT QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_new_rect_parent(const QRectF* rect, QGraphicsItem* parent);
QT_WIDGETS_C_EXPORT QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_new_x_y_w_h(double x, double y, double w, double h);
QT_WIDGETS_C_EXPORT QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_new_x_y_w_h_parent(double x, double y, double w, double h, QGraphicsItem* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEllipseItem_opaqueArea_to_output(const QGraphicsEllipseItem* this_ptr, QPainterPath* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEllipseItem_paint_painter_option(QGraphicsEllipseItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEllipseItem_paint_painter_option_widget(QGraphicsEllipseItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEllipseItem_rect_to_output(const QGraphicsEllipseItem* this_ptr, QRectF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEllipseItem_setRect_rect(QGraphicsEllipseItem* this_ptr, const QRectF* rect);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEllipseItem_setRect_x_y_w_h(QGraphicsEllipseItem* this_ptr, double x, double y, double w, double h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEllipseItem_setSpanAngle(QGraphicsEllipseItem* this_ptr, int angle);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEllipseItem_setStartAngle(QGraphicsEllipseItem* this_ptr, int angle);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEllipseItem_shape_to_output(const QGraphicsEllipseItem* this_ptr, QPainterPath* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsEllipseItem_spanAngle(const QGraphicsEllipseItem* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsEllipseItem_startAngle(const QGraphicsEllipseItem* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsEllipseItem_type(const QGraphicsEllipseItem* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSELLIPSEITEM_H
