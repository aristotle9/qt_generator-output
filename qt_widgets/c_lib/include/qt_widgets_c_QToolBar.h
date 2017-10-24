#ifndef QT_WIDGETS_C_QTOOLBAR_H
#define QT_WIDGETS_C_QTOOLBAR_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QToolBar* qt_widgets_c_QToolBar_G_dynamic_cast_QToolBar_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QToolBar_G_static_cast_QObject_ptr(QToolBar* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QToolBar_G_static_cast_QPaintDevice_ptr(QToolBar* ptr);
QT_WIDGETS_C_EXPORT QToolBar* qt_widgets_c_QToolBar_G_static_cast_QToolBar_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QToolBar* qt_widgets_c_QToolBar_G_static_cast_QToolBar_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QToolBar* qt_widgets_c_QToolBar_G_static_cast_QToolBar_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QToolBar_G_static_cast_QWidget_ptr(QToolBar* ptr);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QToolBar_actionAt_p(const QToolBar* this_ptr, const QPoint* p);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QToolBar_actionAt_x_y(const QToolBar* this_ptr, int x, int y);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBar_actionGeometry_to_output(const QToolBar* this_ptr, QAction* action, QRect* output);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QToolBar_addAction_icon_text(QToolBar* this_ptr, const QIcon* icon, const QString* text);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QToolBar_addAction_icon_text_receiver_member(QToolBar* this_ptr, const QIcon* icon, const QString* text, const QObject* receiver, const char* member);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QToolBar_addAction_text(QToolBar* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QToolBar_addAction_text_receiver_member(QToolBar* this_ptr, const QString* text, const QObject* receiver, const char* member);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QToolBar_addSeparator(QToolBar* this_ptr);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QToolBar_addWidget(QToolBar* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBar_clear(QToolBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBar_delete(QToolBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBar_iconSize_to_output(const QToolBar* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QToolBar_insertSeparator(QToolBar* this_ptr, QAction* before);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QToolBar_insertWidget(QToolBar* this_ptr, QAction* before, QWidget* widget);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QToolBar_isAreaAllowed(const QToolBar* this_ptr, const Qt::ToolBarArea* area);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QToolBar_isFloatable(const QToolBar* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QToolBar_isFloating(const QToolBar* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QToolBar_isMovable(const QToolBar* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QToolBar_metaObject(const QToolBar* this_ptr);
QT_WIDGETS_C_EXPORT QToolBar* qt_widgets_c_QToolBar_new_no_args();
QT_WIDGETS_C_EXPORT QToolBar* qt_widgets_c_QToolBar_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT QToolBar* qt_widgets_c_QToolBar_new_title(const QString* title);
QT_WIDGETS_C_EXPORT QToolBar* qt_widgets_c_QToolBar_new_title_parent(const QString* title, QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QToolBar_qt_metacall(QToolBar* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QToolBar_qt_metacast(QToolBar* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBar_setFloatable(QToolBar* this_ptr, bool floatable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBar_setIconSize(QToolBar* this_ptr, const QSize* iconSize);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBar_setMovable(QToolBar* this_ptr, bool movable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBar_setOrientation(QToolBar* this_ptr, const Qt::Orientation* orientation);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBar_setToolButtonStyle(QToolBar* this_ptr, const Qt::ToolButtonStyle* toolButtonStyle);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QToolBar_toggleViewAction(const QToolBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBar_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBar_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QToolBar_widgetForAction(const QToolBar* this_ptr, QAction* action);

} // extern "C"

#endif // QT_WIDGETS_C_QTOOLBAR_H
