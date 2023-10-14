keyword:global,speak,input,if,exit,loop

identifier:以字母开头，其余可为'_',字母和数字

number：只支持十进制整数和小数

string：字符串 ""包围

';':语句结束符

'(':left_paren

')':right_paren

operator: +,-,*,/,==,=

'{}':用于表示代码块的开始与结束

'""':字符串

Note:

本dsl仅支持单行注释 使用#

BNF:

program := globalvariable* mainloop

globalvariable := "global" identifier "=" (String|Number) ";"

mainloop := "loop" "{" expr* "}"

expr := "speak" expr ";"
     |  "input" identifier ";"
     |  "if" "(" expr ")" "{" expr "}"  ";"
     |  expr + expr
     |  expr − expr
     |  expr ∗ expr
     |  expr / expr
     |  expr = expr
     |  expr == expr
     |  (expr)
     |  Number
     |  String
     |  identifier
     |  "exit"

     

identifier := (letter|"_") (letter|digit|"_")*

String := \" [^"] \"

Number := digit+(\.digit+)?

digit := [0-9]