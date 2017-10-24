#ifndef QT_WIDGETS_C_QGRAPHICSOPACITYEFFECT_H
#define QT_WIDGETS_C_QGRAPHICSOPACITYEFFECT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsOpacityEffect* qt_widgets_c_QGraphicsOpacityEffect_G_dynamic_cast_QGraphicsOpacityEffect_ptr(QGraphicsEffect* ptr);
QT_WIDGETS_C_EXPORT QGraphicsEffect* qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QGraphicsEffect_ptr(QGraphicsOpacityEffect* ptr);
QT_WIDGETS_C_EXPORT QGraphicsOpacityEffect* qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QGraphicsOpacityEffect_ptr_QGraphicsEffect(QGraphicsEffect* ptr);
QT_WIDGETS_C_EXPORT QGraphicsOpacityEffect* qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QGraphicsOpacityEffect_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QObject_ptr(QGraphicsOpacityEffect* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsOpacityEffect_delete(QGraphicsOpacityEffect* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QGraphicsOpacityEffect_metaObject(const QGraphicsOpacityEffect* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsOpacityEffect* qt_widgets_c_QGraphicsOpacityEffect_new_no_args();
QT_WIDGETS_C_EXPORT QGraphicsOpacityEffect* qt_widgets_c_QGraphicsOpacityEffect_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsOpacityEffect_opacity(const QGraphicsOpacityEffect* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsOpacityEffect_opacityMask_to_output(const QGraphicsOpacityEffect* this_ptr, QBrush* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsOpacityEffect_qt_metacall(QGraphicsOpacityEffect* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QGraphicsOpacityEffect_qt_metacast(QGraphicsOpacityEffect* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsOpacityEffect_setOpacity(QGraphicsOpacityEffect* this_ptr, double opacity);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsOpacityEffect_setOpacityMask(QGraphicsOpacityEffect* this_ptr, const QBrush* mask);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsOpacityEffect_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsOpacityEffect_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSOPACITYEFFECT_H
