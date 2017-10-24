#include "qt_widgets_c_QAction.h"

void qt_widgets_c_QAction_G_operator_shl_to_output(const QDebug* arg1, const QAction* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, arg2));
}

QAction* qt_widgets_c_QAction_G_static_cast_QAction_ptr(QObject* ptr) {
  return static_cast<QAction*>(ptr);
}

QObject* qt_widgets_c_QAction_G_static_cast_QObject_ptr(QAction* ptr) {
  return static_cast<QObject*>(ptr);
}

QActionGroup* qt_widgets_c_QAction_actionGroup(const QAction* this_ptr) {
  return this_ptr->actionGroup();
}

void qt_widgets_c_QAction_activate(QAction* this_ptr, QAction::ActionEvent event) {
  this_ptr->activate(event);
}

void qt_widgets_c_QAction_associatedGraphicsWidgets_to_output(const QAction* this_ptr, QList< QGraphicsWidget* >* output) {
  new(output) QList< QGraphicsWidget* >(this_ptr->associatedGraphicsWidgets());
}

void qt_widgets_c_QAction_associatedWidgets_to_output(const QAction* this_ptr, QList< QWidget* >* output) {
  new(output) QList< QWidget* >(this_ptr->associatedWidgets());
}

bool qt_widgets_c_QAction_autoRepeat(const QAction* this_ptr) {
  return this_ptr->autoRepeat();
}

void qt_widgets_c_QAction_data_to_output(const QAction* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->data());
}

void qt_widgets_c_QAction_delete(QAction* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QAction_font_to_output(const QAction* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->font());
}

void qt_widgets_c_QAction_hover(QAction* this_ptr) {
  this_ptr->hover();
}

void qt_widgets_c_QAction_iconText_to_output(const QAction* this_ptr, QString* output) {
  new(output) QString(this_ptr->iconText());
}

void qt_widgets_c_QAction_icon_to_output(const QAction* this_ptr, QIcon* output) {
  new(output) QIcon(this_ptr->icon());
}

bool qt_widgets_c_QAction_isCheckable(const QAction* this_ptr) {
  return this_ptr->isCheckable();
}

bool qt_widgets_c_QAction_isChecked(const QAction* this_ptr) {
  return this_ptr->isChecked();
}

bool qt_widgets_c_QAction_isEnabled(const QAction* this_ptr) {
  return this_ptr->isEnabled();
}

bool qt_widgets_c_QAction_isIconVisibleInMenu(const QAction* this_ptr) {
  return this_ptr->isIconVisibleInMenu();
}

bool qt_widgets_c_QAction_isSeparator(const QAction* this_ptr) {
  return this_ptr->isSeparator();
}

bool qt_widgets_c_QAction_isVisible(const QAction* this_ptr) {
  return this_ptr->isVisible();
}

QMenu* qt_widgets_c_QAction_menu(const QAction* this_ptr) {
  return this_ptr->menu();
}

QAction::MenuRole qt_widgets_c_QAction_menuRole(const QAction* this_ptr) {
  return this_ptr->menuRole();
}

const QMetaObject* qt_widgets_c_QAction_metaObject(const QAction* this_ptr) {
  return this_ptr->metaObject();
}

QAction* qt_widgets_c_QAction_new_icon_text(const QIcon* icon, const QString* text) {
  return new QAction(*icon, *text);
}

QAction* qt_widgets_c_QAction_new_icon_text_parent(const QIcon* icon, const QString* text, QObject* parent) {
  return new QAction(*icon, *text, parent);
}

QAction* qt_widgets_c_QAction_new_no_args() {
  return new QAction();
}

QAction* qt_widgets_c_QAction_new_parent(QObject* parent) {
  return new QAction(parent);
}

QAction* qt_widgets_c_QAction_new_text(const QString* text) {
  return new QAction(*text);
}

QAction* qt_widgets_c_QAction_new_text_parent(const QString* text, QObject* parent) {
  return new QAction(*text, parent);
}

QWidget* qt_widgets_c_QAction_parentWidget(const QAction* this_ptr) {
  return this_ptr->parentWidget();
}

QAction::Priority qt_widgets_c_QAction_priority(const QAction* this_ptr) {
  return this_ptr->priority();
}

