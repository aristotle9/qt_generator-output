#include "qt_widgets_c_QPlainTextEdit.h"

QPlainTextEdit* qt_widgets_c_QPlainTextEdit_G_dynamic_cast_QPlainTextEdit_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return dynamic_cast<QPlainTextEdit*>(ptr);
}

QPlainTextEdit* qt_widgets_c_QPlainTextEdit_G_dynamic_cast_QPlainTextEdit_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QPlainTextEdit*>(ptr);
}

QPlainTextEdit* qt_widgets_c_QPlainTextEdit_G_dynamic_cast_QPlainTextEdit_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QPlainTextEdit*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QPlainTextEdit_G_static_cast_QAbstractScrollArea_ptr(QPlainTextEdit* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QFrame* qt_widgets_c_QPlainTextEdit_G_static_cast_QFrame_ptr(QPlainTextEdit* ptr) {
  return static_cast<QFrame*>(ptr);
}

QObject* qt_widgets_c_QPlainTextEdit_G_static_cast_QObject_ptr(QPlainTextEdit* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QPlainTextEdit_G_static_cast_QPaintDevice_ptr(QPlainTextEdit* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QPlainTextEdit* qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return static_cast<QPlainTextEdit*>(ptr);
}

QPlainTextEdit* qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QFrame(QFrame* ptr) {
  return static_cast<QPlainTextEdit*>(ptr);
}

QPlainTextEdit* qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QObject(QObject* ptr) {
  return static_cast<QPlainTextEdit*>(ptr);
}

QPlainTextEdit* qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QPlainTextEdit*>(ptr);
}

QPlainTextEdit* qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QWidget(QWidget* ptr) {
  return static_cast<QPlainTextEdit*>(ptr);
}

QWidget* qt_widgets_c_QPlainTextEdit_G_static_cast_QWidget_ptr(QPlainTextEdit* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QPlainTextEdit_anchorAt_to_output(const QPlainTextEdit* this_ptr, const QPoint* pos, QString* output) {
  new(output) QString(this_ptr->anchorAt(*pos));
}

void qt_widgets_c_QPlainTextEdit_appendHtml(QPlainTextEdit* this_ptr, const QString* html) {
  this_ptr->appendHtml(*html);
}

void qt_widgets_c_QPlainTextEdit_appendPlainText(QPlainTextEdit* this_ptr, const QString* text) {
  this_ptr->appendPlainText(*text);
}

bool qt_widgets_c_QPlainTextEdit_backgroundVisible(const QPlainTextEdit* this_ptr) {
  return this_ptr->backgroundVisible();
}

int qt_widgets_c_QPlainTextEdit_blockCount(const QPlainTextEdit* this_ptr) {
  return this_ptr->blockCount();
}

bool qt_widgets_c_QPlainTextEdit_canPaste(const QPlainTextEdit* this_ptr) {
  return this_ptr->canPaste();
}

void qt_widgets_c_QPlainTextEdit_centerCursor(QPlainTextEdit* this_ptr) {
  this_ptr->centerCursor();
}

bool qt_widgets_c_QPlainTextEdit_centerOnScroll(const QPlainTextEdit* this_ptr) {
  return this_ptr->centerOnScroll();
}

void qt_widgets_c_QPlainTextEdit_clear(QPlainTextEdit* this_ptr) {
  this_ptr->clear();
}

void qt_widgets_c_QPlainTextEdit_copy(QPlainTextEdit* this_ptr) {
  this_ptr->copy();
}

QMenu* qt_widgets_c_QPlainTextEdit_createStandardContextMenu_no_args(QPlainTextEdit* this_ptr) {
  return this_ptr->createStandardContextMenu();
}

QMenu* qt_widgets_c_QPlainTextEdit_createStandardContextMenu_position(QPlainTextEdit* this_ptr, const QPoint* position) {
  return this_ptr->createStandardContextMenu(*position);
}

void qt_widgets_c_QPlainTextEdit_currentCharFormat_to_output(const QPlainTextEdit* this_ptr, QTextCharFormat* output) {
  new(output) QTextCharFormat(this_ptr->currentCharFormat());
}

QTextCursor* qt_widgets_c_QPlainTextEdit_cursorForPosition_as_ptr(const QPlainTextEdit* this_ptr, const QPoint* pos) {
  return new QTextCursor(this_ptr->cursorForPosition(*pos));
}

void qt_widgets_c_QPlainTextEdit_cursorRect_to_output_cursor(const QPlainTextEdit* this_ptr, const QTextCursor* cursor, QRect* output) {
  new(output) QRect(this_ptr->cursorRect(*cursor));
}

void qt_widgets_c_QPlainTextEdit_cursorRect_to_output_no_args(const QPlainTextEdit* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->cursorRect());
}

int qt_widgets_c_QPlainTextEdit_cursorWidth(const QPlainTextEdit* this_ptr) {
  return this_ptr->cursorWidth();
}

void qt_widgets_c_QPlainTextEdit_cut(QPlainTextEdit* this_ptr) {
  this_ptr->cut();
}

void qt_widgets_c_QPlainTextEdit_delete(QPlainTextEdit* this_ptr) {
  delete this_ptr;
}

QTextDocument* qt_widgets_c_QPlainTextEdit_document(const QPlainTextEdit* this_ptr) {
  return this_ptr->document();
}

void qt_widgets_c_QPlainTextEdit_documentTitle_to_output(const QPlainTextEdit* this_ptr, QString* output) {
  new(output) QString(this_ptr->documentTitle());
}

void qt_widgets_c_QPlainTextEdit_ensureCursorVisible(QPlainTextEdit* this_ptr) {
  this_ptr->ensureCursorVisible();
}

void qt_widgets_c_QPlainTextEdit_extraSelections_to_output(const QPlainTextEdit* this_ptr, QList< QTextEdit::ExtraSelection >* output) {
  new(output) QList< QTextEdit::ExtraSelection >(this_ptr->extraSelections());
}

void qt_widgets_c_QPlainTextEdit_inputMethodQuery_to_output_property(const QPlainTextEdit* this_ptr, const Qt::InputMethodQuery* property, QVariant* output) {
  new(output) QVariant(this_ptr->inputMethodQuery(*property));
}

void qt_widgets_c_QPlainTextEdit_inputMethodQuery_to_output_query_argument(const QPlainTextEdit* this_ptr, const Qt::InputMethodQuery* query, const QVariant* argument, QVariant* output) {
  new(output) QVariant(this_ptr->inputMethodQuery(*query, *argument));
}

void qt_widgets_c_QPlainTextEdit_insertPlainText(QPlainTextEdit* this_ptr, const QString* text) {
  this_ptr->insertPlainText(*text);
}

bool qt_widgets_c_QPlainTextEdit_isReadOnly(const QPlainTextEdit* this_ptr) {
  return this_ptr->isReadOnly();
}

bool qt_widgets_c_QPlainTextEdit_isUndoRedoEnabled(const QPlainTextEdit* this_ptr) {
  return this_ptr->isUndoRedoEnabled();
}

QPlainTextEdit::LineWrapMode qt_widgets_c_QPlainTextEdit_lineWrapMode(const QPlainTextEdit* this_ptr) {
  return this_ptr->lineWrapMode();
}

void qt_widgets_c_QPlainTextEdit_loadResource_to_output(QPlainTextEdit* this_ptr, int type, const QUrl* name, QVariant* output) {
  new(output) QVariant(this_ptr->loadResource(type, *name));
}

int qt_widgets_c_QPlainTextEdit_maximumBlockCount(const QPlainTextEdit* this_ptr) {
  return this_ptr->maximumBlockCount();
}

void qt_widgets_c_QPlainTextEdit_mergeCurrentCharFormat(QPlainTextEdit* this_ptr, const QTextCharFormat* modifier) {
  this_ptr->mergeCurrentCharFormat(*modifier);
}

const QMetaObject* qt_widgets_c_QPlainTextEdit_metaObject(const QPlainTextEdit* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QPlainTextEdit_moveCursor_operation(QPlainTextEdit* this_ptr, const QTextCursor::MoveOperation* operation) {
  this_ptr->moveCursor(*operation);
}

void qt_widgets_c_QPlainTextEdit_moveCursor_operation_mode(QPlainTextEdit* this_ptr, const QTextCursor::MoveOperation* operation, const QTextCursor::MoveMode* mode) {
  this_ptr->moveCursor(*operation, *mode);
}

QPlainTextEdit* qt_widgets_c_QPlainTextEdit_new_no_args() {
  return new QPlainTextEdit();
}

QPlainTextEdit* qt_widgets_c_QPlainTextEdit_new_parent(QWidget* parent) {
  return new QPlainTextEdit(parent);
}

QPlainTextEdit* qt_widgets_c_QPlainTextEdit_new_text(const QString* text) {
  return new QPlainTextEdit(*text);
}

QPlainTextEdit* qt_widgets_c_QPlainTextEdit_new_text_parent(const QString* text, QWidget* parent) {
  return new QPlainTextEdit(*text, parent);
}

bool qt_widgets_c_QPlainTextEdit_overwriteMode(const QPlainTextEdit* this_ptr) {
  return this_ptr->overwriteMode();
}

void qt_widgets_c_QPlainTextEdit_paste(QPlainTextEdit* this_ptr) {
  this_ptr->paste();
}

void qt_widgets_c_QPlainTextEdit_placeholderText_to_output(const QPlainTextEdit* this_ptr, QString* output) {
  new(output) QString(this_ptr->placeholderText());
}

void qt_widgets_c_QPlainTextEdit_print(const QPlainTextEdit* this_ptr, QPagedPaintDevice* printer) {
  this_ptr->print(printer);
}

int qt_widgets_c_QPlainTextEdit_qt_metacall(QPlainTextEdit* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QPlainTextEdit_qt_metacast(QPlainTextEdit* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QPlainTextEdit_redo(QPlainTextEdit* this_ptr) {
  this_ptr->redo();
}

void qt_widgets_c_QPlainTextEdit_selectAll(QPlainTextEdit* this_ptr) {
  this_ptr->selectAll();
}

void qt_widgets_c_QPlainTextEdit_setBackgroundVisible(QPlainTextEdit* this_ptr, bool visible) {
  this_ptr->setBackgroundVisible(visible);
}

void qt_widgets_c_QPlainTextEdit_setCenterOnScroll(QPlainTextEdit* this_ptr, bool enabled) {
  this_ptr->setCenterOnScroll(enabled);
}

void qt_widgets_c_QPlainTextEdit_setCurrentCharFormat(QPlainTextEdit* this_ptr, const QTextCharFormat* format) {
  this_ptr->setCurrentCharFormat(*format);
}

void qt_widgets_c_QPlainTextEdit_setCursorWidth(QPlainTextEdit* this_ptr, int width) {
  this_ptr->setCursorWidth(width);
}

void qt_widgets_c_QPlainTextEdit_setDocument(QPlainTextEdit* this_ptr, QTextDocument* document) {
  this_ptr->setDocument(document);
}

void qt_widgets_c_QPlainTextEdit_setDocumentTitle(QPlainTextEdit* this_ptr, const QString* title) {
  this_ptr->setDocumentTitle(*title);
}

void qt_widgets_c_QPlainTextEdit_setExtraSelections(QPlainTextEdit* this_ptr, const QList< QTextEdit::ExtraSelection >* selections) {
  this_ptr->setExtraSelections(*selections);
}

void qt_widgets_c_QPlainTextEdit_setLineWrapMode(QPlainTextEdit* this_ptr, QPlainTextEdit::LineWrapMode mode) {
  this_ptr->setLineWrapMode(mode);
}

void qt_widgets_c_QPlainTextEdit_setMaximumBlockCount(QPlainTextEdit* this_ptr, int maximum) {
  this_ptr->setMaximumBlockCount(maximum);
}

void qt_widgets_c_QPlainTextEdit_setOverwriteMode(QPlainTextEdit* this_ptr, bool overwrite) {
  this_ptr->setOverwriteMode(overwrite);
}

void qt_widgets_c_QPlainTextEdit_setPlaceholderText(QPlainTextEdit* this_ptr, const QString* placeholderText) {
  this_ptr->setPlaceholderText(*placeholderText);
}

void qt_widgets_c_QPlainTextEdit_setPlainText(QPlainTextEdit* this_ptr, const QString* text) {
  this_ptr->setPlainText(*text);
}

void qt_widgets_c_QPlainTextEdit_setReadOnly(QPlainTextEdit* this_ptr, bool ro) {
  this_ptr->setReadOnly(ro);
}

void qt_widgets_c_QPlainTextEdit_setTabChangesFocus(QPlainTextEdit* this_ptr, bool b) {
  this_ptr->setTabChangesFocus(b);
}

void qt_widgets_c_QPlainTextEdit_setTabStopWidth(QPlainTextEdit* this_ptr, int width) {
  this_ptr->setTabStopWidth(width);
}

void qt_widgets_c_QPlainTextEdit_setTextCursor(QPlainTextEdit* this_ptr, const QTextCursor* cursor) {
  this_ptr->setTextCursor(*cursor);
}

void qt_widgets_c_QPlainTextEdit_setUndoRedoEnabled(QPlainTextEdit* this_ptr, bool enable) {
  this_ptr->setUndoRedoEnabled(enable);
}

void qt_widgets_c_QPlainTextEdit_setWordWrapMode(QPlainTextEdit* this_ptr, const QTextOption::WrapMode* policy) {
  this_ptr->setWordWrapMode(*policy);
}

bool qt_widgets_c_QPlainTextEdit_tabChangesFocus(const QPlainTextEdit* this_ptr) {
  return this_ptr->tabChangesFocus();
}

int qt_widgets_c_QPlainTextEdit_tabStopWidth(const QPlainTextEdit* this_ptr) {
  return this_ptr->tabStopWidth();
}

QTextCursor* qt_widgets_c_QPlainTextEdit_textCursor_as_ptr(const QPlainTextEdit* this_ptr) {
  return new QTextCursor(this_ptr->textCursor());
}

void qt_widgets_c_QPlainTextEdit_toPlainText_to_output(const QPlainTextEdit* this_ptr, QString* output) {
  new(output) QString(this_ptr->toPlainText());
}

void qt_widgets_c_QPlainTextEdit_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPlainTextEdit::trUtf8(s, c, n));
}

void qt_widgets_c_QPlainTextEdit_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPlainTextEdit::tr(s, c, n));
}

void qt_widgets_c_QPlainTextEdit_undo(QPlainTextEdit* this_ptr) {
  this_ptr->undo();
}

void qt_widgets_c_QPlainTextEdit_zoomIn_no_args(QPlainTextEdit* this_ptr) {
  this_ptr->zoomIn();
}

void qt_widgets_c_QPlainTextEdit_zoomIn_range(QPlainTextEdit* this_ptr, int range) {
  this_ptr->zoomIn(range);
}

void qt_widgets_c_QPlainTextEdit_zoomOut_no_args(QPlainTextEdit* this_ptr) {
  this_ptr->zoomOut();
}

void qt_widgets_c_QPlainTextEdit_zoomOut_range(QPlainTextEdit* this_ptr, int range) {
  this_ptr->zoomOut(range);
}

