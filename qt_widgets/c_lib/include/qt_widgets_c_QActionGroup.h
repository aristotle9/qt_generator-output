#ifndef QT_WIDGETS_C_QACTIONGROUP_H
#define QT_WIDGETS_C_QACTIONGROUP_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QActionGroup* qt_widgets_c_QActionGroup_G_static_cast_QActionGroup_ptr(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QActionGroup_G_static_cast_QObject_ptr(QActionGroup* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QActionGroup_actions_to_output(const QActionGroup* this_ptr, QList< QAction* >* output);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QActionGroup_addAction_a(QActionGroup* this_ptr, QAction* a);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QActionGroup_addAction_icon_text(QActionGroup* this_ptr, const QIcon* icon, const QString* text);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QActionGroup_addAction_text(QActionGroup* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QActionGroup_checkedAction(const QActionGroup* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QActionGroup_delete(QActionGroup* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QActionGroup_isEnabled(const QActionGroup* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QActionGroup_isExclusive(const QActionGroup* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QActionGroup_isVisible(const QActionGroup* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QActionGroup_metaObject(const QActionGroup* this_ptr);
QT_WIDGETS_C_EXPORT QActionGroup* qt_widgets_c_QActionGroup_new(QObject* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QActionGroup_qt_metacall(QActionGroup* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QActionGroup_qt_metacast(QActionGroup* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QActionGroup_removeAction(QActionGroup* this_ptr, QAction* a);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QActionGroup_setDisabled(QActionGroup* this_ptr, bool b);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QActionGroup_setEnabled(QActionGroup* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QActionGroup_setExclusive(QActionGroup* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QActionGroup_setVisible(QActionGroup* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QActionGroup_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QActionGroup_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QACTIONGROUP_H
