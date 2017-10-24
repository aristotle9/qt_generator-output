#include "qt_gui_c_QTextDocument.h"

QTextCodec* qt_gui_c_QTextDocument_G_Qt_codecForHtml(const QByteArray* ba) {
  return Qt::codecForHtml(*ba);
}

void qt_gui_c_QTextDocument_G_Qt_convertFromPlainText_to_output_plain(const QString* plain, QString* output) {
  new(output) QString(Qt::convertFromPlainText(*plain));
}

void qt_gui_c_QTextDocument_G_Qt_convertFromPlainText_to_output_plain_mode(const QString* plain, Qt::WhiteSpaceMode mode, QString* output) {
  new(output) QString(Qt::convertFromPlainText(*plain, mode));
}

bool qt_gui_c_QTextDocument_G_Qt_mightBeRichText(const QString* arg1) {
  return Qt::mightBeRichText(*arg1);
}

QObject* qt_gui_c_QTextDocument_G_static_cast_QObject_ptr(QTextDocument* ptr) {
  return static_cast<QObject*>(ptr);
}

QTextDocument* qt_gui_c_QTextDocument_G_static_cast_QTextDocument_ptr(QObject* ptr) {
  return static_cast<QTextDocument*>(ptr);
}

void qt_gui_c_QTextDocument_addResource(QTextDocument* this_ptr, int type, const QUrl* name, const QVariant* resource) {
  this_ptr->addResource(type, *name, *resource);
}

void qt_gui_c_QTextDocument_adjustSize(QTextDocument* this_ptr) {
  this_ptr->adjustSize();
}

void qt_gui_c_QTextDocument_allFormats_to_output(const QTextDocument* this_ptr, QVector< QTextFormat >* output) {
  new(output) QVector< QTextFormat >(this_ptr->allFormats());
}

void qt_gui_c_QTextDocument_appendUndoItem(QTextDocument* this_ptr, QAbstractUndoItem* arg1) {
  this_ptr->appendUndoItem(arg1);
}

int qt_gui_c_QTextDocument_availableRedoSteps(const QTextDocument* this_ptr) {
  return this_ptr->availableRedoSteps();
}

int qt_gui_c_QTextDocument_availableUndoSteps(const QTextDocument* this_ptr) {
  return this_ptr->availableUndoSteps();
}

void qt_gui_c_QTextDocument_baseUrl_to_output(const QTextDocument* this_ptr, QUrl* output) {
  new(output) QUrl(this_ptr->baseUrl());
}

void qt_gui_c_QTextDocument_begin_to_output(const QTextDocument* this_ptr, QTextBlock* output) {
  new(output) QTextBlock(this_ptr->begin());
}

int qt_gui_c_QTextDocument_blockCount(const QTextDocument* this_ptr) {
  return this_ptr->blockCount();
}

void qt_gui_c_QTextDocument_characterAt_to_output(const QTextDocument* this_ptr, int pos, QChar* output) {
  new(output) QChar(this_ptr->characterAt(pos));
}

int qt_gui_c_QTextDocument_characterCount(const QTextDocument* this_ptr) {
  return this_ptr->characterCount();
}

void qt_gui_c_QTextDocument_clear(QTextDocument* this_ptr) {
  this_ptr->clear();
}

void qt_gui_c_QTextDocument_clearUndoRedoStacks_historyToClear(QTextDocument* this_ptr, QTextDocument::Stacks historyToClear) {
  this_ptr->clearUndoRedoStacks(historyToClear);
}

void qt_gui_c_QTextDocument_clearUndoRedoStacks_no_args(QTextDocument* this_ptr) {
  this_ptr->clearUndoRedoStacks();
}

QTextDocument* qt_gui_c_QTextDocument_clone_no_args(const QTextDocument* this_ptr) {
  return this_ptr->clone();
}

QTextDocument* qt_gui_c_QTextDocument_clone_parent(const QTextDocument* this_ptr, QObject* parent) {
  return this_ptr->clone(parent);
}

void qt_gui_c_QTextDocument_defaultFont_to_output(const QTextDocument* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->defaultFont());
}

void qt_gui_c_QTextDocument_defaultStyleSheet_to_output(const QTextDocument* this_ptr, QString* output) {
  new(output) QString(this_ptr->defaultStyleSheet());
}

void qt_gui_c_QTextDocument_defaultTextOption_to_output(const QTextDocument* this_ptr, QTextOption* output) {
  new(output) QTextOption(this_ptr->defaultTextOption());
}

void qt_gui_c_QTextDocument_delete(QTextDocument* this_ptr) {
  delete this_ptr;
}

QAbstractTextDocumentLayout* qt_gui_c_QTextDocument_documentLayout(const QTextDocument* this_ptr) {
  return this_ptr->documentLayout();
}

