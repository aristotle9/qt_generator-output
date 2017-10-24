#ifndef QT_WIDGETS_C_QGESTURERECOGNIZER_H
#define QT_WIDGETS_C_QGESTURERECOGNIZER_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGesture* qt_widgets_c_QGestureRecognizer_create(QGestureRecognizer* this_ptr, QObject* target);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGestureRecognizer_delete(QGestureRecognizer* this_ptr);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QGestureRecognizer_recognize(QGestureRecognizer* this_ptr, QGesture* state, QObject* watched, QEvent* event);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGestureRecognizer_reset(QGestureRecognizer* this_ptr, QGesture* state);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGestureRecognizer_unregisterRecognizer(const Qt::GestureType* type);

} // extern "C"

#endif // QT_WIDGETS_C_QGESTURERECOGNIZER_H
