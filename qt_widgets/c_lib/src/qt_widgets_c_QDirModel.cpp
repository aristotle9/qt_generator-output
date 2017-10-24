#include "qt_widgets_c_QDirModel.h"

QAbstractItemModel* qt_widgets_c_QDirModel_G_static_cast_QAbstractItemModel_ptr(QDirModel* ptr) {
  return static_cast<QAbstractItemModel*>(ptr);
}

QDirModel* qt_widgets_c_QDirModel_G_static_cast_QDirModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr) {
  return static_cast<QDirModel*>(ptr);
}

QDirModel* qt_widgets_c_QDirModel_G_static_cast_QDirModel_ptr_QObject(QObject* ptr) {
  return static_cast<QDirModel*>(ptr);
}

QObject* qt_widgets_c_QDirModel_G_static_cast_QObject_ptr(QDirModel* ptr) {
  return static_cast<QObject*>(ptr);
}

int qt_widgets_c_QDirModel_columnCount_no_args(const QDirModel* this_ptr) {
  return this_ptr->columnCount();
}

int qt_widgets_c_QDirModel_columnCount_parent(const QDirModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->columnCount(*parent);
}

void qt_widgets_c_QDirModel_data_to_output_index(const QDirModel* this_ptr, const QModelIndex* index, QVariant* output) {
  new(output) QVariant(this_ptr->data(*index));
}

void qt_widgets_c_QDirModel_data_to_output_index_role(const QDirModel* this_ptr, const QModelIndex* index, int role, QVariant* output) {
  new(output) QVariant(this_ptr->data(*index, role));
}

void qt_widgets_c_QDirModel_delete(QDirModel* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QDirModel_dropMimeData(QDirModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent) {
  return this_ptr->dropMimeData(data, *action, row, column, *parent);
}

void qt_widgets_c_QDirModel_fileIcon_to_output(const QDirModel* this_ptr, const QModelIndex* index, QIcon* output) {
  new(output) QIcon(this_ptr->fileIcon(*index));
}

void qt_widgets_c_QDirModel_fileInfo_to_output(const QDirModel* this_ptr, const QModelIndex* index, QFileInfo* output) {
  new(output) QFileInfo(this_ptr->fileInfo(*index));
}

void qt_widgets_c_QDirModel_fileName_to_output(const QDirModel* this_ptr, const QModelIndex* index, QString* output) {
  new(output) QString(this_ptr->fileName(*index));
}

void qt_widgets_c_QDirModel_filePath_to_output(const QDirModel* this_ptr, const QModelIndex* index, QString* output) {
  new(output) QString(this_ptr->filePath(*index));
}

bool qt_widgets_c_QDirModel_hasChildren_index(const QDirModel* this_ptr, const QModelIndex* index) {
  return this_ptr->hasChildren(*index);
}

bool qt_widgets_c_QDirModel_hasChildren_no_args(const QDirModel* this_ptr) {
  return this_ptr->hasChildren();
}

void qt_widgets_c_QDirModel_headerData_to_output_section_orientation(const QDirModel* this_ptr, int section, const Qt::Orientation* orientation, QVariant* output) {
  new(output) QVariant(this_ptr->headerData(section, *orientation));
}

void qt_widgets_c_QDirModel_headerData_to_output_section_orientation_role(const QDirModel* this_ptr, int section, const Qt::Orientation* orientation, int role, QVariant* output) {
  new(output) QVariant(this_ptr->headerData(section, *orientation, role));
}

QFileIconProvider* qt_widgets_c_QDirModel_iconProvider(const QDirModel* this_ptr) {
  return this_ptr->iconProvider();
}

void qt_widgets_c_QDirModel_index_to_output_path(const QDirModel* this_ptr, const QString* path, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(*path));
}

void qt_widgets_c_QDirModel_index_to_output_path_column(const QDirModel* this_ptr, const QString* path, int column, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(*path, column));
}

void qt_widgets_c_QDirModel_index_to_output_row_column(const QDirModel* this_ptr, int row, int column, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(row, column));
}

void qt_widgets_c_QDirModel_index_to_output_row_column_parent(const QDirModel* this_ptr, int row, int column, const QModelIndex* parent, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(row, column, *parent));
}

bool qt_widgets_c_QDirModel_isDir(const QDirModel* this_ptr, const QModelIndex* index) {
  return this_ptr->isDir(*index);
}

