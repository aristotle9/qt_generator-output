#include "qt_widgets_c_QTreeWidgetItem.h"

void qt_widgets_c_QTreeWidgetItem_addChild(QTreeWidgetItem* this_ptr, QTreeWidgetItem* child) {
  this_ptr->addChild(child);
}

void qt_widgets_c_QTreeWidgetItem_addChildren(QTreeWidgetItem* this_ptr, const QList< QTreeWidgetItem* >* children) {
  this_ptr->addChildren(*children);
}

void qt_widgets_c_QTreeWidgetItem_backgroundColor_to_output(const QTreeWidgetItem* this_ptr, int column, QColor* output) {
  new(output) QColor(this_ptr->backgroundColor(column));
}

void qt_widgets_c_QTreeWidgetItem_background_to_output(const QTreeWidgetItem* this_ptr, int column, QBrush* output) {
  new(output) QBrush(this_ptr->background(column));
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_child(const QTreeWidgetItem* this_ptr, int index) {
  return this_ptr->child(index);
}

int qt_widgets_c_QTreeWidgetItem_childCount(const QTreeWidgetItem* this_ptr) {
  return this_ptr->childCount();
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_clone(const QTreeWidgetItem* this_ptr) {
  return this_ptr->clone();
}

int qt_widgets_c_QTreeWidgetItem_columnCount(const QTreeWidgetItem* this_ptr) {
  return this_ptr->columnCount();
}

void qt_widgets_c_QTreeWidgetItem_data_to_output(const QTreeWidgetItem* this_ptr, int column, int role, QVariant* output) {
  new(output) QVariant(this_ptr->data(column, role));
}

void qt_widgets_c_QTreeWidgetItem_delete(QTreeWidgetItem* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QTreeWidgetItem_font_to_output(const QTreeWidgetItem* this_ptr, int column, QFont* output) {
  new(output) QFont(this_ptr->font(column));
}

void qt_widgets_c_QTreeWidgetItem_foreground_to_output(const QTreeWidgetItem* this_ptr, int column, QBrush* output) {
  new(output) QBrush(this_ptr->foreground(column));
}

void qt_widgets_c_QTreeWidgetItem_icon_to_output(const QTreeWidgetItem* this_ptr, int column, QIcon* output) {
  new(output) QIcon(this_ptr->icon(column));
}

int qt_widgets_c_QTreeWidgetItem_indexOfChild(const QTreeWidgetItem* this_ptr, QTreeWidgetItem* child) {
  return this_ptr->indexOfChild(child);
}

void qt_widgets_c_QTreeWidgetItem_insertChild(QTreeWidgetItem* this_ptr, int index, QTreeWidgetItem* child) {
  this_ptr->insertChild(index, child);
}

void qt_widgets_c_QTreeWidgetItem_insertChildren(QTreeWidgetItem* this_ptr, int index, const QList< QTreeWidgetItem* >* children) {
  this_ptr->insertChildren(index, *children);
}

bool qt_widgets_c_QTreeWidgetItem_isDisabled(const QTreeWidgetItem* this_ptr) {
  return this_ptr->isDisabled();
}

bool qt_widgets_c_QTreeWidgetItem_isExpanded(const QTreeWidgetItem* this_ptr) {
  return this_ptr->isExpanded();
}

bool qt_widgets_c_QTreeWidgetItem_isFirstColumnSpanned(const QTreeWidgetItem* this_ptr) {
  return this_ptr->isFirstColumnSpanned();
}

bool qt_widgets_c_QTreeWidgetItem_isHidden(const QTreeWidgetItem* this_ptr) {
  return this_ptr->isHidden();
}

bool qt_widgets_c_QTreeWidgetItem_isSelected(const QTreeWidgetItem* this_ptr) {
  return this_ptr->isSelected();
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_no_args() {
  return new QTreeWidgetItem();
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_other(const QTreeWidgetItem* other) {
  return new QTreeWidgetItem(*other);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_parent(QTreeWidgetItem* parent) {
  return new QTreeWidgetItem(parent);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_parent_after(QTreeWidgetItem* parent, QTreeWidgetItem* after) {
  return new QTreeWidgetItem(parent, after);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_parent_after_type(QTreeWidgetItem* parent, QTreeWidgetItem* after, int type) {
  return new QTreeWidgetItem(parent, after, type);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_parent_strings(QTreeWidgetItem* parent, const QStringList* strings) {
  return new QTreeWidgetItem(parent, *strings);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_parent_strings_type(QTreeWidgetItem* parent, const QStringList* strings, int type) {
  return new QTreeWidgetItem(parent, *strings, type);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_parent_type(QTreeWidgetItem* parent, int type) {
  return new QTreeWidgetItem(parent, type);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_strings(const QStringList* strings) {
  return new QTreeWidgetItem(*strings);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_strings_type(const QStringList* strings, int type) {
  return new QTreeWidgetItem(*strings, type);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_type(int type) {
  return new QTreeWidgetItem(type);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_view(QTreeWidget* view) {
  return new QTreeWidgetItem(view);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_view_after(QTreeWidget* view, QTreeWidgetItem* after) {
  return new QTreeWidgetItem(view, after);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_view_after_type(QTreeWidget* view, QTreeWidgetItem* after, int type) {
  return new QTreeWidgetItem(view, after, type);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_view_strings(QTreeWidget* view, const QStringList* strings) {
  return new QTreeWidgetItem(view, *strings);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_view_strings_type(QTreeWidget* view, const QStringList* strings, int type) {
  return new QTreeWidgetItem(view, *strings, type);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_view_type(QTreeWidget* view, int type) {
  return new QTreeWidgetItem(view, type);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_operator_assign(QTreeWidgetItem* this_ptr, const QTreeWidgetItem* other) {
  return &this_ptr->operator=(*other);
}

bool qt_widgets_c_QTreeWidgetItem_operator_lt(const QTreeWidgetItem* this_ptr, const QTreeWidgetItem* other) {
  return this_ptr->operator<(*other);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_parent(const QTreeWidgetItem* this_ptr) {
  return this_ptr->parent();
}

void qt_widgets_c_QTreeWidgetItem_read(QTreeWidgetItem* this_ptr, QDataStream* in) {
  this_ptr->read(*in);
}

void qt_widgets_c_QTreeWidgetItem_removeChild(QTreeWidgetItem* this_ptr, QTreeWidgetItem* child) {
  this_ptr->removeChild(child);
}

void qt_widgets_c_QTreeWidgetItem_setBackground(QTreeWidgetItem* this_ptr, int column, const QBrush* brush) {
  this_ptr->setBackground(column, *brush);
}

void qt_widgets_c_QTreeWidgetItem_setBackgroundColor(QTreeWidgetItem* this_ptr, int column, const QColor* color) {
  this_ptr->setBackgroundColor(column, *color);
}

void qt_widgets_c_QTreeWidgetItem_setCheckState(QTreeWidgetItem* this_ptr, int column, const Qt::CheckState* state) {
  this_ptr->setCheckState(column, *state);
}

void qt_widgets_c_QTreeWidgetItem_setChildIndicatorPolicy(QTreeWidgetItem* this_ptr, const QTreeWidgetItem::ChildIndicatorPolicy* policy) {
  this_ptr->setChildIndicatorPolicy(*policy);
}

void qt_widgets_c_QTreeWidgetItem_setData(QTreeWidgetItem* this_ptr, int column, int role, const QVariant* value) {
  this_ptr->setData(column, role, *value);
}

void qt_widgets_c_QTreeWidgetItem_setDisabled(QTreeWidgetItem* this_ptr, bool disabled) {
  this_ptr->setDisabled(disabled);
}

void qt_widgets_c_QTreeWidgetItem_setExpanded(QTreeWidgetItem* this_ptr, bool expand) {
  this_ptr->setExpanded(expand);
}

void qt_widgets_c_QTreeWidgetItem_setFirstColumnSpanned(QTreeWidgetItem* this_ptr, bool span) {
  this_ptr->setFirstColumnSpanned(span);
}

void qt_widgets_c_QTreeWidgetItem_setFont(QTreeWidgetItem* this_ptr, int column, const QFont* font) {
  this_ptr->setFont(column, *font);
}

void qt_widgets_c_QTreeWidgetItem_setForeground(QTreeWidgetItem* this_ptr, int column, const QBrush* brush) {
  this_ptr->setForeground(column, *brush);
}

void qt_widgets_c_QTreeWidgetItem_setHidden(QTreeWidgetItem* this_ptr, bool hide) {
  this_ptr->setHidden(hide);
}

void qt_widgets_c_QTreeWidgetItem_setIcon(QTreeWidgetItem* this_ptr, int column, const QIcon* icon) {
  this_ptr->setIcon(column, *icon);
}

void qt_widgets_c_QTreeWidgetItem_setSelected(QTreeWidgetItem* this_ptr, bool select) {
  this_ptr->setSelected(select);
}

void qt_widgets_c_QTreeWidgetItem_setSizeHint(QTreeWidgetItem* this_ptr, int column, const QSize* size) {
  this_ptr->setSizeHint(column, *size);
}

void qt_widgets_c_QTreeWidgetItem_setStatusTip(QTreeWidgetItem* this_ptr, int column, const QString* statusTip) {
  this_ptr->setStatusTip(column, *statusTip);
}

void qt_widgets_c_QTreeWidgetItem_setText(QTreeWidgetItem* this_ptr, int column, const QString* text) {
  this_ptr->setText(column, *text);
}

void qt_widgets_c_QTreeWidgetItem_setTextAlignment(QTreeWidgetItem* this_ptr, int column, int alignment) {
  this_ptr->setTextAlignment(column, alignment);
}

void qt_widgets_c_QTreeWidgetItem_setTextColor(QTreeWidgetItem* this_ptr, int column, const QColor* color) {
  this_ptr->setTextColor(column, *color);
}

void qt_widgets_c_QTreeWidgetItem_setToolTip(QTreeWidgetItem* this_ptr, int column, const QString* toolTip) {
  this_ptr->setToolTip(column, *toolTip);
}

void qt_widgets_c_QTreeWidgetItem_setWhatsThis(QTreeWidgetItem* this_ptr, int column, const QString* whatsThis) {
  this_ptr->setWhatsThis(column, *whatsThis);
}

void qt_widgets_c_QTreeWidgetItem_sizeHint_to_output(const QTreeWidgetItem* this_ptr, int column, QSize* output) {
  new(output) QSize(this_ptr->sizeHint(column));
}

void qt_widgets_c_QTreeWidgetItem_sortChildren(QTreeWidgetItem* this_ptr, int column, const Qt::SortOrder* order) {
  this_ptr->sortChildren(column, *order);
}

void qt_widgets_c_QTreeWidgetItem_statusTip_to_output(const QTreeWidgetItem* this_ptr, int column, QString* output) {
  new(output) QString(this_ptr->statusTip(column));
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_takeChild(QTreeWidgetItem* this_ptr, int index) {
  return this_ptr->takeChild(index);
}

void qt_widgets_c_QTreeWidgetItem_takeChildren_to_output(QTreeWidgetItem* this_ptr, QList< QTreeWidgetItem* >* output) {
  new(output) QList< QTreeWidgetItem* >(this_ptr->takeChildren());
}

int qt_widgets_c_QTreeWidgetItem_textAlignment(const QTreeWidgetItem* this_ptr, int column) {
  return this_ptr->textAlignment(column);
}

void qt_widgets_c_QTreeWidgetItem_textColor_to_output(const QTreeWidgetItem* this_ptr, int column, QColor* output) {
  new(output) QColor(this_ptr->textColor(column));
}

void qt_widgets_c_QTreeWidgetItem_text_to_output(const QTreeWidgetItem* this_ptr, int column, QString* output) {
  new(output) QString(this_ptr->text(column));
}

void qt_widgets_c_QTreeWidgetItem_toolTip_to_output(const QTreeWidgetItem* this_ptr, int column, QString* output) {
  new(output) QString(this_ptr->toolTip(column));
}

QTreeWidget* qt_widgets_c_QTreeWidgetItem_treeWidget(const QTreeWidgetItem* this_ptr) {
  return this_ptr->treeWidget();
}

int qt_widgets_c_QTreeWidgetItem_type(const QTreeWidgetItem* this_ptr) {
  return this_ptr->type();
}

void qt_widgets_c_QTreeWidgetItem_whatsThis_to_output(const QTreeWidgetItem* this_ptr, int column, QString* output) {
  new(output) QString(this_ptr->whatsThis(column));
}

void qt_widgets_c_QTreeWidgetItem_write(const QTreeWidgetItem* this_ptr, QDataStream* out) {
  this_ptr->write(*out);
}

