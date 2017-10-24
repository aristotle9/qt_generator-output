#ifndef QT_WIDGETS_C_QGRAPHICSITEMGROUP_H
#define QT_WIDGETS_C_QGRAPHICSITEMGROUP_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsItemGroup* qt_widgets_c_QGraphicsItemGroup_G_dynamic_cast_QGraphicsItemGroup_ptr(QGraphicsItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsItemGroup* qt_widgets_c_QGraphicsItemGroup_G_static_cast_QGraphicsItemGroup_ptr(QGraphicsItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsItem* qt_widgets_c_QGraphicsItemGroup_G_static_cast_QGraphicsItem_ptr(QGraphicsItemGroup* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemGroup_addToGroup(QGraphicsItemGroup* this_ptr, QGraphicsItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemGroup_boundingRect_to_output(const QGraphicsItemGroup* this_ptr, QRectF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemGroup_delete(QGraphicsItemGroup* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsItemGroup_isObscuredBy(const QGraphicsItemGroup* this_ptr, const QGraphicsItem* item);
QT_WIDGETS_C_EXPORT QGraphicsItemGroup* qt_widgets_c_QGraphicsItemGroup_new_no_args();
QT_WIDGETS_C_EXPORT QGraphicsItemGroup* qt_widgets_c_QGraphicsItemGroup_new_parent(QGraphicsItem* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemGroup_opaqueArea_to_output(const QGraphicsItemGroup* this_ptr, QPainterPath* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemGroup_paint_painter_option(QGraphicsItemGroup* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemGroup_paint_painter_option_widget(QGraphicsItemGroup* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemGroup_removeFromGroup(QGraphicsItemGroup* this_ptr, QGraphicsItem* item);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsItemGroup_type(const QGraphicsItemGroup* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSITEMGROUP_H