double qt_gui_c_QTextDocument_documentMargin(const QTextDocument* this_ptr) {
  return this_ptr->documentMargin();
}

void qt_gui_c_QTextDocument_drawContents_painter(QTextDocument* this_ptr, QPainter* painter) {
  this_ptr->drawContents(painter);
}

void qt_gui_c_QTextDocument_drawContents_painter_rect(QTextDocument* this_ptr, QPainter* painter, const QRectF* rect) {
  this_ptr->drawContents(painter, *rect);
}

void qt_gui_c_QTextDocument_end_to_output(const QTextDocument* this_ptr, QTextBlock* output) {
  new(output) QTextBlock(this_ptr->end());
}

void qt_gui_c_QTextDocument_findBlockByLineNumber_to_output(const QTextDocument* this_ptr, int blockNumber, QTextBlock* output) {
  new(output) QTextBlock(this_ptr->findBlockByLineNumber(blockNumber));
}

void qt_gui_c_QTextDocument_findBlockByNumber_to_output(const QTextDocument* this_ptr, int blockNumber, QTextBlock* output) {
  new(output) QTextBlock(this_ptr->findBlockByNumber(blockNumber));
}

void qt_gui_c_QTextDocument_findBlock_to_output(const QTextDocument* this_ptr, int pos, QTextBlock* output) {
  new(output) QTextBlock(this_ptr->findBlock(pos));
}

QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegExp(const QTextDocument* this_ptr, const QRegExp* expr) {
  return new QTextCursor(this_ptr->find(*expr));
}

QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegExp_QTextCursor(const QTextDocument* this_ptr, const QRegExp* expr, const QTextCursor* cursor) {
  return new QTextCursor(this_ptr->find(*expr, *cursor));
}

QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegExp_QTextCursor_QFlags_QTextDocument_FindFlag(const QTextDocument* this_ptr, const QRegExp* expr, const QTextCursor* cursor, unsigned int options) {
  return new QTextCursor(this_ptr->find(*expr, *cursor, QFlags< QTextDocument::FindFlag >(options)));
}

QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegExp_int(const QTextDocument* this_ptr, const QRegExp* expr, int from) {
  return new QTextCursor(this_ptr->find(*expr, from));
}

QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegExp_int_QFlags_QTextDocument_FindFlag(const QTextDocument* this_ptr, const QRegExp* expr, int from, unsigned int options) {
  return new QTextCursor(this_ptr->find(*expr, from, QFlags< QTextDocument::FindFlag >(options)));
}

QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegularExpression(const QTextDocument* this_ptr, const QRegularExpression* expr) {
  return new QTextCursor(this_ptr->find(*expr));
}

QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegularExpression_QTextCursor(const QTextDocument* this_ptr, const QRegularExpression* expr, const QTextCursor* cursor) {
  return new QTextCursor(this_ptr->find(*expr, *cursor));
}

QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegularExpression_QTextCursor_QFlags_QTextDocument_FindFlag(const QTextDocument* this_ptr, const QRegularExpression* expr, const QTextCursor* cursor, unsigned int options) {
  return new QTextCursor(this_ptr->find(*expr, *cursor, QFlags< QTextDocument::FindFlag >(options)));
}

QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegularExpression_int(const QTextDocument* this_ptr, const QRegularExpression* expr, int from) {
  return new QTextCursor(this_ptr->find(*expr, from));
}

QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QRegularExpression_int_QFlags_QTextDocument_FindFlag(const QTextDocument* this_ptr, const QRegularExpression* expr, int from, unsigned int options) {
  return new QTextCursor(this_ptr->find(*expr, from, QFlags< QTextDocument::FindFlag >(options)));
}

QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QString(const QTextDocument* this_ptr, const QString* subString) {
  return new QTextCursor(this_ptr->find(*subString));
}

QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QString_QTextCursor(const QTextDocument* this_ptr, const QString* subString, const QTextCursor* cursor) {
  return new QTextCursor(this_ptr->find(*subString, *cursor));
}

QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QString_QTextCursor_QFlags_QTextDocument_FindFlag(const QTextDocument* this_ptr, const QString* subString, const QTextCursor* cursor, unsigned int options) {
  return new QTextCursor(this_ptr->find(*subString, *cursor, QFlags< QTextDocument::FindFlag >(options)));
}

QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QString_int(const QTextDocument* this_ptr, const QString* subString, int from) {
  return new QTextCursor(this_ptr->find(*subString, from));
}

