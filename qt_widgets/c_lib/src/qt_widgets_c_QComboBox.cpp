#include "qt_widgets_c_QComboBox.h"

QComboBox* qt_widgets_c_QComboBox_G_dynamic_cast_QComboBox_ptr(QWidget* ptr) {
  return dynamic_cast<QComboBox*>(ptr);
}

QComboBox* qt_widgets_c_QComboBox_G_static_cast_QComboBox_ptr_QObject(QObject* ptr) {
  return static_cast<QComboBox*>(ptr);
}

QComboBox* qt_widgets_c_QComboBox_G_static_cast_QComboBox_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QComboBox*>(ptr);
}

QComboBox* qt_widgets_c_QComboBox_G_static_cast_QComboBox_ptr_QWidget(QWidget* ptr) {
  return static_cast<QComboBox*>(ptr);
}

QObject* qt_widgets_c_QComboBox_G_static_cast_QObject_ptr(QComboBox* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QComboBox_G_static_cast_QPaintDevice_ptr(QComboBox* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QComboBox_G_static_cast_QWidget_ptr(QComboBox* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QComboBox_addItem_icon_text(QComboBox* this_ptr, const QIcon* icon, const QString* text) {
  this_ptr->addItem(*icon, *text);
}

void qt_widgets_c_QComboBox_addItem_icon_text_userData(QComboBox* this_ptr, const QIcon* icon, const QString* text, const QVariant* userData) {
  this_ptr->addItem(*icon, *text, *userData);
}

void qt_widgets_c_QComboBox_addItem_text(QComboBox* this_ptr, const QString* text) {
  this_ptr->addItem(*text);
}

void qt_widgets_c_QComboBox_addItem_text_userData(QComboBox* this_ptr, const QString* text, const QVariant* userData) {
  this_ptr->addItem(*text, *userData);
}

void qt_widgets_c_QComboBox_addItems(QComboBox* this_ptr, const QStringList* texts) {
  this_ptr->addItems(*texts);
}

bool qt_widgets_c_QComboBox_autoCompletion(const QComboBox* this_ptr) {
  return this_ptr->autoCompletion();
}

void qt_widgets_c_QComboBox_clear(QComboBox* this_ptr) {
  this_ptr->clear();
}

void qt_widgets_c_QComboBox_clearEditText(QComboBox* this_ptr) {
  this_ptr->clearEditText();
}

QCompleter* qt_widgets_c_QComboBox_completer(const QComboBox* this_ptr) {
  return this_ptr->completer();
}

int qt_widgets_c_QComboBox_count(const QComboBox* this_ptr) {
  return this_ptr->count();
}

void qt_widgets_c_QComboBox_currentData_to_output_no_args(const QComboBox* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->currentData());
}

void qt_widgets_c_QComboBox_currentData_to_output_role(const QComboBox* this_ptr, int role, QVariant* output) {
  new(output) QVariant(this_ptr->currentData(role));
}

int qt_widgets_c_QComboBox_currentIndex(const QComboBox* this_ptr) {
  return this_ptr->currentIndex();
}

void qt_widgets_c_QComboBox_currentText_to_output(const QComboBox* this_ptr, QString* output) {
  new(output) QString(this_ptr->currentText());
}

void qt_widgets_c_QComboBox_delete(QComboBox* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QComboBox_duplicatesEnabled(const QComboBox* this_ptr) {
  return this_ptr->duplicatesEnabled();
}

bool qt_widgets_c_QComboBox_event(QComboBox* this_ptr, QEvent* event) {
  return this_ptr->event(event);
}

bool qt_widgets_c_QComboBox_hasFrame(const QComboBox* this_ptr) {
  return this_ptr->hasFrame();
}

void qt_widgets_c_QComboBox_hidePopup(QComboBox* this_ptr) {
  this_ptr->hidePopup();
}

void qt_widgets_c_QComboBox_iconSize_to_output(const QComboBox* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->iconSize());
}

void qt_widgets_c_QComboBox_inputMethodQuery_to_output_arg1(const QComboBox* this_ptr, const Qt::InputMethodQuery* arg1, QVariant* output) {
  new(output) QVariant(this_ptr->inputMethodQuery(*arg1));
}

void qt_widgets_c_QComboBox_inputMethodQuery_to_output_query_argument(const QComboBox* this_ptr, const Qt::InputMethodQuery* query, const QVariant* argument, QVariant* output) {
  new(output) QVariant(this_ptr->inputMethodQuery(*query, *argument));
}

void qt_widgets_c_QComboBox_insertItem_index_icon_text(QComboBox* this_ptr, int index, const QIcon* icon, const QString* text) {
  this_ptr->insertItem(index, *icon, *text);
}

void qt_widgets_c_QComboBox_insertItem_index_icon_text_userData(QComboBox* this_ptr, int index, const QIcon* icon, const QString* text, const QVariant* userData) {
  this_ptr->insertItem(index, *icon, *text, *userData);
}

void qt_widgets_c_QComboBox_insertItem_index_text(QComboBox* this_ptr, int index, const QString* text) {
  this_ptr->insertItem(index, *text);
}

void qt_widgets_c_QComboBox_insertItem_index_text_userData(QComboBox* this_ptr, int index, const QString* text, const QVariant* userData) {
  this_ptr->insertItem(index, *text, *userData);
}

void qt_widgets_c_QComboBox_insertItems(QComboBox* this_ptr, int index, const QStringList* texts) {
  this_ptr->insertItems(index, *texts);
}

QComboBox::InsertPolicy qt_widgets_c_QComboBox_insertPolicy(const QComboBox* this_ptr) {
  return this_ptr->insertPolicy();
}

void qt_widgets_c_QComboBox_insertSeparator(QComboBox* this_ptr, int index) {
  this_ptr->insertSeparator(index);
}

bool qt_widgets_c_QComboBox_isEditable(const QComboBox* this_ptr) {
  return this_ptr->isEditable();
}

void qt_widgets_c_QComboBox_itemData_to_output_index(const QComboBox* this_ptr, int index, QVariant* output) {
  new(output) QVariant(this_ptr->itemData(index));
}

void qt_widgets_c_QComboBox_itemData_to_output_index_role(const QComboBox* this_ptr, int index, int role, QVariant* output) {
  new(output) QVariant(this_ptr->itemData(index, role));
}

QAbstractItemDelegate* qt_widgets_c_QComboBox_itemDelegate(const QComboBox* this_ptr) {
  return this_ptr->itemDelegate();
}

void qt_widgets_c_QComboBox_itemIcon_to_output(const QComboBox* this_ptr, int index, QIcon* output) {
  new(output) QIcon(this_ptr->itemIcon(index));
}

void qt_widgets_c_QComboBox_itemText_to_output(const QComboBox* this_ptr, int index, QString* output) {
  new(output) QString(this_ptr->itemText(index));
}

QLineEdit* qt_widgets_c_QComboBox_lineEdit(const QComboBox* this_ptr) {
  return this_ptr->lineEdit();
}

int qt_widgets_c_QComboBox_maxCount(const QComboBox* this_ptr) {
  return this_ptr->maxCount();
}

int qt_widgets_c_QComboBox_maxVisibleItems(const QComboBox* this_ptr) {
  return this_ptr->maxVisibleItems();
}

const QMetaObject* qt_widgets_c_QComboBox_metaObject(const QComboBox* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QComboBox_minimumContentsLength(const QComboBox* this_ptr) {
  return this_ptr->minimumContentsLength();
}

void qt_widgets_c_QComboBox_minimumSizeHint_to_output(const QComboBox* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

QAbstractItemModel* qt_widgets_c_QComboBox_model(const QComboBox* this_ptr) {
  return this_ptr->model();
}

int qt_widgets_c_QComboBox_modelColumn(const QComboBox* this_ptr) {
  return this_ptr->modelColumn();
}

QComboBox* qt_widgets_c_QComboBox_new_no_args() {
  return new QComboBox();
}

QComboBox* qt_widgets_c_QComboBox_new_parent(QWidget* parent) {
  return new QComboBox(parent);
}

int qt_widgets_c_QComboBox_qt_metacall(QComboBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QComboBox_qt_metacast(QComboBox* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QComboBox_removeItem(QComboBox* this_ptr, int index) {
  this_ptr->removeItem(index);
}

void qt_widgets_c_QComboBox_rootModelIndex_to_output(const QComboBox* this_ptr, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->rootModelIndex());
}

void qt_widgets_c_QComboBox_setAutoCompletion(QComboBox* this_ptr, bool enable) {
  this_ptr->setAutoCompletion(enable);
}

void qt_widgets_c_QComboBox_setAutoCompletionCaseSensitivity(QComboBox* this_ptr, const Qt::CaseSensitivity* sensitivity) {
  this_ptr->setAutoCompletionCaseSensitivity(*sensitivity);
}

void qt_widgets_c_QComboBox_setCompleter(QComboBox* this_ptr, QCompleter* c) {
  this_ptr->setCompleter(c);
}

void qt_widgets_c_QComboBox_setCurrentIndex(QComboBox* this_ptr, int index) {
  this_ptr->setCurrentIndex(index);
}

void qt_widgets_c_QComboBox_setCurrentText(QComboBox* this_ptr, const QString* text) {
  this_ptr->setCurrentText(*text);
}

void qt_widgets_c_QComboBox_setDuplicatesEnabled(QComboBox* this_ptr, bool enable) {
  this_ptr->setDuplicatesEnabled(enable);
}

void qt_widgets_c_QComboBox_setEditText(QComboBox* this_ptr, const QString* text) {
  this_ptr->setEditText(*text);
}

void qt_widgets_c_QComboBox_setEditable(QComboBox* this_ptr, bool editable) {
  this_ptr->setEditable(editable);
}

void qt_widgets_c_QComboBox_setFrame(QComboBox* this_ptr, bool arg1) {
  this_ptr->setFrame(arg1);
}

void qt_widgets_c_QComboBox_setIconSize(QComboBox* this_ptr, const QSize* size) {
  this_ptr->setIconSize(*size);
}

void qt_widgets_c_QComboBox_setInsertPolicy(QComboBox* this_ptr, QComboBox::InsertPolicy policy) {
  this_ptr->setInsertPolicy(policy);
}

void qt_widgets_c_QComboBox_setItemData_index_value(QComboBox* this_ptr, int index, const QVariant* value) {
  this_ptr->setItemData(index, *value);
}

void qt_widgets_c_QComboBox_setItemData_index_value_role(QComboBox* this_ptr, int index, const QVariant* value, int role) {
  this_ptr->setItemData(index, *value, role);
}

void qt_widgets_c_QComboBox_setItemDelegate(QComboBox* this_ptr, QAbstractItemDelegate* delegate) {
  this_ptr->setItemDelegate(delegate);
}

void qt_widgets_c_QComboBox_setItemIcon(QComboBox* this_ptr, int index, const QIcon* icon) {
  this_ptr->setItemIcon(index, *icon);
}

void qt_widgets_c_QComboBox_setItemText(QComboBox* this_ptr, int index, const QString* text) {
  this_ptr->setItemText(index, *text);
}

void qt_widgets_c_QComboBox_setLineEdit(QComboBox* this_ptr, QLineEdit* edit) {
  this_ptr->setLineEdit(edit);
}

void qt_widgets_c_QComboBox_setMaxCount(QComboBox* this_ptr, int max) {
  this_ptr->setMaxCount(max);
}

void qt_widgets_c_QComboBox_setMaxVisibleItems(QComboBox* this_ptr, int maxItems) {
  this_ptr->setMaxVisibleItems(maxItems);
}

void qt_widgets_c_QComboBox_setMinimumContentsLength(QComboBox* this_ptr, int characters) {
  this_ptr->setMinimumContentsLength(characters);
}

void qt_widgets_c_QComboBox_setModel(QComboBox* this_ptr, QAbstractItemModel* model) {
  this_ptr->setModel(model);
}

void qt_widgets_c_QComboBox_setModelColumn(QComboBox* this_ptr, int visibleColumn) {
  this_ptr->setModelColumn(visibleColumn);
}

void qt_widgets_c_QComboBox_setRootModelIndex(QComboBox* this_ptr, const QModelIndex* index) {
  this_ptr->setRootModelIndex(*index);
}

void qt_widgets_c_QComboBox_setSizeAdjustPolicy(QComboBox* this_ptr, QComboBox::SizeAdjustPolicy policy) {
  this_ptr->setSizeAdjustPolicy(policy);
}

void qt_widgets_c_QComboBox_setValidator(QComboBox* this_ptr, const QValidator* v) {
  this_ptr->setValidator(v);
}

void qt_widgets_c_QComboBox_setView(QComboBox* this_ptr, QAbstractItemView* itemView) {
  this_ptr->setView(itemView);
}

void qt_widgets_c_QComboBox_showPopup(QComboBox* this_ptr) {
  this_ptr->showPopup();
}

QComboBox::SizeAdjustPolicy qt_widgets_c_QComboBox_sizeAdjustPolicy(const QComboBox* this_ptr) {
  return this_ptr->sizeAdjustPolicy();
}

void qt_widgets_c_QComboBox_sizeHint_to_output(const QComboBox* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QComboBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QComboBox::trUtf8(s, c, n));
}

void qt_widgets_c_QComboBox_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QComboBox::tr(s, c, n));
}

const QValidator* qt_widgets_c_QComboBox_validator(const QComboBox* this_ptr) {
  return this_ptr->validator();
}

QAbstractItemView* qt_widgets_c_QComboBox_view(const QComboBox* this_ptr) {
  return this_ptr->view();
}

