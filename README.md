# Binsic

二进制文件文本探查器，用于从二进制文件中寻找可能为字符串的内容。

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

例: `binsic -r [\u4E00-\u9FFF]+ -l 10 -e utf-8 -o output.txt target.bin`