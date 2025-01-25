## 部分语言规则：

- 单行注释以`//`开头
- 语句以`;`结尾（）
- 赋值语句类型:
```
let x: i8 = 123;
let x = 123.4;
let x = 'c';
let x: str = "hello world!";
```

### 复合语句

#### 循环语句：

```
// while循环
while ([bool]) [statements];
// loop循环
loop {
    [statements]
}
// for循环
for (i in [array]) [statements]
// for循环: 
for (k, v of [key:value]) [statements]
```

#### 选择语句：

```
if ([bool]) [statements];

if ([bool]) [statements];
else [statements];
```

### 声明

#### 函数

```
// 静态
fn<pub, i8> [func_name]() {
    ...
    return [expression: type = i8];
}

// 动态
let [func_name]: fn<u8> = fn() {
    ...
    return [expression: type = u8];
}
```

#### 类

```
class Father: Object {
    let age: i8 = 18;
    fn<pub> say() {
        print("i am father, i am $age years old");
    }
}

class Son: Father {
    fn<pub, override> say() {
        print("i am son, i am $age years old");
    }
}
```
