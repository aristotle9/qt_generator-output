#include "qt_widgets_c_QMenu.h"

QMenu* qt_widgets_c_QMenu_G_dynamic_cast_QMenu_ptr(QWidget* ptr) {
  return dynamic_cast<QMenu*>(ptr);
}

void qt_widgets_c_QMenu_G_qt_mac_set_dock_menu(QMenu* menu) {
  qt_mac_set_dock_menu(menu);
}

QMenu* qt_widgets_c_QMenu_G_static_cast_QMenu_ptr_QObject(QObject* ptr) {
  return static_cast<QMenu*>(ptr);
}

QMenu* qt_widgets_c_QMenu_G_static_cast_QMenu_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QMenu*>(ptr);
}

QMenu* qt_widgets_c_QMenu_G_static_cast_QMenu_ptr_QWidget(QWidget* ptr) {
  return static_cast<QMenu*>(ptr);
}

QObject* qt_widgets_c_QMenu_G_static_cast_QObject_ptr(QMenu* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QMenu_G_static_cast_QPaintDevice_ptr(QMenu* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QMenu_G_static_cast_QWidget_ptr(QMenu* ptr) {
  return static_cast<QWidget*>(ptr);
}

QAction* qt_widgets_c_QMenu_actionAt(const QMenu* this_ptr, const QPoint* arg1) {
  return this_ptr->actionAt(*arg1);
}

void qt_widgets_c_QMenu_actionGeometry_to_output(const QMenu* this_ptr, QAction* arg1, QRect* output) {
  new(output) QRect(this_ptr->actionGeometry(arg1));
}

QAction* qt_widgets_c_QMenu_activeAction(const QMenu* this_ptr) {
  return this_ptr->activeAction();
}

QAction* qt_widgets_c_QMenu_addAction_icon_text(QMenu* this_ptr, const QIcon* icon, const QString* text) {
  return this_ptr->addAction(*icon, *text);
}

QAction* qt_widgets_c_QMenu_addAction_icon_text_receiver_member(QMenu* this_ptr, const QIcon* icon, const QString* text, const QObject* receiver, const char* member) {
  return this_ptr->addAction(*icon, *text, receiver, member);
}

QAction* qt_widgets_c_QMenu_addAction_icon_text_receiver_member_shortcut(QMenu* this_ptr, const QIcon* icon, const QString* text, const QObject* receiver, const char* member, const QKeySequence* shortcut) {
  return this_ptr->addAction(*icon, *text, receiver, member, *shortcut);
}

QAction* qt_widgets_c_QMenu_addAction_text(QMenu* this_ptr, const QString* text) {
  return this_ptr->addAction(*text);
}

QAction* qt_widgets_c_QMenu_addAction_text_receiver_member(QMenu* this_ptr, const QString* text, const QObject* receiver, const char* member) {
  return this_ptr->addAction(*text, receiver, member);
}

QAction* qt_widgets_c_QMenu_addAction_text_receiver_member_shortcut(QMenu* this_ptr, const QString* text, const QObject* receiver, const char* member, const QKeySequence* shortcut) {
  return this_ptr->addAction(*text, receiver, member, *shortcut);
}

QMenu* qt_widgets_c_QMenu_addMenu_icon_title(QMenu* this_ptr, const QIcon* icon, const QString* title) {
  return this_ptr->addMenu(*icon, *title);
}

QAction* qt_widgets_c_QMenu_addMenu_menu(QMenu* this_ptr, QMenu* menu) {
  return this_ptr->addMenu(menu);
}

QMenu* qt_widgets_c_QMenu_addMenu_title(QMenu* this_ptr, const QString* title) {
  return this_ptr->addMenu(*title);
}

QAction* qt_widgets_c_QMenu_addSection_icon_text(QMenu* this_ptr, const QIcon* icon, const QString* text) {
  return this_ptr->addSection(*icon, *text);
}

QAction* qt_widgets_c_QMenu_addSection_text(QMenu* this_ptr, const QString* text) {
  return this_ptr->addSection(*text);
}

QAction* qt_widgets_c_QMenu_addSeparator(QMenu* this_ptr) {
  return this_ptr->addSeparator();
}

void qt_widgets_c_QMenu_clear(QMenu* this_ptr) {
  this_ptr->clear();
}

QAction* qt_widgets_c_QMenu_defaultAction(const QMenu* this_ptr) {
  return this_ptr->defaultAction();
}

void qt_widgets_c_QMenu_delete(QMenu* this_ptr) {
  delete this_ptr;
}

QAction* qt_widgets_c_QMenu_exec_actions_pos(const QList< QAction* >* actions, const QPoint* pos) {
  return QMenu::exec(*actions, *pos);
}

QAction* qt_widgets_c_QMenu_exec_actions_pos_at(const QList< QAction* >* actions, const QPoint* pos, QAction* at) {
  return QMenu::exec(*actions, *pos, at);
}

QAction* qt_widgets_c_QMenu_exec_actions_pos_at_parent(const QList< QAction* >* actions, const QPoint* pos, QAction* at, QWidget* parent) {
  return QMenu::exec(*actions, *pos, at, parent);
}

QAction* qt_widgets_c_QMenu_exec_no_args(QMenu* this_ptr) {
  return this_ptr->exec();
}

QAction* qt_widgets_c_QMenu_exec_pos(QMenu* this_ptr, const QPoint* pos) {
  return this_ptr->exec(*pos);
}

QAction* qt_widgets_c_QMenu_exec_pos_at(QMenu* this_ptr, const QPoint* pos, QAction* at) {
  return this_ptr->exec(*pos, at);
}

void qt_widgets_c_QMenu_hideTearOffMenu(QMenu* this_ptr) {
  this_ptr->hideTearOffMenu();
}

void qt_widgets_c_QMenu_icon_to_output(const QMenu* this_ptr, QIcon* output) {
  new(output) QIcon(this_ptr->icon());
}

QAction* qt_widgets_c_QMenu_insertMenu(QMenu* this_ptr, QAction* before, QMenu* menu) {
  return this_ptr->insertMenu(before, menu);
}

QAction* qt_widgets_c_QMenu_insertSection_before_icon_text(QMenu* this_ptr, QAction* before, const QIcon* icon, const QString* text) {
  return this_ptr->insertSection(before, *icon, *text);
}

QAction* qt_widgets_c_QMenu_insertSection_before_text(QMenu* this_ptr, QAction* before, const QString* text) {
  return this_ptr->insertSection(before, *text);
}

QAction* qt_widgets_c_QMenu_insertSeparator(QMenu* this_ptr, QAction* before) {
  return this_ptr->insertSeparator(before);
}

bool qt_widgets_c_QMenu_isEmpty(const QMenu* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_widgets_c_QMenu_isTearOffEnabled(const QMenu* this_ptr) {
  return this_ptr->isTearOffEnabled();
}

bool qt_widgets_c_QMenu_isTearOffMenuVisible(const QMenu* this_ptr) {
  return this_ptr->isTearOffMenuVisible();
}

QAction* qt_widgets_c_QMenu_menuAction(const QMenu* this_ptr) {
  return this_ptr->menuAction();
}

const QMetaObject* qt_widgets_c_QMenu_metaObject(const QMenu* this_ptr) {
  return this_ptr->metaObject();
}

QMenu* qt_widgets_c_QMenu_new_no_args() {
  return new QMenu();
}

QMenu* qt_widgets_c_QMenu_new_parent(QWidget* parent) {
  return new QMenu(parent);
}

QMenu* qt_widgets_c_QMenu_new_title(const QString* title) {
  return new QMenu(*title);
}

QMenu* qt_widgets_c_QMenu_new_title_parent(const QString* title, QWidget* parent) {
  return new QMenu(*title, parent);
}

void qt_widgets_c_QMenu_popup_pos(QMenu* this_ptr, const QPoint* pos) {
  this_ptr->popup(*pos);
}

void qt_widgets_c_QMenu_popup_pos_at(QMenu* this_ptr, const QPoint* pos, QAction* at) {
  this_ptr->popup(*pos, at);
}

int qt_widgets_c_QMenu_qt_metacall(QMenu* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QMenu_qt_metacast(QMenu* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

bool qt_widgets_c_QMenu_separatorsCollapsible(const QMenu* this_ptr) {
  return this_ptr->separatorsCollapsible();
}

void qt_widgets_c_QMenu_setActiveAction(QMenu* this_ptr, QAction* act) {
  this_ptr->setActiveAction(act);
}

void qt_widgets_c_QMenu_setAsDockMenu(QMenu* this_ptr) {
  this_ptr->setAsDockMenu();
}

void qt_widgets_c_QMenu_setDefaultAction(QMenu* this_ptr, QAction* arg1) {
  this_ptr->setDefaultAction(arg1);
}

void qt_widgets_c_QMenu_setIcon(QMenu* this_ptr, const QIcon* icon) {
  this_ptr->setIcon(*icon);
}

void qt_widgets_c_QMenu_setNoReplayFor(QMenu* this_ptr, QWidget* widget) {
  this_ptr->setNoReplayFor(widget);
}

void qt_widgets_c_QMenu_setSeparatorsCollapsible(QMenu* this_ptr, bool collapse) {
  this_ptr->setSeparatorsCollapsible(collapse);
}

void qt_widgets_c_QMenu_setTearOffEnabled(QMenu* this_ptr, bool arg1) {
  this_ptr->setTearOffEnabled(arg1);
}

void qt_widgets_c_QMenu_setTitle(QMenu* this_ptr, const QString* title) {
  this_ptr->setTitle(*title);
}

void qt_widgets_c_QMenu_setToolTipsVisible(QMenu* this_ptr, bool visible) {
  this_ptr->setToolTipsVisible(visible);
}

void qt_widgets_c_QMenu_showTearOffMenu_no_args(QMenu* this_ptr) {
  this_ptr->showTearOffMenu();
}

void qt_widgets_c_QMenu_showTearOffMenu_pos(QMenu* this_ptr, const QPoint* pos) {
  this_ptr->showTearOffMenu(*pos);
}

void qt_widgets_c_QMenu_sizeHint_to_output(const QMenu* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QMenu_title_to_output(const QMenu* this_ptr, QString* output) {
  new(output) QString(this_ptr->title());
}

bool qt_widgets_c_QMenu_toolTipsVisible(const QMenu* this_ptr) {
  return this_ptr->toolTipsVisible();
}

void qt_widgets_c_QMenu_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMenu::trUtf8(s, c, n));
}

void qt_widgets_c_QMenu_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMenu::tr(s, c, n));
}

