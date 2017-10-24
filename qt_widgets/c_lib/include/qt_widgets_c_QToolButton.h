#ifndef QT_WIDGETS_C_QTOOLBUTTON_H
#define QT_WIDGETS_C_QTOOLBUTTON_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QToolButton* qt_widgets_c_QToolButton_G_dynamic_cast_QToolButton_ptr_QAbstractButton(QAbstractButton* ptr);
QT_WIDGETS_C_EXPORT QToolButton* qt_widgets_c_QToolButton_G_dynamic_cast_QToolButton_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractButton* qt_widgets_c_QToolButton_G_static_cast_QAbstractButton_ptr(QToolButton* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QToolButton_G_static_cast_QObject_ptr(QToolButton* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QToolButton_G_static_cast_QPaintDevice_ptr(QToolButton* ptr);
QT_WIDGETS_C_EXPORT QToolButton* qt_widgets_c_QToolButton_G_static_cast_QToolButton_ptr_QAbstractButton(QAbstractButton* ptr);
QT_WIDGETS_C_EXPORT QToolButton* qt_widgets_c_QToolButton_G_static_cast_QToolButton_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QToolButton* qt_widgets_c_QToolButton_G_static_cast_QToolButton_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QToolButton* qt_widgets_c_QToolButton_G_static_cast_QToolButton_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QToolButton_G_static_cast_QWidget_ptr(QToolButton* ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QToolButton_autoRaise(const QToolButton* this_ptr);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QToolButton_defaultAction(const QToolButton* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolButton_delete(QToolButton* this_ptr);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QToolButton_menu(const QToolButton* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QToolButton_metaObject(const QToolButton* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolButton_minimumSizeHint_to_output(const QToolButton* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QToolButton* qt_widgets_c_QToolButton_new_no_args();
QT_WIDGETS_C_EXPORT QToolButton* qt_widgets_c_QToolButton_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT QToolButton::ToolButtonPopupMode qt_widgets_c_QToolButton_popupMode(const QToolButton* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QToolButton_qt_metacall(QToolButton* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QToolButton_qt_metacast(QToolButton* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolButton_setArrowType(QToolButton* this_ptr, const Qt::ArrowType* type);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolButton_setAutoRaise(QToolButton* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolButton_setDefaultAction(QToolButton* this_ptr, QAction* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolButton_setMenu(QToolButton* this_ptr, QMenu* menu);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolButton_setPopupMode(QToolButton* this_ptr, QToolButton::ToolButtonPopupMode mode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolButton_setToolButtonStyle(QToolButton* this_ptr, const Qt::ToolButtonStyle* style);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolButton_showMenu(QToolButton* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolButton_sizeHint_to_output(const QToolButton* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolButton_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolButton_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QTOOLBUTTON_H
