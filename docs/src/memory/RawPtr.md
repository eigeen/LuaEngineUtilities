# RawPtr

保存基址和偏移，计算指针，读取和写入指针指向的内存数据。

## 方法

### setBase(base: usize) -> Self

设置指针基址。

### setOffset(...offsets: isize) -> Self

设置指针偏移量。会覆盖先前设置的偏移。

### addOffset(...offsets: isize) -> Self

在序列末尾追加指针偏移量。

### read(type_name: String) -> T

读取指针指向的内存数据。

> `type_name` - 类型名
>
> `type_name` 可以为以下内容：
>
> - "i8"
> - "i16"
> - "i32"
> - "i64"
> - "f32"
> - "f64"
> - "bool"
> - "string"
>
> 返回值类型取决于 `type_name`。

### :write(value: T, type_name: String)

写入指针指向的内存数据。

> `type_name` - 类型名
>
> `type_name` 约束同上

## 示例

```lua
local ptr = Memory.newPtr():setBase(0x12345678):setOffset(0x10, 0x20)
ptr:addOffset(0x30)

local value = ptr:read("i32")
```