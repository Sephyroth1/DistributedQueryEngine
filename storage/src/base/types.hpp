#ifndef STORAGE_BASE_TYPES_HPP
#define STORAGE_BASE_TYPES_HPP

#include <iostream>
#include <memory>
#include <string>
#include <vector>

enum DataTypes { INT32, FLOAT, STRING, BOOL, ARRAY, ERROR };

class Value {
public:
  virtual ~Value() = default;
  virtual DataTypes getType() const = 0;
  virtual std::string toString() const = 0;
  virtual void serialize(std::ostream &os) const = 0;
  virtual std::unique_ptr<Value> clone() const = 0;
  virtual void deserialize(std::istream &is, DataTypes type) = 0;

  virtual bool operator==(const Value &other) const = 0;
  virtual bool operator!=(const Value &other) const {
    return !(*this == other);
  }
  virtual bool operator<(const Value &other) const = 0;
  virtual bool operator>(const Value &other) const = 0;
  virtual bool operator<=(const Value &other) const { return !(*this > other); }
  virtual bool operator>=(const Value &other) const { return !(*this < other); }
};

template <typename T> void writeBinary(std::ostream &os, T value) {
  os.write(reinterpret_cast<const char *>(&value), sizeof(value));
}

template <typename T> void readBinary(std::istream &is, T &value) {
  is.read(reinterpret_cast<char *>(&value), sizeof(value));
}

class IntValue : public Value {
public:
  IntValue(int32_t value) : value_(value) {}
  DataTypes getType() const override { return INT32; }
  std::string toString() const override { return std::to_string(value_); }
  void serialize(std::ostream &os) const override {
    writeBinary(os, INT32);
    writeBinary(os, value_);
  }
  void deserialize(std::istream &is, DataTypes type) override {
    readBinary(is, type);
    if (type != INT32)
      throw std::runtime_error("Invalid type");
    readBinary(is, value_);
  }
  std::unique_ptr<Value> clone() const override {
    return std::make_unique<IntValue>(*this);
  }
  int32_t getValue() const { return value_; }

  bool operator==(const Value &other) const override {
    if (other.getType() != INT32)
      return false;
    return value_ == static_cast<const IntValue &>(other).value_;
  }

  bool operator<(const Value &other) const override {
    if (other.getType() != INT32)
      throw std::runtime_error("Invalid type");
    return value_ < static_cast<const IntValue &>(other).value_;
  }

  bool operator>(const Value &other) const override {
    if (other.getType() != INT32)
      throw std::runtime_error("Invalid type");
    return value_ > static_cast<const IntValue &>(other).value_;
  }

private:
  int32_t value_;
};

class FloatValue : public Value {
private:
  float value_;

public:
  FloatValue(float value) { this->value_ = value; }
  DataTypes getType() const override { return FLOAT; }
  std::string toString() const override { return std::to_string(value_); }
  void serialize(std::ostream &os) const override {
    writeBinary(os, FLOAT);
    writeBinary(os, value_);
  }
  void deserialize(std::istream &is, DataTypes type) override {
    readBinary(is, type);
    if (type != FLOAT)
      throw std::runtime_error("Invalid type");
    readBinary(is, value_);
  }

  std::unique_ptr<Value> clone() const override {
    return std::make_unique<FloatValue>(*this);
  }
  int32_t getValue() const { return value_; }

  bool operator==(const Value &other) const override {
    if (other.getType() != FLOAT)
      return false;
    return value_ == static_cast<const FloatValue &>(other).value_;
  }

  bool operator<(const Value &other) const override {
    if (other.getType() != FLOAT)
      throw std::runtime_error("Invalid type");
    return value_ < static_cast<const FloatValue &>(other).value_;
  }

  bool operator>(const Value &other) const override {
    if (other.getType() != FLOAT)
      throw std::runtime_error("Invalid type");
    return value_ > static_cast<const FloatValue &>(other).value_;
  }
};

