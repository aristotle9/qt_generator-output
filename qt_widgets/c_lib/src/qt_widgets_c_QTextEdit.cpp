#include "qt_widgets_c_QTextEdit.h"

const QTextCursor* qt_widgets_c_QTextEdit_ExtraSelection_cursor(const QTextEdit::ExtraSelection* this_ptr) {
  return &this_ptr->cursor;
}

QTextCursor* qt_widgets_c_QTextEdit_ExtraSelection_cursor_mut(QTextEdit::ExtraSelection* this_ptr) {
  return &this_ptr->cursor;
}

void qt_widgets_c_QTextEdit_ExtraSelection_destructor(QTextEdit::ExtraSelection* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

const QTextCharFormat* qt_widgets_c_QTextEdit_ExtraSelection_format(const QTextEdit::ExtraSelection* this_ptr) {
  return &this_ptr->format;
}

QTextCharFormat* qt_widgets_c_QTextEdit_ExtraSelection_format_mut(QTextEdit::ExtraSelection* this_ptr) {
  return &this_ptr->format;
}

void qt_widgets_c_QTextEdit_ExtraSelection_set_cursor(QTextEdit::ExtraSelection* this_ptr, const QTextCursor* value) {
  this_ptr->cursor = *value;
}

void qt_widgets_c_QTextEdit_ExtraSelection_set_format(QTextEdit::ExtraSelection* this_ptr, const QTextCharFormat* value) {
  this_ptr->format = *value;
}

QTextEdit* qt_widgets_c_QTextEdit_G_dynamic_cast_QTextEdit_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return dynamic_cast<QTextEdit*>(ptr);
}

QTextEdit* qt_widgets_c_QTextEdit_G_dynamic_cast_QTextEdit_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QTextEdit*>(ptr);
}

QTextEdit* qt_widgets_c_QTextEdit_G_dynamic_cast_QTextEdit_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QTextEdit*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QTextEdit_G_static_cast_QAbstractScrollArea_ptr(QTextEdit* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QFrame* qt_widgets_c_QTextEdit_G_static_cast_QFrame_ptr(QTextEdit* ptr) {
  return static_cast<QFrame*>(ptr);
}

QObject* qt_widgets_c_QTextEdit_G_static_cast_QObject_ptr(QTextEdit* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QTextEdit_G_static_cast_QPaintDevice_ptr(QTextEdit* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QTextEdit* qt_widgets_c_QTextEdit_G_static_cast_QTextEdit_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return static_cast<QTextEdit*>(ptr);
}

QTextEdit* qt_widgets_c_QTextEdit_G_static_cast_QTextEdit_ptr_QFrame(QFrame* ptr) {
  return static_cast<QTextEdit*>(ptr);
}

QTextEdit* qt_widgets_c_QTextEdit_G_static_cast_QTextEdit_ptr_QObject(QObject* ptr) {
  return static_cast<QTextEdit*>(ptr);
}

QTextEdit* qt_widgets_c_QTextEdit_G_static_cast_QTextEdit_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QTextEdit*>(ptr);
}

QTextEdit* qt_widgets_c_QTextEdit_G_static_cast_QTextEdit_ptr_QWidget(QWidget* ptr) {
  return static_cast<QTextEdit*>(ptr);
}

QWidget* qt_widgets_c_QTextEdit_G_static_cast_QWidget_ptr(QTextEdit* ptr) {
  return static_cast<QWidget*>(ptr);
}

bool qt_widgets_c_QTextEdit_acceptRichText(const QTextEdit* this_ptr) {
  return this_ptr->acceptRichText();
}

void qt_widgets_c_QTextEdit_anchorAt_to_output(const QTextEdit* this_ptr, const QPoint* pos, QString* output) {
  new(output) QString(this_ptr->anchorAt(*pos));
}

void qt_widgets_c_QTextEdit_append(QTextEdit* this_ptr, const QString* text) {
  this_ptr->append(*text);
}

unsigned int qt_widgets_c_QTextEdit_autoFormatting(const QTextEdit* this_ptr) {
  return uint(this_ptr->autoFormatting());
}

bool qt_widgets_c_QTextEdit_canPaste(const QTextEdit* this_ptr) {
  return this_ptr->canPaste();
}

void qt_widgets_c_QTextEdit_clear(QTextEdit* this_ptr) {
  this_ptr->clear();
}

void qt_widgets_c_QTextEdit_copy(QTextEdit* this_ptr) {
  this_ptr->copy();
}

QMenu* qt_widgets_c_QTextEdit_createStandardContextMenu_no_args(QTextEdit* this_ptr) {
  return this_ptr->createStandardContextMenu();
}

QMenu* qt_widgets_c_QTextEdit_createStandardContextMenu_position(QTextEdit* this_ptr, const QPoint* position) {
  return this_ptr->createStandardContextMenu(*position);
}

void qt_widgets_c_QTextEdit_currentCharFormat_to_output(const QTextEdit* this_ptr, QTextCharFormat* output) {
  new(output) QTextCharFormat(this_ptr->currentCharFormat());
}

void qt_widgets_c_QTextEdit_currentFont_to_output(const QTextEdit* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->currentFont());
}

