#ifndef QT_GUI_C_QTEXTDOCUMENT_H
#define QT_GUI_C_QTEXTDOCUMENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QTextCodec* qt_gui_c_QTextDocument_G_Qt_codecForHtml(const QByteArray* ba);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_G_Qt_convertFromPlainText_to_output_plain(const QString* plain, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_G_Qt_convertFromPlainText_to_output_plain_mode(const QString* plain, Qt::WhiteSpaceMode mode, QString* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextDocument_G_Qt_mightBeRichText(const QString* arg1);
QT_GUI_C_EXPORT QObject* qt_gui_c_QTextDocument_G_static_cast_QObject_ptr(QTextDocument* ptr);
QT_GUI_C_EXPORT QTextDocument* qt_gui_c_QTextDocument_G_static_cast_QTextDocument_ptr(QObject* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_addResource(QTextDocument* this_ptr, int type, const QUrl* name, const QVariant* resource);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_adjustSize(QTextDocument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_allFormats_to_output(const QTextDocument* this_ptr, QVector< QTextFormat >* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_appendUndoItem(QTextDocument* this_ptr, QAbstractUndoItem* arg1);
QT_GUI_C_EXPORT int qt_gui_c_QTextDocument_availableRedoSteps(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextDocument_availableUndoSteps(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_baseUrl_to_output(const QTextDocument* this_ptr, QUrl* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_begin_to_output(const QTextDocument* this_ptr, QTextBlock* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextDocument_blockCount(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_characterAt_to_output(const QTextDocument* this_ptr, int pos, QChar* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextDocument_characterCount(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_clear(QTextDocument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_clearUndoRedoStacks_historyToClear(QTextDocument* this_ptr, QTextDocument::Stacks historyToClear);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_clearUndoRedoStacks_no_args(QTextDocument* this_ptr);
QT_GUI_C_EXPORT QTextDocument* qt_gui_c_QTextDocument_clone_no_args(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT QTextDocument* qt_gui_c_QTextDocument_clone_parent(const QTextDocument* this_ptr, QObject* parent);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_defaultFont_to_output(const QTextDocument* this_ptr, QFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_defaultStyleSheet_to_output(const QTextDocument* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_defaultTextOption_to_output(const QTextDocument* this_ptr, QTextOption* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_delete(QTextDocument* this_ptr);
QT_GUI_C_EXPORT QAbstractTextDocumentLayout* qt_gui_c_QTextDocument_documentLayout(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextDocument_documentMargin(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_drawContents_painter(QTextDocument* this_ptr, QPainter* painter);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_drawContents_painter_rect(QTextDocument* this_ptr, QPainter* painter, const QRectF* rect);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_end_to_output(const QTextDocument* this_ptr, QTextBlock* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_findBlockByLineNumber_to_output(const QTextDocument* this_ptr, int blockNumber, QTextBlock* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_findBlockByNumber_to_output(const QTextDocument* this_ptr, int blockNumber, QTextBlock* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_findBlock_to_output(const QTextDocument* this_ptr, int pos, QTextBlock* output);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegExp(const QTextDocument* this_ptr, const QRegExp* expr);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegExp_QTextCursor(const QTextDocument* this_ptr, const QRegExp* expr, const QTextCursor* cursor);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegExp_QTextCursor_QFlags_QTextDocument_FindFlag(const QTextDocument* this_ptr, const QRegExp* expr, const QTextCursor* cursor, unsigned int options);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegExp_int(const QTextDocument* this_ptr, const QRegExp* expr, int from);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegExp_int_QFlags_QTextDocument_FindFlag(const QTextDocument* this_ptr, const QRegExp* expr, int from, unsigned int options);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegularExpression(const QTextDocument* this_ptr, const QRegularExpression* expr);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegularExpression_QTextCursor(const QTextDocument* this_ptr, const QRegularExpression* expr, const QTextCursor* cursor);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegularExpression_QTextCursor_QFlags_QTextDocument_FindFlag(const QTextDocument* this_ptr, const QRegularExpression* expr, const QTextCursor* cursor, unsigned int options);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegularExpression_int(const QTextDocument* this_ptr, const QRegularExpression* expr, int from);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegularExpression_int_QFlags_QTextDocument_FindFlag(const QTextDocument* this_ptr, const QRegularExpression* expr, int from, unsigned int options);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QString(const QTextDocument* this_ptr, const QString* subString);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QString_QTextCursor(const QTextDocument* this_ptr, const QString* subString, const QTextCursor* cursor);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QString_QTextCursor_QFlags_QTextDocument_FindFlag(const QTextDocument* this_ptr, const QString* subString, const QTextCursor* cursor, unsigned int options);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QString_int(const QTextDocument* this_ptr, const QString* subString, int from);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QString_int_QFlags_QTextDocument_FindFlag(const QTextDocument* this_ptr, const QString* subString, int from, unsigned int options);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_firstBlock_to_output(const QTextDocument* this_ptr, QTextBlock* output);
QT_GUI_C_EXPORT QTextFrame* qt_gui_c_QTextDocument_frameAt(const QTextDocument* this_ptr, int pos);
QT_GUI_C_EXPORT double qt_gui_c_QTextDocument_idealWidth(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextDocument_indentWidth(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextDocument_isEmpty(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextDocument_isModified(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextDocument_isRedoAvailable(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextDocument_isUndoAvailable(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextDocument_isUndoRedoEnabled(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_lastBlock_to_output(const QTextDocument* this_ptr, QTextBlock* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextDocument_lineCount(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_markContentsDirty(QTextDocument* this_ptr, int from, int length);
QT_GUI_C_EXPORT int qt_gui_c_QTextDocument_maximumBlockCount(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_metaInformation_to_output(const QTextDocument* this_ptr, QTextDocument::MetaInformation info, QString* output);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QTextDocument_metaObject(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT QTextDocument* qt_gui_c_QTextDocument_new_no_args();
QT_GUI_C_EXPORT QTextDocument* qt_gui_c_QTextDocument_new_parent(QObject* parent);
QT_GUI_C_EXPORT QTextDocument* qt_gui_c_QTextDocument_new_text(const QString* text);
QT_GUI_C_EXPORT QTextDocument* qt_gui_c_QTextDocument_new_text_parent(const QString* text, QObject* parent);
QT_GUI_C_EXPORT QTextObject* qt_gui_c_QTextDocument_object(const QTextDocument* this_ptr, int objectIndex);
QT_GUI_C_EXPORT QTextObject* qt_gui_c_QTextDocument_objectForFormat(const QTextDocument* this_ptr, const QTextFormat* arg1);
QT_GUI_C_EXPORT int qt_gui_c_QTextDocument_pageCount(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_pageSize_to_output(const QTextDocument* this_ptr, QSizeF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_print(const QTextDocument* this_ptr, QPagedPaintDevice* printer);
QT_GUI_C_EXPORT int qt_gui_c_QTextDocument_qt_metacall(QTextDocument* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QTextDocument_qt_metacast(QTextDocument* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_redo_cursor(QTextDocument* this_ptr, QTextCursor* cursor);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_redo_no_args(QTextDocument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_resource_to_output(const QTextDocument* this_ptr, int type, const QUrl* name, QVariant* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextDocument_revision(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT QTextFrame* qt_gui_c_QTextDocument_rootFrame(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setBaseUrl(QTextDocument* this_ptr, const QUrl* url);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setDefaultCursorMoveStyle(QTextDocument* this_ptr, const Qt::CursorMoveStyle* style);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setDefaultFont(QTextDocument* this_ptr, const QFont* font);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setDefaultStyleSheet(QTextDocument* this_ptr, const QString* sheet);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setDefaultTextOption(QTextDocument* this_ptr, const QTextOption* option);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setDocumentLayout(QTextDocument* this_ptr, QAbstractTextDocumentLayout* layout);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setDocumentMargin(QTextDocument* this_ptr, double margin);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setHtml(QTextDocument* this_ptr, const QString* html);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setIndentWidth(QTextDocument* this_ptr, double width);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setMaximumBlockCount(QTextDocument* this_ptr, int maximum);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setMetaInformation(QTextDocument* this_ptr, QTextDocument::MetaInformation info, const QString* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setModified_m(QTextDocument* this_ptr, bool m);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setModified_no_args(QTextDocument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setPageSize(QTextDocument* this_ptr, const QSizeF* size);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setPlainText(QTextDocument* this_ptr, const QString* text);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setTextWidth(QTextDocument* this_ptr, double width);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setUndoRedoEnabled(QTextDocument* this_ptr, bool enable);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_setUseDesignMetrics(QTextDocument* this_ptr, bool b);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_size_to_output(const QTextDocument* this_ptr, QSizeF* output);
QT_GUI_C_EXPORT double qt_gui_c_QTextDocument_textWidth(const QTextDocument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_toHtml_to_output_encoding(const QTextDocument* this_ptr, const QByteArray* encoding, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_toHtml_to_output_no_args(const QTextDocument* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_toPlainText_to_output(const QTextDocument* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_toRawText_to_output(const QTextDocument* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_undo_cursor(QTextDocument* this_ptr, QTextCursor* cursor);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocument_undo_no_args(QTextDocument* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextDocument_useDesignMetrics(const QTextDocument* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTEXTDOCUMENT_H