bool qt_widgets_c_QDirModel_isReadOnly(const QDirModel* this_ptr) {
  return this_ptr->isReadOnly();
}

bool qt_widgets_c_QDirModel_lazyChildCount(const QDirModel* this_ptr) {
  return this_ptr->lazyChildCount();
}

const QMetaObject* qt_widgets_c_QDirModel_metaObject(const QDirModel* this_ptr) {
  return this_ptr->metaObject();
}

QMimeData* qt_widgets_c_QDirModel_mimeData(const QDirModel* this_ptr, const QList< QModelIndex >* indexes) {
  return this_ptr->mimeData(*indexes);
}

void qt_widgets_c_QDirModel_mimeTypes_to_output(const QDirModel* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->mimeTypes());
}

void qt_widgets_c_QDirModel_mkdir_to_output(QDirModel* this_ptr, const QModelIndex* parent, const QString* name, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->mkdir(*parent, *name));
}

void qt_widgets_c_QDirModel_nameFilters_to_output(const QDirModel* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->nameFilters());
}

QDirModel* qt_widgets_c_QDirModel_new_no_args() {
  return new QDirModel();
}

QDirModel* qt_widgets_c_QDirModel_new_parent(QObject* parent) {
  return new QDirModel(parent);
}

void qt_widgets_c_QDirModel_parent_to_output(const QDirModel* this_ptr, const QModelIndex* child, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->parent(*child));
}

int qt_widgets_c_QDirModel_qt_metacall(QDirModel* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QDirModel_qt_metacast(QDirModel* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QDirModel_refresh_no_args(QDirModel* this_ptr) {
  this_ptr->refresh();
}

void qt_widgets_c_QDirModel_refresh_parent(QDirModel* this_ptr, const QModelIndex* parent) {
  this_ptr->refresh(*parent);
}

bool qt_widgets_c_QDirModel_remove(QDirModel* this_ptr, const QModelIndex* index) {
  return this_ptr->remove(*index);
}

bool qt_widgets_c_QDirModel_resolveSymlinks(const QDirModel* this_ptr) {
  return this_ptr->resolveSymlinks();
}

bool qt_widgets_c_QDirModel_rmdir(QDirModel* this_ptr, const QModelIndex* index) {
  return this_ptr->rmdir(*index);
}

int qt_widgets_c_QDirModel_rowCount_no_args(const QDirModel* this_ptr) {
  return this_ptr->rowCount();
}

int qt_widgets_c_QDirModel_rowCount_parent(const QDirModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->rowCount(*parent);
}

bool qt_widgets_c_QDirModel_setData_index_value(QDirModel* this_ptr, const QModelIndex* index, const QVariant* value) {
  return this_ptr->setData(*index, *value);
}

bool qt_widgets_c_QDirModel_setData_index_value_role(QDirModel* this_ptr, const QModelIndex* index, const QVariant* value, int role) {
  return this_ptr->setData(*index, *value, role);
}

void qt_widgets_c_QDirModel_setIconProvider(QDirModel* this_ptr, QFileIconProvider* provider) {
  this_ptr->setIconProvider(provider);
}

void qt_widgets_c_QDirModel_setLazyChildCount(QDirModel* this_ptr, bool enable) {
  this_ptr->setLazyChildCount(enable);
}

void qt_widgets_c_QDirModel_setNameFilters(QDirModel* this_ptr, const QStringList* filters) {
  this_ptr->setNameFilters(*filters);
}

void qt_widgets_c_QDirModel_setReadOnly(QDirModel* this_ptr, bool enable) {
  this_ptr->setReadOnly(enable);
}

void qt_widgets_c_QDirModel_setResolveSymlinks(QDirModel* this_ptr, bool enable) {
  this_ptr->setResolveSymlinks(enable);
}

void qt_widgets_c_QDirModel_sort_column(QDirModel* this_ptr, int column) {
  this_ptr->sort(column);
}

void qt_widgets_c_QDirModel_sort_column_order(QDirModel* this_ptr, int column, const Qt::SortOrder* order) {
  this_ptr->sort(column, *order);
}

void qt_widgets_c_QDirModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDirModel::trUtf8(s, c, n));
}

void qt_widgets_c_QDirModel_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDirModel::tr(s, c, n));
}

