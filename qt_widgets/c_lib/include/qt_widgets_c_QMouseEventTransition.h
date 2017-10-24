#ifndef QT_WIDGETS_C_QMOUSEEVENTTRANSITION_H
#define QT_WIDGETS_C_QMOUSEEVENTTRANSITION_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QAbstractTransition* qt_widgets_c_QMouseEventTransition_G_static_cast_QAbstractTransition_ptr(QMouseEventTransition* ptr);
QT_WIDGETS_C_EXPORT QEventTransition* qt_widgets_c_QMouseEventTransition_G_static_cast_QEventTransition_ptr(QMouseEventTransition* ptr);
QT_WIDGETS_C_EXPORT QMouseEventTransition* qt_widgets_c_QMouseEventTransition_G_static_cast_QMouseEventTransition_ptr_QAbstractTransition(QAbstractTransition* ptr);
QT_WIDGETS_C_EXPORT QMouseEventTransition* qt_widgets_c_QMouseEventTransition_G_static_cast_QMouseEventTransition_ptr_QEventTransition(QEventTransition* ptr);
QT_WIDGETS_C_EXPORT QMouseEventTransition* qt_widgets_c_QMouseEventTransition_G_static_cast_QMouseEventTransition_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QMouseEventTransition_G_static_cast_QObject_ptr(QMouseEventTransition* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMouseEventTransition_delete(QMouseEventTransition* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMouseEventTransition_hitTestPath_to_output(const QMouseEventTransition* this_ptr, QPainterPath* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QMouseEventTransition_metaObject(const QMouseEventTransition* this_ptr);
QT_WIDGETS_C_EXPORT QMouseEventTransition* qt_widgets_c_QMouseEventTransition_new_no_args();
QT_WIDGETS_C_EXPORT QMouseEventTransition* qt_widgets_c_QMouseEventTransition_new_object_type_button(QObject* object, const QEvent::Type* type, const Qt::MouseButton* button);
QT_WIDGETS_C_EXPORT QMouseEventTransition* qt_widgets_c_QMouseEventTransition_new_object_type_button_sourceState(QObject* object, const QEvent::Type* type, const Qt::MouseButton* button, QState* sourceState);
QT_WIDGETS_C_EXPORT QMouseEventTransition* qt_widgets_c_QMouseEventTransition_new_sourceState(QState* sourceState);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMouseEventTransition_qt_metacall(QMouseEventTransition* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QMouseEventTransition_qt_metacast(QMouseEventTransition* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMouseEventTransition_setButton(QMouseEventTransition* this_ptr, const Qt::MouseButton* button);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMouseEventTransition_setHitTestPath(QMouseEventTransition* this_ptr, const QPainterPath* path);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMouseEventTransition_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMouseEventTransition_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QMOUSEEVENTTRANSITION_H
