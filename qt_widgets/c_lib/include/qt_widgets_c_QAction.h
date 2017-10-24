#ifndef QT_WIDGETS_C_QACTION_H
#define QT_WIDGETS_C_QACTION_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_G_operator_shl_to_output(const QDebug* arg1, const QAction* arg2, QDebug* output);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QAction_G_static_cast_QAction_ptr(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QAction_G_static_cast_QObject_ptr(QAction* ptr);
QT_WIDGETS_C_EXPORT QActionGroup* qt_widgets_c_QAction_actionGroup(const QAction* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_activate(QAction* this_ptr, QAction::ActionEvent event);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_associatedGraphicsWidgets_to_output(const QAction* this_ptr, QList< QGraphicsWidget* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_associatedWidgets_to_output(const QAction* this_ptr, QList< QWidget* >* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAction_autoRepeat(const QAction* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_data_to_output(const QAction* this_ptr, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_delete(QAction* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_font_to_output(const QAction* this_ptr, QFont* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_hover(QAction* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_iconText_to_output(const QAction* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_icon_to_output(const QAction* this_ptr, QIcon* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAction_isCheckable(const QAction* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAction_isChecked(const QAction* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAction_isEnabled(const QAction* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAction_isIconVisibleInMenu(const QAction* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAction_isSeparator(const QAction* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAction_isVisible(const QAction* this_ptr);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QAction_menu(const QAction* this_ptr);
QT_WIDGETS_C_EXPORT QAction::MenuRole qt_widgets_c_QAction_menuRole(const QAction* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QAction_metaObject(const QAction* this_ptr);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QAction_new_icon_text(const QIcon* icon, const QString* text);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QAction_new_icon_text_parent(const QIcon* icon, const QString* text, QObject* parent);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QAction_new_no_args();
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QAction_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QAction_new_text(const QString* text);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QAction_new_text_parent(const QString* text, QObject* parent);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QAction_parentWidget(const QAction* this_ptr);
QT_WIDGETS_C_EXPORT QAction::Priority qt_widgets_c_QAction_priority(const QAction* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QAction_qt_metacall(QAction* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QAction_qt_metacast(QAction* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setActionGroup(QAction* this_ptr, QActionGroup* group);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setAutoRepeat(QAction* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setCheckable(QAction* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setChecked(QAction* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setData(QAction* this_ptr, const QVariant* var);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setDisabled(QAction* this_ptr, bool b);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setEnabled(QAction* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setFont(QAction* this_ptr, const QFont* font);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setIcon(QAction* this_ptr, const QIcon* icon);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setIconText(QAction* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setIconVisibleInMenu(QAction* this_ptr, bool visible);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setMenu(QAction* this_ptr, QMenu* menu);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setMenuRole(QAction* this_ptr, QAction::MenuRole menuRole);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setPriority(QAction* this_ptr, QAction::Priority priority);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setSeparator(QAction* this_ptr, bool b);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setShortcut(QAction* this_ptr, const QKeySequence* shortcut);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setShortcutContext(QAction* this_ptr, const Qt::ShortcutContext* context);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setShortcuts_arg1(QAction* this_ptr, const QKeySequence::StandardKey* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setShortcuts_shortcuts(QAction* this_ptr, const QList< QKeySequence >* shortcuts);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setStatusTip(QAction* this_ptr, const QString* statusTip);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setText(QAction* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setToolTip(QAction* this_ptr, const QString* tip);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setVisible(QAction* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_setWhatsThis(QAction* this_ptr, const QString* what);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_shortcut_to_output(const QAction* this_ptr, QKeySequence* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_shortcuts_to_output(const QAction* this_ptr, QList< QKeySequence >* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAction_showStatusText_no_args(QAction* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAction_showStatusText_widget(QAction* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_statusTip_to_output(const QAction* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_text_to_output(const QAction* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_toggle(QAction* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_toolTip_to_output(const QAction* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_trigger(QAction* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAction_whatsThis_to_output(const QAction* this_ptr, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QACTION_H