QTextCursor* qt_gui_c_QTextDocument_find_as_ptr_QString_int_QFlags_QTextDocument_FindFlag(const QTextDocument* this_ptr, const QString* subString, int from, unsigned int options) {
  return new QTextCursor(this_ptr->find(*subString, from, QFlags< QTextDocument::FindFlag >(options)));
}

void qt_gui_c_QTextDocument_firstBlock_to_output(const QTextDocument* this_ptr, QTextBlock* output) {
  new(output) QTextBlock(this_ptr->firstBlock());
}

QTextFrame* qt_gui_c_QTextDocument_frameAt(const QTextDocument* this_ptr, int pos) {
  return this_ptr->frameAt(pos);
}

double qt_gui_c_QTextDocument_idealWidth(const QTextDocument* this_ptr) {
  return this_ptr->idealWidth();
}

double qt_gui_c_QTextDocument_indentWidth(const QTextDocument* this_ptr) {
  return this_ptr->indentWidth();
}

bool qt_gui_c_QTextDocument_isEmpty(const QTextDocument* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_gui_c_QTextDocument_isModified(const QTextDocument* this_ptr) {
  return this_ptr->isModified();
}

bool qt_gui_c_QTextDocument_isRedoAvailable(const QTextDocument* this_ptr) {
  return this_ptr->isRedoAvailable();
}

bool qt_gui_c_QTextDocument_isUndoAvailable(const QTextDocument* this_ptr) {
  return this_ptr->isUndoAvailable();
}

bool qt_gui_c_QTextDocument_isUndoRedoEnabled(const QTextDocument* this_ptr) {
  return this_ptr->isUndoRedoEnabled();
}

void qt_gui_c_QTextDocument_lastBlock_to_output(const QTextDocument* this_ptr, QTextBlock* output) {
  new(output) QTextBlock(this_ptr->lastBlock());
}

int qt_gui_c_QTextDocument_lineCount(const QTextDocument* this_ptr) {
  return this_ptr->lineCount();
}

void qt_gui_c_QTextDocument_markContentsDirty(QTextDocument* this_ptr, int from, int length) {
  this_ptr->markContentsDirty(from, length);
}

int qt_gui_c_QTextDocument_maximumBlockCount(const QTextDocument* this_ptr) {
  return this_ptr->maximumBlockCount();
}

void qt_gui_c_QTextDocument_metaInformation_to_output(const QTextDocument* this_ptr, QTextDocument::MetaInformation info, QString* output) {
  new(output) QString(this_ptr->metaInformation(info));
}

const QMetaObject* qt_gui_c_QTextDocument_metaObject(const QTextDocument* this_ptr) {
  return this_ptr->metaObject();
}

QTextDocument* qt_gui_c_QTextDocument_new_no_args() {
  return new QTextDocument();
}

QTextDocument* qt_gui_c_QTextDocument_new_parent(QObject* parent) {
  return new QTextDocument(parent);
}

QTextDocument* qt_gui_c_QTextDocument_new_text(const QString* text) {
  return new QTextDocument(*text);
}

QTextDocument* qt_gui_c_QTextDocument_new_text_parent(const QString* text, QObject* parent) {
  return new QTextDocument(*text, parent);
}

QTextObject* qt_gui_c_QTextDocument_object(const QTextDocument* this_ptr, int objectIndex) {
  return this_ptr->object(objectIndex);
}

QTextObject* qt_gui_c_QTextDocument_objectForFormat(const QTextDocument* this_ptr, const QTextFormat* arg1) {
  return this_ptr->objectForFormat(*arg1);
}

int qt_gui_c_QTextDocument_pageCount(const QTextDocument* this_ptr) {
  return this_ptr->pageCount();
}

void qt_gui_c_QTextDocument_pageSize_to_output(const QTextDocument* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->pageSize());
}

void qt_gui_c_QTextDocument_print(const QTextDocument* this_ptr, QPagedPaintDevice* printer) {
  this_ptr->print(printer);
}

