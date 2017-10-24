#ifndef QT_WIDGETS_C_QPLAINTEXTEDIT_H
#define QT_WIDGETS_C_QPLAINTEXTEDIT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QPlainTextEdit* qt_widgets_c_QPlainTextEdit_G_dynamic_cast_QPlainTextEdit_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QPlainTextEdit* qt_widgets_c_QPlainTextEdit_G_dynamic_cast_QPlainTextEdit_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QPlainTextEdit* qt_widgets_c_QPlainTextEdit_G_dynamic_cast_QPlainTextEdit_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractScrollArea* qt_widgets_c_QPlainTextEdit_G_static_cast_QAbstractScrollArea_ptr(QPlainTextEdit* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QPlainTextEdit_G_static_cast_QFrame_ptr(QPlainTextEdit* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QPlainTextEdit_G_static_cast_QObject_ptr(QPlainTextEdit* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QPlainTextEdit_G_static_cast_QPaintDevice_ptr(QPlainTextEdit* ptr);
QT_WIDGETS_C_EXPORT QPlainTextEdit* qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QPlainTextEdit* qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QPlainTextEdit* qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QPlainTextEdit* qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QPlainTextEdit* qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QPlainTextEdit_G_static_cast_QWidget_ptr(QPlainTextEdit* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_anchorAt_to_output(const QPlainTextEdit* this_ptr, const QPoint* pos, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_appendHtml(QPlainTextEdit* this_ptr, const QString* html);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_appendPlainText(QPlainTextEdit* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QPlainTextEdit_backgroundVisible(const QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QPlainTextEdit_blockCount(const QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QPlainTextEdit_canPaste(const QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_centerCursor(QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QPlainTextEdit_centerOnScroll(const QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_clear(QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_copy(QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QPlainTextEdit_createStandardContextMenu_no_args(QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QPlainTextEdit_createStandardContextMenu_position(QPlainTextEdit* this_ptr, const QPoint* position);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_currentCharFormat_to_output(const QPlainTextEdit* this_ptr, QTextCharFormat* output);
QT_WIDGETS_C_EXPORT QTextCursor* qt_widgets_c_QPlainTextEdit_cursorForPosition_as_ptr(const QPlainTextEdit* this_ptr, const QPoint* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_cursorRect_to_output_cursor(const QPlainTextEdit* this_ptr, const QTextCursor* cursor, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_cursorRect_to_output_no_args(const QPlainTextEdit* this_ptr, QRect* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QPlainTextEdit_cursorWidth(const QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_cut(QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_delete(QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT QTextDocument* qt_widgets_c_QPlainTextEdit_document(const QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_documentTitle_to_output(const QPlainTextEdit* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_ensureCursorVisible(QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_extraSelections_to_output(const QPlainTextEdit* this_ptr, QList< QTextEdit::ExtraSelection >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_inputMethodQuery_to_output_property(const QPlainTextEdit* this_ptr, const Qt::InputMethodQuery* property, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_inputMethodQuery_to_output_query_argument(const QPlainTextEdit* this_ptr, const Qt::InputMethodQuery* query, const QVariant* argument, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_insertPlainText(QPlainTextEdit* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QPlainTextEdit_isReadOnly(const QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QPlainTextEdit_isUndoRedoEnabled(const QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT QPlainTextEdit::LineWrapMode qt_widgets_c_QPlainTextEdit_lineWrapMode(const QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_loadResource_to_output(QPlainTextEdit* this_ptr, int type, const QUrl* name, QVariant* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QPlainTextEdit_maximumBlockCount(const QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_mergeCurrentCharFormat(QPlainTextEdit* this_ptr, const QTextCharFormat* modifier);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QPlainTextEdit_metaObject(const QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_moveCursor_operation(QPlainTextEdit* this_ptr, const QTextCursor::MoveOperation* operation);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_moveCursor_operation_mode(QPlainTextEdit* this_ptr, const QTextCursor::MoveOperation* operation, const QTextCursor::MoveMode* mode);
QT_WIDGETS_C_EXPORT QPlainTextEdit* qt_widgets_c_QPlainTextEdit_new_no_args();
QT_WIDGETS_C_EXPORT QPlainTextEdit* qt_widgets_c_QPlainTextEdit_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT QPlainTextEdit* qt_widgets_c_QPlainTextEdit_new_text(const QString* text);
QT_WIDGETS_C_EXPORT QPlainTextEdit* qt_widgets_c_QPlainTextEdit_new_text_parent(const QString* text, QWidget* parent);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QPlainTextEdit_overwriteMode(const QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_paste(QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_placeholderText_to_output(const QPlainTextEdit* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_print(const QPlainTextEdit* this_ptr, QPagedPaintDevice* printer);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QPlainTextEdit_qt_metacall(QPlainTextEdit* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QPlainTextEdit_qt_metacast(QPlainTextEdit* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_redo(QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_selectAll(QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setBackgroundVisible(QPlainTextEdit* this_ptr, bool visible);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setCenterOnScroll(QPlainTextEdit* this_ptr, bool enabled);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setCurrentCharFormat(QPlainTextEdit* this_ptr, const QTextCharFormat* format);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setCursorWidth(QPlainTextEdit* this_ptr, int width);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setDocument(QPlainTextEdit* this_ptr, QTextDocument* document);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setDocumentTitle(QPlainTextEdit* this_ptr, const QString* title);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setExtraSelections(QPlainTextEdit* this_ptr, const QList< QTextEdit::ExtraSelection >* selections);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setLineWrapMode(QPlainTextEdit* this_ptr, QPlainTextEdit::LineWrapMode mode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setMaximumBlockCount(QPlainTextEdit* this_ptr, int maximum);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setOverwriteMode(QPlainTextEdit* this_ptr, bool overwrite);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setPlaceholderText(QPlainTextEdit* this_ptr, const QString* placeholderText);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setPlainText(QPlainTextEdit* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setReadOnly(QPlainTextEdit* this_ptr, bool ro);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setTabChangesFocus(QPlainTextEdit* this_ptr, bool b);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setTabStopWidth(QPlainTextEdit* this_ptr, int width);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setTextCursor(QPlainTextEdit* this_ptr, const QTextCursor* cursor);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setUndoRedoEnabled(QPlainTextEdit* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_setWordWrapMode(QPlainTextEdit* this_ptr, const QTextOption::WrapMode* policy);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QPlainTextEdit_tabChangesFocus(const QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QPlainTextEdit_tabStopWidth(const QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT QTextCursor* qt_widgets_c_QPlainTextEdit_textCursor_as_ptr(const QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_toPlainText_to_output(const QPlainTextEdit* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_undo(QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_zoomIn_no_args(QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_zoomIn_range(QPlainTextEdit* this_ptr, int range);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_zoomOut_no_args(QPlainTextEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPlainTextEdit_zoomOut_range(QPlainTextEdit* this_ptr, int range);

} // extern "C"

#endif // QT_WIDGETS_C_QPLAINTEXTEDIT_H
