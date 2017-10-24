#ifndef QT_WIDGETS_C_QGRAPHICSRECTITEM_H
#define QT_WIDGETS_C_QGRAPHICSRECTITEM_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_G_dynamic_cast_QGraphicsRectItem_ptr_QAbstractGraphicsShapeItem(QAbstractGraphicsShapeItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_G_dynamic_cast_QGraphicsRectItem_ptr_QGraphicsItem(QGraphicsItem* ptr);
QT_WIDGETS_C_EXPORT QAbstractGraphicsShapeItem* qt_widgets_c_QGraphicsRectItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(QGraphicsRectItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsItem* qt_widgets_c_QGraphicsRectItem_G_static_cast_QGraphicsItem_ptr(QGraphicsRectItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_G_static_cast_QGraphicsRectItem_ptr_QAbstractGraphicsShapeItem(QAbstractGraphicsShapeItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_G_static_cast_QGraphicsRectItem_ptr_QGraphicsItem(QGraphicsItem* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsRectItem_boundingRect_to_output(const QGraphicsRectItem* this_ptr, QRectF* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsRectItem_contains(const QGraphicsRectItem* this_ptr, const QPointF* point);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsRectItem_delete(QGraphicsRectItem* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsRectItem_isObscuredBy(const QGraphicsRectItem* this_ptr, const QGraphicsItem* item);
QT_WIDGETS_C_EXPORT QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_new_no_args();
QT_WIDGETS_C_EXPORT QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_new_parent(QGraphicsItem* parent);
QT_WIDGETS_C_EXPORT QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_new_rect(const QRectF* rect);
QT_WIDGETS_C_EXPORT QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_new_rect_parent(const QRectF* rect, QGraphicsItem* parent);
QT_WIDGETS_C_EXPORT QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_new_x_y_w_h(double x, double y, double w, double h);
QT_WIDGETS_C_EXPORT QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_new_x_y_w_h_parent(double x, double y, double w, double h, QGraphicsItem* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsRectItem_opaqueArea_to_output(const QGraphicsRectItem* this_ptr, QPainterPath* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsRectItem_paint_painter_option(QGraphicsRectItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsRectItem_paint_painter_option_widget(QGraphicsRectItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsRectItem_rect_to_output(const QGraphicsRectItem* this_ptr, QRectF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsRectItem_setRect_rect(QGraphicsRectItem* this_ptr, const QRectF* rect);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsRectItem_setRect_x_y_w_h(QGraphicsRectItem* this_ptr, double x, double y, double w, double h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsRectItem_shape_to_output(const QGraphicsRectItem* this_ptr, QPainterPath* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsRectItem_type(const QGraphicsRectItem* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSRECTITEM_H
