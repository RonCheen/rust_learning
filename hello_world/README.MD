# Rust Course
## Chapter2 Rust基础入门
### 变量命名
+ type-level的构造使用驼峰，value-level的构造使用蛇形
+ type-level：
    + Modules
    + Types
    + Traits
    + Structs
### 类型转换
应当遵循as_, to_, into_ 习惯
### 读访问器不使用get，当且仅当有一个值能被Getter获取时才能使用get前缀

### 所有权和借用
1.规则
```
1. Rust中每一个值都被一个变量所拥有，该变量被称为值的所有者
2. 一个值只能拥有一个所有者
3.当所有者离开作用域范围时，这个值将被丢弃
```

2.字符串String类型（动态类型，分配在堆上） &str字面量
3.转移所有权
～～～
1.基本类型通过自动拷贝的方式来赋值
～～～