int qt_gui_c_QTextDocument_qt_metacall(QTextDocument* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QTextDocument_qt_metacast(QTextDocument* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QTextDocument_redo_cursor(QTextDocument* this_ptr, QTextCursor* cursor) {
  this_ptr->redo(cursor);
}

void qt_gui_c_QTextDocument_redo_no_args(QTextDocument* this_ptr) {
  this_ptr->redo();
}

void qt_gui_c_QTextDocument_resource_to_output(const QTextDocument* this_ptr, int type, const QUrl* name, QVariant* output) {
  new(output) QVariant(this_ptr->resource(type, *name));
}

int qt_gui_c_QTextDocument_revision(const QTextDocument* this_ptr) {
  return this_ptr->revision();
}

QTextFrame* qt_gui_c_QTextDocument_rootFrame(const QTextDocument* this_ptr) {
  return this_ptr->rootFrame();
}

void qt_gui_c_QTextDocument_setBaseUrl(QTextDocument* this_ptr, const QUrl* url) {
  this_ptr->setBaseUrl(*url);
}

void qt_gui_c_QTextDocument_setDefaultCursorMoveStyle(QTextDocument* this_ptr, const Qt::CursorMoveStyle* style) {
  this_ptr->setDefaultCursorMoveStyle(*style);
}

void qt_gui_c_QTextDocument_setDefaultFont(QTextDocument* this_ptr, const QFont* font) {
  this_ptr->setDefaultFont(*font);
}

void qt_gui_c_QTextDocument_setDefaultStyleSheet(QTextDocument* this_ptr, const QString* sheet) {
  this_ptr->setDefaultStyleSheet(*sheet);
}

void qt_gui_c_QTextDocument_setDefaultTextOption(QTextDocument* this_ptr, const QTextOption* option) {
  this_ptr->setDefaultTextOption(*option);
}

void qt_gui_c_QTextDocument_setDocumentLayout(QTextDocument* this_ptr, QAbstractTextDocumentLayout* layout) {
  this_ptr->setDocumentLayout(layout);
}

void qt_gui_c_QTextDocument_setDocumentMargin(QTextDocument* this_ptr, double margin) {
  this_ptr->setDocumentMargin(margin);
}

void qt_gui_c_QTextDocument_setHtml(QTextDocument* this_ptr, const QString* html) {
  this_ptr->setHtml(*html);
}

void qt_gui_c_QTextDocument_setIndentWidth(QTextDocument* this_ptr, double width) {
  this_ptr->setIndentWidth(width);
}

void qt_gui_c_QTextDocument_setMaximumBlockCount(QTextDocument* this_ptr, int maximum) {
  this_ptr->setMaximumBlockCount(maximum);
}

void qt_gui_c_QTextDocument_setMetaInformation(QTextDocument* this_ptr, QTextDocument::MetaInformation info, const QString* arg2) {
  this_ptr->setMetaInformation(info, *arg2);
}

void qt_gui_c_QTextDocument_setModified_m(QTextDocument* this_ptr, bool m) {
  this_ptr->setModified(m);
}

void qt_gui_c_QTextDocument_setModified_no_args(QTextDocument* this_ptr) {
  this_ptr->setModified();
}

void qt_gui_c_QTextDocument_setPageSize(QTextDocument* this_ptr, const QSizeF* size) {
  this_ptr->setPageSize(*size);
}

void qt_gui_c_QTextDocument_setPlainText(QTextDocument* this_ptr, const QString* text) {
  this_ptr->setPlainText(*text);
}

void qt_gui_c_QTextDocument_setTextWidth(QTextDocument* this_ptr, double width) {
  this_ptr->setTextWidth(width);
}

void qt_gui_c_QTextDocument_setUndoRedoEnabled(QTextDocument* this_ptr, bool enable) {
  this_ptr->setUndoRedoEnabled(enable);
}

void qt_gui_c_QTextDocument_setUseDesignMetrics(QTextDocument* this_ptr, bool b) {
  this_ptr->setUseDesignMetrics(b);
}

void qt_gui_c_QTextDocument_size_to_output(const QTextDocument* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->size());
}

double qt_gui_c_QTextDocument_textWidth(const QTextDocument* this_ptr) {
  return this_ptr->textWidth();
}

void qt_gui_c_QTextDocument_toHtml_to_output_encoding(const QTextDocument* this_ptr, const QByteArray* encoding, QString* output) {
  new(output) QString(this_ptr->toHtml(*encoding));
}

void qt_gui_c_QTextDocument_toHtml_to_output_no_args(const QTextDocument* this_ptr, QString* output) {
  new(output) QString(this_ptr->toHtml());
}

void qt_gui_c_QTextDocument_toPlainText_to_output(const QTextDocument* this_ptr, QString* output) {
  new(output) QString(this_ptr->toPlainText());
}

void qt_gui_c_QTextDocument_toRawText_to_output(const QTextDocument* this_ptr, QString* output) {
  new(output) QString(this_ptr->toRawText());
}

void qt_gui_c_QTextDocument_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTextDocument::trUtf8(s, c, n));
}

void qt_gui_c_QTextDocument_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTextDocument::tr(s, c, n));
}

void qt_gui_c_QTextDocument_undo_cursor(QTextDocument* this_ptr, QTextCursor* cursor) {
  this_ptr->undo(cursor);
}

void qt_gui_c_QTextDocument_undo_no_args(QTextDocument* this_ptr) {
  this_ptr->undo();
}

bool qt_gui_c_QTextDocument_useDesignMetrics(const QTextDocument* this_ptr) {
  return this_ptr->useDesignMetrics();
}

