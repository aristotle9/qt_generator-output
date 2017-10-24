#ifndef QT_WIDGETS_C_QGRAPHICSSCENEMOVEEVENT_H
#define QT_WIDGETS_C_QGRAPHICSSCENEMOVEEVENT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsSceneMoveEvent* qt_widgets_c_QGraphicsSceneMoveEvent_G_dynamic_cast_QGraphicsSceneMoveEvent_ptr(QGraphicsSceneEvent* ptr);
QT_WIDGETS_C_EXPORT QEvent* qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QEvent_ptr(QGraphicsSceneMoveEvent* ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneEvent* qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QGraphicsSceneEvent_ptr(QGraphicsSceneMoveEvent* ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneMoveEvent* qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QGraphicsSceneMoveEvent_ptr_QEvent(QEvent* ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneMoveEvent* qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QGraphicsSceneMoveEvent_ptr_QGraphicsSceneEvent(QGraphicsSceneEvent* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMoveEvent_delete(QGraphicsSceneMoveEvent* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneMoveEvent* qt_widgets_c_QGraphicsSceneMoveEvent_new();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMoveEvent_newPos_to_output(const QGraphicsSceneMoveEvent* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMoveEvent_oldPos_to_output(const QGraphicsSceneMoveEvent* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMoveEvent_setNewPos(QGraphicsSceneMoveEvent* this_ptr, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneMoveEvent_setOldPos(QGraphicsSceneMoveEvent* this_ptr, const QPointF* pos);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSSCENEMOVEEVENT_H
