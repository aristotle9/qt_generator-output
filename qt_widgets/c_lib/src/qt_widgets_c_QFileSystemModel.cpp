#include "qt_widgets_c_QFileSystemModel.h"

QAbstractItemModel* qt_widgets_c_QFileSystemModel_G_static_cast_QAbstractItemModel_ptr(QFileSystemModel* ptr) {
  return static_cast<QAbstractItemModel*>(ptr);
}

QFileSystemModel* qt_widgets_c_QFileSystemModel_G_static_cast_QFileSystemModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr) {
  return static_cast<QFileSystemModel*>(ptr);
}

QFileSystemModel* qt_widgets_c_QFileSystemModel_G_static_cast_QFileSystemModel_ptr_QObject(QObject* ptr) {
  return static_cast<QFileSystemModel*>(ptr);
}

QObject* qt_widgets_c_QFileSystemModel_G_static_cast_QObject_ptr(QFileSystemModel* ptr) {
  return static_cast<QObject*>(ptr);
}

bool qt_widgets_c_QFileSystemModel_canFetchMore(const QFileSystemModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->canFetchMore(*parent);
}

int qt_widgets_c_QFileSystemModel_columnCount_no_args(const QFileSystemModel* this_ptr) {
  return this_ptr->columnCount();
}

int qt_widgets_c_QFileSystemModel_columnCount_parent(const QFileSystemModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->columnCount(*parent);
}

void qt_widgets_c_QFileSystemModel_data_to_output_index(const QFileSystemModel* this_ptr, const QModelIndex* index, QVariant* output) {
  new(output) QVariant(this_ptr->data(*index));
}

void qt_widgets_c_QFileSystemModel_data_to_output_index_role(const QFileSystemModel* this_ptr, const QModelIndex* index, int role, QVariant* output) {
  new(output) QVariant(this_ptr->data(*index, role));
}

void qt_widgets_c_QFileSystemModel_delete(QFileSystemModel* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QFileSystemModel_dropMimeData(QFileSystemModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent) {
  return this_ptr->dropMimeData(data, *action, row, column, *parent);
}

void qt_widgets_c_QFileSystemModel_fetchMore(QFileSystemModel* this_ptr, const QModelIndex* parent) {
  this_ptr->fetchMore(*parent);
}

void qt_widgets_c_QFileSystemModel_fileIcon_to_output(const QFileSystemModel* this_ptr, const QModelIndex* index, QIcon* output) {
  new(output) QIcon(this_ptr->fileIcon(*index));
}

void qt_widgets_c_QFileSystemModel_fileInfo_to_output(const QFileSystemModel* this_ptr, const QModelIndex* index, QFileInfo* output) {
  new(output) QFileInfo(this_ptr->fileInfo(*index));
}

void qt_widgets_c_QFileSystemModel_fileName_to_output(const QFileSystemModel* this_ptr, const QModelIndex* index, QString* output) {
  new(output) QString(this_ptr->fileName(*index));
}

void qt_widgets_c_QFileSystemModel_filePath_to_output(const QFileSystemModel* this_ptr, const QModelIndex* index, QString* output) {
  new(output) QString(this_ptr->filePath(*index));
}

bool qt_widgets_c_QFileSystemModel_hasChildren_no_args(const QFileSystemModel* this_ptr) {
  return this_ptr->hasChildren();
}

bool qt_widgets_c_QFileSystemModel_hasChildren_parent(const QFileSystemModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->hasChildren(*parent);
}

void qt_widgets_c_QFileSystemModel_headerData_to_output_section_orientation(const QFileSystemModel* this_ptr, int section, const Qt::Orientation* orientation, QVariant* output) {
  new(output) QVariant(this_ptr->headerData(section, *orientation));
}

void qt_widgets_c_QFileSystemModel_headerData_to_output_section_orientation_role(const QFileSystemModel* this_ptr, int section, const Qt::Orientation* orientation, int role, QVariant* output) {
  new(output) QVariant(this_ptr->headerData(section, *orientation, role));
}

QFileIconProvider* qt_widgets_c_QFileSystemModel_iconProvider(const QFileSystemModel* this_ptr) {
  return this_ptr->iconProvider();
}

void qt_widgets_c_QFileSystemModel_index_to_output_path(const QFileSystemModel* this_ptr, const QString* path, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(*path));
}

void qt_widgets_c_QFileSystemModel_index_to_output_path_column(const QFileSystemModel* this_ptr, const QString* path, int column, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(*path, column));
}

void qt_widgets_c_QFileSystemModel_index_to_output_row_column(const QFileSystemModel* this_ptr, int row, int column, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(row, column));
}

void qt_widgets_c_QFileSystemModel_index_to_output_row_column_parent(const QFileSystemModel* this_ptr, int row, int column, const QModelIndex* parent, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(row, column, *parent));
}

bool qt_widgets_c_QFileSystemModel_isDir(const QFileSystemModel* this_ptr, const QModelIndex* index) {
  return this_ptr->isDir(*index);
}

bool qt_widgets_c_QFileSystemModel_isReadOnly(const QFileSystemModel* this_ptr) {
  return this_ptr->isReadOnly();
}

void qt_widgets_c_QFileSystemModel_lastModified_to_output(const QFileSystemModel* this_ptr, const QModelIndex* index, QDateTime* output) {
  new(output) QDateTime(this_ptr->lastModified(*index));
}

const QMetaObject* qt_widgets_c_QFileSystemModel_metaObject(const QFileSystemModel* this_ptr) {
  return this_ptr->metaObject();
}

QMimeData* qt_widgets_c_QFileSystemModel_mimeData(const QFileSystemModel* this_ptr, const QList< QModelIndex >* indexes) {
  return this_ptr->mimeData(*indexes);
}

void qt_widgets_c_QFileSystemModel_mimeTypes_to_output(const QFileSystemModel* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->mimeTypes());
}

void qt_widgets_c_QFileSystemModel_mkdir_to_output(QFileSystemModel* this_ptr, const QModelIndex* parent, const QString* name, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->mkdir(*parent, *name));
}

void qt_widgets_c_QFileSystemModel_myComputer_to_output_no_args(const QFileSystemModel* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->myComputer());
}

void qt_widgets_c_QFileSystemModel_myComputer_to_output_role(const QFileSystemModel* this_ptr, int role, QVariant* output) {
  new(output) QVariant(this_ptr->myComputer(role));
}

bool qt_widgets_c_QFileSystemModel_nameFilterDisables(const QFileSystemModel* this_ptr) {
  return this_ptr->nameFilterDisables();
}

void qt_widgets_c_QFileSystemModel_nameFilters_to_output(const QFileSystemModel* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->nameFilters());
}

QFileSystemModel* qt_widgets_c_QFileSystemModel_new_no_args() {
  return new QFileSystemModel();
}

QFileSystemModel* qt_widgets_c_QFileSystemModel_new_parent(QObject* parent) {
  return new QFileSystemModel(parent);
}

void qt_widgets_c_QFileSystemModel_parent_to_output(const QFileSystemModel* this_ptr, const QModelIndex* child, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->parent(*child));
}

int qt_widgets_c_QFileSystemModel_qt_metacall(QFileSystemModel* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QFileSystemModel_qt_metacast(QFileSystemModel* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

bool qt_widgets_c_QFileSystemModel_remove(QFileSystemModel* this_ptr, const QModelIndex* index) {
  return this_ptr->remove(*index);
}

bool qt_widgets_c_QFileSystemModel_resolveSymlinks(const QFileSystemModel* this_ptr) {
  return this_ptr->resolveSymlinks();
}

bool qt_widgets_c_QFileSystemModel_rmdir(QFileSystemModel* this_ptr, const QModelIndex* index) {
  return this_ptr->rmdir(*index);
}

void qt_widgets_c_QFileSystemModel_rootDirectory_to_output(const QFileSystemModel* this_ptr, QDir* output) {
  new(output) QDir(this_ptr->rootDirectory());
}

void qt_widgets_c_QFileSystemModel_rootPath_to_output(const QFileSystemModel* this_ptr, QString* output) {
  new(output) QString(this_ptr->rootPath());
}

int qt_widgets_c_QFileSystemModel_rowCount_no_args(const QFileSystemModel* this_ptr) {
  return this_ptr->rowCount();
}

int qt_widgets_c_QFileSystemModel_rowCount_parent(const QFileSystemModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->rowCount(*parent);
}

bool qt_widgets_c_QFileSystemModel_setData_index_value(QFileSystemModel* this_ptr, const QModelIndex* index, const QVariant* value) {
  return this_ptr->setData(*index, *value);
}

bool qt_widgets_c_QFileSystemModel_setData_index_value_role(QFileSystemModel* this_ptr, const QModelIndex* index, const QVariant* value, int role) {
  return this_ptr->setData(*index, *value, role);
}

void qt_widgets_c_QFileSystemModel_setIconProvider(QFileSystemModel* this_ptr, QFileIconProvider* provider) {
  this_ptr->setIconProvider(provider);
}

void qt_widgets_c_QFileSystemModel_setNameFilterDisables(QFileSystemModel* this_ptr, bool enable) {
  this_ptr->setNameFilterDisables(enable);
}

void qt_widgets_c_QFileSystemModel_setNameFilters(QFileSystemModel* this_ptr, const QStringList* filters) {
  this_ptr->setNameFilters(*filters);
}

void qt_widgets_c_QFileSystemModel_setReadOnly(QFileSystemModel* this_ptr, bool enable) {
  this_ptr->setReadOnly(enable);
}

void qt_widgets_c_QFileSystemModel_setResolveSymlinks(QFileSystemModel* this_ptr, bool enable) {
  this_ptr->setResolveSymlinks(enable);
}

void qt_widgets_c_QFileSystemModel_setRootPath_to_output(QFileSystemModel* this_ptr, const QString* path, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->setRootPath(*path));
}

void qt_widgets_c_QFileSystemModel_sibling_to_output(const QFileSystemModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->sibling(row, column, *idx));
}

qint64 qt_widgets_c_QFileSystemModel_size(const QFileSystemModel* this_ptr, const QModelIndex* index) {
  return this_ptr->size(*index);
}

void qt_widgets_c_QFileSystemModel_sort_column(QFileSystemModel* this_ptr, int column) {
  this_ptr->sort(column);
}

void qt_widgets_c_QFileSystemModel_sort_column_order(QFileSystemModel* this_ptr, int column, const Qt::SortOrder* order) {
  this_ptr->sort(column, *order);
}

void qt_widgets_c_QFileSystemModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFileSystemModel::trUtf8(s, c, n));
}

void qt_widgets_c_QFileSystemModel_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFileSystemModel::tr(s, c, n));
}

void qt_widgets_c_QFileSystemModel_type_to_output(const QFileSystemModel* this_ptr, const QModelIndex* index, QString* output) {
  new(output) QString(this_ptr->type(*index));
}

