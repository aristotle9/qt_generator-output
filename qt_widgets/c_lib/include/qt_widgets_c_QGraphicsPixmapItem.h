#ifndef QT_WIDGETS_C_QGRAPHICSPIXMAPITEM_H
#define QT_WIDGETS_C_QGRAPHICSPIXMAPITEM_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsPixmapItem* qt_widgets_c_QGraphicsPixmapItem_G_dynamic_cast_QGraphicsPixmapItem_ptr(QGraphicsItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsItem* qt_widgets_c_QGraphicsPixmapItem_G_static_cast_QGraphicsItem_ptr(QGraphicsPixmapItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsPixmapItem* qt_widgets_c_QGraphicsPixmapItem_G_static_cast_QGraphicsPixmapItem_ptr(QGraphicsItem* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsPixmapItem_boundingRect_to_output(const QGraphicsPixmapItem* this_ptr, QRectF* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsPixmapItem_contains(const QGraphicsPixmapItem* this_ptr, const QPointF* point);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsPixmapItem_delete(QGraphicsPixmapItem* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsPixmapItem_isObscuredBy(const QGraphicsPixmapItem* this_ptr, const QGraphicsItem* item);
QT_WIDGETS_C_EXPORT QGraphicsPixmapItem* qt_widgets_c_QGraphicsPixmapItem_new_no_args();
QT_WIDGETS_C_EXPORT QGraphicsPixmapItem* qt_widgets_c_QGraphicsPixmapItem_new_parent(QGraphicsItem* parent);
QT_WIDGETS_C_EXPORT QGraphicsPixmapItem* qt_widgets_c_QGraphicsPixmapItem_new_pixmap(const QPixmap* pixmap);
QT_WIDGETS_C_EXPORT QGraphicsPixmapItem* qt_widgets_c_QGraphicsPixmapItem_new_pixmap_parent(const QPixmap* pixmap, QGraphicsItem* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsPixmapItem_offset_to_output(const QGraphicsPixmapItem* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsPixmapItem_opaqueArea_to_output(const QGraphicsPixmapItem* this_ptr, QPainterPath* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsPixmapItem_paint(QGraphicsPixmapItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QGraphicsPixmapItem_pixmap_as_ptr(const QGraphicsPixmapItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsPixmapItem_setOffset_offset(QGraphicsPixmapItem* this_ptr, const QPointF* offset);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsPixmapItem_setOffset_x_y(QGraphicsPixmapItem* this_ptr, double x, double y);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsPixmapItem_setPixmap(QGraphicsPixmapItem* this_ptr, const QPixmap* pixmap);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsPixmapItem_setShapeMode(QGraphicsPixmapItem* this_ptr, QGraphicsPixmapItem::ShapeMode mode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsPixmapItem_setTransformationMode(QGraphicsPixmapItem* this_ptr, const Qt::TransformationMode* mode);
QT_WIDGETS_C_EXPORT QGraphicsPixmapItem::ShapeMode qt_widgets_c_QGraphicsPixmapItem_shapeMode(const QGraphicsPixmapItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsPixmapItem_shape_to_output(const QGraphicsPixmapItem* this_ptr, QPainterPath* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsPixmapItem_type(const QGraphicsPixmapItem* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSPIXMAPITEM_H