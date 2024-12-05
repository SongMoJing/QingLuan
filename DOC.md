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

### 注释

```ql
 - 单行注释
import org.example.Base // 这是一条单行注释，自双斜杠开始直到行尾

 - 多行注释
/*
这是多行注释，自斜杠+星号开始直到星号+斜杠结束
*/

 - 文档注释
/**
这是文档注释，自斜杠+双星号开始直到星号+斜杠结束
*/
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

### 表达式和语句

> 一般情况下，我们使用不同的行来区别不同的语句，且语句除复合语句外不会跨越多行
> 
> 我们认为表达式是由操作符和操作数构成的式子，且具有返回值；语句是程序执行的基本单位，可以包含表达式
> 
> 可以在行末尾使用`\`续行，代表将下一行拼接到该行的末尾，原反斜杠被替换为空格，由多行组成一个语句
> 
> 也可以在行尾添加一个`;`，后接新语句以表示在一行内执行多条语句

**例**

```ql
import org.example \
	{Base as bs, tool.Lib}

import org.examle.Base;import org.example.tool.Lib
// 等同于
import org.examle.Base
import org.example.tool.Lib
```

### 变量

量的声明默认不可为空`null`，除非显式声明为`nullable`

使用`let`声明一个变量

```ql
// 声明一个不可为空的变量
let NUM: int = 12

// 不可赋值
NUM = null

// 声明一个可为空的变量
let num ? = null
// 或者 `let num: int ? = null`

// 检查num变量是否为空，为空则赋值 0
num ?= 0

// 重用变量
let num = "hello world"

// 回收变量，释放其空间
recycle (num, NUM)
```

### 复合语句

> 复合语句是包含多个语句的语句，如条件语句和循环语句等

#### 条件语句 **if-else**

```ql
if (条件表达式) {
	// 条件为真时执行
} else // 条件为假时执行
```

#### 选择分支语句 **switch**

```ql
switch (条件表达式) {
	值1 : {
		// 值1匹配时执行
	}
	值2 -> // 值2匹配时执行
	_ -> // 值1和值2都不匹配时执行
}
```

#### 循环语句 **for(-each)**

```ql
for (变量 in 集合) {
	// 遍历集合中的元素
}	
```

#### 循环语句 **while**

```ql
while (条件表达式) {
	// 条件为真时执行
}	
```

#### 循环语句 **loop**
```ql
loop {
	// 无限循环，执行完毕或执行contiue语句后继续循环，直到执行break语句后才能跳出循环
}
```

### 函数

可以使用`fn`关键字声明函数，并在其生命周期内调用

```ql
import ql.lang.type.number as num

// 声明一个函数
fn add(val num1: num, val num2: num) : num {
	return (num1 + num2)
}

// 创建一个函数类型的变量
let func: fn(val num, val num | num) = (num1, num2) -> {
	return (num1 + num2)
}
```

## 面向对象

### 类和方法





