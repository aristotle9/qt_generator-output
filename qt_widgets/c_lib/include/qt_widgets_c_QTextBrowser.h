#ifndef QT_WIDGETS_C_QTEXTBROWSER_H
#define QT_WIDGETS_C_QTEXTBROWSER_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QTextBrowser* qt_widgets_c_QTextBrowser_G_dynamic_cast_QTextBrowser_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QTextBrowser* qt_widgets_c_QTextBrowser_G_dynamic_cast_QTextBrowser_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QTextBrowser* qt_widgets_c_QTextBrowser_G_dynamic_cast_QTextBrowser_ptr_QTextEdit(QTextEdit* ptr);
QT_WIDGETS_C_EXPORT QTextBrowser* qt_widgets_c_QTextBrowser_G_dynamic_cast_QTextBrowser_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractScrollArea* qt_widgets_c_QTextBrowser_G_static_cast_QAbstractScrollArea_ptr(QTextBrowser* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QTextBrowser_G_static_cast_QFrame_ptr(QTextBrowser* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QTextBrowser_G_static_cast_QObject_ptr(QTextBrowser* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QTextBrowser_G_static_cast_QPaintDevice_ptr(QTextBrowser* ptr);
QT_WIDGETS_C_EXPORT QTextBrowser* qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QTextBrowser* qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QTextBrowser* qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QTextBrowser* qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QTextBrowser* qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QTextEdit(QTextEdit* ptr);
QT_WIDGETS_C_EXPORT QTextBrowser* qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QTextEdit* qt_widgets_c_QTextBrowser_G_static_cast_QTextEdit_ptr(QTextBrowser* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QTextBrowser_G_static_cast_QWidget_ptr(QTextBrowser* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTextBrowser_backward(QTextBrowser* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTextBrowser_backwardHistoryCount(const QTextBrowser* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTextBrowser_clearHistory(QTextBrowser* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTextBrowser_delete(QTextBrowser* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTextBrowser_forward(QTextBrowser* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTextBrowser_forwardHistoryCount(const QTextBrowser* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTextBrowser_historyTitle_to_output(const QTextBrowser* this_ptr, int arg1, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTextBrowser_historyUrl_to_output(const QTextBrowser* this_ptr, int arg1, QUrl* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTextBrowser_home(QTextBrowser* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTextBrowser_isBackwardAvailable(const QTextBrowser* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTextBrowser_isForwardAvailable(const QTextBrowser* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTextBrowser_loadResource_to_output(QTextBrowser* this_ptr, int type, const QUrl* name, QVariant* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QTextBrowser_metaObject(const QTextBrowser* this_ptr);
QT_WIDGETS_C_EXPORT QTextBrowser* qt_widgets_c_QTextBrowser_new_no_args();
QT_WIDGETS_C_EXPORT QTextBrowser* qt_widgets_c_QTextBrowser_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTextBrowser_openExternalLinks(const QTextBrowser* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTextBrowser_openLinks(const QTextBrowser* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTextBrowser_qt_metacall(QTextBrowser* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QTextBrowser_qt_metacast(QTextBrowser* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTextBrowser_reload(QTextBrowser* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTextBrowser_searchPaths_to_output(const QTextBrowser* this_ptr, QStringList* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTextBrowser_setOpenExternalLinks(QTextBrowser* this_ptr, bool open);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTextBrowser_setOpenLinks(QTextBrowser* this_ptr, bool open);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTextBrowser_setSearchPaths(QTextBrowser* this_ptr, const QStringList* paths);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTextBrowser_setSource(QTextBrowser* this_ptr, const QUrl* name);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTextBrowser_source_to_output(const QTextBrowser* this_ptr, QUrl* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTextBrowser_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTextBrowser_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QTEXTBROWSER_H
