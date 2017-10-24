#ifndef QT_WIDGETS_C_QCOMPLETER_H
#define QT_WIDGETS_C_QCOMPLETER_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QCompleter* qt_widgets_c_QCompleter_G_static_cast_QCompleter_ptr(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QCompleter_G_static_cast_QObject_ptr(QCompleter* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_complete_no_args(QCompleter* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_complete_rect(QCompleter* this_ptr, const QRect* rect);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCompleter_completionColumn(const QCompleter* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCompleter_completionCount(const QCompleter* this_ptr);
QT_WIDGETS_C_EXPORT QCompleter::CompletionMode qt_widgets_c_QCompleter_completionMode(const QCompleter* this_ptr);
QT_WIDGETS_C_EXPORT QAbstractItemModel* qt_widgets_c_QCompleter_completionModel(const QCompleter* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_completionPrefix_to_output(const QCompleter* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCompleter_completionRole(const QCompleter* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_currentCompletion_to_output(const QCompleter* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_currentIndex_to_output(const QCompleter* this_ptr, QModelIndex* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCompleter_currentRow(const QCompleter* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_delete(QCompleter* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCompleter_maxVisibleItems(const QCompleter* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QCompleter_metaObject(const QCompleter* this_ptr);
QT_WIDGETS_C_EXPORT QAbstractItemModel* qt_widgets_c_QCompleter_model(const QCompleter* this_ptr);
QT_WIDGETS_C_EXPORT QCompleter::ModelSorting qt_widgets_c_QCompleter_modelSorting(const QCompleter* this_ptr);
QT_WIDGETS_C_EXPORT QCompleter* qt_widgets_c_QCompleter_new_completions(const QStringList* completions);
QT_WIDGETS_C_EXPORT QCompleter* qt_widgets_c_QCompleter_new_completions_parent(const QStringList* completions, QObject* parent);
QT_WIDGETS_C_EXPORT QCompleter* qt_widgets_c_QCompleter_new_model(QAbstractItemModel* model);
QT_WIDGETS_C_EXPORT QCompleter* qt_widgets_c_QCompleter_new_model_parent(QAbstractItemModel* model, QObject* parent);
QT_WIDGETS_C_EXPORT QCompleter* qt_widgets_c_QCompleter_new_no_args();
QT_WIDGETS_C_EXPORT QCompleter* qt_widgets_c_QCompleter_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_pathFromIndex_to_output(const QCompleter* this_ptr, const QModelIndex* index, QString* output);
QT_WIDGETS_C_EXPORT QAbstractItemView* qt_widgets_c_QCompleter_popup(const QCompleter* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCompleter_qt_metacall(QCompleter* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QCompleter_qt_metacast(QCompleter* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_setCaseSensitivity(QCompleter* this_ptr, const Qt::CaseSensitivity* caseSensitivity);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_setCompletionColumn(QCompleter* this_ptr, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_setCompletionMode(QCompleter* this_ptr, QCompleter::CompletionMode mode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_setCompletionPrefix(QCompleter* this_ptr, const QString* prefix);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_setCompletionRole(QCompleter* this_ptr, int role);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QCompleter_setCurrentRow(QCompleter* this_ptr, int row);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_setMaxVisibleItems(QCompleter* this_ptr, int maxItems);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_setModel(QCompleter* this_ptr, QAbstractItemModel* c);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_setModelSorting(QCompleter* this_ptr, QCompleter::ModelSorting sorting);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_setPopup(QCompleter* this_ptr, QAbstractItemView* popup);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_setWidget(QCompleter* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_setWrapAround(QCompleter* this_ptr, bool wrap);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_splitPath_to_output(const QCompleter* this_ptr, const QString* path, QStringList* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCompleter_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QCompleter_widget(const QCompleter* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QCompleter_wrapAround(const QCompleter* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QCOMPLETER_H
