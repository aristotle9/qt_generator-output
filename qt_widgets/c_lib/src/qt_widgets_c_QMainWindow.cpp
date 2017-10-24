#include "qt_widgets_c_QMainWindow.h"

QMainWindow* qt_widgets_c_QMainWindow_G_dynamic_cast_QMainWindow_ptr(QWidget* ptr) {
  return dynamic_cast<QMainWindow*>(ptr);
}

QMainWindow* qt_widgets_c_QMainWindow_G_static_cast_QMainWindow_ptr_QObject(QObject* ptr) {
  return static_cast<QMainWindow*>(ptr);
}

QMainWindow* qt_widgets_c_QMainWindow_G_static_cast_QMainWindow_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QMainWindow*>(ptr);
}

QMainWindow* qt_widgets_c_QMainWindow_G_static_cast_QMainWindow_ptr_QWidget(QWidget* ptr) {
  return static_cast<QMainWindow*>(ptr);
}

QObject* qt_widgets_c_QMainWindow_G_static_cast_QObject_ptr(QMainWindow* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QMainWindow_G_static_cast_QPaintDevice_ptr(QMainWindow* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QMainWindow_G_static_cast_QWidget_ptr(QMainWindow* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QMainWindow_addDockWidget_area_dockwidget(QMainWindow* this_ptr, const Qt::DockWidgetArea* area, QDockWidget* dockwidget) {
  this_ptr->addDockWidget(*area, dockwidget);
}

void qt_widgets_c_QMainWindow_addDockWidget_area_dockwidget_orientation(QMainWindow* this_ptr, const Qt::DockWidgetArea* area, QDockWidget* dockwidget, const Qt::Orientation* orientation) {
  this_ptr->addDockWidget(*area, dockwidget, *orientation);
}

void qt_widgets_c_QMainWindow_addToolBarBreak_area(QMainWindow* this_ptr, const Qt::ToolBarArea* area) {
  this_ptr->addToolBarBreak(*area);
}

void qt_widgets_c_QMainWindow_addToolBarBreak_no_args(QMainWindow* this_ptr) {
  this_ptr->addToolBarBreak();
}

void qt_widgets_c_QMainWindow_addToolBar_area_toolbar(QMainWindow* this_ptr, const Qt::ToolBarArea* area, QToolBar* toolbar) {
  this_ptr->addToolBar(*area, toolbar);
}

QToolBar* qt_widgets_c_QMainWindow_addToolBar_title(QMainWindow* this_ptr, const QString* title) {
  return this_ptr->addToolBar(*title);
}

void qt_widgets_c_QMainWindow_addToolBar_toolbar(QMainWindow* this_ptr, QToolBar* toolbar) {
  this_ptr->addToolBar(toolbar);
}

QWidget* qt_widgets_c_QMainWindow_centralWidget(const QMainWindow* this_ptr) {
  return this_ptr->centralWidget();
}

QMenu* qt_widgets_c_QMainWindow_createPopupMenu(QMainWindow* this_ptr) {
  return this_ptr->createPopupMenu();
}

void qt_widgets_c_QMainWindow_delete(QMainWindow* this_ptr) {
  delete this_ptr;
}

unsigned int qt_widgets_c_QMainWindow_dockOptions(const QMainWindow* this_ptr) {
  return uint(this_ptr->dockOptions());
}

bool qt_widgets_c_QMainWindow_documentMode(const QMainWindow* this_ptr) {
  return this_ptr->documentMode();
}

void qt_widgets_c_QMainWindow_iconSize_to_output(const QMainWindow* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->iconSize());
}

void qt_widgets_c_QMainWindow_insertToolBar(QMainWindow* this_ptr, QToolBar* before, QToolBar* toolbar) {
  this_ptr->insertToolBar(before, toolbar);
}

void qt_widgets_c_QMainWindow_insertToolBarBreak(QMainWindow* this_ptr, QToolBar* before) {
  this_ptr->insertToolBarBreak(before);
}

bool qt_widgets_c_QMainWindow_isAnimated(const QMainWindow* this_ptr) {
  return this_ptr->isAnimated();
}

bool qt_widgets_c_QMainWindow_isDockNestingEnabled(const QMainWindow* this_ptr) {
  return this_ptr->isDockNestingEnabled();
}

bool qt_widgets_c_QMainWindow_isSeparator(const QMainWindow* this_ptr, const QPoint* pos) {
  return this_ptr->isSeparator(*pos);
}

QMenuBar* qt_widgets_c_QMainWindow_menuBar(const QMainWindow* this_ptr) {
  return this_ptr->menuBar();
}

QWidget* qt_widgets_c_QMainWindow_menuWidget(const QMainWindow* this_ptr) {
  return this_ptr->menuWidget();
}

const QMetaObject* qt_widgets_c_QMainWindow_metaObject(const QMainWindow* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QMainWindow_qt_metacall(QMainWindow* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QMainWindow_qt_metacast(QMainWindow* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QMainWindow_removeDockWidget(QMainWindow* this_ptr, QDockWidget* dockwidget) {
  this_ptr->removeDockWidget(dockwidget);
}

void qt_widgets_c_QMainWindow_removeToolBar(QMainWindow* this_ptr, QToolBar* toolbar) {
  this_ptr->removeToolBar(toolbar);
}

void qt_widgets_c_QMainWindow_removeToolBarBreak(QMainWindow* this_ptr, QToolBar* before) {
  this_ptr->removeToolBarBreak(before);
}

void qt_widgets_c_QMainWindow_resizeDocks(QMainWindow* this_ptr, const QList< QDockWidget* >* docks, const QList< int >* sizes, const Qt::Orientation* orientation) {
  this_ptr->resizeDocks(*docks, *sizes, *orientation);
}

bool qt_widgets_c_QMainWindow_restoreDockWidget(QMainWindow* this_ptr, QDockWidget* dockwidget) {
  return this_ptr->restoreDockWidget(dockwidget);
}

bool qt_widgets_c_QMainWindow_restoreState_state(QMainWindow* this_ptr, const QByteArray* state) {
  return this_ptr->restoreState(*state);
}

bool qt_widgets_c_QMainWindow_restoreState_state_version(QMainWindow* this_ptr, const QByteArray* state, int version) {
  return this_ptr->restoreState(*state, version);
}

void qt_widgets_c_QMainWindow_saveState_to_output_no_args(const QMainWindow* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->saveState());
}

void qt_widgets_c_QMainWindow_saveState_to_output_version(const QMainWindow* this_ptr, int version, QByteArray* output) {
  new(output) QByteArray(this_ptr->saveState(version));
}

void qt_widgets_c_QMainWindow_setAnimated(QMainWindow* this_ptr, bool enabled) {
  this_ptr->setAnimated(enabled);
}

void qt_widgets_c_QMainWindow_setCentralWidget(QMainWindow* this_ptr, QWidget* widget) {
  this_ptr->setCentralWidget(widget);
}

void qt_widgets_c_QMainWindow_setCorner(QMainWindow* this_ptr, const Qt::Corner* corner, const Qt::DockWidgetArea* area) {
  this_ptr->setCorner(*corner, *area);
}

void qt_widgets_c_QMainWindow_setDockNestingEnabled(QMainWindow* this_ptr, bool enabled) {
  this_ptr->setDockNestingEnabled(enabled);
}

void qt_widgets_c_QMainWindow_setDockOptions(QMainWindow* this_ptr, unsigned int options) {
  this_ptr->setDockOptions(QFlags< QMainWindow::DockOption >(options));
}

void qt_widgets_c_QMainWindow_setDocumentMode(QMainWindow* this_ptr, bool enabled) {
  this_ptr->setDocumentMode(enabled);
}

void qt_widgets_c_QMainWindow_setIconSize(QMainWindow* this_ptr, const QSize* iconSize) {
  this_ptr->setIconSize(*iconSize);
}

void qt_widgets_c_QMainWindow_setMenuBar(QMainWindow* this_ptr, QMenuBar* menubar) {
  this_ptr->setMenuBar(menubar);
}

void qt_widgets_c_QMainWindow_setMenuWidget(QMainWindow* this_ptr, QWidget* menubar) {
  this_ptr->setMenuWidget(menubar);
}

void qt_widgets_c_QMainWindow_setStatusBar(QMainWindow* this_ptr, QStatusBar* statusbar) {
  this_ptr->setStatusBar(statusbar);
}

void qt_widgets_c_QMainWindow_setTabShape(QMainWindow* this_ptr, const QTabWidget::TabShape* tabShape) {
  this_ptr->setTabShape(*tabShape);
}

void qt_widgets_c_QMainWindow_setToolButtonStyle(QMainWindow* this_ptr, const Qt::ToolButtonStyle* toolButtonStyle) {
  this_ptr->setToolButtonStyle(*toolButtonStyle);
}

void qt_widgets_c_QMainWindow_setUnifiedTitleAndToolBarOnMac(QMainWindow* this_ptr, bool set) {
  this_ptr->setUnifiedTitleAndToolBarOnMac(set);
}

void qt_widgets_c_QMainWindow_splitDockWidget(QMainWindow* this_ptr, QDockWidget* after, QDockWidget* dockwidget, const Qt::Orientation* orientation) {
  this_ptr->splitDockWidget(after, dockwidget, *orientation);
}

QStatusBar* qt_widgets_c_QMainWindow_statusBar(const QMainWindow* this_ptr) {
  return this_ptr->statusBar();
}

void qt_widgets_c_QMainWindow_tabifiedDockWidgets_to_output(const QMainWindow* this_ptr, QDockWidget* dockwidget, QList< QDockWidget* >* output) {
  new(output) QList< QDockWidget* >(this_ptr->tabifiedDockWidgets(dockwidget));
}

void qt_widgets_c_QMainWindow_tabifyDockWidget(QMainWindow* this_ptr, QDockWidget* first, QDockWidget* second) {
  this_ptr->tabifyDockWidget(first, second);
}

QWidget* qt_widgets_c_QMainWindow_takeCentralWidget(QMainWindow* this_ptr) {
  return this_ptr->takeCentralWidget();
}

bool qt_widgets_c_QMainWindow_toolBarBreak(const QMainWindow* this_ptr, QToolBar* toolbar) {
  return this_ptr->toolBarBreak(toolbar);
}

void qt_widgets_c_QMainWindow_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMainWindow::trUtf8(s, c, n));
}

void qt_widgets_c_QMainWindow_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMainWindow::tr(s, c, n));
}

bool qt_widgets_c_QMainWindow_unifiedTitleAndToolBarOnMac(const QMainWindow* this_ptr) {
  return this_ptr->unifiedTitleAndToolBarOnMac();
}

