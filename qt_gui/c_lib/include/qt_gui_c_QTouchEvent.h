#ifndef QT_GUI_C_QTOUCHEVENT_H
#define QT_GUI_C_QTOUCHEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QTouchEvent* qt_gui_c_QTouchEvent_G_dynamic_cast_QTouchEvent_ptr(QInputEvent* ptr);
QT_GUI_C_EXPORT QEvent* qt_gui_c_QTouchEvent_G_static_cast_QEvent_ptr(QTouchEvent* ptr);
QT_GUI_C_EXPORT QInputEvent* qt_gui_c_QTouchEvent_G_static_cast_QInputEvent_ptr(QTouchEvent* ptr);
QT_GUI_C_EXPORT QTouchEvent* qt_gui_c_QTouchEvent_G_static_cast_QTouchEvent_ptr_QEvent(QEvent* ptr);
QT_GUI_C_EXPORT QTouchEvent* qt_gui_c_QTouchEvent_G_static_cast_QTouchEvent_ptr_QInputEvent(QInputEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_constructor_id(int id, QTouchEvent::TouchPoint* output);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_constructor_no_args(QTouchEvent::TouchPoint* output);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_constructor_other(const QTouchEvent::TouchPoint* other, QTouchEvent::TouchPoint* output);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_destructor(QTouchEvent::TouchPoint* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_ellipseDiameters_to_output(const QTouchEvent::TouchPoint* this_ptr, QSizeF* output);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QTouchEvent_TouchPoint_flags(const QTouchEvent::TouchPoint* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTouchEvent_TouchPoint_id(const QTouchEvent::TouchPoint* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_lastNormalizedPos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_lastPos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_lastScenePos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_lastScreenPos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_normalizedPos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output);
QT_GUI_C_EXPORT QTouchEvent::TouchPoint* qt_gui_c_QTouchEvent_TouchPoint_operator_assign(QTouchEvent::TouchPoint* this_ptr, const QTouchEvent::TouchPoint* other);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_pos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output);
QT_GUI_C_EXPORT double qt_gui_c_QTouchEvent_TouchPoint_pressure(const QTouchEvent::TouchPoint* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_rawScreenPositions_to_output(const QTouchEvent::TouchPoint* this_ptr, QVector< QPointF >* output);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_rect_to_output(const QTouchEvent::TouchPoint* this_ptr, QRectF* output);
QT_GUI_C_EXPORT double qt_gui_c_QTouchEvent_TouchPoint_rotation(const QTouchEvent::TouchPoint* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_scenePos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_sceneRect_to_output(const QTouchEvent::TouchPoint* this_ptr, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_screenPos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_screenRect_to_output(const QTouchEvent::TouchPoint* this_ptr, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setEllipseDiameters(QTouchEvent::TouchPoint* this_ptr, const QSizeF* dia);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setFlags(QTouchEvent::TouchPoint* this_ptr, unsigned int flags);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setId(QTouchEvent::TouchPoint* this_ptr, int id);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setLastNormalizedPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* lastNormalizedPos);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setLastPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* lastPos);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setLastScenePos(QTouchEvent::TouchPoint* this_ptr, const QPointF* lastScenePos);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setLastScreenPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* lastScreenPos);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setNormalizedPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* normalizedPos);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* pos);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setPressure(QTouchEvent::TouchPoint* this_ptr, double pressure);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setRawScreenPositions(QTouchEvent::TouchPoint* this_ptr, const QVector< QPointF >* positions);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setRect(QTouchEvent::TouchPoint* this_ptr, const QRectF* rect);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setRotation(QTouchEvent::TouchPoint* this_ptr, double angle);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setScenePos(QTouchEvent::TouchPoint* this_ptr, const QPointF* scenePos);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setSceneRect(QTouchEvent::TouchPoint* this_ptr, const QRectF* sceneRect);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setScreenPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* screenPos);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setScreenRect(QTouchEvent::TouchPoint* this_ptr, const QRectF* screenRect);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setStartNormalizedPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* startNormalizedPos);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setStartPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* startPos);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setStartScenePos(QTouchEvent::TouchPoint* this_ptr, const QPointF* startScenePos);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setStartScreenPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* startScreenPos);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setUniqueId(QTouchEvent::TouchPoint* this_ptr, qint64 uid);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_setVelocity(QTouchEvent::TouchPoint* this_ptr, const QVector2D* v);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_startNormalizedPos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_startPos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_startScenePos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_startScreenPos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_swap(QTouchEvent::TouchPoint* this_ptr, QTouchEvent::TouchPoint* other);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_TouchPoint_uniqueId_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointingDeviceUniqueId* output);
QT_GUI_C_EXPORT QVector2D* qt_gui_c_QTouchEvent_TouchPoint_velocity_as_ptr(const QTouchEvent::TouchPoint* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_delete(QTouchEvent* this_ptr);
QT_GUI_C_EXPORT QTouchDevice* qt_gui_c_QTouchEvent_device(const QTouchEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_setDevice(QTouchEvent* this_ptr, QTouchDevice* adevice);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_setTarget(QTouchEvent* this_ptr, QObject* atarget);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_setTouchPoints(QTouchEvent* this_ptr, const QList< QTouchEvent::TouchPoint >* atouchPoints);
QT_GUI_C_EXPORT void qt_gui_c_QTouchEvent_setWindow(QTouchEvent* this_ptr, QWindow* awindow);
QT_GUI_C_EXPORT QObject* qt_gui_c_QTouchEvent_target(const QTouchEvent* this_ptr);
QT_GUI_C_EXPORT const QList< QTouchEvent::TouchPoint >* qt_gui_c_QTouchEvent_touchPoints(const QTouchEvent* this_ptr);
QT_GUI_C_EXPORT QWindow* qt_gui_c_QTouchEvent_window(const QTouchEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTOUCHEVENT_H
