#ifndef QT_WIDGETS_C_QTAPANDHOLDGESTURE_H
#define QT_WIDGETS_C_QTAPANDHOLDGESTURE_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QTapAndHoldGesture* qt_widgets_c_QTapAndHoldGesture_G_dynamic_cast_QTapAndHoldGesture_ptr(QGesture* ptr);
QT_WIDGETS_C_EXPORT QGesture* qt_widgets_c_QTapAndHoldGesture_G_static_cast_QGesture_ptr(QTapAndHoldGesture* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QTapAndHoldGesture_G_static_cast_QObject_ptr(QTapAndHoldGesture* ptr);
QT_WIDGETS_C_EXPORT QTapAndHoldGesture* qt_widgets_c_QTapAndHoldGesture_G_static_cast_QTapAndHoldGesture_ptr_QGesture(QGesture* ptr);
QT_WIDGETS_C_EXPORT QTapAndHoldGesture* qt_widgets_c_QTapAndHoldGesture_G_static_cast_QTapAndHoldGesture_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTapAndHoldGesture_delete(QTapAndHoldGesture* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QTapAndHoldGesture_metaObject(const QTapAndHoldGesture* this_ptr);
QT_WIDGETS_C_EXPORT QTapAndHoldGesture* qt_widgets_c_QTapAndHoldGesture_new_no_args();
QT_WIDGETS_C_EXPORT QTapAndHoldGesture* qt_widgets_c_QTapAndHoldGesture_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTapAndHoldGesture_position_to_output(const QTapAndHoldGesture* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTapAndHoldGesture_qt_metacall(QTapAndHoldGesture* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QTapAndHoldGesture_qt_metacast(QTapAndHoldGesture* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTapAndHoldGesture_setPosition(QTapAndHoldGesture* this_ptr, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTapAndHoldGesture_setTimeout(int msecs);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTapAndHoldGesture_timeout();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTapAndHoldGesture_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTapAndHoldGesture_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QTAPANDHOLDGESTURE_H
