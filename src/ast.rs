use crate::tokens::Token;
use std::fmt;

/*
 * 语法树中表达式的枚举类型，方便递归下降分析
 * 共有如下表达式：
 * - 变量
 * - 二元表达式
 * - 字面量
 * - 赋值语句
 */

 #[derive(Debug, Clone,PartialEq)]
pub enum Expr {
    /* 赋值表达式 example: a=b+c */
    Assign {
        name: String,
        value: Box<Expr>,
    },
    /* 二元表达式 example: b+c */
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    /* 字面量 string或number的值 */
    Literal {
        value: LiteralValue,
    },
    /* example input x 中的x */
    Variable {
        name: String,
    },
}

/*
 * 字面量枚举类型，作为表达式中的字面量类型使用
 * 共有如下类型：
 * - 数字
 * - 字符串
 */

 #[derive(Debug, Clone,PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Number(n) => write!(f, "{}", n),
            LiteralValue::String(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/*
 * 语法树中语句的枚举类型
 * 共有如下语句：
 * - 表达式语句
 * - 打印语句(speak ...)
 * - 变量声明语句(global id=value)
 * - 输入语句(input id)
 * - 循环语句(loop{ code })
 * - 条件语句(if (expr) func();)
 * - 函数声明语句(fn id(){code})
 * - 退出语句(exit;)
 */

 #[derive(Debug, Clone,PartialEq)]
pub enum Statement {
    /* 块语句 */
    Block {
        statements: Vec<Box<Statement>>,
    },
    /* 表达式语句 */
    Expression {
        expression: Box<Expr>,
    },
    /* 分支语句 */
    Branch {
        /// 分支语句中的条件表达式
        condition: Box<Expr>,
        /// 分支语句中的执行语句
        then: Box<Statement>,
    },
    /* mainloop 中的语句 */
    Loop {
        body: Box<Statement>,
    },
    /* 打印语句 */
    Speak {
        expression: Box<Expr>,
    },
    /* 输入字符串语句 */
    Input {
        /* 输入字符串语句中的变量名 */
        input: String,
    },
    /*  变量声明语句  */
    Var {
        /* 变量声明语句中的变量名 */
        name: String,
        /* 变量声明语句中的变量值 */
        init: Box<Expr>,
    },
    /// 退出语句
    Exit,
}
