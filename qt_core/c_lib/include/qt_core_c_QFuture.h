#ifndef QT_CORE_C_QFUTURE_H
#define QT_CORE_C_QFUTURE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QFuture_void_cancel(QFuture< void >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFuture_void_constructor(QFuture< void >* output);
QT_CORE_C_EXPORT void qt_core_c_QFuture_void_destructor(QFuture< void >* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFuture_void_isCanceled(const QFuture< void >* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFuture_void_isFinished(const QFuture< void >* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFuture_void_isPaused(const QFuture< void >* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFuture_void_isRunning(const QFuture< void >* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFuture_void_isStarted(const QFuture< void >* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFuture_void_operator_eq(const QFuture< void >* this_ptr, const QFuture< void >* other);
QT_CORE_C_EXPORT bool qt_core_c_QFuture_void_operator_neq(const QFuture< void >* this_ptr, const QFuture< void >* other);
QT_CORE_C_EXPORT void qt_core_c_QFuture_void_pause(QFuture< void >* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QFuture_void_progressMaximum(const QFuture< void >* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QFuture_void_progressMinimum(const QFuture< void >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFuture_void_progressText_to_output(const QFuture< void >* this_ptr, QString* output);
QT_CORE_C_EXPORT int qt_core_c_QFuture_void_progressValue(const QFuture< void >* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QFuture_void_resultCount(const QFuture< void >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFuture_void_resume(QFuture< void >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFuture_void_setPaused(QFuture< void >* this_ptr, bool paused);
QT_CORE_C_EXPORT void qt_core_c_QFuture_void_togglePaused(QFuture< void >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFuture_void_waitForFinished(QFuture< void >* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QFUTURE_H
