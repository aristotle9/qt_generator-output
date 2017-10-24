#include "qt_widgets_c_QTableWidgetItem.h"

void qt_widgets_c_QTableWidgetItem_backgroundColor_to_output(const QTableWidgetItem* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->backgroundColor());
}

void qt_widgets_c_QTableWidgetItem_background_to_output(const QTableWidgetItem* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->background());
}

QTableWidgetItem* qt_widgets_c_QTableWidgetItem_clone(const QTableWidgetItem* this_ptr) {
  return this_ptr->clone();
}

int qt_widgets_c_QTableWidgetItem_column(const QTableWidgetItem* this_ptr) {
  return this_ptr->column();
}

void qt_widgets_c_QTableWidgetItem_data_to_output(const QTableWidgetItem* this_ptr, int role, QVariant* output) {
  new(output) QVariant(this_ptr->data(role));
}

void qt_widgets_c_QTableWidgetItem_delete(QTableWidgetItem* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QTableWidgetItem_font_to_output(const QTableWidgetItem* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->font());
}

void qt_widgets_c_QTableWidgetItem_foreground_to_output(const QTableWidgetItem* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->foreground());
}

void qt_widgets_c_QTableWidgetItem_icon_to_output(const QTableWidgetItem* this_ptr, QIcon* output) {
  new(output) QIcon(this_ptr->icon());
}

bool qt_widgets_c_QTableWidgetItem_isSelected(const QTableWidgetItem* this_ptr) {
  return this_ptr->isSelected();
}

QTableWidgetItem* qt_widgets_c_QTableWidgetItem_new_icon_text(const QIcon* icon, const QString* text) {
  return new QTableWidgetItem(*icon, *text);
}

QTableWidgetItem* qt_widgets_c_QTableWidgetItem_new_icon_text_type(const QIcon* icon, const QString* text, int type) {
  return new QTableWidgetItem(*icon, *text, type);
}

QTableWidgetItem* qt_widgets_c_QTableWidgetItem_new_no_args() {
  return new QTableWidgetItem();
}

QTableWidgetItem* qt_widgets_c_QTableWidgetItem_new_other(const QTableWidgetItem* other) {
  return new QTableWidgetItem(*other);
}

QTableWidgetItem* qt_widgets_c_QTableWidgetItem_new_text(const QString* text) {
  return new QTableWidgetItem(*text);
}

QTableWidgetItem* qt_widgets_c_QTableWidgetItem_new_text_type(const QString* text, int type) {
  return new QTableWidgetItem(*text, type);
}

QTableWidgetItem* qt_widgets_c_QTableWidgetItem_new_type(int type) {
  return new QTableWidgetItem(type);
}

QTableWidgetItem* qt_widgets_c_QTableWidgetItem_operator_assign(QTableWidgetItem* this_ptr, const QTableWidgetItem* other) {
  return &this_ptr->operator=(*other);
}

bool qt_widgets_c_QTableWidgetItem_operator_lt(const QTableWidgetItem* this_ptr, const QTableWidgetItem* other) {
  return this_ptr->operator<(*other);
}

void qt_widgets_c_QTableWidgetItem_read(QTableWidgetItem* this_ptr, QDataStream* in) {
  this_ptr->read(*in);
}

int qt_widgets_c_QTableWidgetItem_row(const QTableWidgetItem* this_ptr) {
  return this_ptr->row();
}

void qt_widgets_c_QTableWidgetItem_setBackground(QTableWidgetItem* this_ptr, const QBrush* brush) {
  this_ptr->setBackground(*brush);
}

void qt_widgets_c_QTableWidgetItem_setBackgroundColor(QTableWidgetItem* this_ptr, const QColor* color) {
  this_ptr->setBackgroundColor(*color);
}

void qt_widgets_c_QTableWidgetItem_setCheckState(QTableWidgetItem* this_ptr, const Qt::CheckState* state) {
  this_ptr->setCheckState(*state);
}

void qt_widgets_c_QTableWidgetItem_setData(QTableWidgetItem* this_ptr, int role, const QVariant* value) {
  this_ptr->setData(role, *value);
}

void qt_widgets_c_QTableWidgetItem_setFont(QTableWidgetItem* this_ptr, const QFont* font) {
  this_ptr->setFont(*font);
}

void qt_widgets_c_QTableWidgetItem_setForeground(QTableWidgetItem* this_ptr, const QBrush* brush) {
  this_ptr->setForeground(*brush);
}

void qt_widgets_c_QTableWidgetItem_setIcon(QTableWidgetItem* this_ptr, const QIcon* icon) {
  this_ptr->setIcon(*icon);
}

void qt_widgets_c_QTableWidgetItem_setSelected(QTableWidgetItem* this_ptr, bool select) {
  this_ptr->setSelected(select);
}

void qt_widgets_c_QTableWidgetItem_setSizeHint(QTableWidgetItem* this_ptr, const QSize* size) {
  this_ptr->setSizeHint(*size);
}

void qt_widgets_c_QTableWidgetItem_setStatusTip(QTableWidgetItem* this_ptr, const QString* statusTip) {
  this_ptr->setStatusTip(*statusTip);
}

void qt_widgets_c_QTableWidgetItem_setText(QTableWidgetItem* this_ptr, const QString* text) {
  this_ptr->setText(*text);
}

void qt_widgets_c_QTableWidgetItem_setTextAlignment(QTableWidgetItem* this_ptr, int alignment) {
  this_ptr->setTextAlignment(alignment);
}

void qt_widgets_c_QTableWidgetItem_setTextColor(QTableWidgetItem* this_ptr, const QColor* color) {
  this_ptr->setTextColor(*color);
}

void qt_widgets_c_QTableWidgetItem_setToolTip(QTableWidgetItem* this_ptr, const QString* toolTip) {
  this_ptr->setToolTip(*toolTip);
}

void qt_widgets_c_QTableWidgetItem_setWhatsThis(QTableWidgetItem* this_ptr, const QString* whatsThis) {
  this_ptr->setWhatsThis(*whatsThis);
}

void qt_widgets_c_QTableWidgetItem_sizeHint_to_output(const QTableWidgetItem* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QTableWidgetItem_statusTip_to_output(const QTableWidgetItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->statusTip());
}

QTableWidget* qt_widgets_c_QTableWidgetItem_tableWidget(const QTableWidgetItem* this_ptr) {
  return this_ptr->tableWidget();
}

int qt_widgets_c_QTableWidgetItem_textAlignment(const QTableWidgetItem* this_ptr) {
  return this_ptr->textAlignment();
}

void qt_widgets_c_QTableWidgetItem_textColor_to_output(const QTableWidgetItem* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->textColor());
}

void qt_widgets_c_QTableWidgetItem_text_to_output(const QTableWidgetItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_widgets_c_QTableWidgetItem_toolTip_to_output(const QTableWidgetItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->toolTip());
}

int qt_widgets_c_QTableWidgetItem_type(const QTableWidgetItem* this_ptr) {
  return this_ptr->type();
}

void qt_widgets_c_QTableWidgetItem_whatsThis_to_output(const QTableWidgetItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->whatsThis());
}

void qt_widgets_c_QTableWidgetItem_write(const QTableWidgetItem* this_ptr, QDataStream* out) {
  this_ptr->write(*out);
}

