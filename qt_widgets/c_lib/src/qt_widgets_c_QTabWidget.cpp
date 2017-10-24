#include "qt_widgets_c_QTabWidget.h"

QTabWidget* qt_widgets_c_QTabWidget_G_dynamic_cast_QTabWidget_ptr(QWidget* ptr) {
  return dynamic_cast<QTabWidget*>(ptr);
}

QObject* qt_widgets_c_QTabWidget_G_static_cast_QObject_ptr(QTabWidget* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QTabWidget_G_static_cast_QPaintDevice_ptr(QTabWidget* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QTabWidget* qt_widgets_c_QTabWidget_G_static_cast_QTabWidget_ptr_QObject(QObject* ptr) {
  return static_cast<QTabWidget*>(ptr);
}

QTabWidget* qt_widgets_c_QTabWidget_G_static_cast_QTabWidget_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QTabWidget*>(ptr);
}

QTabWidget* qt_widgets_c_QTabWidget_G_static_cast_QTabWidget_ptr_QWidget(QWidget* ptr) {
  return static_cast<QTabWidget*>(ptr);
}

QWidget* qt_widgets_c_QTabWidget_G_static_cast_QWidget_ptr(QTabWidget* ptr) {
  return static_cast<QWidget*>(ptr);
}

int qt_widgets_c_QTabWidget_addTab_widget_arg2(QTabWidget* this_ptr, QWidget* widget, const QString* arg2) {
  return this_ptr->addTab(widget, *arg2);
}

int qt_widgets_c_QTabWidget_addTab_widget_icon_label(QTabWidget* this_ptr, QWidget* widget, const QIcon* icon, const QString* label) {
  return this_ptr->addTab(widget, *icon, *label);
}

void qt_widgets_c_QTabWidget_clear(QTabWidget* this_ptr) {
  this_ptr->clear();
}

QWidget* qt_widgets_c_QTabWidget_cornerWidget_corner(const QTabWidget* this_ptr, const Qt::Corner* corner) {
  return this_ptr->cornerWidget(*corner);
}

QWidget* qt_widgets_c_QTabWidget_cornerWidget_no_args(const QTabWidget* this_ptr) {
  return this_ptr->cornerWidget();
}

int qt_widgets_c_QTabWidget_count(const QTabWidget* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QTabWidget_currentIndex(const QTabWidget* this_ptr) {
  return this_ptr->currentIndex();
}

QWidget* qt_widgets_c_QTabWidget_currentWidget(const QTabWidget* this_ptr) {
  return this_ptr->currentWidget();
}

void qt_widgets_c_QTabWidget_delete(QTabWidget* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QTabWidget_documentMode(const QTabWidget* this_ptr) {
  return this_ptr->documentMode();
}

bool qt_widgets_c_QTabWidget_hasHeightForWidth(const QTabWidget* this_ptr) {
  return this_ptr->hasHeightForWidth();
}

int qt_widgets_c_QTabWidget_heightForWidth(const QTabWidget* this_ptr, int width) {
  return this_ptr->heightForWidth(width);
}

void qt_widgets_c_QTabWidget_iconSize_to_output(const QTabWidget* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->iconSize());
}

int qt_widgets_c_QTabWidget_indexOf(const QTabWidget* this_ptr, QWidget* widget) {
  return this_ptr->indexOf(widget);
}

int qt_widgets_c_QTabWidget_insertTab_index_widget_arg3(QTabWidget* this_ptr, int index, QWidget* widget, const QString* arg3) {
  return this_ptr->insertTab(index, widget, *arg3);
}

int qt_widgets_c_QTabWidget_insertTab_index_widget_icon_label(QTabWidget* this_ptr, int index, QWidget* widget, const QIcon* icon, const QString* label) {
  return this_ptr->insertTab(index, widget, *icon, *label);
}

bool qt_widgets_c_QTabWidget_isMovable(const QTabWidget* this_ptr) {
  return this_ptr->isMovable();
}

bool qt_widgets_c_QTabWidget_isTabEnabled(const QTabWidget* this_ptr, int index) {
  return this_ptr->isTabEnabled(index);
}

const QMetaObject* qt_widgets_c_QTabWidget_metaObject(const QTabWidget* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QTabWidget_minimumSizeHint_to_output(const QTabWidget* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

QTabWidget* qt_widgets_c_QTabWidget_new_no_args() {
  return new QTabWidget();
}

QTabWidget* qt_widgets_c_QTabWidget_new_parent(QWidget* parent) {
  return new QTabWidget(parent);
}

int qt_widgets_c_QTabWidget_qt_metacall(QTabWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QTabWidget_qt_metacast(QTabWidget* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QTabWidget_removeTab(QTabWidget* this_ptr, int index) {
  this_ptr->removeTab(index);
}

void qt_widgets_c_QTabWidget_setCornerWidget_w(QTabWidget* this_ptr, QWidget* w) {
  this_ptr->setCornerWidget(w);
}

void qt_widgets_c_QTabWidget_setCornerWidget_w_corner(QTabWidget* this_ptr, QWidget* w, const Qt::Corner* corner) {
  this_ptr->setCornerWidget(w, *corner);
}

void qt_widgets_c_QTabWidget_setCurrentIndex(QTabWidget* this_ptr, int index) {
  this_ptr->setCurrentIndex(index);
}

void qt_widgets_c_QTabWidget_setCurrentWidget(QTabWidget* this_ptr, QWidget* widget) {
  this_ptr->setCurrentWidget(widget);
}

void qt_widgets_c_QTabWidget_setDocumentMode(QTabWidget* this_ptr, bool set) {
  this_ptr->setDocumentMode(set);
}

void qt_widgets_c_QTabWidget_setElideMode(QTabWidget* this_ptr, const Qt::TextElideMode* arg1) {
  this_ptr->setElideMode(*arg1);
}

void qt_widgets_c_QTabWidget_setIconSize(QTabWidget* this_ptr, const QSize* size) {
  this_ptr->setIconSize(*size);
}

void qt_widgets_c_QTabWidget_setMovable(QTabWidget* this_ptr, bool movable) {
  this_ptr->setMovable(movable);
}

void qt_widgets_c_QTabWidget_setTabBarAutoHide(QTabWidget* this_ptr, bool enabled) {
  this_ptr->setTabBarAutoHide(enabled);
}

void qt_widgets_c_QTabWidget_setTabEnabled(QTabWidget* this_ptr, int index, bool arg2) {
  this_ptr->setTabEnabled(index, arg2);
}

void qt_widgets_c_QTabWidget_setTabIcon(QTabWidget* this_ptr, int index, const QIcon* icon) {
  this_ptr->setTabIcon(index, *icon);
}

void qt_widgets_c_QTabWidget_setTabPosition(QTabWidget* this_ptr, QTabWidget::TabPosition arg1) {
  this_ptr->setTabPosition(arg1);
}

void qt_widgets_c_QTabWidget_setTabShape(QTabWidget* this_ptr, QTabWidget::TabShape s) {
  this_ptr->setTabShape(s);
}

void qt_widgets_c_QTabWidget_setTabText(QTabWidget* this_ptr, int index, const QString* arg2) {
  this_ptr->setTabText(index, *arg2);
}

void qt_widgets_c_QTabWidget_setTabToolTip(QTabWidget* this_ptr, int index, const QString* tip) {
  this_ptr->setTabToolTip(index, *tip);
}

void qt_widgets_c_QTabWidget_setTabWhatsThis(QTabWidget* this_ptr, int index, const QString* text) {
  this_ptr->setTabWhatsThis(index, *text);
}

void qt_widgets_c_QTabWidget_setTabsClosable(QTabWidget* this_ptr, bool closeable) {
  this_ptr->setTabsClosable(closeable);
}

void qt_widgets_c_QTabWidget_setUsesScrollButtons(QTabWidget* this_ptr, bool useButtons) {
  this_ptr->setUsesScrollButtons(useButtons);
}

void qt_widgets_c_QTabWidget_sizeHint_to_output(const QTabWidget* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

QTabBar* qt_widgets_c_QTabWidget_tabBar(const QTabWidget* this_ptr) {
  return this_ptr->tabBar();
}

bool qt_widgets_c_QTabWidget_tabBarAutoHide(const QTabWidget* this_ptr) {
  return this_ptr->tabBarAutoHide();
}

void qt_widgets_c_QTabWidget_tabIcon_to_output(const QTabWidget* this_ptr, int index, QIcon* output) {
  new(output) QIcon(this_ptr->tabIcon(index));
}

QTabWidget::TabPosition qt_widgets_c_QTabWidget_tabPosition(const QTabWidget* this_ptr) {
  return this_ptr->tabPosition();
}

QTabWidget::TabShape qt_widgets_c_QTabWidget_tabShape(const QTabWidget* this_ptr) {
  return this_ptr->tabShape();
}

void qt_widgets_c_QTabWidget_tabText_to_output(const QTabWidget* this_ptr, int index, QString* output) {
  new(output) QString(this_ptr->tabText(index));
}

void qt_widgets_c_QTabWidget_tabToolTip_to_output(const QTabWidget* this_ptr, int index, QString* output) {
  new(output) QString(this_ptr->tabToolTip(index));
}

void qt_widgets_c_QTabWidget_tabWhatsThis_to_output(const QTabWidget* this_ptr, int index, QString* output) {
  new(output) QString(this_ptr->tabWhatsThis(index));
}

bool qt_widgets_c_QTabWidget_tabsClosable(const QTabWidget* this_ptr) {
  return this_ptr->tabsClosable();
}

void qt_widgets_c_QTabWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTabWidget::trUtf8(s, c, n));
}

void qt_widgets_c_QTabWidget_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTabWidget::tr(s, c, n));
}

bool qt_widgets_c_QTabWidget_usesScrollButtons(const QTabWidget* this_ptr) {
  return this_ptr->usesScrollButtons();
}

QWidget* qt_widgets_c_QTabWidget_widget(const QTabWidget* this_ptr, int index) {
  return this_ptr->widget(index);
}

