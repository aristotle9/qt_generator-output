#ifndef QT_WIDGETS_C_QCOMBOBOX_H
#define QT_WIDGETS_C_QCOMBOBOX_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QComboBox* qt_widgets_c_QComboBox_G_dynamic_cast_QComboBox_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QComboBox* qt_widgets_c_QComboBox_G_static_cast_QComboBox_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QComboBox* qt_widgets_c_QComboBox_G_static_cast_QComboBox_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QComboBox* qt_widgets_c_QComboBox_G_static_cast_QComboBox_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QComboBox_G_static_cast_QObject_ptr(QComboBox* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QComboBox_G_static_cast_QPaintDevice_ptr(QComboBox* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QComboBox_G_static_cast_QWidget_ptr(QComboBox* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_addItem_icon_text(QComboBox* this_ptr, const QIcon* icon, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_addItem_icon_text_userData(QComboBox* this_ptr, const QIcon* icon, const QString* text, const QVariant* userData);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_addItem_text(QComboBox* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_addItem_text_userData(QComboBox* this_ptr, const QString* text, const QVariant* userData);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_addItems(QComboBox* this_ptr, const QStringList* texts);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QComboBox_autoCompletion(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_clear(QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_clearEditText(QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT QCompleter* qt_widgets_c_QComboBox_completer(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QComboBox_count(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_currentData_to_output_no_args(const QComboBox* this_ptr, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_currentData_to_output_role(const QComboBox* this_ptr, int role, QVariant* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QComboBox_currentIndex(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_currentText_to_output(const QComboBox* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_delete(QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QComboBox_duplicatesEnabled(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QComboBox_event(QComboBox* this_ptr, QEvent* event);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QComboBox_hasFrame(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_hidePopup(QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_iconSize_to_output(const QComboBox* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_inputMethodQuery_to_output_arg1(const QComboBox* this_ptr, const Qt::InputMethodQuery* arg1, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_inputMethodQuery_to_output_query_argument(const QComboBox* this_ptr, const Qt::InputMethodQuery* query, const QVariant* argument, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_insertItem_index_icon_text(QComboBox* this_ptr, int index, const QIcon* icon, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_insertItem_index_icon_text_userData(QComboBox* this_ptr, int index, const QIcon* icon, const QString* text, const QVariant* userData);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_insertItem_index_text(QComboBox* this_ptr, int index, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_insertItem_index_text_userData(QComboBox* this_ptr, int index, const QString* text, const QVariant* userData);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_insertItems(QComboBox* this_ptr, int index, const QStringList* texts);
QT_WIDGETS_C_EXPORT QComboBox::InsertPolicy qt_widgets_c_QComboBox_insertPolicy(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_insertSeparator(QComboBox* this_ptr, int index);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QComboBox_isEditable(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_itemData_to_output_index(const QComboBox* this_ptr, int index, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_itemData_to_output_index_role(const QComboBox* this_ptr, int index, int role, QVariant* output);
QT_WIDGETS_C_EXPORT QAbstractItemDelegate* qt_widgets_c_QComboBox_itemDelegate(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_itemIcon_to_output(const QComboBox* this_ptr, int index, QIcon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_itemText_to_output(const QComboBox* this_ptr, int index, QString* output);
QT_WIDGETS_C_EXPORT QLineEdit* qt_widgets_c_QComboBox_lineEdit(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QComboBox_maxCount(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QComboBox_maxVisibleItems(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QComboBox_metaObject(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QComboBox_minimumContentsLength(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_minimumSizeHint_to_output(const QComboBox* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QAbstractItemModel* qt_widgets_c_QComboBox_model(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QComboBox_modelColumn(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT QComboBox* qt_widgets_c_QComboBox_new_no_args();
QT_WIDGETS_C_EXPORT QComboBox* qt_widgets_c_QComboBox_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QComboBox_qt_metacall(QComboBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QComboBox_qt_metacast(QComboBox* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_removeItem(QComboBox* this_ptr, int index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_rootModelIndex_to_output(const QComboBox* this_ptr, QModelIndex* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setAutoCompletion(QComboBox* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setAutoCompletionCaseSensitivity(QComboBox* this_ptr, const Qt::CaseSensitivity* sensitivity);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setCompleter(QComboBox* this_ptr, QCompleter* c);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setCurrentIndex(QComboBox* this_ptr, int index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setCurrentText(QComboBox* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setDuplicatesEnabled(QComboBox* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setEditText(QComboBox* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setEditable(QComboBox* this_ptr, bool editable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setFrame(QComboBox* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setIconSize(QComboBox* this_ptr, const QSize* size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setInsertPolicy(QComboBox* this_ptr, QComboBox::InsertPolicy policy);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setItemData_index_value(QComboBox* this_ptr, int index, const QVariant* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setItemData_index_value_role(QComboBox* this_ptr, int index, const QVariant* value, int role);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setItemDelegate(QComboBox* this_ptr, QAbstractItemDelegate* delegate);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setItemIcon(QComboBox* this_ptr, int index, const QIcon* icon);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setItemText(QComboBox* this_ptr, int index, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setLineEdit(QComboBox* this_ptr, QLineEdit* edit);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setMaxCount(QComboBox* this_ptr, int max);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setMaxVisibleItems(QComboBox* this_ptr, int maxItems);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setMinimumContentsLength(QComboBox* this_ptr, int characters);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setModel(QComboBox* this_ptr, QAbstractItemModel* model);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setModelColumn(QComboBox* this_ptr, int visibleColumn);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setRootModelIndex(QComboBox* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setSizeAdjustPolicy(QComboBox* this_ptr, QComboBox::SizeAdjustPolicy policy);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setValidator(QComboBox* this_ptr, const QValidator* v);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_setView(QComboBox* this_ptr, QAbstractItemView* itemView);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_showPopup(QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT QComboBox::SizeAdjustPolicy qt_widgets_c_QComboBox_sizeAdjustPolicy(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_sizeHint_to_output(const QComboBox* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QComboBox_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT const QValidator* qt_widgets_c_QComboBox_validator(const QComboBox* this_ptr);
QT_WIDGETS_C_EXPORT QAbstractItemView* qt_widgets_c_QComboBox_view(const QComboBox* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QCOMBOBOX_H
