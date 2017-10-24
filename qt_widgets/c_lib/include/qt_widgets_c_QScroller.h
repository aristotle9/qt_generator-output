#ifndef QT_WIDGETS_C_QSCROLLER_H
#define QT_WIDGETS_C_QSCROLLER_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QScroller_G_static_cast_QObject_ptr(QScroller* ptr);
QT_WIDGETS_C_EXPORT QScroller* qt_widgets_c_QScroller_G_static_cast_QScroller_ptr(QObject* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_activeScrollers_to_output(QList< QScroller* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_ensureVisible_rect_xmargin_ymargin(QScroller* this_ptr, const QRectF* rect, double xmargin, double ymargin);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_ensureVisible_rect_xmargin_ymargin_scrollTime(QScroller* this_ptr, const QRectF* rect, double xmargin, double ymargin, int scrollTime);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_finalPosition_to_output(const QScroller* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QScroller_handleInput_input_position(QScroller* this_ptr, QScroller::Input input, const QPointF* position);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QScroller_handleInput_input_position_timestamp(QScroller* this_ptr, QScroller::Input input, const QPointF* position, qint64 timestamp);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QScroller_hasScroller(QObject* target);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QScroller_metaObject(const QScroller* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_pixelPerMeter_to_output(const QScroller* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QScroller_qt_metacall(QScroller* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QScroller_qt_metacast(QScroller* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_resendPrepareEvent(QScroller* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_scrollTo_pos(QScroller* this_ptr, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_scrollTo_pos_scrollTime(QScroller* this_ptr, const QPointF* pos, int scrollTime);
QT_WIDGETS_C_EXPORT QScrollerProperties* qt_widgets_c_QScroller_scrollerProperties_as_ptr(const QScroller* this_ptr);
QT_WIDGETS_C_EXPORT QScroller* qt_widgets_c_QScroller_scroller_QObject_ptr(QObject* target);
QT_WIDGETS_C_EXPORT const QScroller* qt_widgets_c_QScroller_scroller_const_QObject_ptr(const QObject* target);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_setScrollerProperties(QScroller* this_ptr, const QScrollerProperties* prop);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_setSnapPositionsX_first_interval(QScroller* this_ptr, double first, double interval);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_setSnapPositionsX_positions(QScroller* this_ptr, const QList< double >* positions);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_setSnapPositionsY_first_interval(QScroller* this_ptr, double first, double interval);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_setSnapPositionsY_positions(QScroller* this_ptr, const QList< double >* positions);
QT_WIDGETS_C_EXPORT QScroller::State qt_widgets_c_QScroller_state(const QScroller* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_stop(QScroller* this_ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QScroller_target(const QScroller* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_ungrabGesture(QObject* target);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScroller_velocity_to_output(const QScroller* this_ptr, QPointF* output);

} // extern "C"

#endif // QT_WIDGETS_C_QSCROLLER_H
