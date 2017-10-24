#include "qt_widgets_c_QMenuBar.h"

QMenuBar* qt_widgets_c_QMenuBar_G_dynamic_cast_QMenuBar_ptr(QWidget* ptr) {
  return dynamic_cast<QMenuBar*>(ptr);
}

QMenuBar* qt_widgets_c_QMenuBar_G_static_cast_QMenuBar_ptr_QObject(QObject* ptr) {
  return static_cast<QMenuBar*>(ptr);
}

QMenuBar* qt_widgets_c_QMenuBar_G_static_cast_QMenuBar_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QMenuBar*>(ptr);
}

QMenuBar* qt_widgets_c_QMenuBar_G_static_cast_QMenuBar_ptr_QWidget(QWidget* ptr) {
  return static_cast<QMenuBar*>(ptr);
}

QObject* qt_widgets_c_QMenuBar_G_static_cast_QObject_ptr(QMenuBar* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QMenuBar_G_static_cast_QPaintDevice_ptr(QMenuBar* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QMenuBar_G_static_cast_QWidget_ptr(QMenuBar* ptr) {
  return static_cast<QWidget*>(ptr);
}

QAction* qt_widgets_c_QMenuBar_actionAt(const QMenuBar* this_ptr, const QPoint* arg1) {
  return this_ptr->actionAt(*arg1);
}

void qt_widgets_c_QMenuBar_actionGeometry_to_output(const QMenuBar* this_ptr, QAction* arg1, QRect* output) {
  new(output) QRect(this_ptr->actionGeometry(arg1));
}

QAction* qt_widgets_c_QMenuBar_activeAction(const QMenuBar* this_ptr) {
  return this_ptr->activeAction();
}

QAction* qt_widgets_c_QMenuBar_addAction_text(QMenuBar* this_ptr, const QString* text) {
  return this_ptr->addAction(*text);
}

QAction* qt_widgets_c_QMenuBar_addAction_text_receiver_member(QMenuBar* this_ptr, const QString* text, const QObject* receiver, const char* member) {
  return this_ptr->addAction(*text, receiver, member);
}

QMenu* qt_widgets_c_QMenuBar_addMenu_icon_title(QMenuBar* this_ptr, const QIcon* icon, const QString* title) {
  return this_ptr->addMenu(*icon, *title);
}

QAction* qt_widgets_c_QMenuBar_addMenu_menu(QMenuBar* this_ptr, QMenu* menu) {
  return this_ptr->addMenu(menu);
}

QMenu* qt_widgets_c_QMenuBar_addMenu_title(QMenuBar* this_ptr, const QString* title) {
  return this_ptr->addMenu(*title);
}

QAction* qt_widgets_c_QMenuBar_addSeparator(QMenuBar* this_ptr) {
  return this_ptr->addSeparator();
}

void qt_widgets_c_QMenuBar_clear(QMenuBar* this_ptr) {
  this_ptr->clear();
}

QWidget* qt_widgets_c_QMenuBar_cornerWidget_corner(const QMenuBar* this_ptr, const Qt::Corner* corner) {
  return this_ptr->cornerWidget(*corner);
}

QWidget* qt_widgets_c_QMenuBar_cornerWidget_no_args(const QMenuBar* this_ptr) {
  return this_ptr->cornerWidget();
}

void qt_widgets_c_QMenuBar_delete(QMenuBar* this_ptr) {
  delete this_ptr;
}

int qt_widgets_c_QMenuBar_heightForWidth(const QMenuBar* this_ptr, int arg1) {
  return this_ptr->heightForWidth(arg1);
}

QAction* qt_widgets_c_QMenuBar_insertMenu(QMenuBar* this_ptr, QAction* before, QMenu* menu) {
  return this_ptr->insertMenu(before, menu);
}

QAction* qt_widgets_c_QMenuBar_insertSeparator(QMenuBar* this_ptr, QAction* before) {
  return this_ptr->insertSeparator(before);
}

bool qt_widgets_c_QMenuBar_isDefaultUp(const QMenuBar* this_ptr) {
  return this_ptr->isDefaultUp();
}

bool qt_widgets_c_QMenuBar_isNativeMenuBar(const QMenuBar* this_ptr) {
  return this_ptr->isNativeMenuBar();
}

const QMetaObject* qt_widgets_c_QMenuBar_metaObject(const QMenuBar* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QMenuBar_minimumSizeHint_to_output(const QMenuBar* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

QMenuBar* qt_widgets_c_QMenuBar_new_no_args() {
  return new QMenuBar();
}

QMenuBar* qt_widgets_c_QMenuBar_new_parent(QWidget* parent) {
  return new QMenuBar(parent);
}

int qt_widgets_c_QMenuBar_qt_metacall(QMenuBar* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QMenuBar_qt_metacast(QMenuBar* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QMenuBar_setActiveAction(QMenuBar* this_ptr, QAction* action) {
  this_ptr->setActiveAction(action);
}

void qt_widgets_c_QMenuBar_setCornerWidget_w(QMenuBar* this_ptr, QWidget* w) {
  this_ptr->setCornerWidget(w);
}

void qt_widgets_c_QMenuBar_setCornerWidget_w_corner(QMenuBar* this_ptr, QWidget* w, const Qt::Corner* corner) {
  this_ptr->setCornerWidget(w, *corner);
}

void qt_widgets_c_QMenuBar_setDefaultUp(QMenuBar* this_ptr, bool arg1) {
  this_ptr->setDefaultUp(arg1);
}

void qt_widgets_c_QMenuBar_setNativeMenuBar(QMenuBar* this_ptr, bool nativeMenuBar) {
  this_ptr->setNativeMenuBar(nativeMenuBar);
}

void qt_widgets_c_QMenuBar_setVisible(QMenuBar* this_ptr, bool visible) {
  this_ptr->setVisible(visible);
}

void qt_widgets_c_QMenuBar_sizeHint_to_output(const QMenuBar* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QMenuBar_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMenuBar::trUtf8(s, c, n));
}

void qt_widgets_c_QMenuBar_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMenuBar::tr(s, c, n));
}

