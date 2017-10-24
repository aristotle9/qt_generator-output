#include "qt_widgets_c_QMdiArea.h"

QMdiArea* qt_widgets_c_QMdiArea_G_dynamic_cast_QMdiArea_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return dynamic_cast<QMdiArea*>(ptr);
}

QMdiArea* qt_widgets_c_QMdiArea_G_dynamic_cast_QMdiArea_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QMdiArea*>(ptr);
}

QMdiArea* qt_widgets_c_QMdiArea_G_dynamic_cast_QMdiArea_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QMdiArea*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QMdiArea_G_static_cast_QAbstractScrollArea_ptr(QMdiArea* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QFrame* qt_widgets_c_QMdiArea_G_static_cast_QFrame_ptr(QMdiArea* ptr) {
  return static_cast<QFrame*>(ptr);
}

QMdiArea* qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return static_cast<QMdiArea*>(ptr);
}

QMdiArea* qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QFrame(QFrame* ptr) {
  return static_cast<QMdiArea*>(ptr);
}

QMdiArea* qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QObject(QObject* ptr) {
  return static_cast<QMdiArea*>(ptr);
}

QMdiArea* qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QMdiArea*>(ptr);
}

QMdiArea* qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QWidget(QWidget* ptr) {
  return static_cast<QMdiArea*>(ptr);
}

QObject* qt_widgets_c_QMdiArea_G_static_cast_QObject_ptr(QMdiArea* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QMdiArea_G_static_cast_QPaintDevice_ptr(QMdiArea* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QMdiArea_G_static_cast_QWidget_ptr(QMdiArea* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QMdiArea_activateNextSubWindow(QMdiArea* this_ptr) {
  this_ptr->activateNextSubWindow();
}

void qt_widgets_c_QMdiArea_activatePreviousSubWindow(QMdiArea* this_ptr) {
  this_ptr->activatePreviousSubWindow();
}

QMdiArea::WindowOrder qt_widgets_c_QMdiArea_activationOrder(const QMdiArea* this_ptr) {
  return this_ptr->activationOrder();
}

QMdiSubWindow* qt_widgets_c_QMdiArea_activeSubWindow(const QMdiArea* this_ptr) {
  return this_ptr->activeSubWindow();
}

void qt_widgets_c_QMdiArea_background_to_output(const QMdiArea* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->background());
}

void qt_widgets_c_QMdiArea_cascadeSubWindows(QMdiArea* this_ptr) {
  this_ptr->cascadeSubWindows();
}

void qt_widgets_c_QMdiArea_closeActiveSubWindow(QMdiArea* this_ptr) {
  this_ptr->closeActiveSubWindow();
}

void qt_widgets_c_QMdiArea_closeAllSubWindows(QMdiArea* this_ptr) {
  this_ptr->closeAllSubWindows();
}

QMdiSubWindow* qt_widgets_c_QMdiArea_currentSubWindow(const QMdiArea* this_ptr) {
  return this_ptr->currentSubWindow();
}

void qt_widgets_c_QMdiArea_delete(QMdiArea* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QMdiArea_documentMode(const QMdiArea* this_ptr) {
  return this_ptr->documentMode();
}

const QMetaObject* qt_widgets_c_QMdiArea_metaObject(const QMdiArea* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QMdiArea_minimumSizeHint_to_output(const QMdiArea* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

QMdiArea* qt_widgets_c_QMdiArea_new_no_args() {
  return new QMdiArea();
}

QMdiArea* qt_widgets_c_QMdiArea_new_parent(QWidget* parent) {
  return new QMdiArea(parent);
}

int qt_widgets_c_QMdiArea_qt_metacall(QMdiArea* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QMdiArea_qt_metacast(QMdiArea* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QMdiArea_removeSubWindow(QMdiArea* this_ptr, QWidget* widget) {
  this_ptr->removeSubWindow(widget);
}

void qt_widgets_c_QMdiArea_setActivationOrder(QMdiArea* this_ptr, QMdiArea::WindowOrder order) {
  this_ptr->setActivationOrder(order);
}

void qt_widgets_c_QMdiArea_setActiveSubWindow(QMdiArea* this_ptr, QMdiSubWindow* window) {
  this_ptr->setActiveSubWindow(window);
}

void qt_widgets_c_QMdiArea_setBackground(QMdiArea* this_ptr, const QBrush* background) {
  this_ptr->setBackground(*background);
}

void qt_widgets_c_QMdiArea_setDocumentMode(QMdiArea* this_ptr, bool enabled) {
  this_ptr->setDocumentMode(enabled);
}

void qt_widgets_c_QMdiArea_setOption_option(QMdiArea* this_ptr, QMdiArea::AreaOption option) {
  this_ptr->setOption(option);
}

void qt_widgets_c_QMdiArea_setOption_option_on(QMdiArea* this_ptr, QMdiArea::AreaOption option, bool on) {
  this_ptr->setOption(option, on);
}

void qt_widgets_c_QMdiArea_setTabPosition(QMdiArea* this_ptr, const QTabWidget::TabPosition* position) {
  this_ptr->setTabPosition(*position);
}

void qt_widgets_c_QMdiArea_setTabShape(QMdiArea* this_ptr, const QTabWidget::TabShape* shape) {
  this_ptr->setTabShape(*shape);
}

void qt_widgets_c_QMdiArea_setTabsClosable(QMdiArea* this_ptr, bool closable) {
  this_ptr->setTabsClosable(closable);
}

void qt_widgets_c_QMdiArea_setTabsMovable(QMdiArea* this_ptr, bool movable) {
  this_ptr->setTabsMovable(movable);
}

void qt_widgets_c_QMdiArea_setViewMode(QMdiArea* this_ptr, QMdiArea::ViewMode mode) {
  this_ptr->setViewMode(mode);
}

void qt_widgets_c_QMdiArea_sizeHint_to_output(const QMdiArea* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QMdiArea_subWindowList_to_output_no_args(const QMdiArea* this_ptr, QList< QMdiSubWindow* >* output) {
  new(output) QList< QMdiSubWindow* >(this_ptr->subWindowList());
}

void qt_widgets_c_QMdiArea_subWindowList_to_output_order(const QMdiArea* this_ptr, QMdiArea::WindowOrder order, QList< QMdiSubWindow* >* output) {
  new(output) QList< QMdiSubWindow* >(this_ptr->subWindowList(order));
}

bool qt_widgets_c_QMdiArea_tabsClosable(const QMdiArea* this_ptr) {
  return this_ptr->tabsClosable();
}

bool qt_widgets_c_QMdiArea_tabsMovable(const QMdiArea* this_ptr) {
  return this_ptr->tabsMovable();
}

bool qt_widgets_c_QMdiArea_testOption(const QMdiArea* this_ptr, QMdiArea::AreaOption opton) {
  return this_ptr->testOption(opton);
}

void qt_widgets_c_QMdiArea_tileSubWindows(QMdiArea* this_ptr) {
  this_ptr->tileSubWindows();
}

void qt_widgets_c_QMdiArea_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMdiArea::trUtf8(s, c, n));
}

void qt_widgets_c_QMdiArea_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMdiArea::tr(s, c, n));
}

QMdiArea::ViewMode qt_widgets_c_QMdiArea_viewMode(const QMdiArea* this_ptr) {
  return this_ptr->viewMode();
}

