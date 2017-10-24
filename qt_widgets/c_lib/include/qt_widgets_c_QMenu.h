#ifndef QT_WIDGETS_C_QMENU_H
#define QT_WIDGETS_C_QMENU_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QMenu_G_dynamic_cast_QMenu_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_G_qt_mac_set_dock_menu(QMenu* menu);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QMenu_G_static_cast_QMenu_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QMenu_G_static_cast_QMenu_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QMenu_G_static_cast_QMenu_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QMenu_G_static_cast_QObject_ptr(QMenu* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QMenu_G_static_cast_QPaintDevice_ptr(QMenu* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QMenu_G_static_cast_QWidget_ptr(QMenu* ptr);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_actionAt(const QMenu* this_ptr, const QPoint* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_actionGeometry_to_output(const QMenu* this_ptr, QAction* arg1, QRect* output);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_activeAction(const QMenu* this_ptr);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_addAction_icon_text(QMenu* this_ptr, const QIcon* icon, const QString* text);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_addAction_icon_text_receiver_member(QMenu* this_ptr, const QIcon* icon, const QString* text, const QObject* receiver, const char* member);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_addAction_icon_text_receiver_member_shortcut(QMenu* this_ptr, const QIcon* icon, const QString* text, const QObject* receiver, const char* member, const QKeySequence* shortcut);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_addAction_text(QMenu* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_addAction_text_receiver_member(QMenu* this_ptr, const QString* text, const QObject* receiver, const char* member);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_addAction_text_receiver_member_shortcut(QMenu* this_ptr, const QString* text, const QObject* receiver, const char* member, const QKeySequence* shortcut);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QMenu_addMenu_icon_title(QMenu* this_ptr, const QIcon* icon, const QString* title);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_addMenu_menu(QMenu* this_ptr, QMenu* menu);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QMenu_addMenu_title(QMenu* this_ptr, const QString* title);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_addSection_icon_text(QMenu* this_ptr, const QIcon* icon, const QString* text);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_addSection_text(QMenu* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_addSeparator(QMenu* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_clear(QMenu* this_ptr);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_defaultAction(const QMenu* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_delete(QMenu* this_ptr);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_exec_actions_pos(const QList< QAction* >* actions, const QPoint* pos);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_exec_actions_pos_at(const QList< QAction* >* actions, const QPoint* pos, QAction* at);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_exec_actions_pos_at_parent(const QList< QAction* >* actions, const QPoint* pos, QAction* at, QWidget* parent);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_exec_no_args(QMenu* this_ptr);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_exec_pos(QMenu* this_ptr, const QPoint* pos);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_exec_pos_at(QMenu* this_ptr, const QPoint* pos, QAction* at);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_hideTearOffMenu(QMenu* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_icon_to_output(const QMenu* this_ptr, QIcon* output);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_insertMenu(QMenu* this_ptr, QAction* before, QMenu* menu);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_insertSection_before_icon_text(QMenu* this_ptr, QAction* before, const QIcon* icon, const QString* text);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_insertSection_before_text(QMenu* this_ptr, QAction* before, const QString* text);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_insertSeparator(QMenu* this_ptr, QAction* before);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMenu_isEmpty(const QMenu* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMenu_isTearOffEnabled(const QMenu* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMenu_isTearOffMenuVisible(const QMenu* this_ptr);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenu_menuAction(const QMenu* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QMenu_metaObject(const QMenu* this_ptr);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QMenu_new_no_args();
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QMenu_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QMenu_new_title(const QString* title);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QMenu_new_title_parent(const QString* title, QWidget* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_popup_pos(QMenu* this_ptr, const QPoint* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_popup_pos_at(QMenu* this_ptr, const QPoint* pos, QAction* at);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMenu_qt_metacall(QMenu* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QMenu_qt_metacast(QMenu* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMenu_separatorsCollapsible(const QMenu* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_setActiveAction(QMenu* this_ptr, QAction* act);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_setAsDockMenu(QMenu* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_setDefaultAction(QMenu* this_ptr, QAction* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_setIcon(QMenu* this_ptr, const QIcon* icon);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_setNoReplayFor(QMenu* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_setSeparatorsCollapsible(QMenu* this_ptr, bool collapse);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_setTearOffEnabled(QMenu* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_setTitle(QMenu* this_ptr, const QString* title);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_setToolTipsVisible(QMenu* this_ptr, bool visible);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_showTearOffMenu_no_args(QMenu* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_showTearOffMenu_pos(QMenu* this_ptr, const QPoint* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_sizeHint_to_output(const QMenu* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_title_to_output(const QMenu* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMenu_toolTipsVisible(const QMenu* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenu_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QMENU_H
