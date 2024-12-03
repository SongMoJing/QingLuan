# 青鸾

> 青鸾语言的初衷是成为一个高效、易读写的脚本语言，以应付庞大的数据处理。

## 基本语法

### 标记

> 可以使用以`$`作为开头的键值对作为标记，以明确解释器应该做什么

|     键     |  值   | 信息          |
|:---------:|:----:|-------------|
| `encode`  | 编码格式 | 文件存储的默认编码格式 |
| `decode`  | 解码格式 | 文件读取的默认解码格式 |
| `project` |  路径  | 项目的根目录      |
|    `/`    |      |             |
|    `/`    |      |             |
|    `/`    |      |             |

**例**

```ql
$encode utf8
$ decode gbk
```

### 导包

> 可以使用`import`关键字导入包并使用`as`关键字备注别名以防止重复，备注别名后不可使用原名访问。
>
> 当需要在同一个根包下导入多个子包，可以使用大括号一次性导入。

**例**
导入`org.example`包下的`Base`文件

```ql
import org.example.Base
```

导入`org.example`包下的`Base`文件，取别名为`bs`

```ql
import org.example.Base as bs
```

导入`org.example`根包下的`Base`文件（取别名为`bs`）、和`tool`子包下的`Lib`文件

```ql
import org.example{Base as bs, tool.Lib}
```

### 续行

> 可以在行末尾使用`\`续行，以代表将下一行拼接到该行的末尾，原反斜杠被替换为新行的空格

**例**

```ql
import org.example \
	{Base as bs, tool.Lib}
```

### 变量和常量

使用`var`声明一个变量

```ql
var num = 12
```

使用`val`声明一个常量，该常量在确定一个值后不可更改。

```ql
val NUM = 34
```

### 函数

可以使用`fn`关键字生命一个函数，并在其生命周期内调用

```ql
import ql.lang.type.number as num
fn add(val num1: num, val num2: num) : num {
	return (num1 + num2)
}
```

## 面向对象

### 类和方法