QTextCursor* qt_widgets_c_QTextEdit_cursorForPosition_as_ptr(const QTextEdit* this_ptr, const QPoint* pos) {
  return new QTextCursor(this_ptr->cursorForPosition(*pos));
}

void qt_widgets_c_QTextEdit_cursorRect_to_output_cursor(const QTextEdit* this_ptr, const QTextCursor* cursor, QRect* output) {
  new(output) QRect(this_ptr->cursorRect(*cursor));
}

void qt_widgets_c_QTextEdit_cursorRect_to_output_no_args(const QTextEdit* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->cursorRect());
}

int qt_widgets_c_QTextEdit_cursorWidth(const QTextEdit* this_ptr) {
  return this_ptr->cursorWidth();
}

void qt_widgets_c_QTextEdit_cut(QTextEdit* this_ptr) {
  this_ptr->cut();
}

void qt_widgets_c_QTextEdit_delete(QTextEdit* this_ptr) {
  delete this_ptr;
}

QTextDocument* qt_widgets_c_QTextEdit_document(const QTextEdit* this_ptr) {
  return this_ptr->document();
}

void qt_widgets_c_QTextEdit_documentTitle_to_output(const QTextEdit* this_ptr, QString* output) {
  new(output) QString(this_ptr->documentTitle());
}

void qt_widgets_c_QTextEdit_ensureCursorVisible(QTextEdit* this_ptr) {
  this_ptr->ensureCursorVisible();
}

void qt_widgets_c_QTextEdit_extraSelections_to_output(const QTextEdit* this_ptr, QList< QTextEdit::ExtraSelection >* output) {
  new(output) QList< QTextEdit::ExtraSelection >(this_ptr->extraSelections());
}

void qt_widgets_c_QTextEdit_fontFamily_to_output(const QTextEdit* this_ptr, QString* output) {
  new(output) QString(this_ptr->fontFamily());
}

bool qt_widgets_c_QTextEdit_fontItalic(const QTextEdit* this_ptr) {
  return this_ptr->fontItalic();
}

double qt_widgets_c_QTextEdit_fontPointSize(const QTextEdit* this_ptr) {
  return this_ptr->fontPointSize();
}

bool qt_widgets_c_QTextEdit_fontUnderline(const QTextEdit* this_ptr) {
  return this_ptr->fontUnderline();
}

int qt_widgets_c_QTextEdit_fontWeight(const QTextEdit* this_ptr) {
  return this_ptr->fontWeight();
}

void qt_widgets_c_QTextEdit_inputMethodQuery_to_output_property(const QTextEdit* this_ptr, const Qt::InputMethodQuery* property, QVariant* output) {
  new(output) QVariant(this_ptr->inputMethodQuery(*property));
}

void qt_widgets_c_QTextEdit_inputMethodQuery_to_output_query_argument(const QTextEdit* this_ptr, const Qt::InputMethodQuery* query, const QVariant* argument, QVariant* output) {
  new(output) QVariant(this_ptr->inputMethodQuery(*query, *argument));
}

void qt_widgets_c_QTextEdit_insertHtml(QTextEdit* this_ptr, const QString* text) {
  this_ptr->insertHtml(*text);
}

void qt_widgets_c_QTextEdit_insertPlainText(QTextEdit* this_ptr, const QString* text) {
  this_ptr->insertPlainText(*text);
}

bool qt_widgets_c_QTextEdit_isReadOnly(const QTextEdit* this_ptr) {
  return this_ptr->isReadOnly();
}

bool qt_widgets_c_QTextEdit_isUndoRedoEnabled(const QTextEdit* this_ptr) {
  return this_ptr->isUndoRedoEnabled();
}

int qt_widgets_c_QTextEdit_lineWrapColumnOrWidth(const QTextEdit* this_ptr) {
  return this_ptr->lineWrapColumnOrWidth();
}

QTextEdit::LineWrapMode qt_widgets_c_QTextEdit_lineWrapMode(const QTextEdit* this_ptr) {
  return this_ptr->lineWrapMode();
}

void qt_widgets_c_QTextEdit_loadResource_to_output(QTextEdit* this_ptr, int type, const QUrl* name, QVariant* output) {
  new(output) QVariant(this_ptr->loadResource(type, *name));
}

