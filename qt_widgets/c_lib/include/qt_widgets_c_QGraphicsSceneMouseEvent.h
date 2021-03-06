#ifndef QT_WIDGETS_C_QGRAPHICSSCENEMOUSEEVENT_H
#define QT_WIDGETS_C_QGRAPHICSSCENEMOUSEEVENT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsSceneMouseEvent* qt_widgets_c_QGraphicsSceneMouseEvent_G_dynamic_cast_QGraphicsSceneMouseEvent_ptr(QGraphicsSceneEvent* ptr);
QT_WIDGETS_C_EXPORT QEvent* qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QEvent_ptr(QGraphicsSceneMouseEvent* ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneEvent* qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QGraphicsSceneEvent_ptr(QGraphicsSceneMouseEvent* ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneMouseEvent* qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QGraphicsSceneMouseEvent_ptr_QEvent(QEvent* ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneMouseEvent* qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QGraphicsSceneMouseEvent_ptr_QGraphicsSceneEvent(QGraphicsSceneEvent* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_buttonDownPos_to_output(const QGraphicsSceneMouseEvent* this_ptr, const Qt::MouseButton* button, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_buttonDownScenePos_to_output(const QGraphicsSceneMouseEvent* this_ptr, const Qt::MouseButton* button, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_buttonDownScreenPos_to_output(const QGraphicsSceneMouseEvent* this_ptr, const Qt::MouseButton* button, QPoint* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_delete(QGraphicsSceneMouseEvent* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_lastPos_to_output(const QGraphicsSceneMouseEvent* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_lastScenePos_to_output(const QGraphicsSceneMouseEvent* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_lastScreenPos_to_output(const QGraphicsSceneMouseEvent* this_ptr, QPoint* output);
QT_WIDGETS_C_EXPORT QGraphicsSceneMouseEvent* qt_widgets_c_QGraphicsSceneMouseEvent_new_no_args();
QT_WIDGETS_C_EXPORT QGraphicsSceneMouseEvent* qt_widgets_c_QGraphicsSceneMouseEvent_new_type(QEvent::Type type);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_pos_to_output(const QGraphicsSceneMouseEvent* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_scenePos_to_output(const QGraphicsSceneMouseEvent* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_screenPos_to_output(const QGraphicsSceneMouseEvent* this_ptr, QPoint* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_setButton(QGraphicsSceneMouseEvent* this_ptr, const Qt::MouseButton* button);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_setButtonDownPos(QGraphicsSceneMouseEvent* this_ptr, const Qt::MouseButton* button, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_setButtonDownScenePos(QGraphicsSceneMouseEvent* this_ptr, const Qt::MouseButton* button, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_setButtonDownScreenPos(QGraphicsSceneMouseEvent* this_ptr, const Qt::MouseButton* button, const QPoint* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_setLastPos(QGraphicsSceneMouseEvent* this_ptr, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_setLastScenePos(QGraphicsSceneMouseEvent* this_ptr, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_setLastScreenPos(QGraphicsSceneMouseEvent* this_ptr, const QPoint* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_setPos(QGraphicsSceneMouseEvent* this_ptr, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_setScenePos(QGraphicsSceneMouseEvent* this_ptr, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_setScreenPos(QGraphicsSceneMouseEvent* this_ptr, const QPoint* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMouseEvent_setSource(QGraphicsSceneMouseEvent* this_ptr, const Qt::MouseEventSource* source);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSSCENEMOUSEEVENT_H
