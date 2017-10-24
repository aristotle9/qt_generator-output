#ifndef QT_WIDGETS_C_QGRAPHICSSCENEHOVEREVENT_H
#define QT_WIDGETS_C_QGRAPHICSSCENEHOVEREVENT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsSceneHoverEvent* qt_widgets_c_QGraphicsSceneHoverEvent_G_dynamic_cast_QGraphicsSceneHoverEvent_ptr(QGraphicsSceneEvent* ptr);
QT_WIDGETS_C_EXPORT QEvent* qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QEvent_ptr(QGraphicsSceneHoverEvent* ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneEvent* qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QGraphicsSceneEvent_ptr(QGraphicsSceneHoverEvent* ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneHoverEvent* qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QGraphicsSceneHoverEvent_ptr_QEvent(QEvent* ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneHoverEvent* qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QGraphicsSceneHoverEvent_ptr_QGraphicsSceneEvent(QGraphicsSceneEvent* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneHoverEvent_delete(QGraphicsSceneHoverEvent* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneHoverEvent_lastPos_to_output(const QGraphicsSceneHoverEvent* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneHoverEvent_lastScenePos_to_output(const QGraphicsSceneHoverEvent* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneHoverEvent_lastScreenPos_to_output(const QGraphicsSceneHoverEvent* this_ptr, QPoint* output);
QT_WIDGETS_C_EXPORT QGraphicsSceneHoverEvent* qt_widgets_c_QGraphicsSceneHoverEvent_new_no_args();
QT_WIDGETS_C_EXPORT QGraphicsSceneHoverEvent* qt_widgets_c_QGraphicsSceneHoverEvent_new_type(QEvent::Type type);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneHoverEvent_pos_to_output(const QGraphicsSceneHoverEvent* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneHoverEvent_scenePos_to_output(const QGraphicsSceneHoverEvent* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneHoverEvent_screenPos_to_output(const QGraphicsSceneHoverEvent* this_ptr, QPoint* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneHoverEvent_setLastPos(QGraphicsSceneHoverEvent* this_ptr, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneHoverEvent_setLastScenePos(QGraphicsSceneHoverEvent* this_ptr, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneHoverEvent_setLastScreenPos(QGraphicsSceneHoverEvent* this_ptr, const QPoint* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneHoverEvent_setPos(QGraphicsSceneHoverEvent* this_ptr, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneHoverEvent_setScenePos(QGraphicsSceneHoverEvent* this_ptr, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneHoverEvent_setScreenPos(QGraphicsSceneHoverEvent* this_ptr, const QPoint* pos);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSSCENEHOVEREVENT_H
