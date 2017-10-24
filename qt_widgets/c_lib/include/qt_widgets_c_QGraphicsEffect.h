#ifndef QT_WIDGETS_C_QGRAPHICSEFFECT_H
#define QT_WIDGETS_C_QGRAPHICSEFFECT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsEffect* qt_widgets_c_QGraphicsEffect_G_static_cast_QGraphicsEffect_ptr(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QGraphicsEffect_G_static_cast_QObject_ptr(QGraphicsEffect* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEffect_boundingRectFor_to_output(const QGraphicsEffect* this_ptr, const QRectF* sourceRect, QRectF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEffect_boundingRect_to_output(const QGraphicsEffect* this_ptr, QRectF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEffect_delete(QGraphicsEffect* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsEffect_isEnabled(const QGraphicsEffect* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QGraphicsEffect_metaObject(const QGraphicsEffect* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsEffect_qt_metacall(QGraphicsEffect* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QGraphicsEffect_qt_metacast(QGraphicsEffect* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEffect_setEnabled(QGraphicsEffect* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEffect_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEffect_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsEffect_update(QGraphicsEffect* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSEFFECT_H