int qt_widgets_c_QAction_qt_metacall(QAction* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QAction_qt_metacast(QAction* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QAction_setActionGroup(QAction* this_ptr, QActionGroup* group) {
  this_ptr->setActionGroup(group);
}

void qt_widgets_c_QAction_setAutoRepeat(QAction* this_ptr, bool arg1) {
  this_ptr->setAutoRepeat(arg1);
}

void qt_widgets_c_QAction_setCheckable(QAction* this_ptr, bool arg1) {
  this_ptr->setCheckable(arg1);
}

void qt_widgets_c_QAction_setChecked(QAction* this_ptr, bool arg1) {
  this_ptr->setChecked(arg1);
}

void qt_widgets_c_QAction_setData(QAction* this_ptr, const QVariant* var) {
  this_ptr->setData(*var);
}

void qt_widgets_c_QAction_setDisabled(QAction* this_ptr, bool b) {
  this_ptr->setDisabled(b);
}

void qt_widgets_c_QAction_setEnabled(QAction* this_ptr, bool arg1) {
  this_ptr->setEnabled(arg1);
}

void qt_widgets_c_QAction_setFont(QAction* this_ptr, const QFont* font) {
  this_ptr->setFont(*font);
}

void qt_widgets_c_QAction_setIcon(QAction* this_ptr, const QIcon* icon) {
  this_ptr->setIcon(*icon);
}

void qt_widgets_c_QAction_setIconText(QAction* this_ptr, const QString* text) {
  this_ptr->setIconText(*text);
}

void qt_widgets_c_QAction_setIconVisibleInMenu(QAction* this_ptr, bool visible) {
  this_ptr->setIconVisibleInMenu(visible);
}

void qt_widgets_c_QAction_setMenu(QAction* this_ptr, QMenu* menu) {
  this_ptr->setMenu(menu);
}

void qt_widgets_c_QAction_setMenuRole(QAction* this_ptr, QAction::MenuRole menuRole) {
  this_ptr->setMenuRole(menuRole);
}

void qt_widgets_c_QAction_setPriority(QAction* this_ptr, QAction::Priority priority) {
  this_ptr->setPriority(priority);
}

void qt_widgets_c_QAction_setSeparator(QAction* this_ptr, bool b) {
  this_ptr->setSeparator(b);
}

void qt_widgets_c_QAction_setShortcut(QAction* this_ptr, const QKeySequence* shortcut) {
  this_ptr->setShortcut(*shortcut);
}

void qt_widgets_c_QAction_setShortcutContext(QAction* this_ptr, const Qt::ShortcutContext* context) {
  this_ptr->setShortcutContext(*context);
}

void qt_widgets_c_QAction_setShortcuts_arg1(QAction* this_ptr, const QKeySequence::StandardKey* arg1) {
  this_ptr->setShortcuts(*arg1);
}

void qt_widgets_c_QAction_setShortcuts_shortcuts(QAction* this_ptr, const QList< QKeySequence >* shortcuts) {
  this_ptr->setShortcuts(*shortcuts);
}

void qt_widgets_c_QAction_setStatusTip(QAction* this_ptr, const QString* statusTip) {
  this_ptr->setStatusTip(*statusTip);
}

void qt_widgets_c_QAction_setText(QAction* this_ptr, const QString* text) {
  this_ptr->setText(*text);
}

void qt_widgets_c_QAction_setToolTip(QAction* this_ptr, const QString* tip) {
  this_ptr->setToolTip(*tip);
}

void qt_widgets_c_QAction_setVisible(QAction* this_ptr, bool arg1) {
  this_ptr->setVisible(arg1);
}

void qt_widgets_c_QAction_setWhatsThis(QAction* this_ptr, const QString* what) {
  this_ptr->setWhatsThis(*what);
}

void qt_widgets_c_QAction_shortcut_to_output(const QAction* this_ptr, QKeySequence* output) {
  new(output) QKeySequence(this_ptr->shortcut());
}

void qt_widgets_c_QAction_shortcuts_to_output(const QAction* this_ptr, QList< QKeySequence >* output) {
  new(output) QList< QKeySequence >(this_ptr->shortcuts());
}

bool qt_widgets_c_QAction_showStatusText_no_args(QAction* this_ptr) {
  return this_ptr->showStatusText();
}

bool qt_widgets_c_QAction_showStatusText_widget(QAction* this_ptr, QWidget* widget) {
  return this_ptr->showStatusText(widget);
}

void qt_widgets_c_QAction_statusTip_to_output(const QAction* this_ptr, QString* output) {
  new(output) QString(this_ptr->statusTip());
}

void qt_widgets_c_QAction_text_to_output(const QAction* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_widgets_c_QAction_toggle(QAction* this_ptr) {
  this_ptr->toggle();
}

void qt_widgets_c_QAction_toolTip_to_output(const QAction* this_ptr, QString* output) {
  new(output) QString(this_ptr->toolTip());
}

void qt_widgets_c_QAction_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAction::trUtf8(s, c, n));
}

void qt_widgets_c_QAction_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAction::tr(s, c, n));
}

void qt_widgets_c_QAction_trigger(QAction* this_ptr) {
  this_ptr->trigger();
}

void qt_widgets_c_QAction_whatsThis_to_output(const QAction* this_ptr, QString* output) {
  new(output) QString(this_ptr->whatsThis());
}

