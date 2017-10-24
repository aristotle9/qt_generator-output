#ifndef QT_WIDGETS_C_QABSTRACTBUTTON_H
#define QT_WIDGETS_C_QABSTRACTBUTTON_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QAbstractButton* qt_widgets_c_QAbstractButton_G_dynamic_cast_QAbstractButton_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractButton* qt_widgets_c_QAbstractButton_G_static_cast_QAbstractButton_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QAbstractButton* qt_widgets_c_QAbstractButton_G_static_cast_QAbstractButton_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QAbstractButton* qt_widgets_c_QAbstractButton_G_static_cast_QAbstractButton_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QAbstractButton_G_static_cast_QObject_ptr(QAbstractButton* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QAbstractButton_G_static_cast_QPaintDevice_ptr(QAbstractButton* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QAbstractButton_G_static_cast_QWidget_ptr(QAbstractButton* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_animateClick_msec(QAbstractButton* this_ptr, int msec);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_animateClick_no_args(QAbstractButton* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractButton_autoExclusive(const QAbstractButton* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractButton_autoRepeat(const QAbstractButton* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QAbstractButton_autoRepeatDelay(const QAbstractButton* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QAbstractButton_autoRepeatInterval(const QAbstractButton* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_click(QAbstractButton* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_delete(QAbstractButton* this_ptr);
QT_WIDGETS_C_EXPORT QButtonGroup* qt_widgets_c_QAbstractButton_group(const QAbstractButton* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_iconSize_to_output(const QAbstractButton* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_icon_to_output(const QAbstractButton* this_ptr, QIcon* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractButton_isCheckable(const QAbstractButton* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractButton_isChecked(const QAbstractButton* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractButton_isDown(const QAbstractButton* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QAbstractButton_metaObject(const QAbstractButton* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QAbstractButton_qt_metacall(QAbstractButton* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QAbstractButton_qt_metacast(QAbstractButton* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_setAutoExclusive(QAbstractButton* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_setAutoRepeat(QAbstractButton* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_setAutoRepeatDelay(QAbstractButton* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_setAutoRepeatInterval(QAbstractButton* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_setCheckable(QAbstractButton* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_setChecked(QAbstractButton* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_setDown(QAbstractButton* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_setIcon(QAbstractButton* this_ptr, const QIcon* icon);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_setIconSize(QAbstractButton* this_ptr, const QSize* size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_setShortcut(QAbstractButton* this_ptr, const QKeySequence* key);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_setText(QAbstractButton* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_shortcut_to_output(const QAbstractButton* this_ptr, QKeySequence* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_text_to_output(const QAbstractButton* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_toggle(QAbstractButton* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractButton_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QABSTRACTBUTTON_H
