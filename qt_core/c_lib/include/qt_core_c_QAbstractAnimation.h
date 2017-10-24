#ifndef QT_CORE_C_QABSTRACTANIMATION_H
#define QT_CORE_C_QABSTRACTANIMATION_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QAbstractAnimation* qt_core_c_QAbstractAnimation_G_dynamic_cast_QAbstractAnimation_ptr(QObject* ptr);
QT_CORE_C_EXPORT QAbstractAnimation* qt_core_c_QAbstractAnimation_G_static_cast_QAbstractAnimation_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QAbstractAnimation_G_static_cast_QObject_ptr(QAbstractAnimation* ptr);
QT_CORE_C_EXPORT int qt_core_c_QAbstractAnimation_currentLoop(const QAbstractAnimation* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QAbstractAnimation_currentLoopTime(const QAbstractAnimation* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QAbstractAnimation_currentTime(const QAbstractAnimation* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractAnimation_delete(QAbstractAnimation* this_ptr);
QT_CORE_C_EXPORT QAbstractAnimation::Direction qt_core_c_QAbstractAnimation_direction(const QAbstractAnimation* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QAbstractAnimation_duration(const QAbstractAnimation* this_ptr);
QT_CORE_C_EXPORT QAnimationGroup* qt_core_c_QAbstractAnimation_group(const QAbstractAnimation* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QAbstractAnimation_loopCount(const QAbstractAnimation* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QAbstractAnimation_metaObject(const QAbstractAnimation* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractAnimation_pause(QAbstractAnimation* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractAnimation_resume(QAbstractAnimation* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractAnimation_setCurrentTime(QAbstractAnimation* this_ptr, int msecs);
QT_CORE_C_EXPORT void qt_core_c_QAbstractAnimation_setDirection(QAbstractAnimation* this_ptr, QAbstractAnimation::Direction direction);
QT_CORE_C_EXPORT void qt_core_c_QAbstractAnimation_setLoopCount(QAbstractAnimation* this_ptr, int loopCount);
QT_CORE_C_EXPORT void qt_core_c_QAbstractAnimation_setPaused(QAbstractAnimation* this_ptr, bool arg1);
QT_CORE_C_EXPORT void qt_core_c_QAbstractAnimation_start_no_args(QAbstractAnimation* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractAnimation_start_policy(QAbstractAnimation* this_ptr, const QAbstractAnimation::DeletionPolicy* policy);
QT_CORE_C_EXPORT QAbstractAnimation::State qt_core_c_QAbstractAnimation_state(const QAbstractAnimation* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractAnimation_stop(QAbstractAnimation* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QAbstractAnimation_totalDuration(const QAbstractAnimation* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractAnimation_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractAnimation_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QABSTRACTANIMATION_H
