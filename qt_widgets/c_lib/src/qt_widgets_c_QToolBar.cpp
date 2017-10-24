#include "qt_widgets_c_QToolBar.h"

QToolBar* qt_widgets_c_QToolBar_G_dynamic_cast_QToolBar_ptr(QWidget* ptr) {
  return dynamic_cast<QToolBar*>(ptr);
}

QObject* qt_widgets_c_QToolBar_G_static_cast_QObject_ptr(QToolBar* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QToolBar_G_static_cast_QPaintDevice_ptr(QToolBar* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QToolBar* qt_widgets_c_QToolBar_G_static_cast_QToolBar_ptr_QObject(QObject* ptr) {
  return static_cast<QToolBar*>(ptr);
}

QToolBar* qt_widgets_c_QToolBar_G_static_cast_QToolBar_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QToolBar*>(ptr);
}

QToolBar* qt_widgets_c_QToolBar_G_static_cast_QToolBar_ptr_QWidget(QWidget* ptr) {
  return static_cast<QToolBar*>(ptr);
}

QWidget* qt_widgets_c_QToolBar_G_static_cast_QWidget_ptr(QToolBar* ptr) {
  return static_cast<QWidget*>(ptr);
}

QAction* qt_widgets_c_QToolBar_actionAt_p(const QToolBar* this_ptr, const QPoint* p) {
  return this_ptr->actionAt(*p);
}

QAction* qt_widgets_c_QToolBar_actionAt_x_y(const QToolBar* this_ptr, int x, int y) {
  return this_ptr->actionAt(x, y);
}

void qt_widgets_c_QToolBar_actionGeometry_to_output(const QToolBar* this_ptr, QAction* action, QRect* output) {
  new(output) QRect(this_ptr->actionGeometry(action));
}

QAction* qt_widgets_c_QToolBar_addAction_icon_text(QToolBar* this_ptr, const QIcon* icon, const QString* text) {
  return this_ptr->addAction(*icon, *text);
}

QAction* qt_widgets_c_QToolBar_addAction_icon_text_receiver_member(QToolBar* this_ptr, const QIcon* icon, const QString* text, const QObject* receiver, const char* member) {
  return this_ptr->addAction(*icon, *text, receiver, member);
}

QAction* qt_widgets_c_QToolBar_addAction_text(QToolBar* this_ptr, const QString* text) {
  return this_ptr->addAction(*text);
}

QAction* qt_widgets_c_QToolBar_addAction_text_receiver_member(QToolBar* this_ptr, const QString* text, const QObject* receiver, const char* member) {
  return this_ptr->addAction(*text, receiver, member);
}

QAction* qt_widgets_c_QToolBar_addSeparator(QToolBar* this_ptr) {
  return this_ptr->addSeparator();
}

QAction* qt_widgets_c_QToolBar_addWidget(QToolBar* this_ptr, QWidget* widget) {
  return this_ptr->addWidget(widget);
}

void qt_widgets_c_QToolBar_clear(QToolBar* this_ptr) {
  this_ptr->clear();
}

void qt_widgets_c_QToolBar_delete(QToolBar* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QToolBar_iconSize_to_output(const QToolBar* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->iconSize());
}

QAction* qt_widgets_c_QToolBar_insertSeparator(QToolBar* this_ptr, QAction* before) {
  return this_ptr->insertSeparator(before);
}

QAction* qt_widgets_c_QToolBar_insertWidget(QToolBar* this_ptr, QAction* before, QWidget* widget) {
  return this_ptr->insertWidget(before, widget);
}

bool qt_widgets_c_QToolBar_isAreaAllowed(const QToolBar* this_ptr, const Qt::ToolBarArea* area) {
  return this_ptr->isAreaAllowed(*area);
}

bool qt_widgets_c_QToolBar_isFloatable(const QToolBar* this_ptr) {
  return this_ptr->isFloatable();
}

bool qt_widgets_c_QToolBar_isFloating(const QToolBar* this_ptr) {
  return this_ptr->isFloating();
}

bool qt_widgets_c_QToolBar_isMovable(const QToolBar* this_ptr) {
  return this_ptr->isMovable();
}

const QMetaObject* qt_widgets_c_QToolBar_metaObject(const QToolBar* this_ptr) {
  return this_ptr->metaObject();
}

QToolBar* qt_widgets_c_QToolBar_new_no_args() {
  return new QToolBar();
}

QToolBar* qt_widgets_c_QToolBar_new_parent(QWidget* parent) {
  return new QToolBar(parent);
}

QToolBar* qt_widgets_c_QToolBar_new_title(const QString* title) {
  return new QToolBar(*title);
}

QToolBar* qt_widgets_c_QToolBar_new_title_parent(const QString* title, QWidget* parent) {
  return new QToolBar(*title, parent);
}

int qt_widgets_c_QToolBar_qt_metacall(QToolBar* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QToolBar_qt_metacast(QToolBar* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QToolBar_setFloatable(QToolBar* this_ptr, bool floatable) {
  this_ptr->setFloatable(floatable);
}

void qt_widgets_c_QToolBar_setIconSize(QToolBar* this_ptr, const QSize* iconSize) {
  this_ptr->setIconSize(*iconSize);
}

void qt_widgets_c_QToolBar_setMovable(QToolBar* this_ptr, bool movable) {
  this_ptr->setMovable(movable);
}

void qt_widgets_c_QToolBar_setOrientation(QToolBar* this_ptr, const Qt::Orientation* orientation) {
  this_ptr->setOrientation(*orientation);
}

void qt_widgets_c_QToolBar_setToolButtonStyle(QToolBar* this_ptr, const Qt::ToolButtonStyle* toolButtonStyle) {
  this_ptr->setToolButtonStyle(*toolButtonStyle);
}

QAction* qt_widgets_c_QToolBar_toggleViewAction(const QToolBar* this_ptr) {
  return this_ptr->toggleViewAction();
}

void qt_widgets_c_QToolBar_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QToolBar::trUtf8(s, c, n));
}

void qt_widgets_c_QToolBar_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QToolBar::tr(s, c, n));
}

QWidget* qt_widgets_c_QToolBar_widgetForAction(const QToolBar* this_ptr, QAction* action) {
  return this_ptr->widgetForAction(action);
}

