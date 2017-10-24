#include "qt_widgets_c_QListWidgetItem.h"

void qt_widgets_c_QListWidgetItem_backgroundColor_to_output(const QListWidgetItem* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->backgroundColor());
}

void qt_widgets_c_QListWidgetItem_background_to_output(const QListWidgetItem* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->background());
}

QListWidgetItem* qt_widgets_c_QListWidgetItem_clone(const QListWidgetItem* this_ptr) {
  return this_ptr->clone();
}

void qt_widgets_c_QListWidgetItem_data_to_output(const QListWidgetItem* this_ptr, int role, QVariant* output) {
  new(output) QVariant(this_ptr->data(role));
}

void qt_widgets_c_QListWidgetItem_delete(QListWidgetItem* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QListWidgetItem_font_to_output(const QListWidgetItem* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->font());
}

void qt_widgets_c_QListWidgetItem_foreground_to_output(const QListWidgetItem* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->foreground());
}

void qt_widgets_c_QListWidgetItem_icon_to_output(const QListWidgetItem* this_ptr, QIcon* output) {
  new(output) QIcon(this_ptr->icon());
}

bool qt_widgets_c_QListWidgetItem_isHidden(const QListWidgetItem* this_ptr) {
  return this_ptr->isHidden();
}

bool qt_widgets_c_QListWidgetItem_isSelected(const QListWidgetItem* this_ptr) {
  return this_ptr->isSelected();
}

QListWidget* qt_widgets_c_QListWidgetItem_listWidget(const QListWidgetItem* this_ptr) {
  return this_ptr->listWidget();
}

QListWidgetItem* qt_widgets_c_QListWidgetItem_new_icon_text(const QIcon* icon, const QString* text) {
  return new QListWidgetItem(*icon, *text);
}

QListWidgetItem* qt_widgets_c_QListWidgetItem_new_icon_text_view(const QIcon* icon, const QString* text, QListWidget* view) {
  return new QListWidgetItem(*icon, *text, view);
}

QListWidgetItem* qt_widgets_c_QListWidgetItem_new_icon_text_view_type(const QIcon* icon, const QString* text, QListWidget* view, int type) {
  return new QListWidgetItem(*icon, *text, view, type);
}

QListWidgetItem* qt_widgets_c_QListWidgetItem_new_no_args() {
  return new QListWidgetItem();
}

QListWidgetItem* qt_widgets_c_QListWidgetItem_new_other(const QListWidgetItem* other) {
  return new QListWidgetItem(*other);
}

QListWidgetItem* qt_widgets_c_QListWidgetItem_new_text(const QString* text) {
  return new QListWidgetItem(*text);
}

QListWidgetItem* qt_widgets_c_QListWidgetItem_new_text_view(const QString* text, QListWidget* view) {
  return new QListWidgetItem(*text, view);
}

QListWidgetItem* qt_widgets_c_QListWidgetItem_new_text_view_type(const QString* text, QListWidget* view, int type) {
  return new QListWidgetItem(*text, view, type);
}

QListWidgetItem* qt_widgets_c_QListWidgetItem_new_view(QListWidget* view) {
  return new QListWidgetItem(view);
}

QListWidgetItem* qt_widgets_c_QListWidgetItem_new_view_type(QListWidget* view, int type) {
  return new QListWidgetItem(view, type);
}

QListWidgetItem* qt_widgets_c_QListWidgetItem_operator_assign(QListWidgetItem* this_ptr, const QListWidgetItem* other) {
  return &this_ptr->operator=(*other);
}

bool qt_widgets_c_QListWidgetItem_operator_lt(const QListWidgetItem* this_ptr, const QListWidgetItem* other) {
  return this_ptr->operator<(*other);
}

void qt_widgets_c_QListWidgetItem_read(QListWidgetItem* this_ptr, QDataStream* in) {
  this_ptr->read(*in);
}

void qt_widgets_c_QListWidgetItem_setBackground(QListWidgetItem* this_ptr, const QBrush* brush) {
  this_ptr->setBackground(*brush);
}

void qt_widgets_c_QListWidgetItem_setBackgroundColor(QListWidgetItem* this_ptr, const QColor* color) {
  this_ptr->setBackgroundColor(*color);
}

void qt_widgets_c_QListWidgetItem_setCheckState(QListWidgetItem* this_ptr, const Qt::CheckState* state) {
  this_ptr->setCheckState(*state);
}

void qt_widgets_c_QListWidgetItem_setData(QListWidgetItem* this_ptr, int role, const QVariant* value) {
  this_ptr->setData(role, *value);
}

void qt_widgets_c_QListWidgetItem_setFont(QListWidgetItem* this_ptr, const QFont* font) {
  this_ptr->setFont(*font);
}

void qt_widgets_c_QListWidgetItem_setForeground(QListWidgetItem* this_ptr, const QBrush* brush) {
  this_ptr->setForeground(*brush);
}

void qt_widgets_c_QListWidgetItem_setHidden(QListWidgetItem* this_ptr, bool hide) {
  this_ptr->setHidden(hide);
}

void qt_widgets_c_QListWidgetItem_setIcon(QListWidgetItem* this_ptr, const QIcon* icon) {
  this_ptr->setIcon(*icon);
}

void qt_widgets_c_QListWidgetItem_setSelected(QListWidgetItem* this_ptr, bool select) {
  this_ptr->setSelected(select);
}

void qt_widgets_c_QListWidgetItem_setSizeHint(QListWidgetItem* this_ptr, const QSize* size) {
  this_ptr->setSizeHint(*size);
}

void qt_widgets_c_QListWidgetItem_setStatusTip(QListWidgetItem* this_ptr, const QString* statusTip) {
  this_ptr->setStatusTip(*statusTip);
}

void qt_widgets_c_QListWidgetItem_setText(QListWidgetItem* this_ptr, const QString* text) {
  this_ptr->setText(*text);
}

void qt_widgets_c_QListWidgetItem_setTextAlignment(QListWidgetItem* this_ptr, int alignment) {
  this_ptr->setTextAlignment(alignment);
}

void qt_widgets_c_QListWidgetItem_setTextColor(QListWidgetItem* this_ptr, const QColor* color) {
  this_ptr->setTextColor(*color);
}

void qt_widgets_c_QListWidgetItem_setToolTip(QListWidgetItem* this_ptr, const QString* toolTip) {
  this_ptr->setToolTip(*toolTip);
}

void qt_widgets_c_QListWidgetItem_setWhatsThis(QListWidgetItem* this_ptr, const QString* whatsThis) {
  this_ptr->setWhatsThis(*whatsThis);
}

void qt_widgets_c_QListWidgetItem_sizeHint_to_output(const QListWidgetItem* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QListWidgetItem_statusTip_to_output(const QListWidgetItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->statusTip());
}

int qt_widgets_c_QListWidgetItem_textAlignment(const QListWidgetItem* this_ptr) {
  return this_ptr->textAlignment();
}

void qt_widgets_c_QListWidgetItem_textColor_to_output(const QListWidgetItem* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->textColor());
}

void qt_widgets_c_QListWidgetItem_text_to_output(const QListWidgetItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_widgets_c_QListWidgetItem_toolTip_to_output(const QListWidgetItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->toolTip());
}

int qt_widgets_c_QListWidgetItem_type(const QListWidgetItem* this_ptr) {
  return this_ptr->type();
}

void qt_widgets_c_QListWidgetItem_whatsThis_to_output(const QListWidgetItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->whatsThis());
}

void qt_widgets_c_QListWidgetItem_write(const QListWidgetItem* this_ptr, QDataStream* out) {
  this_ptr->write(*out);
}

