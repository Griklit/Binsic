# Binsic

二进制文件文本探查器，用于寻找二进制文件中可能包含的文本内容。

```bash
Usage: binsic.exe [OPTIONS] <PATH>

Arguments:
  <PATH>  目标文件路径

Options:
  -r, --regex <REGEX>        正则表达式
  -l, --min-length <LENGTH>  最小有效长度 [default: 10]
  -e, --encoding <ENCODING>  编码方法 [default: utf-8]
  -o, --output <OUTPUT>      输出文件路径
  -h, --help                 Print help
  -V, --version              Print version
```