#ifndef QT_WIDGETS_C_QKEYEVENTTRANSITION_H
#define QT_WIDGETS_C_QKEYEVENTTRANSITION_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QAbstractTransition* qt_widgets_c_QKeyEventTransition_G_static_cast_QAbstractTransition_ptr(QKeyEventTransition* ptr);
QT_WIDGETS_C_EXPORT QEventTransition* qt_widgets_c_QKeyEventTransition_G_static_cast_QEventTransition_ptr(QKeyEventTransition* ptr);
QT_WIDGETS_C_EXPORT QKeyEventTransition* qt_widgets_c_QKeyEventTransition_G_static_cast_QKeyEventTransition_ptr_QAbstractTransition(QAbstractTransition* ptr);
QT_WIDGETS_C_EXPORT QKeyEventTransition* qt_widgets_c_QKeyEventTransition_G_static_cast_QKeyEventTransition_ptr_QEventTransition(QEventTransition* ptr);
QT_WIDGETS_C_EXPORT QKeyEventTransition* qt_widgets_c_QKeyEventTransition_G_static_cast_QKeyEventTransition_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QKeyEventTransition_G_static_cast_QObject_ptr(QKeyEventTransition* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QKeyEventTransition_delete(QKeyEventTransition* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QKeyEventTransition_key(const QKeyEventTransition* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QKeyEventTransition_metaObject(const QKeyEventTransition* this_ptr);
QT_WIDGETS_C_EXPORT QKeyEventTransition* qt_widgets_c_QKeyEventTransition_new_no_args();
QT_WIDGETS_C_EXPORT QKeyEventTransition* qt_widgets_c_QKeyEventTransition_new_object_type_key(QObject* object, const QEvent::Type* type, int key);
QT_WIDGETS_C_EXPORT QKeyEventTransition* qt_widgets_c_QKeyEventTransition_new_object_type_key_sourceState(QObject* object, const QEvent::Type* type, int key, QState* sourceState);
QT_WIDGETS_C_EXPORT QKeyEventTransition* qt_widgets_c_QKeyEventTransition_new_sourceState(QState* sourceState);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QKeyEventTransition_qt_metacall(QKeyEventTransition* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QKeyEventTransition_qt_metacast(QKeyEventTransition* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QKeyEventTransition_setKey(QKeyEventTransition* this_ptr, int key);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QKeyEventTransition_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QKeyEventTransition_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QKEYEVENTTRANSITION_H
