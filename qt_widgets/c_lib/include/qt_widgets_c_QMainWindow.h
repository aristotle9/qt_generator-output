#ifndef QT_WIDGETS_C_QMAINWINDOW_H
#define QT_WIDGETS_C_QMAINWINDOW_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QMainWindow* qt_widgets_c_QMainWindow_G_dynamic_cast_QMainWindow_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QMainWindow* qt_widgets_c_QMainWindow_G_static_cast_QMainWindow_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QMainWindow* qt_widgets_c_QMainWindow_G_static_cast_QMainWindow_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QMainWindow* qt_widgets_c_QMainWindow_G_static_cast_QMainWindow_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QMainWindow_G_static_cast_QObject_ptr(QMainWindow* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QMainWindow_G_static_cast_QPaintDevice_ptr(QMainWindow* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QMainWindow_G_static_cast_QWidget_ptr(QMainWindow* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_addDockWidget_area_dockwidget(QMainWindow* this_ptr, const Qt::DockWidgetArea* area, QDockWidget* dockwidget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_addDockWidget_area_dockwidget_orientation(QMainWindow* this_ptr, const Qt::DockWidgetArea* area, QDockWidget* dockwidget, const Qt::Orientation* orientation);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_addToolBarBreak_area(QMainWindow* this_ptr, const Qt::ToolBarArea* area);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_addToolBarBreak_no_args(QMainWindow* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_addToolBar_area_toolbar(QMainWindow* this_ptr, const Qt::ToolBarArea* area, QToolBar* toolbar);
QT_WIDGETS_C_EXPORT QToolBar* qt_widgets_c_QMainWindow_addToolBar_title(QMainWindow* this_ptr, const QString* title);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_addToolBar_toolbar(QMainWindow* this_ptr, QToolBar* toolbar);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QMainWindow_centralWidget(const QMainWindow* this_ptr);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QMainWindow_createPopupMenu(QMainWindow* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_delete(QMainWindow* this_ptr);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QMainWindow_dockOptions(const QMainWindow* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMainWindow_documentMode(const QMainWindow* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_iconSize_to_output(const QMainWindow* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_insertToolBar(QMainWindow* this_ptr, QToolBar* before, QToolBar* toolbar);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_insertToolBarBreak(QMainWindow* this_ptr, QToolBar* before);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMainWindow_isAnimated(const QMainWindow* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMainWindow_isDockNestingEnabled(const QMainWindow* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMainWindow_isSeparator(const QMainWindow* this_ptr, const QPoint* pos);
QT_WIDGETS_C_EXPORT QMenuBar* qt_widgets_c_QMainWindow_menuBar(const QMainWindow* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QMainWindow_menuWidget(const QMainWindow* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QMainWindow_metaObject(const QMainWindow* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMainWindow_qt_metacall(QMainWindow* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QMainWindow_qt_metacast(QMainWindow* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_removeDockWidget(QMainWindow* this_ptr, QDockWidget* dockwidget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_removeToolBar(QMainWindow* this_ptr, QToolBar* toolbar);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_removeToolBarBreak(QMainWindow* this_ptr, QToolBar* before);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_resizeDocks(QMainWindow* this_ptr, const QList< QDockWidget* >* docks, const QList< int >* sizes, const Qt::Orientation* orientation);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMainWindow_restoreDockWidget(QMainWindow* this_ptr, QDockWidget* dockwidget);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMainWindow_restoreState_state(QMainWindow* this_ptr, const QByteArray* state);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMainWindow_restoreState_state_version(QMainWindow* this_ptr, const QByteArray* state, int version);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_saveState_to_output_no_args(const QMainWindow* this_ptr, QByteArray* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_saveState_to_output_version(const QMainWindow* this_ptr, int version, QByteArray* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_setAnimated(QMainWindow* this_ptr, bool enabled);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_setCentralWidget(QMainWindow* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_setCorner(QMainWindow* this_ptr, const Qt::Corner* corner, const Qt::DockWidgetArea* area);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_setDockNestingEnabled(QMainWindow* this_ptr, bool enabled);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_setDockOptions(QMainWindow* this_ptr, unsigned int options);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_setDocumentMode(QMainWindow* this_ptr, bool enabled);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_setIconSize(QMainWindow* this_ptr, const QSize* iconSize);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_setMenuBar(QMainWindow* this_ptr, QMenuBar* menubar);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_setMenuWidget(QMainWindow* this_ptr, QWidget* menubar);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_setStatusBar(QMainWindow* this_ptr, QStatusBar* statusbar);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_setTabShape(QMainWindow* this_ptr, const QTabWidget::TabShape* tabShape);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_setToolButtonStyle(QMainWindow* this_ptr, const Qt::ToolButtonStyle* toolButtonStyle);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_setUnifiedTitleAndToolBarOnMac(QMainWindow* this_ptr, bool set);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_splitDockWidget(QMainWindow* this_ptr, QDockWidget* after, QDockWidget* dockwidget, const Qt::Orientation* orientation);
QT_WIDGETS_C_EXPORT QStatusBar* qt_widgets_c_QMainWindow_statusBar(const QMainWindow* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_tabifiedDockWidgets_to_output(const QMainWindow* this_ptr, QDockWidget* dockwidget, QList< QDockWidget* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_tabifyDockWidget(QMainWindow* this_ptr, QDockWidget* first, QDockWidget* second);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QMainWindow_takeCentralWidget(QMainWindow* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMainWindow_toolBarBreak(const QMainWindow* this_ptr, QToolBar* toolbar);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMainWindow_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMainWindow_unifiedTitleAndToolBarOnMac(const QMainWindow* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QMAINWINDOW_H
