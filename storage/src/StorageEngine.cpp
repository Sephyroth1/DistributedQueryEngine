#include "base/table.hpp"

class StorageEngine {
private:
  std::unordered_map<std::string, Table> tables_;

public:
  void CreateTable(const std::string &name,
                   const std::vector<Column> &columns) {
    tables_[name] = Table(name, columns);
  }

  std::vector<Row> ScanTable(const std::string &name) const {
    return tables_.at(name).scan();
  }

  void DropTable(const std::string &name) { tables_.erase(name); }
};
