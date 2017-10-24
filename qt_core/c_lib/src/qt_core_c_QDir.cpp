#include "qt_core_c_QDir.h"

void qt_core_c_QDir_G_operator_shl_to_output(const QDebug* debug, const QDir* dir, QDebug* output) {
  new(output) QDebug(operator<<(*debug, *dir));
}

void qt_core_c_QDir_G_swap(QDir* value1, QDir* value2) {
  swap(*value1, *value2);
}

void qt_core_c_QDir_absoluteFilePath_to_output(const QDir* this_ptr, const QString* fileName, QString* output) {
  new(output) QString(this_ptr->absoluteFilePath(*fileName));
}

void qt_core_c_QDir_absolutePath_to_output(const QDir* this_ptr, QString* output) {
  new(output) QString(this_ptr->absolutePath());
}

void qt_core_c_QDir_addResourceSearchPath(const QString* path) {
  QDir::addResourceSearchPath(*path);
}

void qt_core_c_QDir_addSearchPath(const QString* prefix, const QString* path) {
  QDir::addSearchPath(*prefix, *path);
}

void qt_core_c_QDir_canonicalPath_to_output(const QDir* this_ptr, QString* output) {
  new(output) QString(this_ptr->canonicalPath());
}

bool qt_core_c_QDir_cd(QDir* this_ptr, const QString* dirName) {
  return this_ptr->cd(*dirName);
}

bool qt_core_c_QDir_cdUp(QDir* this_ptr) {
  return this_ptr->cdUp();
}

void qt_core_c_QDir_cleanPath_to_output(const QString* path, QString* output) {
  new(output) QString(QDir::cleanPath(*path));
}

void qt_core_c_QDir_constructor_arg1(const QDir* arg1, QDir* output) {
  new(output) QDir(*arg1);
}

void qt_core_c_QDir_constructor_no_args(QDir* output) {
  new(output) QDir();
}

void qt_core_c_QDir_constructor_path(const QString* path, QDir* output) {
  new(output) QDir(*path);
}

void qt_core_c_QDir_constructor_path_nameFilter(const QString* path, const QString* nameFilter, QDir* output) {
  new(output) QDir(*path, *nameFilter);
}

void qt_core_c_QDir_constructor_path_nameFilter_sort(const QString* path, const QString* nameFilter, unsigned int sort, QDir* output) {
  new(output) QDir(*path, *nameFilter, QFlags< QDir::SortFlag >(sort));
}

void qt_core_c_QDir_constructor_path_nameFilter_sort_filter(const QString* path, const QString* nameFilter, unsigned int sort, unsigned int filter, QDir* output) {
  new(output) QDir(*path, *nameFilter, QFlags< QDir::SortFlag >(sort), QFlags< QDir::Filter >(filter));
}

unsigned int qt_core_c_QDir_count(const QDir* this_ptr) {
  return this_ptr->count();
}

void qt_core_c_QDir_currentPath_to_output(QString* output) {
  new(output) QString(QDir::currentPath());
}

void qt_core_c_QDir_current_to_output(QDir* output) {
  new(output) QDir(QDir::current());
}

void qt_core_c_QDir_destructor(QDir* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QDir_dirName_to_output(const QDir* this_ptr, QString* output) {
  new(output) QString(this_ptr->dirName());
}

void qt_core_c_QDir_drives_to_output(QList< QFileInfo >* output) {
  new(output) QList< QFileInfo >(QDir::drives());
}

void qt_core_c_QDir_entryInfoList_to_output_filters(const QDir* this_ptr, unsigned int filters, QList< QFileInfo >* output) {
  new(output) QList< QFileInfo >(this_ptr->entryInfoList(QFlags< QDir::Filter >(filters)));
}

void qt_core_c_QDir_entryInfoList_to_output_filters_sort(const QDir* this_ptr, unsigned int filters, unsigned int sort, QList< QFileInfo >* output) {
  new(output) QList< QFileInfo >(this_ptr->entryInfoList(QFlags< QDir::Filter >(filters), QFlags< QDir::SortFlag >(sort)));
}

void qt_core_c_QDir_entryInfoList_to_output_nameFilters(const QDir* this_ptr, const QStringList* nameFilters, QList< QFileInfo >* output) {
  new(output) QList< QFileInfo >(this_ptr->entryInfoList(*nameFilters));
}

void qt_core_c_QDir_entryInfoList_to_output_nameFilters_filters(const QDir* this_ptr, const QStringList* nameFilters, unsigned int filters, QList< QFileInfo >* output) {
  new(output) QList< QFileInfo >(this_ptr->entryInfoList(*nameFilters, QFlags< QDir::Filter >(filters)));
}

void qt_core_c_QDir_entryInfoList_to_output_nameFilters_filters_sort(const QDir* this_ptr, const QStringList* nameFilters, unsigned int filters, unsigned int sort, QList< QFileInfo >* output) {
  new(output) QList< QFileInfo >(this_ptr->entryInfoList(*nameFilters, QFlags< QDir::Filter >(filters), QFlags< QDir::SortFlag >(sort)));
}

void qt_core_c_QDir_entryInfoList_to_output_no_args(const QDir* this_ptr, QList< QFileInfo >* output) {
  new(output) QList< QFileInfo >(this_ptr->entryInfoList());
}

void qt_core_c_QDir_entryList_to_output_filters(const QDir* this_ptr, unsigned int filters, QStringList* output) {
  new(output) QStringList(this_ptr->entryList(QFlags< QDir::Filter >(filters)));
}

void qt_core_c_QDir_entryList_to_output_filters_sort(const QDir* this_ptr, unsigned int filters, unsigned int sort, QStringList* output) {
  new(output) QStringList(this_ptr->entryList(QFlags< QDir::Filter >(filters), QFlags< QDir::SortFlag >(sort)));
}

void qt_core_c_QDir_entryList_to_output_nameFilters(const QDir* this_ptr, const QStringList* nameFilters, QStringList* output) {
  new(output) QStringList(this_ptr->entryList(*nameFilters));
}

void qt_core_c_QDir_entryList_to_output_nameFilters_filters(const QDir* this_ptr, const QStringList* nameFilters, unsigned int filters, QStringList* output) {
  new(output) QStringList(this_ptr->entryList(*nameFilters, QFlags< QDir::Filter >(filters)));
}

void qt_core_c_QDir_entryList_to_output_nameFilters_filters_sort(const QDir* this_ptr, const QStringList* nameFilters, unsigned int filters, unsigned int sort, QStringList* output) {
  new(output) QStringList(this_ptr->entryList(*nameFilters, QFlags< QDir::Filter >(filters), QFlags< QDir::SortFlag >(sort)));
}

void qt_core_c_QDir_entryList_to_output_no_args(const QDir* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->entryList());
}

bool qt_core_c_QDir_exists_name(const QDir* this_ptr, const QString* name) {
  return this_ptr->exists(*name);
}

bool qt_core_c_QDir_exists_no_args(const QDir* this_ptr) {
  return this_ptr->exists();
}

void qt_core_c_QDir_filePath_to_output(const QDir* this_ptr, const QString* fileName, QString* output) {
  new(output) QString(this_ptr->filePath(*fileName));
}

unsigned int qt_core_c_QDir_filter(const QDir* this_ptr) {
  return uint(this_ptr->filter());
}

void qt_core_c_QDir_fromNativeSeparators_to_output(const QString* pathName, QString* output) {
  new(output) QString(QDir::fromNativeSeparators(*pathName));
}

void qt_core_c_QDir_homePath_to_output(QString* output) {
  new(output) QString(QDir::homePath());
}

void qt_core_c_QDir_home_to_output(QDir* output) {
  new(output) QDir(QDir::home());
}

bool qt_core_c_QDir_isAbsolute(const QDir* this_ptr) {
  return this_ptr->isAbsolute();
}

bool qt_core_c_QDir_isAbsolutePath(const QString* path) {
  return QDir::isAbsolutePath(*path);
}

bool qt_core_c_QDir_isEmpty_filters(const QDir* this_ptr, unsigned int filters) {
  return this_ptr->isEmpty(QFlags< QDir::Filter >(filters));
}

bool qt_core_c_QDir_isEmpty_no_args(const QDir* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_core_c_QDir_isReadable(const QDir* this_ptr) {
  return this_ptr->isReadable();
}

bool qt_core_c_QDir_isRelative(const QDir* this_ptr) {
  return this_ptr->isRelative();
}

bool qt_core_c_QDir_isRelativePath(const QString* path) {
  return QDir::isRelativePath(*path);
}

bool qt_core_c_QDir_isRoot(const QDir* this_ptr) {
  return this_ptr->isRoot();
}

void qt_core_c_QDir_listSeparator_to_output(QChar* output) {
  new(output) QChar(QDir::listSeparator());
}

bool qt_core_c_QDir_makeAbsolute(QDir* this_ptr) {
  return this_ptr->makeAbsolute();
}

bool qt_core_c_QDir_match_filter_fileName(const QString* filter, const QString* fileName) {
  return QDir::match(*filter, *fileName);
}

bool qt_core_c_QDir_match_filters_fileName(const QStringList* filters, const QString* fileName) {
  return QDir::match(*filters, *fileName);
}

bool qt_core_c_QDir_mkdir(const QDir* this_ptr, const QString* dirName) {
  return this_ptr->mkdir(*dirName);
}

bool qt_core_c_QDir_mkpath(const QDir* this_ptr, const QString* dirPath) {
  return this_ptr->mkpath(*dirPath);
}

void qt_core_c_QDir_nameFiltersFromString_to_output(const QString* nameFilter, QStringList* output) {
  new(output) QStringList(QDir::nameFiltersFromString(*nameFilter));
}

void qt_core_c_QDir_nameFilters_to_output(const QDir* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->nameFilters());
}

QDir* qt_core_c_QDir_operator_assign_arg1(QDir* this_ptr, const QDir* arg1) {
  return &this_ptr->operator=(*arg1);
}

QDir* qt_core_c_QDir_operator_assign_path(QDir* this_ptr, const QString* path) {
  return &this_ptr->operator=(*path);
}

bool qt_core_c_QDir_operator_eq(const QDir* this_ptr, const QDir* dir) {
  return this_ptr->operator==(*dir);
}

void qt_core_c_QDir_operator_index_to_output(const QDir* this_ptr, int arg1, QString* output) {
  new(output) QString(this_ptr->operator[](arg1));
}

bool qt_core_c_QDir_operator_neq(const QDir* this_ptr, const QDir* dir) {
  return this_ptr->operator!=(*dir);
}

void qt_core_c_QDir_path_to_output(const QDir* this_ptr, QString* output) {
  new(output) QString(this_ptr->path());
}

void qt_core_c_QDir_refresh(const QDir* this_ptr) {
  this_ptr->refresh();
}

void qt_core_c_QDir_relativeFilePath_to_output(const QDir* this_ptr, const QString* fileName, QString* output) {
  new(output) QString(this_ptr->relativeFilePath(*fileName));
}

bool qt_core_c_QDir_remove(QDir* this_ptr, const QString* fileName) {
  return this_ptr->remove(*fileName);
}

bool qt_core_c_QDir_removeRecursively(QDir* this_ptr) {
  return this_ptr->removeRecursively();
}

bool qt_core_c_QDir_rename(QDir* this_ptr, const QString* oldName, const QString* newName) {
  return this_ptr->rename(*oldName, *newName);
}

bool qt_core_c_QDir_rmdir(const QDir* this_ptr, const QString* dirName) {
  return this_ptr->rmdir(*dirName);
}

bool qt_core_c_QDir_rmpath(const QDir* this_ptr, const QString* dirPath) {
  return this_ptr->rmpath(*dirPath);
}

void qt_core_c_QDir_rootPath_to_output(QString* output) {
  new(output) QString(QDir::rootPath());
}

void qt_core_c_QDir_root_to_output(QDir* output) {
  new(output) QDir(QDir::root());
}

void qt_core_c_QDir_searchPaths_to_output(const QString* prefix, QStringList* output) {
  new(output) QStringList(QDir::searchPaths(*prefix));
}

void qt_core_c_QDir_separator_to_output(QChar* output) {
  new(output) QChar(QDir::separator());
}

bool qt_core_c_QDir_setCurrent(const QString* path) {
  return QDir::setCurrent(*path);
}

void qt_core_c_QDir_setFilter(QDir* this_ptr, unsigned int filter) {
  this_ptr->setFilter(QFlags< QDir::Filter >(filter));
}

void qt_core_c_QDir_setNameFilters(QDir* this_ptr, const QStringList* nameFilters) {
  this_ptr->setNameFilters(*nameFilters);
}

void qt_core_c_QDir_setPath(QDir* this_ptr, const QString* path) {
  this_ptr->setPath(*path);
}

void qt_core_c_QDir_setSearchPaths(const QString* prefix, const QStringList* searchPaths) {
  QDir::setSearchPaths(*prefix, *searchPaths);
}

void qt_core_c_QDir_setSorting(QDir* this_ptr, unsigned int sort) {
  this_ptr->setSorting(QFlags< QDir::SortFlag >(sort));
}

unsigned int qt_core_c_QDir_sorting(const QDir* this_ptr) {
  return uint(this_ptr->sorting());
}

void qt_core_c_QDir_swap(QDir* this_ptr, QDir* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QDir_tempPath_to_output(QString* output) {
  new(output) QString(QDir::tempPath());
}

void qt_core_c_QDir_temp_to_output(QDir* output) {
  new(output) QDir(QDir::temp());
}

void qt_core_c_QDir_toNativeSeparators_to_output(const QString* pathName, QString* output) {
  new(output) QString(QDir::toNativeSeparators(*pathName));
}

