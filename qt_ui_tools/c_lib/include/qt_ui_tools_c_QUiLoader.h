#ifndef QT_UI_TOOLS_C_QUILOADER_H
#define QT_UI_TOOLS_C_QUILOADER_H

#include "qt_ui_tools_c_global.h"

extern "C" {

QT_UI_TOOLS_C_EXPORT QObject* qt_ui_tools_c_QUiLoader_G_static_cast_QObject_ptr(QUiLoader* ptr);
QT_UI_TOOLS_C_EXPORT QUiLoader* qt_ui_tools_c_QUiLoader_G_static_cast_QUiLoader_ptr(QObject* ptr);
QT_UI_TOOLS_C_EXPORT void qt_ui_tools_c_QUiLoader_addPluginPath(QUiLoader* this_ptr, const QString* path);
QT_UI_TOOLS_C_EXPORT void qt_ui_tools_c_QUiLoader_availableLayouts_to_output(const QUiLoader* this_ptr, QStringList* output);
QT_UI_TOOLS_C_EXPORT void qt_ui_tools_c_QUiLoader_availableWidgets_to_output(const QUiLoader* this_ptr, QStringList* output);
QT_UI_TOOLS_C_EXPORT void qt_ui_tools_c_QUiLoader_clearPluginPaths(QUiLoader* this_ptr);
QT_UI_TOOLS_C_EXPORT QActionGroup* qt_ui_tools_c_QUiLoader_createActionGroup_no_args(QUiLoader* this_ptr);
QT_UI_TOOLS_C_EXPORT QActionGroup* qt_ui_tools_c_QUiLoader_createActionGroup_parent(QUiLoader* this_ptr, QObject* parent);
QT_UI_TOOLS_C_EXPORT QActionGroup* qt_ui_tools_c_QUiLoader_createActionGroup_parent_name(QUiLoader* this_ptr, QObject* parent, const QString* name);
QT_UI_TOOLS_C_EXPORT QAction* qt_ui_tools_c_QUiLoader_createAction_no_args(QUiLoader* this_ptr);
QT_UI_TOOLS_C_EXPORT QAction* qt_ui_tools_c_QUiLoader_createAction_parent(QUiLoader* this_ptr, QObject* parent);
QT_UI_TOOLS_C_EXPORT QAction* qt_ui_tools_c_QUiLoader_createAction_parent_name(QUiLoader* this_ptr, QObject* parent, const QString* name);
QT_UI_TOOLS_C_EXPORT QLayout* qt_ui_tools_c_QUiLoader_createLayout_className(QUiLoader* this_ptr, const QString* className);
QT_UI_TOOLS_C_EXPORT QLayout* qt_ui_tools_c_QUiLoader_createLayout_className_parent(QUiLoader* this_ptr, const QString* className, QObject* parent);
QT_UI_TOOLS_C_EXPORT QLayout* qt_ui_tools_c_QUiLoader_createLayout_className_parent_name(QUiLoader* this_ptr, const QString* className, QObject* parent, const QString* name);
QT_UI_TOOLS_C_EXPORT QWidget* qt_ui_tools_c_QUiLoader_createWidget_className(QUiLoader* this_ptr, const QString* className);
QT_UI_TOOLS_C_EXPORT QWidget* qt_ui_tools_c_QUiLoader_createWidget_className_parent(QUiLoader* this_ptr, const QString* className, QWidget* parent);
QT_UI_TOOLS_C_EXPORT QWidget* qt_ui_tools_c_QUiLoader_createWidget_className_parent_name(QUiLoader* this_ptr, const QString* className, QWidget* parent, const QString* name);
QT_UI_TOOLS_C_EXPORT void qt_ui_tools_c_QUiLoader_delete(QUiLoader* this_ptr);
QT_UI_TOOLS_C_EXPORT void qt_ui_tools_c_QUiLoader_errorString_to_output(const QUiLoader* this_ptr, QString* output);
QT_UI_TOOLS_C_EXPORT bool qt_ui_tools_c_QUiLoader_isLanguageChangeEnabled(const QUiLoader* this_ptr);
QT_UI_TOOLS_C_EXPORT bool qt_ui_tools_c_QUiLoader_isTranslationEnabled(const QUiLoader* this_ptr);
QT_UI_TOOLS_C_EXPORT QWidget* qt_ui_tools_c_QUiLoader_load_device(QUiLoader* this_ptr, QIODevice* device);
QT_UI_TOOLS_C_EXPORT QWidget* qt_ui_tools_c_QUiLoader_load_device_parentWidget(QUiLoader* this_ptr, QIODevice* device, QWidget* parentWidget);
QT_UI_TOOLS_C_EXPORT const QMetaObject* qt_ui_tools_c_QUiLoader_metaObject(const QUiLoader* this_ptr);
QT_UI_TOOLS_C_EXPORT QUiLoader* qt_ui_tools_c_QUiLoader_new_no_args();
QT_UI_TOOLS_C_EXPORT QUiLoader* qt_ui_tools_c_QUiLoader_new_parent(QObject* parent);
QT_UI_TOOLS_C_EXPORT void qt_ui_tools_c_QUiLoader_pluginPaths_to_output(const QUiLoader* this_ptr, QStringList* output);
QT_UI_TOOLS_C_EXPORT int qt_ui_tools_c_QUiLoader_qt_metacall(QUiLoader* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_UI_TOOLS_C_EXPORT void* qt_ui_tools_c_QUiLoader_qt_metacast(QUiLoader* this_ptr, const char* arg1);
QT_UI_TOOLS_C_EXPORT void qt_ui_tools_c_QUiLoader_setLanguageChangeEnabled(QUiLoader* this_ptr, bool enabled);
QT_UI_TOOLS_C_EXPORT void qt_ui_tools_c_QUiLoader_setTranslationEnabled(QUiLoader* this_ptr, bool enabled);
QT_UI_TOOLS_C_EXPORT void qt_ui_tools_c_QUiLoader_setWorkingDirectory(QUiLoader* this_ptr, const QDir* dir);
QT_UI_TOOLS_C_EXPORT void qt_ui_tools_c_QUiLoader_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_UI_TOOLS_C_EXPORT void qt_ui_tools_c_QUiLoader_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_UI_TOOLS_C_EXPORT void qt_ui_tools_c_QUiLoader_workingDirectory_to_output(const QUiLoader* this_ptr, QDir* output);

} // extern "C"

#endif // QT_UI_TOOLS_C_QUILOADER_H
