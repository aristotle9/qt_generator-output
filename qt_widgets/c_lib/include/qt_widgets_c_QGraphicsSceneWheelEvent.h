#ifndef QT_WIDGETS_C_QGRAPHICSSCENEWHEELEVENT_H
#define QT_WIDGETS_C_QGRAPHICSSCENEWHEELEVENT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsSceneWheelEvent* qt_widgets_c_QGraphicsSceneWheelEvent_G_dynamic_cast_QGraphicsSceneWheelEvent_ptr(QGraphicsSceneEvent* ptr);
QT_WIDGETS_C_EXPORT QEvent* qt_widgets_c_QGraphicsSceneWheelEvent_G_static_cast_QEvent_ptr(QGraphicsSceneWheelEvent* ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneEvent* qt_widgets_c_QGraphicsSceneWheelEvent_G_static_cast_QGraphicsSceneEvent_ptr(QGraphicsSceneWheelEvent* ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneWheelEvent* qt_widgets_c_QGraphicsSceneWheelEvent_G_static_cast_QGraphicsSceneWheelEvent_ptr_QEvent(QEvent* ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneWheelEvent* qt_widgets_c_QGraphicsSceneWheelEvent_G_static_cast_QGraphicsSceneWheelEvent_ptr_QGraphicsSceneEvent(QGraphicsSceneEvent* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneWheelEvent_delete(QGraphicsSceneWheelEvent* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsSceneWheelEvent_delta(const QGraphicsSceneWheelEvent* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsSceneWheelEvent* qt_widgets_c_QGraphicsSceneWheelEvent_new_no_args();
QT_WIDGETS_C_EXPORT QGraphicsSceneWheelEvent* qt_widgets_c_QGraphicsSceneWheelEvent_new_type(QEvent::Type type);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneWheelEvent_pos_to_output(const QGraphicsSceneWheelEvent* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneWheelEvent_scenePos_to_output(const QGraphicsSceneWheelEvent* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneWheelEvent_screenPos_to_output(const QGraphicsSceneWheelEvent* this_ptr, QPoint* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneWheelEvent_setDelta(QGraphicsSceneWheelEvent* this_ptr, int delta);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneWheelEvent_setOrientation(QGraphicsSceneWheelEvent* this_ptr, const Qt::Orientation* orientation);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneWheelEvent_setPos(QGraphicsSceneWheelEvent* this_ptr, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneWheelEvent_setScenePos(QGraphicsSceneWheelEvent* this_ptr, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsSceneWheelEvent_setScreenPos(QGraphicsSceneWheelEvent* this_ptr, const QPoint* pos);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSSCENEWHEELEVENT_H
