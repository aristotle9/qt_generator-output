#ifndef QT_CORE_C_QTIMELINE_H
#define QT_CORE_C_QTIMELINE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QTimeLine* qt_core_c_QTimeLine_G_dynamic_cast_QTimeLine_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QTimeLine_G_static_cast_QObject_ptr(QTimeLine* ptr);
QT_CORE_C_EXPORT QTimeLine* qt_core_c_QTimeLine_G_static_cast_QTimeLine_ptr(QObject* ptr);
QT_CORE_C_EXPORT int qt_core_c_QTimeLine_currentFrame(const QTimeLine* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTimeLine_currentTime(const QTimeLine* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QTimeLine_currentValue(const QTimeLine* this_ptr);
QT_CORE_C_EXPORT QTimeLine::CurveShape qt_core_c_QTimeLine_curveShape(const QTimeLine* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_delete(QTimeLine* this_ptr);
QT_CORE_C_EXPORT QTimeLine::Direction qt_core_c_QTimeLine_direction(const QTimeLine* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTimeLine_duration(const QTimeLine* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_easingCurve_to_output(const QTimeLine* this_ptr, QEasingCurve* output);
QT_CORE_C_EXPORT int qt_core_c_QTimeLine_endFrame(const QTimeLine* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTimeLine_frameForTime(const QTimeLine* this_ptr, int msec);
QT_CORE_C_EXPORT int qt_core_c_QTimeLine_loopCount(const QTimeLine* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QTimeLine_metaObject(const QTimeLine* this_ptr);
QT_CORE_C_EXPORT QTimeLine* qt_core_c_QTimeLine_new_duration(int duration);
QT_CORE_C_EXPORT QTimeLine* qt_core_c_QTimeLine_new_duration_parent(int duration, QObject* parent);
QT_CORE_C_EXPORT QTimeLine* qt_core_c_QTimeLine_new_no_args();
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_resume(QTimeLine* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_setCurrentTime(QTimeLine* this_ptr, int msec);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_setCurveShape(QTimeLine* this_ptr, QTimeLine::CurveShape shape);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_setDirection(QTimeLine* this_ptr, QTimeLine::Direction direction);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_setDuration(QTimeLine* this_ptr, int duration);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_setEasingCurve(QTimeLine* this_ptr, const QEasingCurve* curve);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_setEndFrame(QTimeLine* this_ptr, int frame);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_setFrameRange(QTimeLine* this_ptr, int startFrame, int endFrame);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_setLoopCount(QTimeLine* this_ptr, int count);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_setPaused(QTimeLine* this_ptr, bool paused);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_setStartFrame(QTimeLine* this_ptr, int frame);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_setUpdateInterval(QTimeLine* this_ptr, int interval);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_start(QTimeLine* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTimeLine_startFrame(const QTimeLine* this_ptr);
QT_CORE_C_EXPORT QTimeLine::State qt_core_c_QTimeLine_state(const QTimeLine* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_stop(QTimeLine* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_toggleDirection(QTimeLine* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeLine_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT int qt_core_c_QTimeLine_updateInterval(const QTimeLine* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QTimeLine_valueForTime(const QTimeLine* this_ptr, int msec);

} // extern "C"

#endif // QT_CORE_C_QTIMELINE_H