class BOOLValue : public Value {
private:
  bool value_;

public:
  BOOLValue(bool value) { this->value_ = value; }
  DataTypes getType() const override { return BOOL; }
  std::string toString() const override { return value_ ? "true" : "false"; }
  void serialize(std::ostream &os) const override {
    writeBinary(os, BOOL);
    writeBinary(os, value_);
  }
  void deserialize(std::istream &is, DataTypes type) override {
    readBinary(is, type);
    if (type != BOOL)
      throw std::runtime_error("Invalid type");
    readBinary(is, value_);
  }

  std::unique_ptr<Value> clone() const override {
    return std::make_unique<BOOLValue>(*this);
  }
  bool getValue() const { return value_; }

  bool operator==(const Value &other) const override {
    if (other.getType() != BOOL)
      return false;
    return value_ == static_cast<const BOOLValue &>(other).value_;
  }

  bool operator<(const Value &other) const override {
    if (other.getType() != BOOL)
      throw std::runtime_error("Invalid type");
    return value_ < static_cast<const BOOLValue &>(other).value_;
  }

  bool operator>(const Value &other) const override {
    if (other.getType() != BOOL)
      throw std::runtime_error("Invalid type");
    return value_ > static_cast<const BOOLValue &>(other).value_;
  }
};

class StringValue : public Value {
private:
  std::string value_;

public:
  StringValue(const std::string &value) { this->value_ = value; }
  DataTypes getType() const override { return STRING; }
  std::string toString() const override { return value_; }
  void serialize(std::ostream &os) const override {
    // [type][len][value]
    writeBinary(os, STRING);
    size_t len = value_.size();
    writeBinary(os, len);
    os.write(value_.data(), len);
  }
  void deserialize(std::istream &is, DataTypes type) override {
    readBinary(is, type);
    if (type != STRING)
      throw std::runtime_error("Invalid type");
    size_t len;
    readBinary(is, len);
    std::string value;
    value.resize(len);
    is.read(value.data(), len);
    value_ = value;
  }

  std::unique_ptr<Value> clone() const override {
    return std::make_unique<StringValue>(*this);
  }
  const std::string &getValue() const { return value_; }

  bool operator==(const Value &other) const override {
    if (other.getType() != STRING)
      return false;
    return value_ == static_cast<const StringValue &>(other).value_;
  }

  bool operator<(const Value &other) const override {
    if (other.getType() != STRING)
      throw std::runtime_error("Invalid type");
    return value_ < static_cast<const StringValue &>(other).value_;
  }

  bool operator>(const Value &other) const override {
    if (other.getType() != STRING)
      throw std::runtime_error("Invalid type");
    return value_ > static_cast<const StringValue &>(other).value_;
  }
};

class Row {
private:
  std::vector<std::unique_ptr<Value>> values_;

public:
  Row(std::vector<std::unique_ptr<Value>> values)
      : values_(std::move(values)) {}
  void add_value(std::unique_ptr<Value> value) {
    values_.push_back(std::move(value));
  }

  std::unique_ptr<Value> get_value(int index) const {
    if (values_[index]->getType() == DataTypes::INT32) {
      return values_[index]->clone();
    } else if (values_[index]->getType() == DataTypes::FLOAT) {
      return values_[index]->clone();
    } else if (values_[index]->getType() == DataTypes::STRING) {
      return values_[index]->clone();
    } else if (values_[index]->getType() == DataTypes::BOOL) {
      return values_[index]->clone();
    } else if (values_[index]->getType() == DataTypes::ARRAY) {
      return values_[index]->clone();
    } else {
      return nullptr;
    }
  }
};

class Column {
private:
  std::string name_;
  DataTypes type_;
  int column_id_;

public:
  Column(std::string name, DataTypes type, int column_id)
      : name_(name), type_(type), column_id_(column_id) {}

  const std::string &name() const;
  DataTypes type() const;
};

class Table {
private:
  std::string name_;
  std::vector<Column> columns_;
  std::vector<Row> rows_;

public:
  Table(std::string name, std::vector<Column> columns)
      : name_(name), columns_(std::move(columns)) {}

  void insert(Row row) { rows_.push_back(std::move(row)); }

  std::vector<Row> scan() const { return rows_; }
};

#endif