void qt_widgets_c_QTextEdit_mergeCurrentCharFormat(QTextEdit* this_ptr, const QTextCharFormat* modifier) {
  this_ptr->mergeCurrentCharFormat(*modifier);
}

const QMetaObject* qt_widgets_c_QTextEdit_metaObject(const QTextEdit* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QTextEdit_moveCursor_operation(QTextEdit* this_ptr, const QTextCursor::MoveOperation* operation) {
  this_ptr->moveCursor(*operation);
}

void qt_widgets_c_QTextEdit_moveCursor_operation_mode(QTextEdit* this_ptr, const QTextCursor::MoveOperation* operation, const QTextCursor::MoveMode* mode) {
  this_ptr->moveCursor(*operation, *mode);
}

QTextEdit* qt_widgets_c_QTextEdit_new_no_args() {
  return new QTextEdit();
}

QTextEdit* qt_widgets_c_QTextEdit_new_parent(QWidget* parent) {
  return new QTextEdit(parent);
}

QTextEdit* qt_widgets_c_QTextEdit_new_text(const QString* text) {
  return new QTextEdit(*text);
}

QTextEdit* qt_widgets_c_QTextEdit_new_text_parent(const QString* text, QWidget* parent) {
  return new QTextEdit(*text, parent);
}

bool qt_widgets_c_QTextEdit_overwriteMode(const QTextEdit* this_ptr) {
  return this_ptr->overwriteMode();
}

void qt_widgets_c_QTextEdit_paste(QTextEdit* this_ptr) {
  this_ptr->paste();
}

void qt_widgets_c_QTextEdit_placeholderText_to_output(const QTextEdit* this_ptr, QString* output) {
  new(output) QString(this_ptr->placeholderText());
}

void qt_widgets_c_QTextEdit_print(const QTextEdit* this_ptr, QPagedPaintDevice* printer) {
  this_ptr->print(printer);
}

int qt_widgets_c_QTextEdit_qt_metacall(QTextEdit* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QTextEdit_qt_metacast(QTextEdit* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QTextEdit_redo(QTextEdit* this_ptr) {
  this_ptr->redo();
}

void qt_widgets_c_QTextEdit_scrollToAnchor(QTextEdit* this_ptr, const QString* name) {
  this_ptr->scrollToAnchor(*name);
}

void qt_widgets_c_QTextEdit_selectAll(QTextEdit* this_ptr) {
  this_ptr->selectAll();
}

void qt_widgets_c_QTextEdit_setAcceptRichText(QTextEdit* this_ptr, bool accept) {
  this_ptr->setAcceptRichText(accept);
}

void qt_widgets_c_QTextEdit_setAutoFormatting(QTextEdit* this_ptr, unsigned int features) {
  this_ptr->setAutoFormatting(QFlags< QTextEdit::AutoFormattingFlag >(features));
}

void qt_widgets_c_QTextEdit_setCurrentCharFormat(QTextEdit* this_ptr, const QTextCharFormat* format) {
  this_ptr->setCurrentCharFormat(*format);
}

void qt_widgets_c_QTextEdit_setCurrentFont(QTextEdit* this_ptr, const QFont* f) {
  this_ptr->setCurrentFont(*f);
}

void qt_widgets_c_QTextEdit_setCursorWidth(QTextEdit* this_ptr, int width) {
  this_ptr->setCursorWidth(width);
}

void qt_widgets_c_QTextEdit_setDocument(QTextEdit* this_ptr, QTextDocument* document) {
  this_ptr->setDocument(document);
}

void qt_widgets_c_QTextEdit_setDocumentTitle(QTextEdit* this_ptr, const QString* title) {
  this_ptr->setDocumentTitle(*title);
}

void qt_widgets_c_QTextEdit_setExtraSelections(QTextEdit* this_ptr, const QList< QTextEdit::ExtraSelection >* selections) {
  this_ptr->setExtraSelections(*selections);
}

void qt_widgets_c_QTextEdit_setFontFamily(QTextEdit* this_ptr, const QString* fontFamily) {
  this_ptr->setFontFamily(*fontFamily);
}

void qt_widgets_c_QTextEdit_setFontItalic(QTextEdit* this_ptr, bool b) {
  this_ptr->setFontItalic(b);
}

void qt_widgets_c_QTextEdit_setFontPointSize(QTextEdit* this_ptr, double s) {
  this_ptr->setFontPointSize(s);
}

void qt_widgets_c_QTextEdit_setFontUnderline(QTextEdit* this_ptr, bool b) {
  this_ptr->setFontUnderline(b);
}

void qt_widgets_c_QTextEdit_setFontWeight(QTextEdit* this_ptr, int w) {
  this_ptr->setFontWeight(w);
}

void qt_widgets_c_QTextEdit_setHtml(QTextEdit* this_ptr, const QString* text) {
  this_ptr->setHtml(*text);
}

void qt_widgets_c_QTextEdit_setLineWrapColumnOrWidth(QTextEdit* this_ptr, int w) {
  this_ptr->setLineWrapColumnOrWidth(w);
}

void qt_widgets_c_QTextEdit_setLineWrapMode(QTextEdit* this_ptr, QTextEdit::LineWrapMode mode) {
  this_ptr->setLineWrapMode(mode);
}

void qt_widgets_c_QTextEdit_setOverwriteMode(QTextEdit* this_ptr, bool overwrite) {
  this_ptr->setOverwriteMode(overwrite);
}

void qt_widgets_c_QTextEdit_setPlaceholderText(QTextEdit* this_ptr, const QString* placeholderText) {
  this_ptr->setPlaceholderText(*placeholderText);
}

void qt_widgets_c_QTextEdit_setPlainText(QTextEdit* this_ptr, const QString* text) {
  this_ptr->setPlainText(*text);
}

void qt_widgets_c_QTextEdit_setReadOnly(QTextEdit* this_ptr, bool ro) {
  this_ptr->setReadOnly(ro);
}

void qt_widgets_c_QTextEdit_setTabChangesFocus(QTextEdit* this_ptr, bool b) {
  this_ptr->setTabChangesFocus(b);
}

void qt_widgets_c_QTextEdit_setTabStopWidth(QTextEdit* this_ptr, int width) {
  this_ptr->setTabStopWidth(width);
}

void qt_widgets_c_QTextEdit_setText(QTextEdit* this_ptr, const QString* text) {
  this_ptr->setText(*text);
}

void qt_widgets_c_QTextEdit_setTextBackgroundColor(QTextEdit* this_ptr, const QColor* c) {
  this_ptr->setTextBackgroundColor(*c);
}

void qt_widgets_c_QTextEdit_setTextColor(QTextEdit* this_ptr, const QColor* c) {
  this_ptr->setTextColor(*c);
}

void qt_widgets_c_QTextEdit_setTextCursor(QTextEdit* this_ptr, const QTextCursor* cursor) {
  this_ptr->setTextCursor(*cursor);
}

void qt_widgets_c_QTextEdit_setUndoRedoEnabled(QTextEdit* this_ptr, bool enable) {
  this_ptr->setUndoRedoEnabled(enable);
}

void qt_widgets_c_QTextEdit_setWordWrapMode(QTextEdit* this_ptr, const QTextOption::WrapMode* policy) {
  this_ptr->setWordWrapMode(*policy);
}

bool qt_widgets_c_QTextEdit_tabChangesFocus(const QTextEdit* this_ptr) {
  return this_ptr->tabChangesFocus();
}

int qt_widgets_c_QTextEdit_tabStopWidth(const QTextEdit* this_ptr) {
  return this_ptr->tabStopWidth();
}

void qt_widgets_c_QTextEdit_textBackgroundColor_to_output(const QTextEdit* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->textBackgroundColor());
}

void qt_widgets_c_QTextEdit_textColor_to_output(const QTextEdit* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->textColor());
}

QTextCursor* qt_widgets_c_QTextEdit_textCursor_as_ptr(const QTextEdit* this_ptr) {
  return new QTextCursor(this_ptr->textCursor());
}

void qt_widgets_c_QTextEdit_toHtml_to_output(const QTextEdit* this_ptr, QString* output) {
  new(output) QString(this_ptr->toHtml());
}

void qt_widgets_c_QTextEdit_toPlainText_to_output(const QTextEdit* this_ptr, QString* output) {
  new(output) QString(this_ptr->toPlainText());
}

void qt_widgets_c_QTextEdit_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTextEdit::trUtf8(s, c, n));
}

void qt_widgets_c_QTextEdit_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTextEdit::tr(s, c, n));
}

void qt_widgets_c_QTextEdit_undo(QTextEdit* this_ptr) {
  this_ptr->undo();
}

void qt_widgets_c_QTextEdit_zoomIn_no_args(QTextEdit* this_ptr) {
  this_ptr->zoomIn();
}

void qt_widgets_c_QTextEdit_zoomIn_range(QTextEdit* this_ptr, int range) {
  this_ptr->zoomIn(range);
}

void qt_widgets_c_QTextEdit_zoomOut_no_args(QTextEdit* this_ptr) {
  this_ptr->zoomOut();
}

void qt_widgets_c_QTextEdit_zoomOut_range(QTextEdit* this_ptr, int range) {
  this_ptr->zoomOut(range);
}

