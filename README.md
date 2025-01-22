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

自己查~自↘己↘
```powershell
PS D:\Projects\Binsic> .\target\release\binsic.exe .\target\release\binsic.exe -r "[a-zA-Z0-9]{8,}" -l 20
V%@@.reloccp&d8&@BUAWAVVWSH
MpAAAALDAIAA L D AI AA0L0D0AI0AA@L@D@AI@AAPLPDPAIPH
]8KS [8C8CHCKHCXC SXL
z#AFxANAV(A^8AfPAFANxA
ofofoJfob foB0foj@forPfo
fofoJfoR foZ0fob@fojPH
o fo*foRfor foJ0foz@foBPfDoB`foZpfDo
@`EpAGHE0M@UP]`A_8AW(AOAGH
JxEiBiEYBYEIBIEMU)]9Z9R)JBH
W@a Display implementation returned an error unexpectedlyC:\Users\xxxx\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\alloc\src\string.rs_E@q{C:\Users\xxxx\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\str\pattern.rs
E@uL$_utf-8iso-8859iso-8859-2iso-8859-3iso-8859-4iso-8859-5iso-8859-6iso-8859-7iso-8859-8iso-8859-10iso-8859-13iso-8859-14iso-8859-15iso-8859-16windows-874未知编码：''
G@请指定字节序，utf-16be / utf-16leBinsicfileThe following required argument was not provided: fileremin_lengthThe following required argument was not provided: min_lengthdecoderThe following required argument was not provided: decoderoutputCliArgumentsFILEPATH目标文件路径RE
)@P>@*@regexREGEX正则表达式MIN_LENGTH (@ (@
'@P>@`*@min-length最小有效长度DECODERENCODING文本编码OUTPUT输出文件路径二进制文件文本探查器，用于寻找二进制文件中可能包 含的文本内容。1.0.0
...
```
